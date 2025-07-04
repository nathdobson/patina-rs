use crate::{EvalVisitor, Program, ProgramVisit};
use rand::rngs::ThreadRng;
use rand::{Rng, rng};
use std::ops::Range;

pub struct Solver<R = ThreadRng> {
    rng: R,
    iterations: usize,
    starts: usize,
    eps: f64,
    visit: ProgramVisit<EvalVisitor<f64>>,
}

impl Solver {
    pub fn new() -> Self {
        Self::with_rng(rng())
    }
}

impl<R: Rng> Solver<R> {
    pub fn with_rng(rng: R) -> Solver<R> {
        Solver {
            rng,
            iterations: 10,
            starts: 5,
            eps: 1e-10,
            visit: ProgramVisit::new(),
        }
    }
    pub fn solve(&mut self, program: &Program, range: Range<f64>) -> Option<f64> {
        for _ in 0..self.starts {
            if let Some(x) = self.solve_once(program, range.clone()) {
                return Some(x);
            }
        }
        None
    }

    fn solve_once(&mut self, program: &Program, range: Range<f64>) -> Option<f64> {
        let mut x = self.rng.random_range(range.clone());
        for it in 0.. {
            let mut visitor = EvalVisitor::new(vec![x]);
            let mut output = vec![];
            self.visit.visit(program, &mut visitor, &mut output);
            let [y, yp] = output.as_slice().try_into().unwrap();
            if it == self.iterations - 1 {
                if x < range.start {
                    return None;
                } else if x > range.end {
                    return None;
                }
                if y.abs() > self.eps {
                    return None;
                }
                return Some(x);
            } else {
                x = x - y / yp;
            }
        }
        None
    }
}
