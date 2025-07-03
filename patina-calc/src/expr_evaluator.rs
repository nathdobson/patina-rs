use crate::expr::{Expr, ExprInner};
use crate::memoize::Memoize;
use crate::operator::NullaryOperator;
use crate::term_context::TermContext;
use std::collections::HashMap;

pub struct ExprEvaluator<C: TermContext> {
    expr_table: Memoize<Expr, C::Term>,
}

impl<C: TermContext> ExprEvaluator<C> {
    pub fn new() -> Self {
        ExprEvaluator {
            expr_table: Memoize::new(),
        }
    }
}

impl<C: TermContext> ExprEvaluator<C> {
    pub fn evaluate(&mut self, context: &mut C, expr: &Expr) -> C::Term {
        if let Some(result) = self.expr_table.begin(expr.clone()) {
            return result;
        }
        let term = self.evaluate_inner(context, expr.inner());
        self.expr_table.end(expr, term)
    }
    fn evaluate_inner(&mut self, context: &mut C, inner: &ExprInner) -> C::Term {
        match inner {
            ExprInner::Nullary(op, []) => context.term_nullary(op.clone()),
            ExprInner::Unary(op, [e0]) => {
                let e0 = self.evaluate(context, e0).clone();
                context.term_unary(op.clone(), e0)
            }
            ExprInner::Binary(op, [e0, e1]) => {
                let e0 = self.evaluate(context, e0).clone();
                let e1 = self.evaluate(context, e1).clone();
                context.term_binary(op.clone(), e0, e1)
            }
            ExprInner::Trinary(op, [e0, e1, e2]) => {
                let e0 = self.evaluate(context, e0).clone();
                let e1 = self.evaluate(context, e1).clone();
                let e2 = self.evaluate(context, e2).clone();
                context.term_trinary(op.clone(), e0, e1, e2)
            }
        }
    }
}
