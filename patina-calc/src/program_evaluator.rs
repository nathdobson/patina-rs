use crate::numeric::Numeric;
use crate::program::{Instruction, Program};
use crate::term_context::TermContext;
use std::env::var;
use std::rc::Rc;

pub struct ProgramEvaluator<C: TermContext> {
    memory: Vec<C::Term>,
}

impl<C: TermContext> ProgramEvaluator<C> {
    pub fn new(program: &Program) -> Self {
        ProgramEvaluator {
            memory: Vec::with_capacity(program.instructions().len()),
        }
    }
    pub fn evaluate(&mut self, program: &Program, context: &mut C, outputs: &mut Vec<C::Term>) {
        self.memory.clear();
        for instruction in program.instructions() {
            let result = match instruction {
                Instruction::Nullary(op, []) => context.term_nullary(op.clone()),
                Instruction::Unary(op, [t0]) => {
                    context.term_unary(op.clone(), self.memory[t0.index()].clone())
                }
                Instruction::Binary(op, [t0, t1]) => context.term_binary(
                    op.clone(),
                    self.memory[t0.index()].clone(),
                    self.memory[t1.index()].clone(),
                ),
                Instruction::Trinary(op, [t0, t1, t2]) => context.term_trinary(
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
