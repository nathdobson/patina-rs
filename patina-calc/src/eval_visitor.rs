use crate::operator::{OperatorBinary, OperatorNullary, OperatorTrinary, OperatorUnary};
use crate::term_visitor::TermVisitor;
use patina_scalar::Scalar;

/// A visitor for numerically evaluating terms.
pub struct EvalVisitor<T> {
    inputs: Vec<T>,
}

impl<T> EvalVisitor<T> {
    pub fn new(inputs: Vec<T>) -> Self {
        EvalVisitor { inputs }
    }
}

impl<T: Scalar> TermVisitor for EvalVisitor<T> {
    type Output = T;

    fn visit_nullary(&mut self, nullary: OperatorNullary) -> Self::Output {
        match nullary {
            OperatorNullary::Constant(constant) => T::from_f64(constant.into_inner()),
            OperatorNullary::Variable(variable) => self.inputs[variable].clone(),
        }
    }

    fn visit_unary(&mut self, unary: OperatorUnary, t1: Self::Output) -> Self::Output {
        match unary {
            OperatorUnary::Negate => -t1,
            OperatorUnary::Reciprocal => t1.recip(),
            OperatorUnary::Sqrt => t1.sqrt(),
            OperatorUnary::Identity => t1
        }
    }

    fn visit_binary(
        &mut self,
        binary: OperatorBinary,
        t1: Self::Output,
        t2: Self::Output,
    ) -> Self::Output {
        match binary {
            OperatorBinary::Add => t1 + t2,
            OperatorBinary::Subtract => t1 - t2,
            OperatorBinary::Multiply => t1 * t2,
            OperatorBinary::Divide => t1 / t2,
            OperatorBinary::Minimum => t1.minimum(t2),
            OperatorBinary::Maximum => t1.maximum(t2),
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
            OperatorTrinary::Piecewise => t1.piecewise(t2, t3),
        }
    }
}
