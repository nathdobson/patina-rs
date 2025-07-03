use crate::expr::{Expr, ExprInner};
use crate::operator::{BinaryOperator, NullaryOperator, TrinaryOperator, UnaryOperator};
use crate::program::{Program};

// impl Expr {
//     pub fn derivative(&self, dv: usize) -> Expr {
//         match self.inner() {
//             ExprInner::Nullary(op, []) => op.derivative(dv, []),
//             ExprInner::Unary(op, [i0]) => op.derivative(dv, [(i0.clone(), i0.derivative(dv))]),
//             ExprInner::Binary(op, [i0, i1]) => op.derivative(
//                 dv,
//                 [
//                     (i0.clone(), i0.derivative(dv)),
//                     (i1.clone(), i1.derivative(dv)),
//                 ],
//             ),
//             ExprInner::Trinary(op, [i0, i1, i2]) => op.derivative(
//                 dv,
//                 [
//                     (i0.clone(), i0.derivative(dv)),
//                     (i1.clone(), i1.derivative(dv)),
//                     (i2.clone(), i2.derivative(dv)),
//                 ],
//             ),
//         }
//     }
// }
//
// impl NullaryOperator {
//     pub fn derivative(&self, dv: usize, []: [(Expr, Expr); 0]) -> Expr {
//         match self {
//             NullaryOperator::Constant(_) => Expr::from(0.0),
//             NullaryOperator::Variable(v) if *v == dv => Expr::from(1.0),
//             NullaryOperator::Variable(_) => Expr::from(0.0),
//         }
//     }
// }
//
// impl UnaryOperator {
//     pub fn derivative(&self, dv: usize, [(f, fp)]: [(Expr, Expr); 1]) -> Expr {
//         match self {
//             UnaryOperator::Negate => -fp,
//             UnaryOperator::Reciprocal => -fp / (f.clone() * f),
//         }
//     }
// }
//
// impl BinaryOperator {
//     pub fn derivative(&self, dv: usize, [(f, fp), (g, gp)]: [(Expr, Expr); 2]) -> Expr {
//         match self {
//             BinaryOperator::Add => fp + gp,
//             BinaryOperator::Subtract => fp - gp,
//             BinaryOperator::Multiply => f * gp + g * fp,
//             BinaryOperator::Divide => g.clone() * fp - f * gp / (g.clone() * g),
//             BinaryOperator::Min => Expr::piecewise(f - g, fp, gp),
//             BinaryOperator::Max => Expr::piecewise(g - f, fp, gp),
//         }
//     }
// }
//
// impl TrinaryOperator {
//     pub fn derivative(&self, dv: usize, [(f, fp), (g, gp), (h, hp)]: [(Expr, Expr); 3]) -> Expr {
//         match self {
//             TrinaryOperator::Piecewise => Expr::piecewise(f, gp, hp),
//         }
//     }
// }
