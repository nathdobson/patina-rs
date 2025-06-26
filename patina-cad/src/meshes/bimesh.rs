use crate::geo3::ray3::Ray3;
use crate::math::disjoint_paths::DisjointPaths;
use crate::math::float_bool::Epsilon;
use crate::math::vec3::Vec3;
use crate::meshes::bvh::Bvh;
use crate::meshes::intersect_bvh_bvh::{IntersectBvhBvh, MeshMeshIntersection};
use crate::meshes::mesh::Mesh;
use crate::meshes::mesh_edge::MeshEdge;
use crate::meshes::mesh_polygon::MeshPolygon;
use crate::meshes::mesh_triangle::MeshTriangle;
use crate::meshes::ordered_mesh_edge::OrderedMeshEdge;
use crate::meshes::triangulation::Triangulation;
use crate::ser::stl::write_stl_file;
use crate::util::loop_builder::LoopBuilder;
use arrayvec::ArrayVec;
use itertools::Itertools;
use ordered_float::NotNan;
use rand::prelude::SliceRandom;
use rand::{Rng, SeedableRng, rng};
use rand_xorshift::XorShiftRng;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

#[derive(Debug)]
struct NewFace {
    edge_vertices: [Vec<usize>; 3],
    internal_vertices: Vec<usize>,
    border_vertices: Vec<usize>,
    edges: HashSet<MeshEdge>,
}

enum VertexOrigin {
    Mesh(usize),
    Intersect,
}

pub struct Bimesh<'a> {
    eps: Epsilon,
    meshes: [&'a Mesh; 2],
    bvhs: [Bvh; 2],
    vertices: Vec<Vec3>,
    vertex_origins: Vec<VertexOrigin>,
    input_tris: [Vec<MeshTriangle>; 2],
    input_wings: HashMap<MeshEdge, BimeshWings>,
    new_vertices: HashMap<usize, MeshMeshIntersection>,
    new_edges: Vec<OrderedMeshEdge>,
    loops: Vec<Vec<usize>>,
    forward_loop_adjacency: HashMap<usize, usize>,
    reverse_loop_adjacency: HashMap<usize, usize>,
    loop_ids: HashMap<usize, usize>,
    ordered_wings: HashMap<usize, [usize; 2]>,
    new_faces: [Vec<NewFace>; 2],
    new_tris: [Vec<MeshTriangle>; 2],
    new_tri_ccs: [Vec<Vec<MeshTriangle>>; 2],
    new_tri_cats: [[Vec<MeshTriangle>; 2]; 2],
}

#[derive(Debug)]
pub struct BimeshWings {
    mesh: usize,
    wing_tris: [usize; 2],
}

