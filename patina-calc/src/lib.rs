#![feature(iter_order_by)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use crate::expr::Expr;
use crate::operator::{BinaryOperator, NullaryOperator};
use crate::program::Program;
use crate::term_context::TermContext;
use inari::{DecInterval, IntervalError, dec_interval};
use std::ops::{Add, Div, Mul, Neg, Sub};

pub mod bounded_term;
pub mod deep_equal;
pub mod derivative;
pub mod expr;
mod expr_context;
mod expr_evaluator;
mod expr_program;
mod memoize;
pub mod numeric;
pub mod operator;
pub mod program;
pub mod program_evaluator;
pub mod term_context;

#[test]
fn test_evaluator() {
    let expr = Expr::from(1.0) + Expr::from(2.0) * Expr::var(0);
    let program = Program::from(expr);
    assert_eq!(vec![7.0], program.evaluate_f64(vec![3.0]));
}

#[test]
fn test_dedup() {
    let expr = Expr::from(1.0) + Expr::from(1.0);
    let program = Program::from(expr);
    assert_eq!(
        "T0 = 1\nT1 = T0 + T0\nreturn [T1]\n",
        format!("{}", program)
    );
}

#[test]
fn test_constraint() -> Result<(), IntervalError> {
    let expr = Expr::var(0).min(Expr::var(1));
    let program = Program::from(expr);
    {
        let optimized = program.constrain(vec![dec_interval!(0.0, 1.0)?, dec_interval!(0.0, 1.0)?]);
        assert!(
            program.deep_equals(&optimized),
            "{}==\n{}",
            program,
            optimized
        );
    }
    {
        let optimized = program.constrain(vec![dec_interval!(0.0, 0.0)?, dec_interval!(0.0, 1.0)?]);
        let expected = Program::from(Expr::var(0));
        assert!(
            expected.deep_equals(&optimized),
            "{:?} == {:?}",
            expected,
            optimized
        );
    }
    {
        let optimized = program.constrain(vec![dec_interval!(0.0, 1.0)?, dec_interval!(0.0, 0.0)?]);
        let expected = Program::from(Expr::var(1));
        println!("{}", optimized);
        println!("{}", expected);
        assert!(
            expected.deep_equals(&optimized),
            "{}==\n{}",
            expected,
            optimized
        );
    }

    Ok(())
}

#[test]
fn test_remove_dead_code() {
    let mut program = Program::new();
    let x = program.term_nullary(NullaryOperator::Variable(0));
    let y = program.term_nullary(NullaryOperator::Variable(1));
    let z = program.term_nullary(NullaryOperator::Variable(2));
    let w = program.term_nullary(NullaryOperator::Variable(3));
    let xy = program.term_binary(BinaryOperator::Add, x, y);
    let xyz = program.term_binary(BinaryOperator::Add, xy, z);
    let yz = program.term_binary(BinaryOperator::Add, y, z);
    program.push_output(xyz);
    assert_eq!(program.instructions().len(), 7);
    let without = program.without_dead_code();
    assert_eq!(without.instructions().len(), 5);
}

// #[test]
// fn test_trinary() {
//     let x0 = Expr::var(0);
//     let x1 = Expr::var(1);
//     let c0 = Expr::from(0.0);
//     let c1 = Expr::from(1.0);
//     let expr = Expr::min(x0.clone(), x1.clone());
//     let d0 = expr.derivative(0);
//     let d0_expected = Expr::piecewise(x0.clone() - x1.clone(), c1.clone(), c0.clone());
//     assert!(
//         d0.deep_equals(&d0_expected),
//         "{:?} == {:?}",
//         d0,
//         d0_expected
//     );
//     let d1 = Expr::piecewise(x0.clone() - x1.clone(), c0.clone(), c1.clone());
//     let d1_expected = Expr::piecewise(x0.clone() - x1.clone(), c0.clone(), c1.clone());
//     assert!(
//         d1.deep_equals(&d1_expected),
//         "{:?} == {:?}",
//         d1,
//         d1_expected
//     );
//     let d00 = expr.derivative(0).derivative(0);
//     let d00_expected = Expr::piecewise(x0.clone() - x1.clone(), c0.clone(), c0.clone());
//     assert!(
//         d00.deep_equals(&d00_expected),
//         "{:?} == {:?}",
//         d00,
//         d00_expected
//     );
// }
