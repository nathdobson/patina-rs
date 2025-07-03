use crate::expr::{Expr, ExprInner};
use crate::memoize::Memoize;
use crate::operator::{BinaryOperator, NullaryOperator, TrinaryOperator, UnaryOperator};
use crate::term_context::TermContext;

pub struct ExprContext {
    table: Memoize<ExprInner, Expr>,
}

impl ExprContext {
    pub fn new() -> Self {
        ExprContext {
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

impl TermContext for ExprContext {
    type Term = Expr;

    fn term_nullary(&mut self, nullary: NullaryOperator) -> Self::Term {
        self.push(ExprInner::Nullary(nullary, []))
    }

    fn term_unary(&mut self, unary: UnaryOperator, t1: Self::Term) -> Self::Term {
        self.push(ExprInner::Unary(unary, [t1]))
    }

    fn term_binary(
        &mut self,
        binary: BinaryOperator,
        t1: Self::Term,
        t2: Self::Term,
    ) -> Self::Term {
        self.push(ExprInner::Binary(binary, [t1, t2]))
    }

    fn term_trinary(
        &mut self,
        trinary: TrinaryOperator,
        t1: Self::Term,
        t2: Self::Term,
        t3: Self::Term,
    ) -> Self::Term {
        self.push(ExprInner::Trinary(trinary, [t1, t2, t3]))
    }
}
