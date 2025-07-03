use crate::numeric::Numeric;
use crate::term_context::TermContext;
use ordered_float::NotNan;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum Operator {
    Nullary(NullaryOperator),
    Unary(UnaryOperator),
    Binary(BinaryOperator),
}

#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum NullaryOperator {
    Constant(NotNan<f64>),
    Variable(usize),
}

#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum UnaryOperator {
    Negate,
    Reciprocal,
}

#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Min,
    Max,
}

#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum TrinaryOperator {
    Piecewise,
}

impl TrinaryOperator {
    pub fn tokens(&self) -> (&str, &str) {
        match self {
            TrinaryOperator::Piecewise => ("<?", ":"),
        }
    }
}

impl Display for NullaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NullaryOperator::Constant(c) => write!(f, "{}", c),
            NullaryOperator::Variable(v) => write!(f, "x{}", v),
        }
    }
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOperator::Negate => write!(f, "-"),
            UnaryOperator::Reciprocal => write!(f, "1.0/"),
        }
    }
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Subtract => write!(f, "-"),
            BinaryOperator::Multiply => write!(f, "*"),
            BinaryOperator::Divide => write!(f, "/"),
            BinaryOperator::Min => write!(f, "min"),
            BinaryOperator::Max => write!(f, "max"),
        }
    }
}
