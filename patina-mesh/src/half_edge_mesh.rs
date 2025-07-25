use crate::directed_mesh_edge::DirectedMeshEdge;
use crate::mesh::{ManifoldError, Mesh};
use crate::mesh_edge::MeshEdge;
use crate::mesh_triangle::MeshTriangle;
use crate::ser::encode_test_file;
use arrayvec::ArrayVec;
use itertools::Itertools;
use patina_geo::aabb::Aabb;
use patina_geo::geo3::triangle3::Triangle3;
use patina_vec::vec2::Vec2;
use patina_vec::vec3::Vec3;
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use slab::Slab;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut};
use std::{iter, mem};

#[derive(Debug)]
pub struct HalfEdge {
    twin: HalfEdgeId,
    next: HalfEdgeId,
    prev: HalfEdgeId,
    vertex: usize,
}

#[derive(Copy, Clone, Eq, Ord, PartialOrd, PartialEq, Hash)]
pub struct HalfEdgeId(usize);

#[derive(Debug)]
pub struct HalfEdgeVertex {
    position: Vec3,
    edge: HalfEdgeId,
}

pub struct HalfEdgeMesh {
    vertices: Slab<HalfEdgeVertex>,
    edges: Slab<HalfEdge>,
}

impl HalfEdgeId {
    pub fn new(index: usize) -> Self {
        HalfEdgeId(index)
    }
    pub fn index(self) -> usize {
        self.0
    }
}

impl HalfEdgeVertex {
    pub fn position(&self) -> Vec3 {
        self.position
    }
}

#[derive(Debug, Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum HalfEdgeError {
    BadTwin(HalfEdgeId),
    BadFace(HalfEdgeId),
    BadFan(usize, HalfEdgeId),
    FanDuplicate(usize, HalfEdgeId),
    CollapsedEdge(HalfEdgeId, usize),
}

pub struct FanWalker {
    start: HalfEdgeId,
    next: Option<HalfEdgeId>,
}

impl FanWalker {
    pub fn new(start: HalfEdgeId) -> Self {
        FanWalker {
            start,
            next: Some(start),
        }
    }
    pub fn step(&mut self, mesh: &HalfEdgeMesh) -> Option<HalfEdgeId> {
        let ret = self.next?;
        let next = mesh[mesh[ret].prev].twin;
        self.next = (next != self.start).then_some(next);
        Some(ret)
    }
}

