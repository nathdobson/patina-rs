use crate::expr::{Expr, ExprInner};
use crate::memoize::Memoize;
use crate::program::ProgramTerm;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub struct ExprDeepEqual {
    table: Memoize<(Expr, Expr), bool>,
}

impl ExprDeepEqual {
    pub fn new() -> Self {
        ExprDeepEqual {
            table: Memoize::new(),
        }
    }
    pub fn eq_expr(&mut self, e1: &Expr, e2: &Expr) -> bool {
        if let Some(result) = self.table.begin((e1.clone(), e2.clone())) {
            return result;
        }
        let result = self.eq_expr_inner(e1.inner(), e2.inner());
        self.table.end(&(e1.clone(), e2.clone()), result)
    }
    fn eq_expr_inner(&mut self, e1: &ExprInner, e2: &ExprInner) -> bool {
        match (e1, e2) {
            (ExprInner::Nullary(op1, es1), ExprInner::Nullary(op2, es2)) => {
                op1 == op2 && self.eq_expr_array(es1, es2)
            }
            (ExprInner::Unary(op1, es1), ExprInner::Unary(op2, es2)) => {
                op1 == op2 && self.eq_expr_array(es1, es2)
            }
            (ExprInner::Binary(op1, es1), ExprInner::Binary(op2, es2)) => {
                op1 == op2 && self.eq_expr_array(es1, es2)
            }
            (ExprInner::Trinary(op1, es1), ExprInner::Trinary(op2, es2)) => {
                op1 == op2 && self.eq_expr_array(es1, es2)
            }
            _ => false,
        }
    }
    fn eq_expr_array(&mut self, es1: &[Expr], es2: &[Expr]) -> bool {
        for (e1, e2) in es1.iter().zip(es2.iter()) {
            if !self.eq_expr(e1, e2) {
                return false;
            }
        }
        true
    }
}
