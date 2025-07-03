use crate::expr::Expr;
use crate::expr_evaluator::ExprEvaluator;
use crate::program::{Program, ProgramTerm};

pub struct ExprProgram {
    expr_evaluator: ExprEvaluator<Program>,
    program: Program,
}

impl ExprProgram {
    pub fn new() -> ExprProgram {
        ExprProgram {
            expr_evaluator: ExprEvaluator::new(),
            program: Program::new(),
        }
    }
    pub fn push(&mut self, expr: Expr) {
        let term = self.expr_evaluator.evaluate(&mut self.program, &expr);
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
        let mut expr_program = ExprProgram::new();
        expr_program.push(expr);
        expr_program.into_program()
    }
}
