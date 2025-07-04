use crate::term_visitor::TermVisitorExt;
use crate::{OperatorBinary, OperatorNullary, OperatorTrinary, OperatorUnary, TermVisitor};

pub struct SubstituteTransformer<V: TermVisitor> {
    inner: V,
    substitutes: Vec<V::Output>,
}

impl<V: TermVisitor> SubstituteTransformer<V> {
    pub fn new(inner: V, substitutes: Vec<V::Output>) -> Self {
        SubstituteTransformer { inner, substitutes }
    }
    pub fn into_inner(self) -> V {
        self.inner
    }
}

impl<V: TermVisitor> TermVisitor for SubstituteTransformer<V> {
    type Output = V::Output;

    fn visit_nullary(&mut self, nullary: OperatorNullary) -> Self::Output {
        match nullary {
            OperatorNullary::Constant(c) => self.inner.constant(c.into_inner()),
            OperatorNullary::Variable(v) => self.substitutes[v].clone(),
        }
    }

    fn visit_unary(&mut self, unary: OperatorUnary, t1: Self::Output) -> Self::Output {
        self.inner.visit_unary(unary, t1)
    }

    fn visit_binary(
        &mut self,
        binary: OperatorBinary,
        t1: Self::Output,
        t2: Self::Output,
    ) -> Self::Output {
        self.inner.visit_binary(binary, t1, t2)
    }

    fn visit_trinary(
        &mut self,
        trinary: OperatorTrinary,
        t1: Self::Output,
        t2: Self::Output,
        t3: Self::Output,
    ) -> Self::Output {
        self.inner.visit_trinary(trinary, t1, t2, t3)
    }
}
