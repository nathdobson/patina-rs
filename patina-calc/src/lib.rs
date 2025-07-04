//!
//! This crate provides a domain-specific language for representing mathematical computations.
//!
//! Each term in the language represents a function from ℝⁿ to ℝ. Each term is one of the following:
//! * A nullary operator (AKA leaf).
//!     * An `f64` constant.
//!     * A variable specified by an index.
//! * A unary operator applied to another term `T`.
//!     * `-T`
//!     * `1/T`
//! * A binary operator applied to two terms `T1` and `T2`.
//!     * `T1 + T2`
//!     * `T1 - T2`
//!     * `T1 * T2`
//!     * `T1 / T2`
//!     * `min(T1, T2)`
//!     * `max(T1, T2)`
//! * A trinary operator applied to three terms `T1`, `T2`, and `T3`.
//!     * `T1 < 0.0 ? T2 : T3`
//!
//! This crate provides two independent representations of terms: [Expr] and [Program]. Each
//! representation encodes a directed acyclic graph to represent a set of terms. For example, the
//! term `(1.0 + (2.0 * x)) + (2.0 * x)` would be encoded by the following graph:
//! ```text
//!            (1.0 + (2.0 * x)) + (2.0 * x)
//!                 /             /
//!                /             /
//!       1.0 + (2.0 * x)       /
//!         /            \     /
//!        /              \   /
//!      1.0            2.0 * x
//!                       /   \
//!                      /     \
//!                    2.0     x
//! ```
//! ### [Expr]
//! [Expr] uses a "tree-like" representation with each term owning a Rc to it's input terms.
//! [Expr]s support operator overloads and make term construction simple, but they may be less
//! efficient than [Program]s.
//! ```
//! # use patina_calc::Expr;
//! assert_eq!("(1.5 + 2.5)", format!("{}", Expr::constant(1.5) + Expr::constant(2.5)));
//! ```
//!
//! ### [Program]
//! [Program] uses an assembly-like instruction sequence representation. Each term is written in a
//! valid evaluation order, with references to previous terms encoded by an index. One or more
//! references can marked as outputs.
//! ```
//! # use ordered_float::NotNan;
//! # use patina_calc::{OperatorBinary, OperatorNullary};
//! # use patina_calc::{ProgramStep, Program};
//! # use patina_calc::TermVisitor;
//! let mut program = Program::new();
//! let x = program.visit_nullary(OperatorNullary::Constant(NotNan::new(1.5f64).unwrap()));
//! let y = program.visit_nullary(OperatorNullary::Constant(NotNan::new(2.5f64).unwrap()));
//! let z = program.visit_binary(OperatorBinary::Add, x,y);
//! program.push_output(z);
//! assert_eq!("T0 = 1.5\nT1 = 2.5\nT2 = T0 + T1\nreturn [T2]\n", format!("{}", program));
//! ```
//!

#![feature(iter_order_by)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unreachable_code)]

mod derivative;
mod eval_visitor;
mod expr;
mod memoize;
mod numeric;
mod operator;
mod optimize;
mod program;

mod solver;
mod term_visitor;
#[cfg(test)]
mod test;
mod substitute;

pub use eval_visitor::EvalVisitor;
pub use expr::Expr;
pub use expr::ExprInner;
pub use expr::expr_visit::ExprVisit;
pub use expr::expr_visitor::ExprVisitor;
pub use operator::OperatorBinary;
pub use operator::OperatorNullary;
pub use operator::OperatorTrinary;
pub use operator::OperatorUnary;
pub use program::Program;
pub use program::ProgramStep;
pub use program::ProgramTerm;
pub use program::expr_program::ExprProgramBuilder;
pub use program::program_visit::ProgramVisit;
pub use term_visitor::TermVisitor;
pub use solver::Solver;