impl<'a> Bimesh<'a> {
    fn new_uninit(eps: Epsilon, mesh1: &'a Mesh, mesh2: &'a Mesh) -> Self {
        let meshes = [mesh1, mesh2];
        Bimesh {
            eps,
            bvhs: meshes.map(Bvh::from_mesh),
            vertices: vec![],
            vertex_origins: vec![],
            meshes,
            input_tris: [vec![], vec![]],
            input_wings: HashMap::new(),
            loops: vec![],
            new_vertices: HashMap::new(),
            new_edges: vec![],
            forward_loop_adjacency: HashMap::new(),
            reverse_loop_adjacency: HashMap::new(),
            loop_ids: HashMap::new(),
            ordered_wings: HashMap::new(),
            new_faces: [const { vec![] }; 2],
            new_tris: [const { vec![] }; 2],
            new_tri_ccs: [const { vec![] }; 2],
            new_tri_cats: [const { [const { vec![] }; 2] }; 2],
        }
    }
    pub fn new(mesh1: &'a Mesh, mesh2: &'a Mesh, eps: Epsilon, rng: &mut impl Rng) -> Self {
        let mut this = Self::new_uninit(eps, mesh1, mesh2);
        this.build(rng);
        this
    }
    pub fn build_relabel(&mut self) {
        let mut offset = 0;
        for mesh in 0..2 {
            let mut wing_builders = HashMap::new();
            for (ti, tri) in self.meshes[mesh].triangles().iter().enumerate() {
                let new_tri = MeshTriangle::from(tri.vertices().map(|n| n + offset));
                self.input_tris[mesh].push(new_tri);
                for e in new_tri.edges() {
                    wing_builders
                        .entry(e)
                        .or_insert_with(ArrayVec::<_, 2>::new)
                        .push(ti);
                }
            }
            for v in self.meshes[mesh].vertices() {
                self.vertices.push(*v);
                self.vertex_origins.push(VertexOrigin::Mesh(mesh));
                offset += 1;
            }
            for (edge, wing) in wing_builders {
                let wing = wing.into_inner().unwrap();
                self.input_wings.insert(
                    edge,
                    BimeshWings {
                        mesh,
                        wing_tris: wing,
                    },
                );
            }
        }
    }
    pub fn build_new_vertices(&mut self) {
        let mut intersect = IntersectBvhBvh::new(self.eps);
        intersect.intersect_node_node(
            &self.bvhs[0].root_view(&self.input_tris[0], &self.vertices),
            &self.bvhs[1].root_view(&self.input_tris[1], &self.vertices),
        );
        for int in intersect.build() {
            let vertex = self.vertices.len();
            self.vertices.push(int.position);
            self.vertex_origins.push(VertexOrigin::Intersect);
            self.new_vertices.insert(vertex, int);
        }
    }
    pub fn build_new_edges(&mut self) {
        let mut face_pair_to_vertices = HashMap::<[usize; 2], ArrayVec<usize, 6>>::new();
        for (&v, int) in self.new_vertices.iter() {
            let mut pair = [None; 2];
            pair[int.plane_mesh] = Some(int.plane_tri);
            for tri2 in self.input_wings[&int.edge].wing_tris {
                pair[int.edge_mesh] = Some(tri2);
                face_pair_to_vertices
                    .entry(pair.clone().map(|x| x.unwrap()))
                    .or_default()
                    .push(v);
            }
        }
        for (&[tri1, tri2], vs) in &mut face_pair_to_vertices {
            if vs
                .iter()
                .filter(|&&v| self.new_vertices[&v].truth.is_true())
                .count()
                >= 2
            {
                vs.retain(|&mut v| self.new_vertices[&v].truth.is_true());
            }
            for (&v1, &v2) in vs.iter().tuple_combinations() {
                let mut edge = OrderedMeshEdge::new(v1, v2);
                let n1 = self.input_tris[0][tri1]
                    .for_vertices(&self.vertices)
                    .normal();
                let n2 = self.input_tris[1][tri2]
                    .for_vertices(&self.vertices)
                    .normal();
                let ev = self.new_vertices[&v2].position - self.new_vertices[&v1].position;
                let det = n1.cross(n2).dot(ev);
                if self.new_vertices[&v1].truth.is_true() && self.new_vertices[&v2].truth.is_true()
                {
                    if det < 0.0 {
                        edge.invert();
                    }
                    self.new_edges.push(edge);
                } else {
                    let reversed = self.eps.less(det, 0.0);
                    if reversed.not().maybe() {
                        self.new_edges.push(edge);
                    }
                    edge.invert();
                    if reversed.maybe() {
                        self.new_edges.push(edge);
                    }
                };
            }
        }
    }
    pub fn build_loops(&mut self) {
        let mut strong_adj_forward = HashMap::new();
        let mut strong_adj_reverse = HashMap::new();
        let mut weak_edges = vec![];
        for e in &self.new_edges {
            let [v1, v2] = e.vertices();
            if self.new_vertices[&v1].truth.is_true() && self.new_vertices[&v2].truth.is_true() {
                assert!(strong_adj_forward.insert(v1, v2).is_none());
                assert!(strong_adj_reverse.insert(v2, v1).is_none());
            } else {
                weak_edges.push([v1, v2]);
            }
        }
        let mut full_loops = vec![];
        let mut partial_loops = vec![];
        let mut visited = HashSet::new();
        for (&i1, &i2) in strong_adj_forward.iter() {
            if !visited.insert(i1) {
                continue;
            }
            let mut start = i1;
            while let Some(&prev) = strong_adj_reverse.get(&start) {
                start = prev;
                if start == i1 {
                    break;
                }
            }
            let mut loop1 = vec![];
            let mut prev = start;
            loop {
                loop1.push(prev);
                visited.insert(prev);
                if let Some(&next) = strong_adj_forward.get(&prev) {
                    prev = next;
                    if next == start {
                        full_loops.push(loop1);
                        break;
                    }
                } else {
                    partial_loops.push(loop1);
                    break;
                }
            }
        }
        let mut weak_graph = DisjointPaths::new(self.vertices.len());
        for (&v, int) in self.new_vertices.iter() {
            if !int.truth.is_true() {
                weak_graph.add_vertex(v);
            }
        }
        for partial_loop in &partial_loops {
            let begin = *partial_loop.first().unwrap();
            let end = *partial_loop.last().unwrap();
            weak_graph.add_vertex(end);
            weak_graph.add_begin(end);
            weak_graph.add_vertex(begin);
            weak_graph.add_end(begin);
        }
        for weak_edge in &weak_edges {
            weak_graph.add_edge(weak_edge[0], weak_edge[1]);
        }
        weak_graph.solve();
        let weak_paths = weak_graph.paths();
        let mut weak_path_table = HashMap::new();
        for weak_path in weak_paths {
            let begin = *weak_path.first().unwrap();
            weak_path_table.insert(begin, weak_path);
        }
        for mut partial_loop in partial_loops {
            let cont = weak_path_table
                .get(partial_loop.last().unwrap())
                .expect("Missing continuation");
            assert_eq!(
                cont.last(),
                partial_loop.first(),
                "{:?} vs {:?}",
                cont,
                partial_loop
            );
            partial_loop.extend(&cont[1..cont.len() - 1]);
            full_loops.push(partial_loop);
        }
        self.loops = full_loops;
    }
    pub fn build_loop_meta(&mut self) {
        let mut keep_vertices = HashSet::new();
        for (id, loop1) in self.loops.iter().enumerate() {
            for (&v1, &v2) in loop1.iter().circular_tuple_windows() {
                assert!(self.forward_loop_adjacency.insert(v1, v2).is_none());
                assert!(self.reverse_loop_adjacency.insert(v2, v1).is_none());
            }
            for &v in loop1.iter() {
                keep_vertices.insert(v);
                self.loop_ids.insert(v, id);
            }
        }
        self.new_vertices.retain(|k, v| keep_vertices.contains(k));
    }
    pub fn build_new_faces(&mut self) {
        for mesh in 0..2 {
            for tri in self.input_tris[mesh].iter() {
                self.new_faces[mesh].push(NewFace {
                    edge_vertices: [const { vec![] }; 3],
                    internal_vertices: vec![],
                    border_vertices: vec![],
                    edges: HashSet::new(),
                });
            }
        }
    }
    pub fn collect_face_vertices(&mut self) {
        for (&v, int) in self.new_vertices.iter() {
            let int = &self.new_vertices[&v];
            self.new_faces[int.plane_mesh][int.plane_tri]
                .internal_vertices
                .push(v);
            for wing_tri in &self
                .input_wings
                .get(&int.edge)
                .expect("expected wing")
                .wing_tris
            {
                let input_tri = self.input_tris[int.edge_mesh][*wing_tri];
                let edge_index = input_tri
                    .ordered_edges()
                    .iter()
                    .position(|e| e.edge() == int.edge)
                    .unwrap();
                self.new_faces[int.edge_mesh][*wing_tri].edge_vertices[edge_index].push(v);
            }
        }
        for mesh in 0..2 {
            for tri in 0..self.new_faces[mesh].len() {
                let input_tri = &self.input_tris[mesh][tri];
                let new_face = &mut self.new_faces[mesh][tri];
                for ei in 0..3 {
                    let ordered_edge = input_tri.ordered_edges()[ei];
                    let mul = if ordered_edge[0] < ordered_edge[1] {
                        1.0
                    } else {
                        -1.0
                    };
                    new_face.edge_vertices[ei].sort_by_cached_key(|&v| {
                        NotNan::new(mul * self.new_vertices[&v].time).unwrap()
                    });
                }
            }
        }
    }
    fn build_ordered_wings(&mut self) {
        for (&v1, int) in self.new_vertices.iter() {
            let mut result: [usize; 2] = [
                self.reverse_loop_adjacency.get(&v1).unwrap(),
                self.forward_loop_adjacency.get(&v1).unwrap(),
            ]
            .map(|&v2| match self.vertex_origins[v2] {
                VertexOrigin::Mesh(_) => unreachable!(),
                VertexOrigin::Intersect => {
                    let int2 = &self.new_vertices[&v2];
                    if int.edge_mesh == int2.edge_mesh {
                        for &tri1 in &self.input_wings.get(&int.edge).unwrap().wing_tris {
                            for &tri2 in &self.input_wings.get(&int2.edge).unwrap().wing_tris {
                                if tri1 == tri2 {
                                    return tri1;
                                }
                            }
                        }
                        unreachable!()
                    } else if int.edge_mesh == int2.plane_mesh {
                        int2.plane_tri
                    } else {
                        unreachable!()
                    }
                }
            });
            self.ordered_wings.insert(v1, result);
        }
    }
    pub fn build_polygon_edges(&mut self) {
        for mesh in 0..2 {
            for (tri, face) in &mut self.new_faces[mesh].iter_mut().enumerate() {
                for e in 0..3 {
                    face.border_vertices
                        .push(self.input_tris[mesh][tri].vertices()[e]);
                    for &v in &face.edge_vertices[e] {
                        face.border_vertices.push(v);
                    }
                }
                for (&v1, &v2) in face.border_vertices.iter().circular_tuple_windows() {
                    face.edges.insert(MeshEdge::new(v1, v2));
                }
                let vs = face
                    .edge_vertices
                    .iter()
                    .flatten()
                    .chain(face.internal_vertices.iter())
                    .cloned()
                    .collect::<HashSet<_>>();
                for &v1 in &vs {
                    let v2 = self.forward_loop_adjacency[&v1];
                    if vs.contains(&v2) {
                        face.edges.insert(MeshEdge::new(v1, v2));
                    }
                }
            }
        }
    }
    pub fn build_triangulation(&mut self) {
        for mesh in 0..2 {
            for (tri, face) in &mut self.new_faces[mesh].iter_mut().enumerate() {
                let ptri = self.input_tris[mesh][tri].for_vertices(&self.vertices);
                let mut triangulation = Triangulation::new(self.eps);
                for v in self.input_tris[mesh][tri] {
                    let mut proj = ptri.project(self.vertices[v]);
                    triangulation.add_vertex(v, proj);
                }
                for vs in &face.edge_vertices {
                    for &v in vs {
                        triangulation.add_vertex(v, ptri.project(self.vertices[v]));
                    }
                }
                for (&v1, &v2) in face.border_vertices.iter().circular_tuple_windows() {
                    triangulation.add_boundary(v1, v2);
                }
                for &v in &face.internal_vertices {
                    let proj = ptri.project(self.vertices[v]);
                    let proj_mid = ptri.project(ptri.midpoint());
                    triangulation.add_vertex(
                        v,
                        proj * (1.0 - self.eps.value()) + proj_mid * self.eps.value(),
                    );
                }
                for e in &face.edges {
                    triangulation.add_edge(e[0], e[1]);
                }
                for i in 0..3 {
                    let mut evs = vec![];
                    evs.push(self.input_tris[mesh][tri].vertices()[i]);
                    evs.extend(face.edge_vertices[i].iter().cloned());
                    evs.push(self.input_tris[mesh][tri].vertices()[(i + 1) % 3]);
                    for i1 in 0..evs.len() {
                        for i2 in i1 + 2..evs.len() {
                            triangulation.exclude_edge(evs[i1], evs[i2]);
                        }
                    }
                }
                let triangulation = triangulation.solve();
                let expected_area = ptri.area();
                let mut actual_area = 0.0;
                for &tri in &triangulation {
                    let tri3 = tri.for_vertices(&self.vertices);
                    actual_area += tri3.area();
                }
                assert!((actual_area - expected_area).abs() < 1e-10);
                let mut edge_table = HashMap::<MeshEdge, HashSet<OrderedMeshEdge>>::new();

                for (&v1, &v2) in face.border_vertices.iter().circular_tuple_windows() {
                    let e = OrderedMeshEdge::new(v2, v1);
                    assert!(edge_table.entry(e.edge()).or_default().insert(e), "{:?}", e);
                }
                for tri in &triangulation {
                    for e in tri.ordered_edges() {
                        assert!(edge_table.entry(e.edge()).or_default().insert(e), "{:?}", e);
                    }
                }
                assert!(edge_table.values().all(|x| x.len() == 2));
                for tri in triangulation {
                    self.new_tris[mesh].push(tri);
                }
            }
        }
    }
    pub fn build_connected_components(&mut self) {
        for mesh in 0..2 {
            let mut edge_to_tris = HashMap::<_, ArrayVec<_, 2>>::new();
            for (tri_index, tri) in self.new_tris[mesh].iter().enumerate() {
                for edge in tri.edges() {
                    if self.forward_loop_adjacency.get(&edge.vertices()[0])
                        == Some(&edge.vertices()[1])
                    {
                        continue;
                    }
                    if self.forward_loop_adjacency.get(&edge.vertices()[1])
                        == Some(&edge.vertices()[0])
                    {
                        continue;
                    }
                    edge_to_tris.entry(edge).or_default().push(tri_index);
                }
            }
            let mut tri_adj = HashMap::<_, Vec<_>>::new();
            for (_, tris) in edge_to_tris {
                let [t1, t2] = tris.into_inner().unwrap();
                tri_adj.entry(t1).or_default().push(t2);
                tri_adj.entry(t2).or_default().push(t1);
            }
            let mut visited = HashSet::new();
            for start in 0..self.new_tris[mesh].len() {
                if visited.contains(&start) {
                    continue;
                }
                let mut stack = vec![start];
                let mut component = vec![];
                while let Some(next) = stack.pop() {
                    if !visited.insert(next) {
                        continue;
                    }
                    component.push(self.new_tris[mesh][next]);
                    if let Some(adj) = tri_adj.get(&next) {
                        stack.extend(adj);
                    }
                }
                self.new_tri_ccs[mesh].push(component);
            }
        }
    }
    pub fn build_categories(&mut self, rng: &mut impl Rng) {
        for mesh in 0..2 {
            for cc in &self.new_tri_ccs[mesh] {
                let mut forward = false;
                let mut reverse = false;
                for &tri in cc {
                    for e in tri.ordered_edges() {
                        if self.forward_loop_adjacency.get(&e.vertices()[0])
                            == Some(&e.vertices()[1])
                        {
                            forward = true;
                        }
                        if self.forward_loop_adjacency.get(&e.vertices()[1])
                            == Some(&e.vertices()[0])
                        {
                            reverse = true;
                        }
                    }
                }
                if forward && reverse {
                    panic!("inconsistent loop ordering");
                }
                if !forward && !reverse {
                    if self.bvhs[1 - mesh]
                        .root_view(&self.input_tris[1 - mesh], &self.vertices)
                        .intersects_point(self.vertices[cc[0].vertices()[0]], self.eps, rng)
                        .round()
                    {
                        self.new_tri_cats[mesh][1].extend(cc);
                    } else {
                        self.new_tri_cats[mesh][0].extend(cc);
                    }
                } else if forward {
                    self.new_tri_cats[mesh][1 - mesh].extend(cc);
                } else if reverse {
                    self.new_tri_cats[mesh][mesh].extend(cc);
                }
            }
        }
    }
    pub fn build(&mut self, rng: &mut impl Rng) {
        self.build_relabel();
        self.build_new_vertices();
        self.build_new_edges();
        self.build_loops();
        self.build_loop_meta();
        self.build_ordered_wings();
        self.build_new_faces();
        self.collect_face_vertices();
        self.build_polygon_edges();
        self.build_triangulation();
        self.build_connected_components();
        self.build_categories(rng);
    }
    fn new_mesh(&self, mesh: usize, inside: bool) -> Mesh {
        Mesh::new(
            self.vertices.clone(),
            self.new_tri_cats[mesh][inside as usize].clone(),
        )
    }
    pub fn binop(&self, inside1: bool, inside2: bool, invert1: bool, invert2: bool) -> Mesh {
        let vertices = self.vertices.clone();
        let tris1 = self.new_tri_cats[0][inside1 as usize]
            .iter()
            .map(|&(mut tri)| {
                if invert1 {
                    tri.invert();
                }
                tri
            });
        let tris2 = self.new_tri_cats[1][inside2 as usize]
            .iter()
            .map(|&(mut tri)| {
                if invert2 {
                    tri.invert();
                }
                tri
            });
        let tris = tris1.chain(tris2).collect();
        let mesh = Mesh::new(vertices, tris);
        let mesh = mesh.without_dead_vertices();
        mesh.check_manifold().unwrap();
        mesh
    }
    pub fn union(&self) -> Mesh {
        self.binop(false, false, false, false)
    }
    pub fn intersect(&self) -> Mesh {
        self.binop(true, true, false, false)
    }
    pub fn forward_difference(&self) -> Mesh {
        self.binop(false, true, false, true)
    }
    pub fn reverse_difference(&self) -> Mesh {
        self.binop(true, false, true, false)
    }
}

