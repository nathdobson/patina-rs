use crate::numeric::Numeric;
use crate::operator::{OperatorBinary, OperatorNullary, OperatorTrinary, OperatorUnary};
use std::marker::PhantomData;
use crate::Expr;
use crate::Program;

/// Types that can recursively visit a term (e.g. an [Expr] or [Program]).
pub trait TermVisitor {
    type Output: Clone;
    fn visit_nullary(&mut self, nullary: OperatorNullary) -> Self::Output;
    fn visit_unary(&mut self, unary: OperatorUnary, t1: Self::Output) -> Self::Output;
    fn visit_binary(
        &mut self,
        binary: OperatorBinary,
        t1: Self::Output,
        t2: Self::Output,
    ) -> Self::Output;
    fn visit_trinary(
        &mut self,
        trinary: OperatorTrinary,
        t1: Self::Output,
        t2: Self::Output,
        t3: Self::Output,
    ) -> Self::Output;
}
