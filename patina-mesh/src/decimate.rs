use crate::half_edge_mesh::{HalfEdgeId, HalfEdgeMesh};
use crate::mesh::Mesh;
use crate::mesh_triangle::MeshTriangle;
use crate::ser::encode_test_file;
use indicatif::ProgressBar;
use ordered_float::NotNan;
use patina_geo::aabb::Aabb;
use patina_vec::vec3::{Vec3, Vector3};
use priority_queue::PriorityQueue;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng, rng};
use rand_xorshift::XorShiftRng;
use std::collections::{HashMap, HashSet};
use std::time::Instant;
use patina_progress::ProgressBuilder;

pub struct Decimate<'mesh> {
    mesh: &'mesh mut HalfEdgeMesh,
    priorities: PriorityQueue<HalfEdgeId, Score>,
    max_degree: usize,
    min_score: f64,
    progress_builder: ProgressBuilder,
}

#[derive(Ord, Eq, PartialEq, PartialOrd, Clone, Copy, Hash, Debug)]
struct Score {
    score: NotNan<f64>,
    id: HalfEdgeId,
}

impl<'mesh> Decimate<'mesh> {
    pub fn new(mesh: &'mesh mut HalfEdgeMesh) -> Decimate<'mesh> {
        Decimate {
            mesh,
            priorities: PriorityQueue::new(),
            max_degree: 30,
            min_score: 0.9999,
            progress_builder: ProgressBuilder::new(),
        }
    }
    pub fn max_degree(&mut self, degree: usize) -> &mut Self {
        self.max_degree = degree;
        self
    }
    pub fn min_score(&mut self, min_score: f64) -> &mut Self {
        self.min_score = min_score;
        self
    }
    fn priority(&self, id: HalfEdgeId) -> Score {
        let v1 = self.mesh[id].vertex();
        let v2 = self.mesh[self.mesh[id].twin()].vertex();

        let mut count = 0;
        let mut bad = false;
        let mut ns1 = self
            .mesh
            .fan(v1)
            .map(|e| self.mesh[self.mesh[e].twin()].vertex())
            .collect::<HashSet<_>>();
        let mut degree = ns1.len();
        ns1.remove(&self.mesh[self.mesh[id].prev()].vertex());
        ns1.remove(&self.mesh[self.mesh[self.mesh[id].twin()].prev()].vertex());
        for n2 in self.mesh.fan(v2) {
            degree += 1;
            let v2 = self.mesh[self.mesh[n2].twin()].vertex();
            if ns1.contains(&v2) {
                bad = true;
                break;
            }
        }
        if degree >= self.max_degree {
            bad = true;
        }
        let mut area_change = 0.0;
        let mut normal_change = 0.0;
        let mut total_area = 0.0;
        let mut total_dot = 0.0;
        for e in self.mesh.fan(v1) {
            if bad {
                break;
            }
            let mtri1 = self.mesh.mesh_triangle(e);
            let mtri2 = MeshTriangle::from(mtri1.vertices().map(|v| if v == v1 { v2 } else { v }));
            assert_ne!(mtri1, mtri2);
            let tri1 = mtri1.for_half_mesh(self.mesh);
            let tri2 = mtri2.for_half_mesh(self.mesh);
            let area1 = tri1.area_vector();
            let area2 = tri2.area_vector();
            area_change += area2.length() - area1.length();
            total_area += area1.length();
            normal_change += area1.dot(area2);
            total_dot += area1.length() * area2.length();
            count += 1;
        }
        let score: f64;
        if count <= 2 || bad {
            score = -f64::INFINITY;
        } else if total_area == 0.0 || total_dot == 0.0 {
            score = -f64::INFINITY;
        } else {
            score = (normal_change / total_dot).minimum(1.0 - area_change.abs() / total_area);
        }
        Score {
            score: NotNan::new(score).unwrap(),
            id,
        }
    }
    pub fn run_heap(&mut self) {
        for (id, e) in self.mesh.edges() {
            self.priorities.push(id, self.priority(id));
        }
        while let Some((id, score)) = self.priorities.pop() {
            assert_eq!(score, self.priority(id));
            if score.score.into_inner() < self.min_score {
                break;
            }
            let v2 = self.mesh[self.mesh[id].twin()].vertex();
            let mut updated_vertices = self
                .mesh
                .fan(self.mesh[id].vertex())
                .map(|e| self.mesh[self.mesh[e].twin()].vertex())
                .collect::<HashSet<_>>();
            let removed = self.mesh.contract_edge(id);
            for removed in removed {
                self.priorities.remove(&removed);
            }
            let mut updated = HashSet::new();
            for v3 in updated_vertices {
                for e2 in self.mesh.fan(v3) {
                    updated.insert(e2);
                    updated.insert(self.mesh[e2].twin());
                }
            }
            for &updated in &updated {
                self.priorities.push(updated, self.priority(updated));
            }
        }
    }
    pub fn run_arbitrary(&mut self) {
        let progress_bar = self.progress_builder.build(self.mesh.edge_count() as u64);
        loop {
            let mut priorities: Vec<Score> =
                self.mesh.edges().map(|(e, _)| self.priority(e)).collect();
            priorities.sort_by(|x, y| x.cmp(y).reverse());
            let last_index = priorities
                .iter()
                .position(|x| x.score.into_inner() < self.min_score)
                .unwrap_or(priorities.len());

            let mut progress = 0;

            for score in &priorities[0..last_index / 2] {
                if self.mesh.get(score.id).is_some()
                    && self.priority(score.id).score.into_inner() > self.min_score
                {
                    progress_bar.inc(1);
                    self.mesh.contract_edge(score.id);
                    progress += 1;
                }
            }

            if progress < 10000 {
                break;
            }
        }
        self.run_heap();
        progress_bar.finish();
    }
}

#[cfg(test)]
#[tokio::test]
async fn decimate_test() {
    let mut vertices = vec![];
    let mut triangles = vec![];
    let mut vertex_table = HashMap::new();
    let count = 20;
    let mut rng = XorShiftRng::seed_from_u64(123);
    for x in 0..=count {
        for y in 0..=count {
            for z in 0..2 {
                vertex_table.insert(Vector3::new(x, y, z), vertices.len());
                let mut p = Vec3::new(x as f64, y as f64, z as f64);
                p += Vec3::random_normal(&mut rng) * 0.001;
                vertices.push(p);
            }
        }
    }
    for x in 0..count {
        for y in 0..count {
            for z in 0..2 {
                for mut tri in [
                    MeshTriangle::new(
                        vertex_table[&Vector3::new(x, y, z)],
                        vertex_table[&Vector3::new(x + 1, y, z)],
                        vertex_table[&Vector3::new(x, y + 1, z)],
                    ),
                    MeshTriangle::new(
                        vertex_table[&Vector3::new(x + 1, y, z)],
                        vertex_table[&Vector3::new(x + 1, y + 1, z)],
                        vertex_table[&Vector3::new(x, y + 1, z)],
                    ),
                ] {
                    if z == 0 {
                        tri.invert();
                    }
                    triangles.push(tri);
                }
            }
        }
    }
    for x in 0..count {
        for y in [0, count] {
            for mut tri in [
                MeshTriangle::new(
                    vertex_table[&Vector3::new(x + 1, y, 0)],
                    vertex_table[&Vector3::new(x, y, 0)],
                    vertex_table[&Vector3::new(x, y, 1)],
                ),
                MeshTriangle::new(
                    vertex_table[&Vector3::new(x, y, 1)],
                    vertex_table[&Vector3::new(x + 1, y, 1)],
                    vertex_table[&Vector3::new(x + 1, y, 0)],
                ),
            ] {
                if y == 0 {
                    tri.invert();
                }
                triangles.push(tri);
            }
        }
    }
    for x in [0, count] {
        for y in 0..count {
            for mut tri in [
                MeshTriangle::new(
                    vertex_table[&Vector3::new(x, y, 0)],
                    vertex_table[&Vector3::new(x, y + 1, 0)],
                    vertex_table[&Vector3::new(x, y, 1)],
                ),
                MeshTriangle::new(
                    vertex_table[&Vector3::new(x, y + 1, 1)],
                    vertex_table[&Vector3::new(x, y, 1)],
                    vertex_table[&Vector3::new(x, y + 1, 0)],
                ),
            ] {
                if x == 0 {
                    tri.invert();
                }
                triangles.push(tri);
            }
        }
    }
    let mesh = Mesh::new(vertices, triangles);
    let area0 = mesh.area();
    mesh.check_manifold().unwrap();
    encode_test_file(&mesh, "input.stl").await.unwrap();
    let mut hem = HalfEdgeMesh::new(&mesh);
    let mut decimate = Decimate::new(&mut hem);
    decimate.run_heap();
    hem.check_manifold().unwrap();
    let mesh = hem.as_mesh();
    encode_test_file(&mesh, "output.stl").await.unwrap();
    mesh.check_manifold().unwrap();
    println!("{:?}", mesh.area() - area0);
}
