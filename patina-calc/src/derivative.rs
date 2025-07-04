use crate::TermVisitor;
use crate::expr::{Expr, ExprInner};
use crate::operator::{OperatorBinary, OperatorNullary, OperatorTrinary, OperatorUnary};
use crate::program::Program;
use crate::term_visitor::TermVisitorExt;
use ordered_float::NotNan;
use std::ptr::null;

pub struct DerivativeTransform<V> {
    inner: V,
    variable: usize,
}

impl<V> DerivativeTransform<V> {
    pub fn new(inner: V, variable: usize) -> Self {
        DerivativeTransform { inner, variable }
    }
    pub fn into_inner(self) -> V {
        self.inner
    }
}

impl<V: TermVisitor> TermVisitor for DerivativeTransform<V> {
    type Output = (V::Output, V::Output);

    fn visit_nullary(&mut self, nullary: OperatorNullary) -> Self::Output {
        match nullary {
            OperatorNullary::Constant(c) => (
                self.inner.constant(c.into_inner()),
                self.inner.constant(0.0),
            ),
            OperatorNullary::Variable(v) if v == self.variable => {
                (self.inner.var(v), self.inner.constant(1.0))
            }
            OperatorNullary::Variable(v) => (self.inner.var(v), self.inner.constant(0.0)),
        }
    }

    fn visit_unary(&mut self, unary: OperatorUnary, (f, fp): Self::Output) -> Self::Output {
        match unary {
            OperatorUnary::Identity => (f, fp),
            OperatorUnary::Negate => (self.inner.negate(f), self.inner.negate(fp)),
            OperatorUnary::Reciprocal => {
                let nfp = self.inner.negate(fp);
                let f2 = self.inner.mul(f.clone(), f.clone());
                (self.inner.recip(f), self.inner.div(nfp, f2))
            }
            OperatorUnary::Sqrt => {
                let sf = self.inner.sqrt(f);
                let t2 = self.inner.constant(2.0);
                let sf2 = self.inner.mul(t2, sf.clone());
                (sf, self.inner.div(fp, sf2))
            }
        }
    }

    fn visit_binary(
        &mut self,
        binary: OperatorBinary,
        (f, fp): Self::Output,
        (g, gp): Self::Output,
    ) -> Self::Output {
        match binary {
            OperatorBinary::Add => (self.inner.add(f, g), self.inner.add(fp, gp)),
            OperatorBinary::Subtract => (self.inner.sub(f, g), self.inner.sub(fp, gp)),
            OperatorBinary::Multiply => {
                let fpg = self.inner.mul(fp, g.clone());
                let fgp = self.inner.mul(f.clone(), gp);
                (self.inner.mul(f, g), self.inner.add(fpg, fgp))
            }
            OperatorBinary::Divide => todo!(),
            OperatorBinary::Minimum => todo!(),
            OperatorBinary::Maximum => todo!(),
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
            OperatorTrinary::Piecewise => todo!(),
        }
    }
}

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
