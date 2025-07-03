use crate::numeric::Numeric;
use crate::operator::{BinaryOperator, NullaryOperator, TrinaryOperator, UnaryOperator};
use std::marker::PhantomData;

pub trait TermContext {
    type Term: Clone;
    fn term_nullary(&mut self, nullary: NullaryOperator) -> Self::Term;
    fn term_unary(&mut self, unary: UnaryOperator, t1: Self::Term) -> Self::Term;
    fn term_binary(&mut self, binary: BinaryOperator, t1: Self::Term, t2: Self::Term)
    -> Self::Term;
    fn term_trinary(
        &mut self,
        trinary: TrinaryOperator,
        t1: Self::Term,
        t2: Self::Term,
        t3: Self::Term,
    ) -> Self::Term;
}

pub struct NumericContext<T> {
    inputs: Vec<T>,
}

impl<T> NumericContext<T> {
    pub fn new(inputs: Vec<T>) -> Self {
        NumericContext { inputs }
    }
}

impl<T: Numeric> TermContext for NumericContext<T> {
    type Term = T;

    fn term_nullary(&mut self, nullary: NullaryOperator) -> Self::Term {
        match nullary {
            NullaryOperator::Constant(constant) => T::from_f64(constant.into_inner()),
            NullaryOperator::Variable(variable) => self.inputs[variable].clone(),
        }
    }

    fn term_unary(&mut self, unary: UnaryOperator, t1: Self::Term) -> Self::Term {
        match unary {
            UnaryOperator::Negate => -t1,
            UnaryOperator::Reciprocal => t1.recip(),
        }
    }

    fn term_binary(
        &mut self,
        binary: BinaryOperator,
        t1: Self::Term,
        t2: Self::Term,
    ) -> Self::Term {
        match binary {
            BinaryOperator::Add => t1 + t2,
            BinaryOperator::Subtract => t1 - t2,
            BinaryOperator::Multiply => t1 * t2,
            BinaryOperator::Divide => t1 / t2,
            BinaryOperator::Min => t1.min(t2),
            BinaryOperator::Max => t1.max(t2),
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
            TrinaryOperator::Piecewise => t1.piecewise(t2, t3),
        }
    }
}
