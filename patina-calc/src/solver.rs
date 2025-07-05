use crate::{EvalVisitor, Program, ProgramVisit};
use ordered_float::NotNan;
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
            iterations: 20,
            starts: 100,
            eps: 1e-8,
            visit: ProgramVisit::new(),
        }
    }
    pub fn solve(&mut self, program: &Program, range: Range<f64>) -> Option<NotNan<f64>> {
        for _ in 0..self.starts {
            if let Some(x) = self.solve_once(program, range.clone()) {
                return Some(x);
            }
        }
        None
    }

    fn solve_once(&mut self, program: &Program, range: Range<f64>) -> Option<NotNan<f64>> {
        let mut x = self.rng.random_range(range.clone());
        for it in 0.. {
            if !x.is_finite() {
                return None;
            }
            let mut visitor = EvalVisitor::new(vec![x]);
            let mut output = vec![];
            self.visit.visit(program, &mut visitor, &mut output);
            let [y, yp] = output.as_slice().try_into().unwrap();
            if y.abs() < self.eps {
                if x < range.start - self.eps {
                    return None;
                } else if x < range.start {
                    return Some(NotNan::new(range.start).unwrap());
                } else if x > range.end + self.eps {
                    return None;
                } else if x > range.end {
                    return Some(NotNan::new(range.end).unwrap());
                } else {
                    return NotNan::new(x).ok();
                }
            } else if it < self.iterations - 1 {
                x = x - y / yp;
            } else if x < range.start {
                return None;
            } else if x > range.end {
                return None;
            // } else if y.abs() > self.eps {
            //     return None;
            } else {
                return None;
            }
        }
        None
    }
}
