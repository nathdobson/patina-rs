use crate::expr::{Expr, ExprInner};
use crate::expr_context::ExprContext;
use crate::memoize::Memoize;
use crate::numeric::Numeric;
use crate::operator::{BinaryOperator, NullaryOperator, TrinaryOperator, UnaryOperator};
use crate::program_evaluator::ProgramEvaluator;
use crate::term_context::{NumericContext, TermContext};
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

#[derive(Debug, Clone, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub enum Instruction {
    Nullary(NullaryOperator, [ProgramTerm; 0]),
    Unary(UnaryOperator, [ProgramTerm; 1]),
    Binary(BinaryOperator, [ProgramTerm; 2]),
    Trinary(TrinaryOperator, [ProgramTerm; 3]),
}

#[derive(Debug)]
pub struct Program {
    instructions: Vec<Instruction>,
    instruction_table: HashMap<Instruction, ProgramTerm>,
    outputs: Vec<ProgramTerm>,
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Hash, Copy, Clone)]
pub struct ProgramTerm(usize);

impl Instruction {
    pub fn inputs(&self) -> &[ProgramTerm] {
        match self {
            Instruction::Nullary(_, inputs) => inputs,
            Instruction::Unary(_, inputs) => inputs,
            Instruction::Binary(_, inputs) => inputs,
            Instruction::Trinary(_, inputs) => inputs,
        }
    }
}

impl Program {
    pub fn new() -> Self {
        Program {
            instructions: vec![],
            instruction_table: HashMap::new(),
            outputs: vec![],
        }
    }
    pub fn push(&mut self, instruction: Instruction) -> ProgramTerm {
        *self
            .instruction_table
            .entry(instruction.clone())
            .or_insert_with(|| {
                self.instructions.push(instruction);
                ProgramTerm(self.instructions.len() - 1)
            })
    }
    pub fn push_output(&mut self, term: ProgramTerm) {
        self.outputs.push(term);
    }
    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }
    pub fn outputs(&self) -> &[ProgramTerm] {
        &self.outputs
    }
    pub fn evaluate_f64(&self, inputs: Vec<f64>) -> Vec<f64> {
        self.evaluate(&mut NumericContext::<f64>::new(inputs))
    }
    pub fn evaluate<C: TermContext>(&self, context: &mut C) -> Vec<C::Term> {
        let mut evaluator = self.evaluator::<C>();
        let mut outputs = vec![];
        evaluator.evaluate(self, context, &mut outputs);
        outputs
    }
    pub fn evaluator<C: TermContext>(&self) -> ProgramEvaluator<C> {
        ProgramEvaluator::new(self)
    }
    pub fn into_exprs(&self) -> Vec<Expr> {
        self.evaluate(&mut ExprContext::new())
    }
    pub fn deep_equals(&self, other: &Self) -> bool {
        self.into_exprs()
            .into_iter()
            .eq_by(other.into_exprs(), |x, y| x.deep_equals(&y))
    }
    pub fn without_dead_code(&self) -> Program {
        let mut live = vec![false; self.instructions.len()];
        for output in self.outputs.iter() {
            live[output.index()] = true;
        }
        for (index, instruction) in self.instructions.iter().enumerate().rev() {
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
                let new_instruction = match &self.instructions[i] {
                    Instruction::Nullary(op, []) => Instruction::Nullary(op.clone(), []),
                    Instruction::Unary(op, [i0]) => {
                        Instruction::Unary(op.clone(), [index_table[i0.index()].unwrap()])
                    }
                    Instruction::Binary(op, [i0, i1]) => Instruction::Binary(
                        op.clone(),
                        [
                            index_table[i0.index()].unwrap(),
                            index_table[i1.index()].unwrap(),
                        ],
                    ),
                    Instruction::Trinary(op, [i0, i1, i2]) => Instruction::Trinary(
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
        program
    }
}

impl ProgramTerm {
    pub fn index(&self) -> usize {
        self.0
    }
}

impl TermContext for Program {
    type Term = ProgramTerm;

    fn term_nullary(&mut self, nullary: NullaryOperator) -> Self::Term {
        self.push(Instruction::Nullary(nullary, []))
    }

    fn term_unary(&mut self, unary: UnaryOperator, t1: Self::Term) -> Self::Term {
        self.push(Instruction::Unary(unary, [t1]))
    }

    fn term_binary(
        &mut self,
        binary: BinaryOperator,
        t1: Self::Term,
        t2: Self::Term,
    ) -> Self::Term {
        self.push(Instruction::Binary(binary, [t1, t2]))
    }

    fn term_trinary(
        &mut self,
        trinary: TrinaryOperator,
        t1: Self::Term,
        t2: Self::Term,
        t3: Self::Term,
    ) -> Self::Term {
        self.push(Instruction::Trinary(trinary, [t1, t2, t3]))
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (index, instruction) in self.instructions.iter().enumerate() {
            writeln!(f, "T{} = {}", index, instruction)?;
        }
        writeln!(f, "return {:?}", self.outputs)?;
        Ok(())
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Nullary(op, []) => {
                write!(f, "{}", op)?;
            }
            Instruction::Unary(op, [i0]) => {
                write!(f, "{} {}", op, i0)?;
            }
            Instruction::Binary(op, [i0, i1]) => {
                write!(f, "{} {} {}", i0, op, i1)?;
            }
            Instruction::Trinary(op, [i0, i1, i2]) => {
                let (token1, token2) = op.tokens();
                write!(f, "{} {} {} {} {}", i0, token1, i1, token2, i2)?;
            }
        }
        Ok(())
    }
}

impl Display for ProgramTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "T{:?}", self.0)
    }
}

impl Debug for ProgramTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "T{:?}", self.0)
    }
}
