use crate::expr::Expr;
use crate::numeric::Numeric;
use crate::operator::{BinaryOperator, NullaryOperator, TrinaryOperator, UnaryOperator};
use crate::program::Program;
use crate::term_context::{NumericContext, TermContext};
use inari::DecInterval;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::rc::Rc;

#[derive(Clone)]
pub struct BoundedTerm<T> {
    bounds: DecInterval,
    inner: T,
}

pub struct BoundedTermContext<C> {
    bounds: NumericContext<DecInterval>,
    inner: C,
}

impl<C> BoundedTermContext<C> {
    pub fn new(inner: C, bounds: Vec<DecInterval>) -> Self {
        BoundedTermContext {
            inner,
            bounds: NumericContext::new(bounds),
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

impl<C: TermContext> TermContext for BoundedTermContext<C> {
    type Term = BoundedTerm<C::Term>;

    fn term_nullary(&mut self, nullary: NullaryOperator) -> Self::Term {
        match nullary {
            NullaryOperator::Constant(_) | NullaryOperator::Variable(_) => BoundedTerm::new(
                self.bounds.term_nullary(nullary.clone()),
                self.inner.term_nullary(nullary),
            ),
        }
    }

    fn term_unary(&mut self, unary: UnaryOperator, t1: Self::Term) -> Self::Term {
        match unary {
            UnaryOperator::Negate | UnaryOperator::Reciprocal => BoundedTerm::new(
                self.bounds.term_unary(unary.clone(), t1.bounds),
                self.inner.term_unary(unary, t1.inner),
            ),
        }
    }

    fn term_binary(
        &mut self,
        binary: BinaryOperator,
        t1: Self::Term,
        t2: Self::Term,
    ) -> Self::Term {
        match binary {
            BinaryOperator::Min if t1.bounds.precedes(t2.bounds) => t1,
            BinaryOperator::Min if t2.bounds.precedes(t1.bounds) => t2,
            BinaryOperator::Max if t1.bounds.precedes(t2.bounds) => t2,
            BinaryOperator::Max if t2.bounds.precedes(t1.bounds) => t1,
            BinaryOperator::Add
            | BinaryOperator::Subtract
            | BinaryOperator::Multiply
            | BinaryOperator::Divide
            | BinaryOperator::Min
            | BinaryOperator::Max => BoundedTerm::new(
                self.bounds
                    .term_binary(binary.clone(), t1.bounds, t2.bounds),
                self.inner.term_binary(binary, t1.inner, t2.inner),
            ),
        }
    }

    fn term_trinary(
        &mut self,
        trinary: TrinaryOperator,
        t1: Self::Term,
        t2: Self::Term,
        t3: Self::Term,
    ) -> Self::Term {
        match trinary {
            TrinaryOperator::Piecewise if t1.bounds.precedes(DecInterval::from_f64(0.0)) => t2,
            TrinaryOperator::Piecewise if DecInterval::from_f64(0.0).precedes(t1.bounds) => t3,
            TrinaryOperator::Piecewise => BoundedTerm::new(
                self.bounds
                    .term_trinary(trinary.clone(), t1.bounds, t2.bounds, t3.bounds),
                self.inner
                    .term_trinary(trinary, t1.inner, t2.inner, t3.inner),
            ),
        }
    }
}

impl Program {
    pub fn constrain(&self, bounds: Vec<DecInterval>) -> Program {
        let mut new_program = Program::new();
        let mut context = BoundedTermContext::new(new_program, bounds);
        for output in self.evaluate::<BoundedTermContext<Program>>(&mut context) {
            context.inner.push_output(output.inner);
        }
        context.inner
    }
}
