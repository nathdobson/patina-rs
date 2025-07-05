use crate::term_visitor::TermVisitor;
use ordered_float::NotNan;
use std::fmt::{Display, Formatter};

/// An operator that takes 0 inputs.
#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum OperatorNullary {
    Constant(NotNan<f64>),
    Variable(usize),
}

/// An operator that takes 1 input.
#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum OperatorUnary {
    Identity,
    Negate,
    Reciprocal,
    Sqrt,
    Abs,
}

/// An operator that takes 2 inputs.
#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum OperatorBinary {
    Add,
    Subtract,
    Multiply,
    Divide,
    Minimum,
    Maximum,
}

/// An operator that takes 3 inputs.
#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum OperatorTrinary {
    Piecewise,
}

impl OperatorTrinary {
    pub fn tokens(&self) -> (&str, &str) {
        match self {
            OperatorTrinary::Piecewise => ("<?", ":"),
        }
    }
}

impl Display for OperatorNullary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OperatorNullary::Constant(c) => write!(f, "{}", c),
            OperatorNullary::Variable(v) => write!(f, "x{}", v),
        }
    }
}

impl Display for OperatorUnary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OperatorUnary::Negate => write!(f, "-"),
            OperatorUnary::Reciprocal => write!(f, "1.0/"),
            OperatorUnary::Sqrt => write!(f, "sqrt "),
            OperatorUnary::Identity => write!(f, ""),
            OperatorUnary::Abs => write!(f, "abs "),
        }
    }
}

impl Display for OperatorBinary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OperatorBinary::Add => write!(f, "+"),
            OperatorBinary::Subtract => write!(f, "-"),
            OperatorBinary::Multiply => write!(f, "*"),
            OperatorBinary::Divide => write!(f, "/"),
            OperatorBinary::Minimum => write!(f, "min"),
            OperatorBinary::Maximum => write!(f, "max"),
        }
    }
}
