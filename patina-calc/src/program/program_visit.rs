use crate::numeric::Numeric;
use crate::program::{ProgramStep, Program};
use crate::term_visitor::TermVisitor;
use std::env::var;
use std::rc::Rc;

/// Allows a [TermVisitor] to visit a [Program].
pub struct ProgramVisit<C: TermVisitor> {
    memory: Vec<C::Output>,
}

impl<C: TermVisitor> ProgramVisit<C> {
    pub fn new(program: &Program) -> Self {
        ProgramVisit {
            memory: Vec::with_capacity(program.steps().len()),
        }
    }
    pub fn evaluate(&mut self, program: &Program, context: &mut C, outputs: &mut Vec<C::Output>) {
        self.memory.clear();
        for instruction in program.steps() {
            let result = match instruction {
                ProgramStep::Nullary(op, []) => context.visit_nullary(op.clone()),
                ProgramStep::Unary(op, [t0]) => {
                    context.visit_unary(op.clone(), self.memory[t0.index()].clone())
                }
                ProgramStep::Binary(op, [t0, t1]) => context.visit_binary(
                    op.clone(),
                    self.memory[t0.index()].clone(),
                    self.memory[t1.index()].clone(),
                ),
                ProgramStep::Trinary(op, [t0, t1, t2]) => context.visit_trinary(
                    op.clone(),
                    self.memory[t0.index()].clone(),
                    self.memory[t1.index()].clone(),
                    self.memory[t2.index()].clone(),
                ),
            };
            self.memory.push(result);
        }
        outputs.clear();
        for output in program.outputs() {
            outputs.push(self.memory[output.index()].clone());
        }
    }
}
