use crate::expr::Expr;
use crate::expr::expr_visit::ExprVisit;
use crate::program::{Program, ProgramTerm};

/// A builder for converting a sequence of [Expr]s to a [Program].
pub struct ExprProgramBuilder {
    expr_evaluator: ExprVisit<Program>,
    program: Program,
}

impl ExprProgramBuilder {
    pub fn new() -> ExprProgramBuilder {
        ExprProgramBuilder {
            expr_evaluator: ExprVisit::new(),
            program: Program::new(),
        }
    }
    pub fn push(&mut self, expr: Expr) {
        let term = self.expr_evaluator.visit(&mut self.program, &expr);
        self.program.push_output(term);
    }
    pub fn into_program(self) -> Program {
        self.program
    }
    pub fn program(&self) -> &Program {
        &self.program
    }
}

impl From<Expr> for Program {
    fn from(expr: Expr) -> Self {
        let mut expr_program = ExprProgramBuilder::new();
        expr_program.push(expr);
        expr_program.into_program()
    }
}
