use crate::Expr;
use crate::Program;
use crate::operator::{OperatorBinary, OperatorNullary, OperatorTrinary, OperatorUnary};
use ordered_float::NotNan;
use std::fmt::Debug;
use std::marker::PhantomData;

/// Types that can recursively visit a term (e.g. an [Expr] or [Program]).
pub trait TermVisitor {
    type Output: Clone + Debug;
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

pub trait TermVisitorExt: TermVisitor {
    fn constant(&mut self, c: f64) -> Self::Output {
        self.visit_nullary(OperatorNullary::Constant(NotNan::new(c).unwrap()))
    }
    fn var(&mut self, index: usize) -> Self::Output {
        self.visit_nullary(OperatorNullary::Variable(index))
    }
    fn identity(&mut self, t: Self::Output) -> Self::Output {
        self.visit_unary(OperatorUnary::Identity, t)
    }
    fn negate(&mut self, t: Self::Output) -> Self::Output {
        self.visit_unary(OperatorUnary::Negate, t)
    }
    fn recip(&mut self, t: Self::Output) -> Self::Output {
        self.visit_unary(OperatorUnary::Reciprocal, t)
    }
    fn sqrt(&mut self, t: Self::Output) -> Self::Output {
        self.visit_unary(OperatorUnary::Sqrt, t)
    }
    fn add(&mut self, t1: Self::Output, t2: Self::Output) -> Self::Output {
        self.visit_binary(OperatorBinary::Add, t1, t2)
    }
    fn sub(&mut self, t1: Self::Output, t2: Self::Output) -> Self::Output {
        self.visit_binary(OperatorBinary::Subtract, t1, t2)
    }
    fn mul(&mut self, t1: Self::Output, t2: Self::Output) -> Self::Output {
        self.visit_binary(OperatorBinary::Multiply, t1, t2)
    }
    fn div(&mut self, t1: Self::Output, t2: Self::Output) -> Self::Output {
        self.visit_binary(OperatorBinary::Divide, t1, t2)
    }
    fn minimum(&mut self, t1: Self::Output, t2: Self::Output) -> Self::Output {
        self.visit_binary(OperatorBinary::Minimum, t1, t2)
    }
    fn maximum(&mut self, t1: Self::Output, t2: Self::Output) -> Self::Output {
        self.visit_binary(OperatorBinary::Maximum, t1, t2)
    }
    fn piecewise(&mut self, t1: Self::Output, t2: Self::Output, t3: Self::Output) -> Self::Output {
        self.visit_trinary(OperatorTrinary::Piecewise, t1, t2, t3)
    }
}

impl<T: ?Sized + TermVisitor> TermVisitorExt for T {}
