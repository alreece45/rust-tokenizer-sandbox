use std::marker::PhantomData;
use std::ops::Range;
use std::process::exit;

use tokenizer_common::{is_start_prefix, is_token};

pub mod visitor;

use visitor::Visitor;

struct ParsedBinaryOperation<T = ()> {
    op: &'static str,
    left: T,
    right: T,
}
struct ParsedUnaryOperation<T = ()> {
    op: &'static str,
    expr: T,
}
struct ParsedList<T = ()> {
    first: T,
    last: Option<T>,
    size: usize,
}

pub struct RecursiveDescentParser<'a, V>
where
    V: Visitor<'a>,
{
    visitor: V,
    _unused: PhantomData<&'a V>,
}

impl<'a, V> RecursiveDescentParser<'a, V>
where
    V: Visitor<'a>,
{
    pub fn with_visitor(visitor: V) -> Self {
        RecursiveDescentParser {
            visitor,
            _unused: PhantomData,
        }
    }
}

type ParserWarning = &'static str;
type ParserError = &'static str;

pub struct ParserState<'a, T>
where
    T: Iterator<Item = &'a [u8]>,
{
    source: &'a [u8],
    next_token: Option<&'a [u8]>,
    token_iterator: T,
    warnings: Vec<ParserWarning>,
    errors: Vec<ParserError>,
}

impl<'a, T> ParserState<'a, T>
where
    T: Iterator<Item = &'a [u8]>,
{
    fn advance(&mut self) -> Option<&'a [u8]> {
        let current = self.next_token;
        self.next_token = self.token_iterator.next();
        current
    }
}

impl<'a, V> RecursiveDescentParser<'a, V>
where
    V: Visitor<'a>,
{
    pub fn parse<I>(&mut self, source: &'a [u8], iterator: &mut I) -> Option<V::Target>
    where
        I: Iterator<Item = &'a [u8]>,
    {
        let next = iterator.next();
        let mut state = ParserState {
            source,
            token_iterator: iterator,
            next_token: next,
            warnings: vec![],
            errors: vec![],
        };
        self.expr(&mut state)
    }

    fn expr<I>(&mut self, state: &mut ParserState<'a, I>) -> Option<V::Target>
    where
        I: Iterator<Item = &'a [u8]>,
    {
        self.equality(state)
    }

    fn equality<I>(&mut self, state: &mut ParserState<'a, I>) -> Option<V::Target>
    where
        I: Iterator<Item = &'a [u8]>,
    {
        let mut left = self.comparison(state)?;
        loop {
            let op = match state.next_token {
                Some(b"!=") => "!=",
                Some(b"==") => "==",
                _ => return Some(left),
            };

            state.advance();
            match self.comparison(state) {
                Some(right) => {
                    left = self
                        .visitor
                        .visit_equality(&ParsedBinaryOperation { op, left, right })
                }
                None => return Some(left),
            }
        }
    }

    fn comparison<I>(&mut self, state: &mut ParserState<'a, I>) -> Option<V::Target>
    where
        I: Iterator<Item = &'a [u8]>,
    {
        let mut left = self.addition(state)?;
        loop {
            let op = match state.next_token {
                Some(b">") => ">",
                Some(b">=") => ">=",
                Some(b"<") => "<",
                Some(b"<=") => "<=",
                _ => return Some(left),
            };

            state.advance();
            match self.addition(state) {
                Some(right) => {
                    left = self
                        .visitor
                        .visit_comparison(&ParsedBinaryOperation { op, left, right })
                }
                None => return Some(left),
            }
        }
    }

    fn addition<I>(&mut self, state: &mut ParserState<'a, I>) -> Option<V::Target>
    where
        I: Iterator<Item = &'a [u8]>,
    {
        let mut left = self.multiplication(state)?;
        loop {
            let op = match state.next_token {
                Some(b"+") => "+",
                Some(b"-") => "-",
                _ => return Some(left),
            };

            state.advance();
            match self.multiplication(state) {
                Some(right) => {
                    left = self
                        .visitor
                        .visit_addition(&ParsedBinaryOperation { op, left, right })
                }
                None => return Some(left),
            }
        }
    }

    fn multiplication<I>(&mut self, state: &mut ParserState<'a, I>) -> Option<V::Target>
    where
        I: Iterator<Item = &'a [u8]>,
    {
        let mut left = self.unary(state)?;

        loop {
            let op = match state.next_token {
                Some(b"*") => "*",
                Some(b"/") => "/",
                _ => return Some(left),
            };

            state.advance();

            match self.unary(state) {
                Some(right) => {
                    left = self.visitor.visit_multiplication(&ParsedBinaryOperation {
                        op,
                        left,
                        right,
                    })
                }
                None => return Some(left),
            }
        }
    }

    fn unary<I>(&mut self, state: &mut ParserState<'a, I>) -> Option<V::Target>
    where
        I: Iterator<Item = &'a [u8]>,
    {
        let op = match state.next_token {
            Some(b"!") => "!",
            Some(b"-") => "-",
            _ => return self.literal(state),
        };
        state.advance();

        self.literal(state)
            .map(|expr| self.visitor.visit_unary(&ParsedUnaryOperation { op, expr }))
    }

    fn literal<I>(&mut self, state: &mut ParserState<'a, I>) -> Option<V::Target>
    where
        I: Iterator<Item = &'a [u8]>,
    {
        match state.advance() {
            Some(b"") | None => None,
            Some(b"(") => match self.list_items(state, V::visit_tuple_item, &b")"[..]) {
                Some(list) => {
                    if list.last.is_none() {
                        Some(list.first)
                    } else {
                        Some(self.visitor.visit_tuple(&list))
                    }
                }
                None => None,
            },
            Some(b"[") => match self.list_items(state, V::visit_array_item, &b"]"[..]) {
                Some(list) => Some(self.visitor.visit_tuple(&list)),
                None => None,
            },
            Some(literal) => {
                if is_token(literal) {
                    state.errors.push("Unexpected token");
                    None
                } else {
                    Some(self.visitor.visit_literal(literal))
                }
            }
            Some(b")") => {
                state.errors.push("Unexpected ')'");
                None
            }
            Some(b"]") => {
                state.errors.push("Unexpected ']'");
                None
            }
            Some(b">") => {
                state.errors.push("Unexpected ']'");
                None
            }
            _ => None,
        }
    }

    fn list_items<F, I>(
        &mut self,
        state: &mut ParserState<'a, I>,
        mut item_visitor: F,
        expected_end_token: &'static [u8],
    ) -> Option<ParsedList<V::Target>>
    where
        F: FnMut(&mut V, V::Target) -> V::Target,
        I: Iterator<Item = &'a [u8]>,
    {
        if state
            .next_token
            .map(|v| v == expected_end_token)
            .unwrap_or(false)
        {
            return None;
        }

        let first = self
            .expr(state)
            .map(|expr| item_visitor(&mut self.visitor, expr))?;
        let mut last = None;
        let mut size = 0;

        loop {
            match state.advance() {
                Some(b",") => {
                    match state.next_token {
                        Some(token) if token == expected_end_token => break,
                        _ => (),
                    };
                }
                Some(token) if token == expected_end_token => break,
                _ => state.errors.push("Expected , or )"),
            };

            if let Some(result) = self
                .expr(state)
                .map(|expr| item_visitor(&mut self.visitor, expr))
            {
                size += 1;
                last = Some(result);
            } else {
                state.errors.push("Expected expression in list");
                return None;
            }
        }

        Some(ParsedList { first, last, size })
    }
}
