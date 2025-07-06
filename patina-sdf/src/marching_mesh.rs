use crate::marching::{CubeVertex, marching_cube};
use crate::sdf::CompiledSdf;
use arrayvec::ArrayVec;
use itertools::Itertools;
use ordered_float::NotNan;
use patina_calc::{EvalVisitor, Expr, ExprProgramBuilder, Program, ProgramVisit, Solver};
use patina_geo::geo3::aabb::Aabb;
use patina_mesh::mesh::Mesh;
use patina_mesh::mesh_triangle::MeshTriangle;
use patina_vec::vec3::Vec3;
use std::collections::HashMap;

#[derive(Debug)]
struct OctreeCube {
    min: [usize; 3],
    dim: usize,
}

impl OctreeCube {
    pub fn new(min: [usize; 3], dim: usize) -> OctreeCube {
        assert!(dim.is_power_of_two());
        OctreeCube { min, dim }
    }
    pub fn min(&self) -> [usize; 3] {
        self.min
    }
    pub fn max(&self) -> [usize; 3] {
        self.min.map(|x| x + self.dim)
    }

    pub fn dim(&self) -> usize {
        self.dim
    }
    pub fn subcubes(&self) -> Option<([usize; 3], [Self; 8])> {
        if self.dim == 2 {
            return None;
        }
        let dim2 = self.dim / 2;
        let mut subcubes = ArrayVec::new();
        for x in 0..2 {
            for y in 0..2 {
                for z in 0..2 {
                    subcubes.push(OctreeCube::new(
                        [
                            self.min[0] + x * dim2,
                            self.min[1] + y * dim2,
                            self.min[2] + z * dim2,
                        ],
                        dim2,
                    ));
                }
            }
        }
        Some((
            [self.min[0] + dim2, self.min[1] + dim2, self.min[2] + dim2],
            subcubes.into_inner().ok().unwrap(),
        ))
    }
}

pub struct MarchingMesh<'a> {
    sdf: &'a CompiledSdf,
    render_dim: usize,
    aabb: Aabb,
    visit: ProgramVisit<EvalVisitor<f64>>,
    evals: HashMap<[usize; 3], f64>,
    vertices: Vec<Vec3>,
    vertex_table: HashMap<[usize; 3], usize>,
    triangles: Vec<MeshTriangle>,
    solver: Solver,
}

