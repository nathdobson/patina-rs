use crate::deriv::Deriv;
use ordered_float::NotNan;
use rand::rngs::ThreadRng;
use rand::{Rng, rng};
use std::ops::Range;

pub struct Newton<R = ThreadRng> {
    rng: R,
    iterations: usize,
    starts: usize,
    eps: f64,
}

impl Newton {
    pub fn new() -> Self {
        Self::with_rng(rng())
    }
}

impl<R: Rng> Newton<R> {
    pub fn with_rng(rng: R) -> Newton<R> {
        Newton {
            rng,
            iterations: 20,
            starts: 100,
            eps: 1e-8,
        }
    }
    pub fn solve(
        &mut self,
        range: Range<f64>,
        mut eval: impl FnMut(f64) -> Deriv<1>,
    ) -> Option<NotNan<f64>> {
        for _ in 0..self.starts {
            if let Some(x) = self.solve_once(range.clone(), &mut eval) {
                return Some(x);
            }
        }
        None
    }

    fn solve_once(
        &mut self,
        range: Range<f64>,
        eval: &mut impl FnMut(f64) -> Deriv<1>,
    ) -> Option<NotNan<f64>> {
        let mut x = self.rng.random_range(range.clone());
        for it in 0.. {
            if !x.is_finite() {
                return None;
            }
            let yyp = eval(x);
            let y = yyp.value();
            let yp = yyp.deriv()[0];
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
            } else {
                return None;
            }
        }
        None
    }
}