impl HalfEdgeMesh {
    pub fn new(mesh: &Mesh) -> Self {
        let mut vertices: Slab<HalfEdgeVertex> = mesh
            .vertices()
            .iter()
            .cloned()
            .map(|position| HalfEdgeVertex {
                position,
                edge: HalfEdgeId(usize::MAX),
            })
            .enumerate()
            .collect();
        let mut tris = Vec::with_capacity(mesh.triangles().len());
        let mut directed_edges: Vec<DirectedMeshEdge> =
            Vec::with_capacity(mesh.triangles().len() * 3);
        for tri in mesh.triangles() {
            tris.push(HalfEdgeId(directed_edges.len()));
            for edge in tri.ordered_edges() {
                directed_edges.push(edge);
            }
        }
        for (id, edge) in directed_edges.iter().enumerate() {
            vertices[edge.v1()].edge = HalfEdgeId(id);
        }
        assert!(
            vertices
                .iter()
                .all(|(_, e)| e.edge != HalfEdgeId(usize::MAX))
        );
        let edge_ids: HashMap<DirectedMeshEdge, HalfEdgeId> = directed_edges
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, e)| (e, HalfEdgeId(i)))
            .collect();
        let mut edges = Slab::with_capacity(tris.len() * 3);
        for (tri, mtri) in mesh.triangles().iter().enumerate() {
            let tri_edge = HalfEdgeId(edges.len());
            for side in 0..3 {
                edges.insert(HalfEdge {
                    twin: *edge_ids
                        .get(&mtri.ordered_edges()[side].inverted())
                        .unwrap(),
                    next: *edge_ids.get(&mtri.ordered_edges()[(side + 1) % 3]).unwrap(),
                    prev: *edge_ids.get(&mtri.ordered_edges()[(side + 2) % 3]).unwrap(),
                    vertex: mtri.vertices()[side],
                });
            }
        }
        HalfEdgeMesh { vertices, edges }
    }
    pub fn edges(&self) -> impl Iterator<Item = (HalfEdgeId, &'_ HalfEdge)> {
        self.edges.iter().map(|(i, x)| (HalfEdgeId(i), x))
    }
    // pub fn vertices(&self) -> impl Iterator<Item = (usize, &'_ HalfEdgeVertex)> {
    //     self.vertices.iter()
    // }
    pub fn vertices(&self) -> &Slab<HalfEdgeVertex> {
        &self.vertices
    }
    pub fn walk(&self, vertex: usize) -> FanWalker {
        FanWalker::new(self.vertices[vertex].edge)
    }
    pub fn fan(&self, vertex: usize) -> impl Iterator<Item = HalfEdgeId> {
        struct FanIterator<'mesh> {
            mesh: &'mesh HalfEdgeMesh,
            walker: FanWalker,
        }
        impl<'mesh> Iterator for FanIterator<'mesh> {
            type Item = HalfEdgeId;
            fn next(&mut self) -> Option<Self::Item> {
                self.walker.step(self.mesh)
            }
        }
        let e = self.vertices[vertex].edge;
        FanIterator {
            mesh: self,
            walker: FanWalker::new(e),
        }
    }
    pub fn as_mesh(&self) -> Mesh {
        let mut vertices = Vec::with_capacity(self.vertices.len());
        let mut vertex_map = vec![usize::MAX; self.vertices.capacity()];
        for (index, vertex) in self.vertices.iter() {
            vertex_map[index] = vertices.len();
            vertices.push(vertex.position);
        }
        let mut visited_edges = vec![false; self.edges.capacity()];
        let mut tris = vec![];
        for (id1, e1) in self.edges() {
            if !visited_edges[id1.index()] {
                let id2 = e1.next;
                let e2 = &self[id2];
                let id3 = e2.next;
                let e3 = &self[id3];
                visited_edges[id1.index()] = true;
                visited_edges[id2.index()] = true;
                visited_edges[id3.index()] = true;
                tris.push(MeshTriangle::new(
                    vertex_map[e1.vertex],
                    vertex_map[e2.vertex],
                    vertex_map[e3.vertex],
                ));
            }
        }
        Mesh::new(vertices, tris)
    }
    pub fn check_manifold(&self) -> Result<(), HalfEdgeError> {
        for (id, e) in self.edges() {
            if self[e.twin].twin != id {
                return Err(HalfEdgeError::BadTwin(id));
            }
            if self[e.twin].vertex == e.vertex {
                return Err(HalfEdgeError::CollapsedEdge(id, e.vertex));
            }
        }
        for (id1, e1) in self.edges() {
            let id2 = e1.next;
            let id3 = e1.prev;
            if self[id2].next != id3
                || self[id2].prev != id1
                || self[id3].next != id1
                || self[id3].prev != id2
            {
                return Err(HalfEdgeError::BadFace(id1));
            }
        }
        let mut expected_fans = vec![vec![]; self.vertices.capacity()];
        for (id, e) in self.edges() {
            expected_fans[e.vertex].push(id);
        }
        for (vid, v) in self.vertices() {
            let mut expected = expected_fans[vid].iter().cloned().collect::<HashSet<_>>();
            let mut vs = HashSet::new();
            for actual in self.fan(vid) {
                if !expected.remove(&actual) {
                    return Err(HalfEdgeError::BadFan(vid, actual));
                }
                if !vs.insert(self[self[actual].twin].vertex) {
                    return Err(HalfEdgeError::FanDuplicate(vid, actual));
                }
            }
            if let Some(diff) = expected.iter().next() {
                return Err(HalfEdgeError::BadFan(vid, *diff));
            }
        }
        Ok(())
    }
    fn edge_mut(&mut self, index: HalfEdgeId) -> &mut HalfEdge {
        self.edges
            .get_mut(index.0)
            .unwrap_or_else(|| panic!("Cannot find index {:?}", index))
    }
    pub fn contract_edge(&mut self, f1: HalfEdgeId) -> ArrayVec<HalfEdgeId, 6> {

        let mut removed = ArrayVec::new();
        let mut v1 = self[f1].vertex;
        let mut v2 = self[self[f1].twin].vertex;
        assert_ne!(v1, v2);
        // println!("Contracting {:?} into {:?} ({:?})", v1, v2, f1);

        // Eliminate references to the old vertex
        let mut fan = self.walk(self[f1].vertex);
        while let Some(e) = fan.step(self) {
            self.edge_mut(e).vertex = v2;
        }
        let v1 = self.vertices.remove(v1);

        let f2 = self[f1].next;
        let f2p = self[self[f1].next].twin;
        let f3 = self[f1].prev;
        let f3p = self[f3].twin;

        let r1 = self[f1].twin;
        let r2 = self[r1].next;
        let r3 = self[r1].prev;
        let r2p = self[r2].twin;

        // Ensure edges that will be removed are not exemplars for fans.
        if self.vertices[v2].edge == r1 {
            self.vertices[v2].edge = f2;
        }
        if self.vertices[self[f3].vertex].edge == f3 {
            let v = self[f3].vertex;
            self.vertices[v].edge = self[f3p].next;
        }
        if self.vertices[self[r3].vertex].edge == r2p {
            let v = self[r3].vertex;
            self.vertices[v].edge = r3;
        }

        if f3p == r2 {
            assert_eq!(r2p, f3);
            removed.extend([f1, r1, f3, f3p, f2, r2]);
            let stitch = f2p != r3;
            let f1 = self.edges.remove(f1.0);
            let r1 = self.edges.remove(r1.0);
            let f3 = self.edges.remove(f3.0);
            let f3p = self.edges.remove(f3p.0);
            let f2 = self.edges.remove(f2.0);
            let r3 = self.edges.remove(r3.0);
            if stitch {
                // Removed an ear: stitch the edge together.
                self.edge_mut(f2.twin()).twin = r3.twin();
                self.edge_mut(r3.twin()).twin = f2.twin();
            } else {
                // Removed a pair of triangles forming the entire connected component.
                self.vertices.remove(v2);
                self.vertices.remove(r3.vertex);
            }
            return removed;
        }

        removed.extend([f1, f3, f3p, r1, r2, r2p]);

        // remove the "top" edges
        let f1 = self.edges.remove(f1.0);
        let f3 = self.edges.remove(f3.0);
        let f3p = self.edges.remove(f3p.0);

        // Merge the top left edge into the top right edge
        self.edge_mut(f2).next = f3p.next;
        self.edge_mut(f3p.next).prev = f2;
        self.edge_mut(f2).prev = f3p.prev;
        self.edge_mut(f3p.prev).next = f2;

        // remove the "bottom" edges
        let r1 = self.edges.remove(r1.0);
        let r2 = self.edges.remove(r2.0);
        let r2p = self.edges.remove(r2p.0);

        // Merge the bottom left edge into the bottom right edge
        self.edge_mut(r2p.next).prev = r3;
        self.edge_mut(r3).next = r2p.next;
        self.edge_mut(r2p.prev).next = r3;
        self.edge_mut(r3).prev = r2p.prev;

        removed
    }
    pub fn triangle(&self, id: HalfEdgeId) -> Triangle3 {
        Triangle3::new(
            self.mesh_triangle(id)
                .vertices()
                .map(|v| self.vertices[v].position),
        )
    }
    pub fn mesh_triangle(&self, id: HalfEdgeId) -> MeshTriangle {
        MeshTriangle::new(
            self[id].vertex,
            self[self[id].next].vertex,
            self[self[id].prev].vertex,
        )
    }
}

