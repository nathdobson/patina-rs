use crate::derivative::DerivativeTransform;
use crate::eval_visitor::EvalVisitor;
use crate::expr::Expr;
use crate::expr::expr_visitor::ExprVisitor;
use crate::operator::{OperatorBinary, OperatorNullary, OperatorTrinary, OperatorUnary};
use crate::term_visitor::TermVisitor;
use program_visit::ProgramVisit;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

pub mod expr_program;
pub mod program_visit;

/// A single term in a [Program], specifying the operator and indices for inputs.
#[derive(Debug, Clone, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub enum ProgramStep {
    Nullary(OperatorNullary, [ProgramTerm; 0]),
    Unary(OperatorUnary, [ProgramTerm; 1]),
    Binary(OperatorBinary, [ProgramTerm; 2]),
    Trinary(OperatorTrinary, [ProgramTerm; 3]),
}

/// A representation of terms as sequential steps.
#[derive(Debug)]
pub struct Program {
    steps: Vec<ProgramStep>,
    step_table: HashMap<ProgramStep, ProgramTerm>,
    outputs: Vec<ProgramTerm>,
}

/// An index of the step that produces a particular term.
#[derive(Eq, Ord, PartialEq, PartialOrd, Hash, Copy, Clone)]
pub struct ProgramTerm(usize);

impl ProgramStep {
    pub fn inputs(&self) -> &[ProgramTerm] {
        match self {
            ProgramStep::Nullary(_, inputs) => inputs,
            ProgramStep::Unary(_, inputs) => inputs,
            ProgramStep::Binary(_, inputs) => inputs,
            ProgramStep::Trinary(_, inputs) => inputs,
        }
    }
}

impl Program {
    pub fn new() -> Self {
        Program {
            steps: vec![],
            step_table: HashMap::new(),
            outputs: vec![],
        }
    }
    pub fn push(&mut self, step: ProgramStep) -> ProgramTerm {
        *self.step_table.entry(step.clone()).or_insert_with(|| {
            self.steps.push(step);
            ProgramTerm(self.steps.len() - 1)
        })
    }
    pub fn push_output(&mut self, term: ProgramTerm) {
        self.outputs.push(term);
    }
    pub fn steps(&self) -> &[ProgramStep] {
        &self.steps
    }
    pub fn outputs(&self) -> &[ProgramTerm] {
        &self.outputs
    }
    pub fn evaluate_f64(&self, inputs: Vec<f64>) -> Vec<f64> {
        self.visit(&mut EvalVisitor::<f64>::new(inputs))
    }
    pub fn visit<V: TermVisitor>(&self, visitor: &mut V) -> Vec<V::Output> {
        let mut evaluator = ProgramVisit::new(self);
        let mut outputs = vec![];
        evaluator.evaluate(self, visitor, &mut outputs);
        outputs
    }
    pub fn into_exprs(&self) -> Vec<Expr> {
        self.visit(&mut ExprVisitor::new())
    }
    pub fn deep_equals(&self, other: &Self) -> bool {
        self.into_exprs()
            .into_iter()
            .eq_by(other.into_exprs(), |x, y| x.deep_equals(&y))
    }
    pub fn and_then(&self, other: &Self) -> Self {
        let mut output = Program::new();
        for step in &self.steps {
            output.push(step.clone());
        }
        for step in &other.steps {
            match step {
                ProgramStep::Nullary(OperatorNullary::Variable(v), []) => output.push(
                    ProgramStep::Unary(OperatorUnary::Identity, [self.outputs()[*v]]),
                ),
                _ => output.push(step.clone()),
            };
        }
        output
    }
    pub fn derivative(&self, variable: usize) -> Self {
        let mut output = Program::new();
        let mut derivative = DerivativeTransform::new(output, variable);
        let outputs = self.visit(&mut derivative);
        let mut program = derivative.into_inner();
        for (f, fp) in outputs {
            program.push_output(fp);
        }
        program
    }
}

impl ProgramTerm {
    pub fn index(&self) -> usize {
        self.0
    }
}

impl TermVisitor for Program {
    type Output = ProgramTerm;

    fn visit_nullary(&mut self, nullary: OperatorNullary) -> Self::Output {
        self.push(ProgramStep::Nullary(nullary, []))
    }

    fn visit_unary(&mut self, unary: OperatorUnary, t1: Self::Output) -> Self::Output {
        self.push(ProgramStep::Unary(unary, [t1]))
    }

    fn visit_binary(
        &mut self,
        binary: OperatorBinary,
        t1: Self::Output,
        t2: Self::Output,
    ) -> Self::Output {
        self.push(ProgramStep::Binary(binary, [t1, t2]))
    }

    fn visit_trinary(
        &mut self,
        trinary: OperatorTrinary,
        t1: Self::Output,
        t2: Self::Output,
        t3: Self::Output,
    ) -> Self::Output {
        self.push(ProgramStep::Trinary(trinary, [t1, t2, t3]))
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (index, step) in self.steps.iter().enumerate() {
            writeln!(f, "T{} = {}", index, step)?;
        }
        writeln!(f, "return {:?}", self.outputs)?;
        Ok(())
    }
}

impl Display for ProgramStep {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgramStep::Nullary(op, []) => {
                write!(f, "{}", op)?;
            }
            ProgramStep::Unary(op, [i0]) => {
                write!(f, "{} {}", op, i0)?;
            }
            ProgramStep::Binary(op, [i0, i1]) => {
                write!(f, "{} {} {}", i0, op, i1)?;
            }
            ProgramStep::Trinary(op, [i0, i1, i2]) => {
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
