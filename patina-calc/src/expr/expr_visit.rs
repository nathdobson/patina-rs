use crate::expr::{Expr, ExprInner};
use crate::memoize::Memoize;
use crate::operator::OperatorNullary;
use crate::term_visitor::TermVisitor;
use std::collections::HashMap;

/// Allows a [TermVisitor] to visit an [Expr].
pub struct ExprVisit<C: TermVisitor> {
    expr_table: Memoize<Expr, C::Output>,
}

impl<C: TermVisitor> ExprVisit<C> {
    pub fn new() -> Self {
        ExprVisit {
            expr_table: Memoize::new(),
        }
    }
}

impl<C: TermVisitor> ExprVisit<C> {
    pub fn visit(&mut self, context: &mut C, expr: &Expr) -> C::Output {
        if let Some(result) = self.expr_table.begin(expr.clone()) {
            return result;
        }
        let term = self.visit_inner(context, expr.inner());
        self.expr_table.end(expr, term)
    }
    fn visit_inner(&mut self, context: &mut C, inner: &ExprInner) -> C::Output {
        match inner {
            ExprInner::Nullary(op, []) => context.visit_nullary(op.clone()),
            ExprInner::Unary(op, [e0]) => {
                let e0 = self.visit(context, e0).clone();
                context.visit_unary(op.clone(), e0)
            }
            ExprInner::Binary(op, [e0, e1]) => {
                let e0 = self.visit(context, e0).clone();
                let e1 = self.visit(context, e1).clone();
                context.visit_binary(op.clone(), e0, e1)
            }
            ExprInner::Trinary(op, [e0, e1, e2]) => {
                let e0 = self.visit(context, e0).clone();
                let e1 = self.visit(context, e1).clone();
                let e2 = self.visit(context, e2).clone();
                context.visit_trinary(op.clone(), e0, e1, e2)
            }
        }
    }
}
