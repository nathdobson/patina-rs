use crate::expr::{Expr, ExprInner};
use crate::memoize::Memoize;
use crate::operator::{OperatorBinary, OperatorNullary, OperatorTrinary, OperatorUnary};
use crate::term_visitor::TermVisitor;

/// A [TermVisitor] that builds an [Expr] with guaranteed deduplication of subterms.
pub struct ExprVisitor {
    table: Memoize<ExprInner, Expr>,
}

impl ExprVisitor {
    pub fn new() -> Self {
        ExprVisitor {
            table: Memoize::new(),
        }
    }
    pub fn push(&mut self, inner: ExprInner) -> Expr {
        if let Some(expr) = self.table.begin(inner.clone()) {
            return expr;
        }
        let expr = Expr::new(inner.clone());
        self.table.end(&inner, expr)
    }
}

impl TermVisitor for ExprVisitor {
    type Output = Expr;

    fn visit_nullary(&mut self, nullary: OperatorNullary) -> Self::Output {
        self.push(ExprInner::Nullary(nullary, []))
    }

    fn visit_unary(&mut self, unary: OperatorUnary, t1: Self::Output) -> Self::Output {
        self.push(ExprInner::Unary(unary, [t1]))
    }

    fn visit_binary(
        &mut self,
        binary: OperatorBinary,
        t1: Self::Output,
        t2: Self::Output,
    ) -> Self::Output {
        self.push(ExprInner::Binary(binary, [t1, t2]))
    }

    fn visit_trinary(
        &mut self,
        trinary: OperatorTrinary,
        t1: Self::Output,
        t2: Self::Output,
        t3: Self::Output,
    ) -> Self::Output {
        self.push(ExprInner::Trinary(trinary, [t1, t2, t3]))
    }
}
