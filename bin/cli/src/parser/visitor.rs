
mod expression_count;
mod index_expr;

use crate::parser::{ParsedBinaryOperation, ParsedUnaryOperation, ParsedList};

pub trait Visitor<'a> {
    type Target;
    fn visit_equality(&mut self, op: &ParsedBinaryOperation<Self::Target>) -> Self::Target;
    fn visit_comparison(&mut self, op: &ParsedBinaryOperation<Self::Target>) -> Self::Target;
    fn visit_addition(&mut self, op: &ParsedBinaryOperation<Self::Target>) -> Self::Target;
    fn visit_multiplication(&mut self, op: &ParsedBinaryOperation<Self::Target>) -> Self::Target;
    fn visit_unary(&mut self, op: &ParsedUnaryOperation<Self::Target>) -> Self::Target;
    fn visit_literal(&mut self, literal: &'a [u8]) -> Self::Target;

    fn visit_tuple(&mut self, list: &ParsedList<Self::Target>) -> Self::Target;
    fn visit_tuple_item(&mut self, item: Self::Target) -> Self::Target;
    fn visit_array(&mut self, list: &ParsedList<Self::Target>) -> Self::Target;
    fn visit_array_item(&mut self, item: Self::Target) -> Self::Target;
    fn visit_map(&mut self, start: Self::Target, end: Self::Target) -> Self::Target;
    fn visit_map_item(&mut self, value: Self::Target, key: Option<Self::Target>) -> Self::Target;
}