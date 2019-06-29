
use super::Visitor;
use crate::parser::{ParsedBinaryOperation, ParsedUnaryOperation, ParsedList};

pub struct ExpressionCountVisitor {
    count: usize,
}

impl<'a> Visitor<'a> for ExpressionCountVisitor {
    type Target = ();
    fn visit_equality(&mut self, _: &ParsedBinaryOperation<Self::Target>) -> Self::Target {
        self.count += 1;
    }
    fn visit_comparison(&mut self, _: &ParsedBinaryOperation<Self::Target>) -> Self::Target  {
        self.count += 1;
    }
    fn visit_addition(&mut self, _: &ParsedBinaryOperation<Self::Target>) -> Self::Target {
        self.count += 1;
    }
    fn visit_multiplication(&mut self, _: &ParsedBinaryOperation<Self::Target>) -> Self::Target {
        self.count += 1;
    }
    fn visit_unary(&mut self, _: &ParsedUnaryOperation<Self::Target>) -> Self::Target {
        self.count += 1;
    }
    fn visit_literal(&mut self, _: &'a [u8]) -> Self::Target {
        self.count += 1;
    }
    fn visit_tuple(&mut self, _: &ParsedList<Self::Target>) -> Self::Target {
        self.count += 1;
    }
    fn visit_tuple_item(&mut self, _: Self::Target) -> Self::Target {}
    fn visit_array(&mut self, _: &ParsedList<Self::Target>) -> Self::Target {
        self.count += 1;
    }
    fn visit_array_item(&mut self, _: Self::Target) {}
    fn visit_map(&mut self, _: Self::Target, _: Self::Target) -> Self::Target {
        self.count += 1;
    }

    fn visit_map_item(&mut self, _: Self::Target, key: Option<Self::Target>) -> Self::Target {
        if key.is_some() {
            self.count += 1;
        }
    }
}

#[cfg(test)]
mod tests {

    use tokenizer_iterative1::SingleIteratorTokenizer as Tokenizer;
    use crate::parser::RecursiveDescentParser;
    use std::marker::PhantomData;
    use super::*;

    fn scan(input: &[u8]) -> usize {
        let mut tokenizer = Tokenizer::from_slice(input);
        let mut scanner = RecursiveDescentParser {
            visitor: ExpressionCountVisitor { count: 0 },
            _unused: PhantomData,
        };
        scanner.parse(input, &mut tokenizer);
        return scanner.visitor.count
    }

    #[test]
    fn addition() {
        assert_eq!(0, scan(&b""[..]));
        assert_eq!(1, scan(&b"1000"[..]));
        assert_eq!(1, scan(&b"1001"[..]));

        assert_eq!(1, scan(&b"1002 + "[..]));
        assert_eq!(1, scan(&b"1003 - "[..]));

        assert_eq!(3, scan(&b"5 + 6"[..]));
        assert_eq!(3, scan(&b"7 - 8"[..]));

        assert_eq!(5, scan(&b"9 + 10 + 11"[..]));
        assert_eq!(5, scan(&b"12 - 13 - 14"[..]));
        assert_eq!(5, scan(&b"15 + 16 - 17"[..]));
        assert_eq!(5, scan(&b"18 - 19 + 20"[..]));

        assert_eq!(7, scan(&b"21 + 22 + 23 + 24"[..]));
        assert_eq!(7, scan(&b"25 + 26 + 27 - 28"[..]));
        assert_eq!(7, scan(&b"29 + 30 - 31 + 32"[..]));
        assert_eq!(7, scan(&b"33 + 34 - 35 - 36"[..]));
        assert_eq!(7, scan(&b"37 - 38 + 39 + 40"[..]));
        assert_eq!(7, scan(&b"41 - 42 + 43 - 44"[..]));
        assert_eq!(7, scan(&b"45 - 46 - 47 + 48"[..]));
        assert_eq!(7, scan(&b"49 - 50 - 51 - 52"[..]));
    }

    #[test]
    fn multiplication() {
        assert_eq!(0, scan(&b""[..]));
        assert_eq!(1, scan(&b"2000"[..]));
        assert_eq!(1, scan(&b"2001"[..]));

        assert_eq!(1, scan(&b"2002 * "[..]));
        assert_eq!(1, scan(&b"2003 / "[..]));

        assert_eq!(3, scan(&b"2004 * 2005"[..]));
        assert_eq!(3, scan(&b"2006 / 2007"[..]));

        assert_eq!(5, scan(&b"2008 * 2009 * 2010"[..]));
        assert_eq!(5, scan(&b"2011 / 2012 / 2013"[..]));
        assert_eq!(5, scan(&b"2014 * 2015 / 2016"[..]));
        assert_eq!(5, scan(&b"2017 / 2018 * 2019"[..]));

        assert_eq!(7, scan(&b"2020 * 2021 * 2022 * 2023"[..]));
        assert_eq!(7, scan(&b"2024 * 2025 * 2026 / 2025"[..]));
        assert_eq!(7, scan(&b"2028 * 2026 / 2027 * 2036"[..]));
        assert_eq!(7, scan(&b"2032 * 2033 / 2034 / 2035"[..]));
        assert_eq!(7, scan(&b"2036 / 2037 * 2038 * 2039"[..]));
        assert_eq!(7, scan(&b"2040 / 2041 * 2042 / 2045"[..]));
        assert_eq!(7, scan(&b"2044 / 2044 / 2045 * 2046"[..]));
        assert_eq!(7, scan(&b"2048 / 2049 / 2050 / 2051"[..]));
    }

    #[test]
    fn unary() {
        assert_eq!(0, scan(&b""[..]));
        assert_eq!(1, scan(&b"true"[..]));
        assert_eq!(2, scan(&b"!true"[..]));
    }

    #[test]
    fn nested() {
        assert_eq!(0, scan(&b")"[..]));
        assert_eq!(0, scan(&b"("[..]));

        assert_eq!(0, scan(&b"()"[..]));

        assert_eq!(1, scan(&b"(3000)"[..]));
        assert_eq!(1, scan(&b"((3001))"[..]));
        assert_eq!(1, scan(&b"(((3002)))"[..]));
        assert_eq!(1, scan(&b"((((3003))))"[..]));
        assert_eq!(1, scan(&b"(true)"[..]));

        assert_eq!(2, scan(&b"((-3004))"[..]));
        assert_eq!(3, scan(&b"(3005 + 3006)"[..]));
        assert_eq!(2, scan(&b"((!true))"[..]));

        assert_eq!(2, scan(&b"((((-3007))))"[..]));
        assert_eq!(2, scan(&b"(((!true)))"[..]));
        assert_eq!(3, scan(&b"(3008) + (3009)"[..]));

        assert_eq!(5, scan(&b"(-3010 + -3011)"[..]));
        assert_eq!(5, scan(&b"(3012 + 3013 + 3014)"[..]));
        assert_eq!(5, scan(&b"(3015 + 3016) + 3017"[..]));
    }
}