#[cfg(test)]
#[tokio::test]
async fn test_tetrahedron() -> anyhow::Result<()> {
    let eps = Epsilon::new(1e-10);
    let mut mesh1 = Mesh::new(
        vec![Vec3::zero(), Vec3::axis_x(), Vec3::axis_y(), Vec3::axis_z()],
        vec![
            MeshTriangle::new(0, 2, 1),
            MeshTriangle::new(0, 3, 2),
            MeshTriangle::new(0, 1, 3),
            MeshTriangle::new(1, 2, 3),
        ],
    );
    let mut mesh2 = mesh1.clone();
    for v in mesh2.vertices_mut() {
        *v += Vec3::new(0.25, 0.25, 0.25);
    }
    let mut bimesh = Bimesh::new(&mesh1, &mesh2, eps, &mut rng());
    // bimesh.build();
    let dir = PathBuf::from("../")
        .join("target")
        .join("test_outputs")
        .join("test_tetrahedron2");
    tokio::fs::create_dir_all(&dir).await?;
    write_stl_file(&bimesh.new_mesh(0, false), &dir.join("mesh1_outside.stl")).await?;
    write_stl_file(&bimesh.new_mesh(0, true), &dir.join("mesh1_inside.stl")).await?;
    write_stl_file(&bimesh.new_mesh(1, false), &dir.join("mesh2_outside.stl")).await?;
    write_stl_file(&bimesh.new_mesh(1, true), &dir.join("mesh2_inside.stl")).await?;

    write_stl_file(&bimesh.union(), &dir.join("union.stl")).await?;
    write_stl_file(&bimesh.intersect(), &dir.join("intersect.stl")).await?;
    write_stl_file(
        &bimesh.forward_difference(),
        &dir.join("forward_difference.stl"),
    )
    .await?;
    write_stl_file(
        &bimesh.reverse_difference(),
        &dir.join("reverse_difference.stl"),
    )
    .await?;
    Ok(())
}

