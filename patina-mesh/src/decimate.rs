use crate::half_edge_mesh::{HalfEdgeId, HalfEdgeMesh};
use crate::mesh::Mesh;
use crate::mesh_triangle::MeshTriangle;
use crate::ser::encode_test_file;
use ordered_float::NotNan;
use patina_geo::aabb::Aabb;
use patina_vec::vec3::{Vec3, Vector3};
use priority_queue::PriorityQueue;
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use std::collections::{HashMap, HashSet};

pub struct Decimate<'mesh> {
    mesh: &'mesh mut HalfEdgeMesh,
    priorities: PriorityQueue<HalfEdgeId, Score>,
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
        }
    }
    fn priority(&self, id: HalfEdgeId) -> Score {
        let v1 = self.mesh[id].vertex();
        let v2 = self.mesh[self.mesh[id].twin()].vertex();
        let mut total_length = 0.0;
        let mut total_vec = Vec3::zero();
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
        if degree >= 30 {
            bad = true;
        }
        let mut area_change = 0.0;
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
            total_vec += area1;
            total_length += area1.length();
            count += 1;
        }
        let score: f64;
        if count <= 2 || bad {
            score = -f64::INFINITY;
        } else if total_length == 0.0 {
            score = f64::INFINITY;
        } else {
            score =
                (total_vec.length() / total_length).minimum(1.0 - area_change.abs() / total_length);

            if !score.is_finite() {
                println!("{:?} {:?} {:?}", total_vec, total_length, count);
            }
        }
        Score {
            score: NotNan::new(score).unwrap(),
            id,
        }
    }
    pub fn run(&mut self) {
        for (id, e) in self.mesh.edges() {
            self.priorities.push(id, self.priority(id));
        }
        let mut removed = 0;
        while let Some((id, score)) = self.priorities.pop() {
            assert_eq!(score, self.priority(id));
            if score.score.into_inner() < 0.9999 {
                break;
            }
            removed += 1;
            // println!("Removing {:?} {:?}", id, score.score);
            let v2 = self.mesh[self.mesh[id].twin()].vertex();
            let removed = self.mesh.contract_edge(id);
            for removed in removed {
                self.priorities.remove(&removed);
            }
            let mut updated = HashSet::new();
            for e in self.mesh.fan(v2) {
                updated.insert(e);
                updated.insert(self.mesh[e].twin());
                let v3 = self.mesh[self.mesh[e].twin()].vertex();
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
}

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
    decimate.run();
    hem.check_manifold().unwrap();
    let mesh = hem.as_mesh();
    encode_test_file(&mesh, "output.stl").await.unwrap();
    mesh.check_manifold().unwrap();
    println!("{:?}", mesh.area() - area0);
}