impl<'a> MarchingMesh<'a> {
    pub fn new(sdf: &'a CompiledSdf, aabb: Aabb, render_dim: usize) -> Self {
        Self {
            sdf,
            visit: ProgramVisit::with_capacity(sdf.program()),
            aabb,
            evals: Default::default(),
            vertices: vec![],
            vertex_table: HashMap::new(),
            triangles: vec![],
            render_dim,
            solver: Solver::new(),
        }
    }
    fn position(&self, ints: [usize; 3]) -> Vec3 {
        let dimensions = self.aabb.dimensions();
        (0..3)
            .map(|axis| {
                (ints[axis] as f64) / (self.render_dim as f64) * dimensions[axis]
                    + self.aabb.min()[axis]
            })
            .collect()
    }
    fn aabb(&self, cube: &OctreeCube) -> Aabb {
        Aabb::new(self.position(cube.min()), self.position(cube.max()))
    }
    fn evaluate(&mut self, ints: [usize; 3]) -> f64 {
        let position = self.position(ints);
        let sdf = self.sdf;
        let visit = &mut self.visit;
        *self.evals.entry(ints).or_insert_with(|| {
            let mut visitor = EvalVisitor::new(position.into_iter().collect());
            let mut outputs = vec![];
            visit.visit(&sdf.program(), &mut visitor, &mut outputs);
            outputs.into_iter().exactly_one().unwrap()
        })
    }
    fn build_cube(&mut self, cube: OctreeCube) {
        let aabb = self.aabb(&cube);
        if let Some((center, subcubes)) = cube.subcubes() {
            let radius = self.evaluate(center).abs();
            let min_radius = aabb.dimensions().length() / 2.0 + 10e-5;
            if radius <= min_radius {
                for cube in subcubes {
                    self.build_cube(cube);
                }
            }
        } else {
            let mut bits = ArrayVec::new();
            let [x, y, z] = cube.min();
            for dx in 0..2 {
                for dy in 0..2 {
                    for dz in 0..2 {
                        bits.push(self.evaluate([x + dx * 2, y + dy * 2, z + dz * 2]) >= 0.0);
                    }
                }
            }
            let mcube = marching_cube(bits.into_inner().unwrap());
            let mut cube_vertices = vec![];
            for vertex in mcube.vertices() {
                cube_vertices.push(self.add_vertex([
                    x + vertex.x() as usize,
                    y + vertex.y() as usize,
                    z + vertex.z() as usize,
                ]));
            }
            for triangle in mcube.triangles() {
                self.triangles.push(MeshTriangle::from(
                    triangle.vertices().map(|v| cube_vertices[v]),
                ));
            }
        }
    }
    fn add_vertex(&mut self, ints: [usize; 3]) -> usize {
        let position = self.position(ints);
        *self.vertex_table.entry(ints).or_insert_with(|| {
            let mut input_program = ExprProgramBuilder::new();
            for axis in 0..3 {
                if ints[axis] % 2 == 0 {
                    input_program.push(Expr::constant(position[axis]));
                } else {
                    input_program.push(
                        Expr::constant(position[axis])
                            + Expr::var(0)
                                * Expr::constant(
                                    self.aabb.dimensions()[axis] / (self.render_dim as f64),
                                ),
                    );
                }
            }
            let guided_sdf = input_program.program().and_then(self.sdf.program());
            let guided_sdf = guided_sdf.with_derivative(0);
            let t = self.solver.solve(&guided_sdf, -1.0..1.0);
            let t = if let Some(t) = t {
                t.into_inner()
            } else {
                println!("Cannot solve {:?}", ints);
                0.5
            };
            assert!(t >= -1.0 && t <= 1.0, "{:?}", t);
            let t = t.clamp(-0.99, 0.99);
            let position = input_program.program().evaluate_f64(vec![t]);
            let position = position.into_iter().collect::<Vec3>();
            let index = self.vertices.len();
            self.vertices.push(position);
            index
        })
    }
    pub fn build(mut self) -> Mesh {
        self.build_cube(OctreeCube::new([0, 0, 0], self.render_dim));
        // let mut visit = ProgramVisit::with_capacity(sdf.program());
        // let mut outputs = vec![0f64];
        // let mut vertices = vec![];
        // let mut vertex_table = HashMap::new();
        // let mut triangles = vec![];
        // let mut solver = Solver::new();
        // let evals: Vec<Vec<Vec<bool>>> = (0..self.count[0] + 1)
        //     .map(|x| {
        //         (0..self.count[1] + 1)
        //             .map(|y| {
        //                 (0..self.count[2] + 1)
        //                     .map(|z| {
        //                         let position = self.position(x, y, z);
        //
        //                         visit.visit(
        //                             sdf.program(),
        //                             &mut EvalVisitor::new(position.into_iter().collect()),
        //                             &mut outputs,
        //                         );
        //                         let inside = outputs[0] < 0.0;
        //                         if inside {
        //                             // let index = vertices.len();
        //                             // vertices.push(position);
        //                             // vertices.push(position + Vec3::new(0.1, 0.0, 0.0));
        //                             // vertices.push(position + Vec3::new(0.00, 0.1, 0.0));
        //                             // triangles.push(MeshTriangle::new(index, index + 1, index + 2));
        //                         }
        //                         inside
        //                     })
        //                     .collect()
        //             })
        //             .collect()
        //     })
        //     .collect();
        // for x in 0..self.count[0] {
        //     for y in 0..self.count[1] {
        //         for z in 0..self.count[2] {
        //             let mut bits = ArrayVec::new();
        //             for dx in 0..2 {
        //                 for dy in 0..2 {
        //                     for dz in 0..2 {
        //                         bits.push(evals[x + dx][y + dy][z + dz]);
        //                     }
        //                 }
        //             }
        //             let cube = marching_cube(bits.into_inner().unwrap());
        //             let mut cube_vertices = vec![];
        //             for vertex in cube.vertices() {
        //                 let xp = x * 2 + vertex.x() as usize;
        //                 let yp = y * 2 + vertex.y() as usize;
        //                 let zp = z * 2 + vertex.z() as usize;
        //                 let index = *vertex_table.entry((xp, yp, zp)).or_insert_with(|| {
        //                     let mut input_program = ExprProgramBuilder::new();
        //                     for axis in 0..3 {
        //                         match vertex.into_inner()[axis] {
        //                             0 => input_program.push(Expr::constant(
        //                                 self.origin[axis]
        //                                     + ([x, y, z][axis] as f64) * self.delta[axis],
        //                             )),
        //                             1 => input_program.push(
        //                                 Expr::constant(
        //                                     self.origin[axis]
        //                                         + ([x, y, z][axis] as f64) * self.delta[axis],
        //                                 ) + Expr::var(0) * Expr::constant(self.delta[axis]),
        //                             ),
        //                             2 => input_program.push(Expr::constant(
        //                                 self.origin[axis]
        //                                     + (([x, y, z][axis] + 1) as f64) * self.delta[axis],
        //                             )),
        //                             _ => unreachable!(),
        //                         }
        //                     }
        //                     let guided_sdf = input_program.program().and_then(sdf.program());
        //                     let guided_sdf = guided_sdf.with_derivative(0);
        //                     let t = solver.solve(&guided_sdf, 0.0..1.0);
        //                     let t = if let Some(t) = t {
        //                         t.into_inner()
        //                     } else {
        //                         println!("Cannot solve {} {} {} {} {} {}", x, y, z, xp, yp, zp);
        //                         0.5
        //                     };
        //                     assert!(t >= 0.0 && t <= 1.0, "{:?}", t);
        //                     let t = t.clamp(0.01, 0.99);
        //                     let position = input_program.program().evaluate_f64(vec![t]);
        //                     let position = position.into_iter().collect::<Vec3>();
        //                     let index = vertices.len();
        //                     vertices.push(position);
        //                     index
        //                 });
        //                 cube_vertices.push(index);
        //             }
        //             for triangle in cube.triangles() {
        //                 triangles.push(MeshTriangle::from(
        //                     triangle.vertices().map(|v| cube_vertices[v]),
        //                 ));
        //             }
        //         }
        //     }
        // }
        Mesh::new(self.vertices, self.triangles)
    }
}
