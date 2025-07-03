use crate::deep_equal::DeepEqualExpr;
use crate::operator::{BinaryOperator, NullaryOperator, Operator, TrinaryOperator, UnaryOperator};
use by_address::ByAddress;
use ordered_float::NotNan;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::rc::Rc;

#[derive(Eq, Ord, PartialEq, PartialOrd, Hash, Clone)]
pub struct Expr(ByAddress<Rc<ExprInner>>);

#[derive(Eq, Ord, PartialEq, PartialOrd, Hash, Clone, Debug)]
pub enum ExprInner {
    Nullary(NullaryOperator, [Expr; 0]),
    Unary(UnaryOperator, [Expr; 1]),
    Binary(BinaryOperator, [Expr; 2]),
    Trinary(TrinaryOperator, [Expr; 3]),
}

impl Expr {
    pub fn new(inner: ExprInner) -> Expr {
        Expr(ByAddress(Rc::new(inner)))
    }
    pub fn new_nullary(operator: NullaryOperator) -> Expr {
        Expr(ByAddress(Rc::new(ExprInner::Nullary(operator, []))))
    }
    pub fn new_unary(operator: UnaryOperator, expr: Expr) -> Expr {
        Expr(ByAddress(Rc::new(ExprInner::Unary(operator, [expr]))))
    }
    pub fn new_binary(operator: BinaryOperator, left: Expr, right: Expr) -> Expr {
        Expr(ByAddress(Rc::new(ExprInner::Binary(
            operator,
            [left, right],
        ))))
    }
    pub fn new_trinary(operator: TrinaryOperator, e0: Expr, e1: Expr, e2: Expr) -> Expr {
        Expr(ByAddress(Rc::new(ExprInner::Trinary(
            operator,
            [e0, e1, e2],
        ))))
    }
    pub fn inner(&self) -> &ExprInner {
        &**self.0
    }
    pub fn var(variable: usize) -> Self {
        Expr::new_nullary(NullaryOperator::Variable(variable))
    }
    pub fn recip(self) -> Self {
        Expr::new_unary(UnaryOperator::Reciprocal, self)
    }
    pub fn min(self, other: Self) -> Self {
        Expr::new_binary(BinaryOperator::Min, self, other)
    }
    pub fn max(self, other: Self) -> Self {
        Expr::new_binary(BinaryOperator::Max, self, other)
    }
    pub fn piecewise(self, neg: Self, pos: Self) -> Self {
        Expr::new_trinary(TrinaryOperator::Piecewise, self, neg, pos)
    }
    pub fn compose(&self, inputs: &[Self]) -> Self {
        match self.inner() {
            ExprInner::Nullary(NullaryOperator::Variable(v), []) => inputs[*v].clone(),
            ExprInner::Nullary(op, []) => self.clone(),
            ExprInner::Unary(op, [e0]) => Self::new_unary(op.clone(), e0.compose(inputs)),
            ExprInner::Binary(op, [e0, e1]) => {
                Self::new_binary(op.clone(), e0.compose(inputs), e1.compose(inputs))
            }
            ExprInner::Trinary(op, [e0, e1, e2]) => Self::new_trinary(
                op.clone(),
                e0.compose(inputs),
                e1.compose(inputs),
                e2.compose(inputs),
            ),
        }
    }

    pub fn deep_equals(&self, other: &Self) -> bool {
        DeepEqualExpr::new().eq_expr(self, other)
    }
}

impl From<f64> for Expr {
    fn from(value: f64) -> Self {
        Expr::new_nullary(NullaryOperator::Constant(NotNan::new(value).unwrap()))
    }
}

impl Neg for Expr {
    type Output = Expr;
    fn neg(self) -> Self::Output {
        Expr::new_unary(UnaryOperator::Negate, self)
    }
}

impl Add<Expr> for Expr {
    type Output = Expr;
    fn add(self, rhs: Expr) -> Self::Output {
        Expr::new_binary(BinaryOperator::Add, self, rhs)
    }
}

impl Sub<Expr> for Expr {
    type Output = Expr;
    fn sub(self, rhs: Expr) -> Self::Output {
        Expr::new_binary(BinaryOperator::Subtract, self, rhs)
    }
}

impl Mul<Expr> for Expr {
    type Output = Expr;
    fn mul(self, rhs: Expr) -> Self::Output {
        Expr::new_binary(BinaryOperator::Multiply, self, rhs)
    }
}

impl Div<Expr> for Expr {
    type Output = Expr;
    fn div(self, rhs: Expr) -> Self::Output {
        Expr::new_binary(BinaryOperator::Divide, self, rhs)
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.inner() {
            ExprInner::Nullary(op, []) => write!(f, "{}", op),
            ExprInner::Unary(op, [e0]) => write!(f, "({}{})", op, e0),
            ExprInner::Binary(op, [e0, e1]) => write!(f, "({} {} {})", e0, op, e1),
            ExprInner::Trinary(op, [e0, e1, e2]) => {
                let (token1, token2) = op.tokens();
                write!(f, "({} {} {} {} {})", e0, token1, e1, token2, e2)
            }
        }
    }
}

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.inner().fmt(f)
    }
}
