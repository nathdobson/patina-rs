use crate::marching::{CubeVertex, marching_cube};
use arrayvec::ArrayVec;
use itertools::Itertools;
use ordered_float::NotNan;
// use patina_calc::{EvalVisitor, Expr, ExprProgramBuilder, Program, ProgramVisit, Solver};
use crate::sdf::Sdf;
use inari::DecInterval;
use patina_geo::geo3::aabb::Aabb;
use patina_mesh::mesh::Mesh;
use patina_mesh::mesh_triangle::MeshTriangle;
use patina_scalar::deriv::Deriv;
use patina_scalar::newton::Newton;
use patina_vec::vec3::Vec3;
use patina_vec::vector3::Vector3;
use rand::{Rng, rng};
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

pub struct MarchingMesh {
    render_dim: usize,
    aabb: Aabb,
    evals: HashMap<[usize; 3], f64>,
    vertices: Vec<Vec3>,
    vertex_table: HashMap<[usize; 3], usize>,
    triangles: Vec<MeshTriangle>,
}

impl MarchingMesh {
    pub fn new(aabb: Aabb, render_dim: usize) -> Self {
        Self {
            aabb,
            evals: Default::default(),
            vertices: vec![],
            vertex_table: HashMap::new(),
            triangles: vec![],
            render_dim,
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
    fn evaluate(&mut self, ints: [usize; 3], sdf: &Sdf) -> f64 {
        let position = self.position(ints);
        sdf.evaluate(position)
    }
    fn build_cube(&mut self, cube: OctreeCube, sdf: &Sdf) {
        let aabb = self.aabb(&cube);
        let aabb_intervals: Vector3<DecInterval> = (0..3)
            .map(|axis| DecInterval::try_from((aabb.min()[axis], aabb.max()[axis])).unwrap())
            .collect();
        let (sdf2, range) = sdf.evaluate_constrain(aabb_intervals);
        if !range.contains(0.0) {
            return;
        }
        let sdf = sdf2.unwrap_or(sdf.clone());
        if let Some((center, subcubes)) = cube.subcubes() {
            let radius = self.evaluate(center, &sdf).abs();
            let min_radius = aabb.dimensions().length() / 2.0 + 10e-5;
            if radius <= min_radius {
                for cube in subcubes {
                    self.build_cube(cube, &sdf);
                }
            }
        } else {
            let mut bits = ArrayVec::new();
            let [x, y, z] = cube.min();
            for dx in 0..2 {
                for dy in 0..2 {
                    for dz in 0..2 {
                        let ints = [x + dx * 2, y + dy * 2, z + dz * 2];
                        let d = self.evaluate(ints, &sdf);
                        bits.push(d >= 0.0);
                    }
                }
            }
            let mcube = marching_cube(bits.into_inner().unwrap());
            let mut cube_vertices = vec![];
            for vertex in mcube.vertices() {
                cube_vertices.push(self.add_vertex(
                    [
                        x + vertex.x() as usize,
                        y + vertex.y() as usize,
                        z + vertex.z() as usize,
                    ],
                    &sdf,
                ));
            }
            for triangle in mcube.triangles() {
                self.triangles.push(MeshTriangle::from(
                    triangle.vertices().map(|v| cube_vertices[v]),
                ));
            }
        }
    }
    fn add_vertex(&mut self, ints: [usize; 3], sdf: &Sdf) -> usize {
        let position = self.position(ints);
        *self.vertex_table.entry(ints).or_insert_with(|| {
            let position_deriv = |t| {
                let mut inputs = Vector3::splat(Deriv::nan());
                for axis in 0..3 {
                    if ints[axis] % 2 == 0 {
                        inputs[axis] = Deriv::constant(position[axis]);
                    } else {
                        inputs[axis] = Deriv::constant(position[axis])
                            + Deriv::variable(t)
                                * Deriv::constant(
                                    self.aabb.dimensions()[axis] / (self.render_dim as f64),
                                );
                    }
                }
                inputs
            };
            let lsdf = |t| sdf.evaluate_deriv(position_deriv(t));
            let t = Newton::new().solve(-1.0..1.0, lsdf);
            let t = if let Some(t) = t {
                t.into_inner()
            } else {
                println!("Cannot solve {:?}", ints);
                0.5
            };
            assert!(t >= -1.0 && t <= 1.0, "{:?}", t);
            let t = t.clamp(-0.99, 0.99);
            let position = position_deriv(t).map(|x| x.value());
            let index = self.vertices.len();
            self.vertices.push(position);
            index
        })
    }
    pub fn build(mut self, sdf: &Sdf) -> Mesh {
        self.build_cube(OctreeCube::new([0, 0, 0], self.render_dim), sdf);
        Mesh::new(self.vertices, self.triangles)
    }
}
