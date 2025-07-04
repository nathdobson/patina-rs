use itertools::Itertools;
use patina_calc::{Expr, Program};
use patina_scalar::Scalar;
use patina_vec::vec3::Vec3;
use patina_vec::vector3::Vector3;

pub struct Sdf {
    expr: Expr,
}

pub struct CompiledSdf {
    program: Program,
}

impl Sdf {
    pub fn new(expr: Expr) -> Sdf {
        Sdf { expr }
    }
    pub fn compile(self) -> CompiledSdf {
        CompiledSdf {
            program: self.expr.into(),
        }
    }
}

impl CompiledSdf {
    pub fn program(&self) -> &Program {
        &self.program
    }
    pub fn evaluate(&self, x: Vec3) -> f64 {
        self.program
            .evaluate_f64(x.into_iter().collect())
            .into_iter()
            .exactly_one()
            .unwrap()
    }
}

pub fn position3() -> Vector3<Expr> {
    Vector3::new(Expr::var(0), Expr::var(1), Expr::var(2))
}

pub fn constant3(x: Vec3) -> Vector3<Expr> {
    Vector3::from(<[f64; 3]>::from(x).map(|x| Expr::from_f64(x)))
}
