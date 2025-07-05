use crate::derivative::DerivativeTransform;
use crate::operator::{OperatorBinary, OperatorNullary, OperatorTrinary, OperatorUnary};
use crate::{ExprVisit, ExprVisitor, TermVisitor};
use by_address::ByAddress;
use deep_equal::ExprDeepEqual;
use ordered_float::NotNan;
use patina_scalar::Scalar;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::rc::Rc;

pub mod deep_equal;
pub mod expr_visit;
pub mod expr_visitor;

/// A "tree-like" representation of terms.
#[derive(Eq, Ord, PartialEq, PartialOrd, Hash, Clone)]
pub struct Expr(ByAddress<Rc<ExprInner>>);

/// The inner representation of an [Expr].
#[derive(Eq, Ord, PartialEq, PartialOrd, Hash, Clone, Debug)]
pub enum ExprInner {
    Nullary(OperatorNullary, [Expr; 0]),
    Unary(OperatorUnary, [Expr; 1]),
    Binary(OperatorBinary, [Expr; 2]),
    Trinary(OperatorTrinary, [Expr; 3]),
}

impl Expr {
    pub fn new(inner: ExprInner) -> Expr {
        Expr(ByAddress(Rc::new(inner)))
    }
    pub fn new_nullary(operator: OperatorNullary) -> Expr {
        Expr(ByAddress(Rc::new(ExprInner::Nullary(operator, []))))
    }
    pub fn new_unary(operator: OperatorUnary, expr: Expr) -> Expr {
        Expr(ByAddress(Rc::new(ExprInner::Unary(operator, [expr]))))
    }
    pub fn new_binary(operator: OperatorBinary, left: Expr, right: Expr) -> Expr {
        Expr(ByAddress(Rc::new(ExprInner::Binary(
            operator,
            [left, right],
        ))))
    }
    pub fn new_trinary(operator: OperatorTrinary, e0: Expr, e1: Expr, e2: Expr) -> Expr {
        Expr(ByAddress(Rc::new(ExprInner::Trinary(
            operator,
            [e0, e1, e2],
        ))))
    }
    pub fn constant(x: f64) -> Self {
        Expr::from_f64(x)
    }
    pub fn inner(&self) -> &ExprInner {
        &**self.0
    }
    pub fn var(variable: usize) -> Self {
        Expr::new_nullary(OperatorNullary::Variable(variable))
    }
    pub fn compose(&self, inputs: &[Self]) -> Self {
        match self.inner() {
            ExprInner::Nullary(OperatorNullary::Variable(v), []) => inputs[*v].clone(),
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
        ExprDeepEqual::new().eq_expr(self, other)
    }
    pub fn visit<V: TermVisitor>(&self, visitor: &mut V) -> V::Output {
        let mut visit = ExprVisit::new();
        visit.visit(visitor, self)
    }
    pub fn derivative(&self, var: usize) -> Self {
        let output = ExprVisitor::new();
        let mut derivative = DerivativeTransform::new(output, var);
        let (f, fp) = self.visit(&mut derivative);
        fp
    }
}

impl Neg for Expr {
    type Output = Expr;
    fn neg(self) -> Self::Output {
        Expr::new_unary(OperatorUnary::Negate, self)
    }
}

impl Add<Expr> for Expr {
    type Output = Expr;
    fn add(self, rhs: Expr) -> Self::Output {
        Expr::new_binary(OperatorBinary::Add, self, rhs)
    }
}

impl Sub<Expr> for Expr {
    type Output = Expr;
    fn sub(self, rhs: Expr) -> Self::Output {
        Expr::new_binary(OperatorBinary::Subtract, self, rhs)
    }
}

impl Mul<Expr> for Expr {
    type Output = Expr;
    fn mul(self, rhs: Expr) -> Self::Output {
        Expr::new_binary(OperatorBinary::Multiply, self, rhs)
    }
}

impl Div<Expr> for Expr {
    type Output = Expr;
    fn div(self, rhs: Expr) -> Self::Output {
        Expr::new_binary(OperatorBinary::Divide, self, rhs)
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

impl AddAssign<Expr> for Expr {
    fn add_assign(&mut self, rhs: Expr) {
        *self = self.clone() + rhs;
    }
}

impl SubAssign<Expr> for Expr {
    fn sub_assign(&mut self, rhs: Expr) {
        *self = self.clone() - rhs;
    }
}

impl MulAssign<Expr> for Expr {
    fn mul_assign(&mut self, rhs: Expr) {
        *self = self.clone() * rhs;
    }
}

impl DivAssign<Expr> for Expr {
    fn div_assign(&mut self, rhs: Expr) {
        *self = self.clone() / rhs;
    }
}

impl Scalar for Expr {
    fn recip(self) -> Self {
        Expr::new_unary(OperatorUnary::Reciprocal, self)
    }

    fn minimum(self, other: Self) -> Self {
        Expr::new_binary(OperatorBinary::Minimum, self, other)
    }

    fn maximum(self, other: Self) -> Self {
        Expr::new_binary(OperatorBinary::Maximum, self, other)
    }

    fn piecewise(self, neg: Self, pos: Self) -> Self {
        Expr::new_trinary(OperatorTrinary::Piecewise, self, neg, pos)
    }

    fn from_f64(value: f64) -> Self {
        Expr::new_nullary(OperatorNullary::Constant(NotNan::new(value).unwrap()))
    }

    fn sqrt(self) -> Self {
        Expr::new_unary(OperatorUnary::Sqrt, self)
    }

    fn abs(self) -> Self {
        Expr::new_unary(OperatorUnary::Abs, self)
    }
}
