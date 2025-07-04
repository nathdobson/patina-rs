use crate::program::{ProgramStep, Program};

impl Program {
    pub fn without_dead_code(&self) -> Program {
        let mut live = vec![false; self.steps().len()];
        for output in self.outputs().iter() {
            live[output.index()] = true;
        }
        for (index, instruction) in self.steps().iter().enumerate().rev() {
            if live[index] {
                for inputs in instruction.inputs() {
                    live[inputs.index()] = true;
                }
            }
        }
        let mut index_table = vec![None; live.len()];
        let mut program = Program::new();
        for i in 0..live.len() {
            if live[i] {
                let new_instruction = match &self.steps()[i] {
                    ProgramStep::Nullary(op, []) => ProgramStep::Nullary(op.clone(), []),
                    ProgramStep::Unary(op, [i0]) => {
                        ProgramStep::Unary(op.clone(), [index_table[i0.index()].unwrap()])
                    }
                    ProgramStep::Binary(op, [i0, i1]) => ProgramStep::Binary(
                        op.clone(),
                        [
                            index_table[i0.index()].unwrap(),
                            index_table[i1.index()].unwrap(),
                        ],
                    ),
                    ProgramStep::Trinary(op, [i0, i1, i2]) => ProgramStep::Trinary(
                        op.clone(),
                        [
                            index_table[i0.index()].unwrap(),
                            index_table[i1.index()].unwrap(),
                            index_table[i2.index()].unwrap(),
                        ],
                    ),
                };
                index_table[i] = Some(program.push(new_instruction));
            }
        }
        for output in self.outputs().iter() {
            program.push_output(index_table[output.index()].unwrap())
        }
        program
    }
}
