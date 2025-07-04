use crate::solver::Solver;
use crate::term_visitor::TermVisitorExt;
use crate::{Expr, OperatorBinary, OperatorNullary, Program, TermVisitor};
use inari::{IntervalError, dec_interval};
use patina_scalar::Scalar;
use rand::{SeedableRng, rng};
use rand_xorshift::XorShiftRng;

#[test]
fn test_evaluator() {
    let expr = Expr::constant(1.0) + Expr::constant(2.0) * Expr::var(0);
    let program = Program::from(expr);
    assert_eq!(vec![7.0], program.evaluate_f64(vec![3.0]));
}

#[test]
fn test_dedup() {
    let expr = Expr::constant(1.0) + Expr::constant(1.0);
    let program = Program::from(expr);
    assert_eq!(
        "T0 = 1\nT1 = T0 + T0\nreturn [T1]\n",
        format!("{}", program)
    );
}

#[test]
fn test_constraint() -> Result<(), IntervalError> {
    let expr = Expr::var(0).minimum(Expr::var(1));
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
            "{}==\n{}",
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
    let x = program.var(0);
    let y = program.var(1);
    let z = program.var(2);
    let w = program.var(3);
    let xy = program.add(x, y);
    let xyz = program.add(xy, z);
    let yz = program.add(y, z);
    program.push_output(xyz);
    assert_eq!(program.steps().len(), 7);
    let without = program.without_dead_code();
    assert_eq!(without.steps().len(), 5);
}

#[test]
fn test_derivative() {
    let x = Expr::var(0);
    let y = Expr::var(1);
    let c0 = Expr::constant(0.0);
    let c1 = Expr::constant(1.0);
    let c2 = Expr::constant(2.0);
    assert!(x.derivative(0).deep_equals(&c1));
    assert!(x.derivative(1).deep_equals(&c0));
    assert!(y.derivative(0).deep_equals(&c0));
    assert!(y.derivative(1).deep_equals(&c1));
    assert!(
        (x.clone() + y.clone())
            .derivative(0)
            .deep_equals(&(c1.clone() + c0.clone()))
    );
    assert!(
        (x.clone() * y.clone())
            .derivative(0)
            .deep_equals(&(c1.clone() * y.clone() + x.clone() * c0.clone()))
    );
}

#[test]
fn test_solve() {
    let x = Expr::var(0);
    let x2 = x.clone() * x.clone();
    let x2m1 = x2 - Expr::constant(1.0);
    let program = Program::from(x2m1);
    let program = program.with_derivative(0);
    for seed in 0..100 {
        let mut rng = XorShiftRng::seed_from_u64(seed);
        let mut solver = Solver::new();
        assert_eq!(solver.solve(&program, -2.0..2.0).unwrap().abs(), 1.0);
    }
}