fn rand_tetr(rng: &mut XorShiftRng, steps: usize) -> Mesh {
    loop {
        let mut vs = (0..4)
            .map(|_| {
                Vec3::new(
                    rng.random_range(0..steps) as f64 / (steps as f64),
                    rng.random_range(0..steps) as f64 / (steps as f64),
                    rng.random_range(0..steps) as f64 / (steps as f64),
                )
            })
            .collect::<Vec<Vec3>>();
        let volume = (vs[1] - vs[0]).cross(vs[2] - vs[0]).dot(vs[3] - vs[0]);
        if volume < -1e-10 {
            vs.swap(2, 3);
        } else if volume < 1e-10 {
            continue;
        }
        let mut ts = vec![
            MeshTriangle::new(0, 2, 1),
            MeshTriangle::new(0, 3, 2),
            MeshTriangle::new(0, 1, 3),
            MeshTriangle::new(1, 2, 3),
        ];
        ts.shuffle(rng);
        return Mesh::new(vs, ts);
    }
}

#[tokio::test]
async fn test_random() -> anyhow::Result<()> {
    let eps = Epsilon::new(1e-10);
    let dir = PathBuf::from("../")
        .join("target")
        .join("test_outputs")
        .join("test_random");
    tokio::fs::create_dir_all(&dir).await?;

    for steps in 3..100 {
        println!("steps={:?}", steps);
        for seed in 2..1000 {
            println!("seed={:?}", seed);
            let mut rng = XorShiftRng::seed_from_u64(seed);
            let m1 = rand_tetr(&mut rng, steps);
            let m2 = rand_tetr(&mut rng, steps);
            write_stl_file(&m1, &dir.join("mesh1.stl")).await?;
            write_stl_file(&m2, &dir.join("mesh2.stl")).await?;
            let bm = Bimesh::new(&m1, &m2, eps, &mut rng);
            write_stl_file(&bm.new_mesh(0, false), &dir.join("mesh1_outside.stl")).await?;
            write_stl_file(&bm.new_mesh(0, true), &dir.join("mesh1_inside.stl")).await?;
            write_stl_file(&bm.new_mesh(1, false), &dir.join("mesh2_outside.stl")).await?;
            write_stl_file(&bm.new_mesh(1, true), &dir.join("mesh2_inside.stl")).await?;
            let union = bm.union();
            let inter = bm.intersect();
            write_stl_file(&union, &dir.join("union.stl")).await?;
            write_stl_file(&inter, &dir.join("inter.stl")).await?;
            for i in 2..100 {
                let p: Vec3 = rng.random();
                let in_m1 = m1.intersects_point(p, eps, &mut rng);
                let in_m2 = m2.intersects_point(p, eps, &mut rng);
                let in_union = union.intersects_point(p, eps, &mut rng);
                let in_inter = inter.intersects_point(p, eps, &mut rng);
                assert!(in_union.matches(in_m1.or(in_m2)));
                assert!(in_inter.matches(in_m1.and(in_m2)));
            }

            // for m in 0..2 {
            //     for (cci, cc) in bm.new_tri_ccs[m].iter().enumerate() {
            //         let mesh = Mesh::new(bm.vertices.clone(), cc.clone());
            //         write_stl_file(
            //             &mesh,
            //             &PathBuf::from(format!("../target/mesh_{}_cc_{}.stl", m, cci)),
            //         )
            //         .await?
            //     }
            // }
        }
    }
    Ok(())
}