impl Index<HalfEdgeId> for HalfEdgeMesh {
    type Output = HalfEdge;
    fn index(&self, index: HalfEdgeId) -> &Self::Output {
        self.edges
            .get(index.0)
            .unwrap_or_else(|| panic!("Cannot find index {:?}", index))
    }
}

impl Debug for HalfEdgeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "H{}", self.0)
    }
}

impl Debug for HalfEdgeMesh {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HalfEdgeMesh")
            .field_with("edges", |f| f.debug_map().entries(self.edges()).finish())
            .field("vertices", &self.vertices)
            .finish()
    }
}

impl From<usize> for HalfEdgeId {
    fn from(value: usize) -> Self {
        HalfEdgeId(value)
    }
}

impl From<HalfEdgeId> for usize {
    fn from(value: HalfEdgeId) -> Self {
        value.0
    }
}

impl HalfEdge {
    pub fn vertex(&self) -> usize {
        self.vertex
    }
    pub fn twin(&self) -> HalfEdgeId {
        self.twin
    }
    pub fn next(&self) -> HalfEdgeId {
        self.next
    }
    pub fn prev(&self) -> HalfEdgeId {
        self.prev
    }
}

#[test]
fn test() {
    let mesh = Mesh::new(
        vec![
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        ],
        vec![
            MeshTriangle::new(0, 1, 2),
            MeshTriangle::new(1, 0, 3),
            MeshTriangle::new(2, 1, 3),
            MeshTriangle::new(0, 2, 3),
        ],
    );
    let mut hem = HalfEdgeMesh::new(&mesh);
    hem.check_manifold().unwrap();
    assert_eq!(
        vec![9, 3, 0],
        hem.fan(0).map(usize::from).collect::<Vec<_>>()
    );
    assert_eq!(
        vec![6, 1, 5],
        hem.fan(1).map(usize::from).collect::<Vec<_>>()
    );
    assert_eq!(
        vec![10, 2, 8],
        hem.fan(2).map(usize::from).collect::<Vec<_>>()
    );
    assert_eq!(
        vec![11, 7, 4],
        hem.fan(3).map(usize::from).collect::<Vec<_>>()
    );
    let mesh2 = hem.as_mesh();
    assert_eq!(mesh, mesh2);

    hem.contract_edge(HalfEdgeId(0));
    println!("{:#?}", hem);
    hem.check_manifold().unwrap();
    hem.contract_edge(HalfEdgeId(1));
    println!("{:#?}", hem);
    hem.check_manifold().unwrap();
}

#[tokio::test]
async fn test_cube() {
    for seed in 51..1000 {
        println!("\n seed {:?}", seed);
        let mut rng = XorShiftRng::seed_from_u64(seed);
        let mesh = Mesh::from_aabb(Aabb::new(Vec3::splat(0.0), Vec3::splat(1.0)));
        let mut hem = HalfEdgeMesh::new(&mesh);
        while hem.vertices.len() > 5 {
            let next = rng.random_range(0..hem.edges.capacity());
            if hem.edges.contains(next) {
                hem.contract_edge(HalfEdgeId(next));
                encode_test_file(
                    &hem.as_mesh(),
                    &format!("seed_{}_{}.stl", seed, hem.vertices.len()),
                ).await.unwrap();
                println!("{:#?}", hem);
                hem.check_manifold().unwrap();
            }
        }
    }
}
