use crate::marching::marching_cube;
use crate::sdf::CompiledSdf;
use arrayvec::ArrayVec;
use itertools::Itertools;
use patina_calc::{EvalVisitor, Expr, ExprProgramBuilder, Program, ProgramVisit, Solver};
use patina_mesh::mesh::Mesh;
use patina_mesh::mesh_triangle::MeshTriangle;
use patina_vec::vec3::Vec3;
use std::collections::HashMap;

pub struct MarchingMesh {
    origin: Vec3,
    delta: Vec3,
    count: [usize; 3],
}

impl MarchingMesh {
    pub fn new(origin: Vec3, delta: Vec3, count: [usize; 3]) -> Self {
        Self {
            origin,
            delta,
            count,
        }
    }
    fn position(&self, x: usize, y: usize, z: usize) -> Vec3 {
        self.origin
            + Vec3::new(
                self.delta.x() * (x as f64),
                self.delta.y() * (y as f64),
                self.delta.z() * (z as f64),
            )
    }
    pub fn build(self, sdf: &CompiledSdf) -> Mesh {
        let mut visit = ProgramVisit::with_capacity(sdf.program());
        let mut outputs = vec![0f64];
        let mut vertices = vec![];
        let mut vertex_table = HashMap::new();
        let mut triangles = vec![];
        let mut solver = Solver::new();
        let evals: Vec<Vec<Vec<bool>>> = (0..self.count[0] + 1)
            .map(|x| {
                (0..self.count[1] + 1)
                    .map(|y| {
                        (0..self.count[2] + 1)
                            .map(|z| {
                                let position = self.position(x, y, z);

                                visit.visit(
                                    sdf.program(),
                                    &mut EvalVisitor::new(position.into_iter().collect()),
                                    &mut outputs,
                                );
                                let inside = outputs[0] < 0.0;
                                if inside {
                                    let index = vertices.len();
                                    vertices.push(position);
                                    vertices.push(position + Vec3::new(0.1, 0.0, 0.0));
                                    vertices.push(position + Vec3::new(0.00, 0.1, 0.0));
                                    triangles.push(MeshTriangle::new(index, index + 1, index + 2));
                                }
                                inside
                            })
                            .collect()
                    })
                    .collect()
            })
            .collect();
        for x in 0..self.count[0] {
            for y in 0..self.count[1] {
                for z in 0..self.count[2] {
                    let mut bits = ArrayVec::new();
                    for dx in 0..2 {
                        for dy in 0..2 {
                            for dz in 0..2 {
                                bits.push(evals[x + dx][y + dy][z + dz]);
                            }
                        }
                    }
                    let cube = marching_cube(bits.into_inner().unwrap());
                    let mut cube_vertices = vec![];
                    for vertex in cube.vertices() {
                        let xp = x * 2 + vertex.x() as usize;
                        let yp = y * 2 + vertex.y() as usize;
                        let zp = z * 2 + vertex.z() as usize;
                        let index = *vertex_table.entry((xp, yp, zp)).or_insert_with(|| {
                            let mut input_program = ExprProgramBuilder::new();
                            for axis in 0..3 {
                                match vertex.into_inner()[axis] {
                                    0 => input_program.push(Expr::constant(
                                        self.origin[axis]
                                            + ([x, y, z][axis] as f64) * self.delta[axis],
                                    )),
                                    1 => input_program.push(
                                        Expr::constant(
                                            self.origin[axis]
                                                + ([x, y, z][axis] as f64) * self.delta[axis],
                                        ) + Expr::var(0),
                                    ),
                                    2 => input_program.push(Expr::constant(
                                        self.origin[axis]
                                            + (([x, y, z][axis] + 1) as f64) * self.delta[axis],
                                    )),
                                    _ => unreachable!(),
                                }
                            }
                            let guided_sdf = input_program.program().and_then(sdf.program());
                            let guided_sdf = guided_sdf.with_derivative(0);
                            let t = solver.solve(&guided_sdf, 0.0..1.0).unwrap();
                            let position = input_program.program().evaluate_f64(vec![t]);
                            let position = position.into_iter().collect::<Vec3>();
                            let index = vertices.len();
                            vertices.push(position);
                            index
                        });
                        cube_vertices.push(index);
                    }
                    for triangle in cube.triangles() {
                        triangles.push(MeshTriangle::from(
                            triangle.vertices().map(|v| cube_vertices[v]),
                        ));
                    }
                }
            }
        }
        Mesh::new(vertices, triangles)
    }
}
