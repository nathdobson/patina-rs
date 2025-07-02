use crate::math::float_bool::Epsilon;
use crate::meshes::mesh::Mesh;
use crate::meshes::mesh_edge::MeshEdge;
use crate::meshes::mesh_triangle::MeshTriangle;
use crate::meshes::ordered_mesh_edge::OrderedMeshEdge;
use arrayvec::ArrayVec;
use itertools::Itertools;
use ordered_float::NotNan;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Component {
    tris: Vec<usize>,
    lop: Vec<usize>,
    paths: [Vec<usize>; 2],
    outer_paths: [Vec<usize>; 2],
    new_tris: Vec<MeshTriangle>,
}
pub struct RemoveEmptyTriangles<'a> {
    eps: Epsilon,
    mesh: &'a Mesh,
    bad_tris: Vec<usize>,
    bad_wings: HashMap<MeshEdge, ArrayVec<usize, 2>>,
    edges: HashMap<OrderedMeshEdge, usize>,
    tri_graph: HashMap<usize, HashSet<usize>>,
    components: Vec<Component>,
    removed_tris: HashSet<usize>,
}

impl<'a> RemoveEmptyTriangles<'a> {
    pub fn new(eps: Epsilon, mesh: &'a Mesh) -> Self {
        RemoveEmptyTriangles {
            eps,
            mesh,
            bad_tris: vec![],
            bad_wings: HashMap::new(),
            edges: Default::default(),
            tri_graph: HashMap::new(),
            components: vec![],
            removed_tris: HashSet::new(),
        }
    }
    fn find_bad_tris(&mut self) {
        for (tri, mtri) in self.mesh.triangles().iter().enumerate() {
            if mtri.for_vertices(self.mesh.vertices()).area() <= self.eps.value() {
                self.bad_tris.push(tri);
            }
        }
    }
    fn build_wings(&mut self) {
        for &tri in &self.bad_tris {
            for edge in self.mesh.triangles()[tri].edges() {
                self.bad_wings.entry(edge).or_default().push(tri);
            }
        }
    }
    fn build_edges(&mut self) {
        for (tri, mtri) in self.mesh.triangles().iter().enumerate() {
            for edge in mtri.ordered_edges() {
                self.edges.insert(edge, tri);
            }
        }
    }
    fn build_graph(&mut self) {
        for (edge, wing) in self.bad_wings.iter() {
            if let Ok([tri1, tri2]) = wing.clone().into_inner() {
                self.tri_graph.entry(tri1).or_default().insert(tri2);
                self.tri_graph.entry(tri2).or_default().insert(tri1);
            }
        }
    }
    fn build_bad_components(&mut self) {
        let mut visited = HashSet::new();
        for &bad_tri in &self.bad_tris {
            let mut frontier = vec![bad_tri];
            let mut component = vec![];
            while let Some(next) = frontier.pop() {
                if visited.insert(next) {
                    component.push(next);
                    if let Some(another) = self.tri_graph.get(&next) {
                        for &another in another {
                            frontier.push(another);
                        }
                    }
                }
            }
            if !component.is_empty() {
                self.components.push(Component {
                    tris: component,
                    lop: vec![],
                    paths: [vec![], vec![]],
                    outer_paths: [vec![], vec![]],
                    new_tris: vec![],
                });
            }
        }
        // TODO: this is super slow
        while self.components.len() > 1 {
            self.components.pop();
        }
    }
    fn build_loops(&mut self) {
        for component in &mut self.components {
            let mut edge_table = HashMap::<MeshEdge, HashSet<OrderedMeshEdge>>::new();
            for &tri in &component.tris {
                for edge in self.mesh.triangles()[tri].ordered_edges() {
                    edge_table.entry(edge.edge()).or_default().insert(edge);
                }
            }
            let mut adj = HashMap::new();
            for (edge, dirs) in edge_table.into_iter() {
                if dirs.len() == 1 {
                    let edge = dirs.iter().next().unwrap().clone();
                    assert!(adj.insert(edge.vertices()[0], edge.vertices()[1]).is_none());
                }
            }
            let start = *adj.keys().next().unwrap();
            let mut next = start;
            loop {
                component.lop.push(next);
                next = adj[&next];
                if next == start {
                    break;
                }
            }
        }
    }
    fn build_paths(&mut self) {
        for component in &mut self.components {
            let vec =
                self.mesh.vertices()[component.lop[0]] - self.mesh.vertices()[component.lop[1]];
            let (&t1, &t2) = component
                .lop
                .iter()
                .minmax_by_key(|&&x| NotNan::new(self.mesh.vertices()[x].dot(vec)).unwrap())
                .into_option()
                .unwrap();
            let t1p = component.lop.iter().position(|&x| x == t1).unwrap();
            let t2p = component.lop.iter().position(|&x| x == t2).unwrap();
            for (i, t1p, t2p) in [(0, t1p, t2p), (1, t2p, t1p)] {
                component.paths[i] = component
                    .lop
                    .iter()
                    .chain(component.lop.iter())
                    .cloned()
                    .skip(t1p)
                    .take(1 + (component.lop.len() + t2p - t1p) % component.lop.len())
                    .collect();
            }
            component.paths[1].reverse();
        }
    }
    fn build_outer_paths(&mut self) {
        for component in &mut self.components {
            for side in 0..2 {
                for &[v1, v2] in component.paths[side].array_windows::<2>() {
                    let mut edge = OrderedMeshEdge::new(v1, v2);
                    if side == 0 {
                        edge.invert();
                    }
                    let other_tri = *self.edges.get(&edge).unwrap();
                    let other_vertex = *self.mesh.triangles()[other_tri]
                        .vertices()
                        .iter()
                        .find(|x| !edge.vertices().contains(x))
                        .unwrap();
                    component.outer_paths[side].push(other_vertex);
                }
            }
        }
    }
    fn build_new_tris(&mut self) {
        for component in &mut self.components {
            println!();
            let tv = self.mesh.vertices()[*component.paths[0].last().unwrap()]
                - self.mesh.vertices()[*component.paths[0].first().unwrap()];
            let mut sequence: Vec<(usize, usize, usize)> = component
                .paths
                .iter()
                .enumerate()
                .flat_map(|(side, path)| {
                    path.iter()
                        .enumerate()
                        .map(move |(index, vertex)| (side, index, *vertex))
                })
                .collect();
            sequence.sort_by_cached_key(|(side, index, v)| {
                (
                    NotNan::new(self.mesh.vertices()[*v].dot(tv)).unwrap(),
                    *side,
                    *index,
                )
            });
            let mut i1 = 0;
            let mut i2 = 0;
            let mut prev = component.paths[0][0];
            for &(side, index, vertex) in &sequence[2..sequence.len() - 2] {
                component.new_tris.push(MeshTriangle::new(
                    vertex,
                    prev,
                    component.outer_paths[0][i1],
                ));
                component.new_tris.push(MeshTriangle::new(
                    prev,
                    vertex,
                    component.outer_paths[1][i2],
                ));
                if side == 0 {
                    i1 = index;
                    prev = vertex;
                } else {
                    i2 = index;
                    prev = vertex;
                }
            }
            component.new_tris.push(MeshTriangle::new(
                *component.paths[0].last().unwrap(),
                prev,
                component.outer_paths[0][i1],
            ));
            component.new_tris.push(MeshTriangle::new(
                prev,
                *component.paths[0].last().unwrap(),
                component.outer_paths[1][i2],
            ));
        }
    }
    fn build_removed_tris(&mut self) {
        for component in &self.components {
            for &bad_tri in &component.tris {
                self.removed_tris.insert(bad_tri);
                for mut edge in self.mesh.triangles()[bad_tri].ordered_edges() {
                    edge.invert();
                    if let Some(tri2) = self.edges.get(&edge) {
                        self.removed_tris.insert(*tri2);
                    }
                }
            }
        }
    }
    pub fn build_mesh(&self) -> Mesh {
        let vertices = self.mesh.vertices().to_vec();
        let mut triangles: Vec<MeshTriangle> = self
            .mesh
            .triangles()
            .iter()
            .enumerate()
            .filter(|(tri, mtri)| !self.removed_tris.contains(tri))
            .map(|x| *x.1)
            .collect();
        for component in &self.components {
            triangles.extend(component.new_tris.iter().cloned());
        }
        Mesh::new(vertices, triangles)
    }
    pub fn build(mut self) -> Option<Mesh> {
        self.find_bad_tris();
        self.build_wings();
        self.build_edges();
        self.build_graph();
        self.build_bad_components();
        if self.components.is_empty() {
            return None;
        }
        self.build_loops();
        self.build_paths();
        self.build_outer_paths();
        self.build_new_tris();
        self.build_removed_tris();
        // println!(
        //     "removed {:#?}",
        //     self.removed_tris
        //         .iter()
        //         .map(|x| self.mesh.triangles()[*x])
        //         .collect::<Vec<_>>()
        // );
        // println!("added {:#?}", self.components);
        let result = self.build_mesh();
        // println!("{:#?}", result);
        Some(result)
    }
}
