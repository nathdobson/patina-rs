use crate::eval_visitor::EvalVisitor;
use crate::expr::Expr;
use crate::operator::{OperatorBinary, OperatorNullary, OperatorTrinary, OperatorUnary};
use crate::program::Program;
use crate::term_visitor::TermVisitor;
use inari::DecInterval;
use patina_scalar::Scalar;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct BoundedTerm<T> {
    bounds: DecInterval,
    inner: T,
}

pub struct BoundedTermVisitor<C> {
    bounds: EvalVisitor<DecInterval>,
    inner: C,
}

impl<C> BoundedTermVisitor<C> {
    pub fn new(inner: C, bounds: Vec<DecInterval>) -> Self {
        BoundedTermVisitor {
            inner,
            bounds: EvalVisitor::new(bounds),
        }
    }
}

impl<T> BoundedTerm<T> {
    pub fn new(bounds: DecInterval, inner: T) -> BoundedTerm<T> {
        BoundedTerm { bounds, inner }
    }
    pub fn bounds(&self) -> DecInterval {
        self.bounds
    }
    pub fn inner(&self) -> &T {
        &self.inner
    }
}

impl<C: TermVisitor> TermVisitor for BoundedTermVisitor<C> {
    type Output = BoundedTerm<C::Output>;

    fn visit_nullary(&mut self, nullary: OperatorNullary) -> Self::Output {
        match nullary {
            OperatorNullary::Constant(_) | OperatorNullary::Variable(_) => BoundedTerm::new(
                self.bounds.visit_nullary(nullary.clone()),
                self.inner.visit_nullary(nullary),
            ),
        }
    }

    fn visit_unary(&mut self, unary: OperatorUnary, t1: Self::Output) -> Self::Output {
        match unary {
            OperatorUnary::Negate
            | OperatorUnary::Reciprocal
            | OperatorUnary::Sqrt
            | OperatorUnary::Identity => BoundedTerm::new(
                self.bounds.visit_unary(unary.clone(), t1.bounds),
                self.inner.visit_unary(unary, t1.inner),
            ),
        }
    }

    fn visit_binary(
        &mut self,
        binary: OperatorBinary,
        t1: Self::Output,
        t2: Self::Output,
    ) -> Self::Output {
        match binary {
            OperatorBinary::Minimum if t1.bounds.precedes(t2.bounds) => t1,
            OperatorBinary::Minimum if t2.bounds.precedes(t1.bounds) => t2,
            OperatorBinary::Maximum if t1.bounds.precedes(t2.bounds) => t2,
            OperatorBinary::Maximum if t2.bounds.precedes(t1.bounds) => t1,
            OperatorBinary::Add
            | OperatorBinary::Subtract
            | OperatorBinary::Multiply
            | OperatorBinary::Divide
            | OperatorBinary::Minimum
            | OperatorBinary::Maximum => BoundedTerm::new(
                self.bounds
                    .visit_binary(binary.clone(), t1.bounds, t2.bounds),
                self.inner.visit_binary(binary, t1.inner, t2.inner),
            ),
        }
    }

    fn visit_trinary(
        &mut self,
        trinary: OperatorTrinary,
        t1: Self::Output,
        t2: Self::Output,
        t3: Self::Output,
    ) -> Self::Output {
        match trinary {
            OperatorTrinary::Piecewise if t1.bounds.precedes(DecInterval::from_f64(0.0)) => t2,
            OperatorTrinary::Piecewise if DecInterval::from_f64(0.0).precedes(t1.bounds) => t3,
            OperatorTrinary::Piecewise => BoundedTerm::new(
                self.bounds
                    .visit_trinary(trinary.clone(), t1.bounds, t2.bounds, t3.bounds),
                self.inner
                    .visit_trinary(trinary, t1.inner, t2.inner, t3.inner),
            ),
        }
    }
}

impl Program {
    pub fn constrain(&self, bounds: Vec<DecInterval>) -> Program {
        let mut new_program = Program::new();
        let mut context = BoundedTermVisitor::new(new_program, bounds);
        for output in self.visit::<BoundedTermVisitor<Program>>(&mut context) {
            context.inner.push_output(output.inner);
        }
        context.inner
    }
}
