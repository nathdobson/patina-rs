use crate::geo3::ray3::Ray3;
use crate::math::vec3::Vec3;
use crate::meshes::bvh::Bvh;
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

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash, Debug)]
pub struct IntersectVertex {
    edge_mesh: usize,
    edge: MeshEdge,
    tri_mesh: usize,
    tri: usize,
}

#[derive(Debug)]
struct NewFace {
    edge_vertices: [Vec<usize>; 3],
    internal_vertices: Vec<usize>,
    edges: HashSet<MeshEdge>,
    // polygon_edges: [LoopBuilder<usize>; 2],
    // exterior_polygons: Vec<MeshPolygon>,
    // internal_polygons: Vec<MeshPolygon>,
}

enum VertexOrigin {
    Mesh(usize),
    Intersect(IntersectVertex),
}

pub struct Bimesh<'a> {
    meshes: [&'a Mesh; 2],
    bvhs: [Bvh; 2],
    vertices: Vec<Vec3>,
    vertex_origins: Vec<VertexOrigin>,
    input_tris: [Vec<MeshTriangle>; 2],
    input_wings: HashMap<MeshEdge, BimeshWings>,
    new_vertices: HashMap<IntersectVertex, usize>,
    edge_times: HashMap<usize, f64>,
    edge_orders: HashMap<MeshEdge, Vec<usize>>,
    new_loop_edges: Vec<OrderedMeshEdge>,
    forward_loop_adjacency: HashMap<usize, usize>,
    reverse_loop_adjacency: HashMap<usize, usize>,
    loops: Vec<Vec<usize>>,
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
    fn new_uninit(mesh1: &'a Mesh, mesh2: &'a Mesh) -> Self {
        let meshes = [mesh1, mesh2];
        Bimesh {
            bvhs: meshes.map(Bvh::from_mesh),
            vertices: vec![],
            vertex_origins: vec![],
            meshes,
            input_tris: [vec![], vec![]],
            input_wings: HashMap::new(),
            new_vertices: HashMap::new(),
            edge_times: HashMap::new(),
            edge_orders: HashMap::new(),
            new_loop_edges: vec![],
            forward_loop_adjacency: HashMap::new(),
            reverse_loop_adjacency: HashMap::new(),
            loops: vec![],
            loop_ids: HashMap::new(),
            ordered_wings: HashMap::new(),
            new_faces: [const { vec![] }; 2],
            new_tris: [const { vec![] }; 2],
            new_tri_ccs: [const { vec![] }; 2],
            new_tri_cats: [const { [const { vec![] }; 2] }; 2],
        }
    }
    pub fn new(mesh1: &'a Mesh, mesh2: &'a Mesh, rng: &mut impl Rng) -> Self {
        let mut this = Self::new_uninit(mesh1, mesh2);
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
    pub fn build_new_edges(&mut self) {
        let tri_pairs = self.bvhs[0].intersect_bvh(&self.bvhs[1]);
        for (tri1, tri2) in tri_pairs {
            let mut new_edge = ArrayVec::<_, 2>::new();
            for (mesh1, mesh2, tri1, tri2) in [(0, 1, tri1, tri2), (1, 0, tri2, tri1)] {
                for ordered_edge in self.input_tris[mesh1][tri1].ordered_edges() {
                    let edge = ordered_edge.edge();
                    let segment = edge.for_vertices(&self.vertices);
                    if let Some(time) = self.input_tris[mesh2][tri2]
                        .for_vertices(&self.vertices)
                        .intersect_segment(&segment)
                    {
                        let intersect = IntersectVertex {
                            edge_mesh: mesh1,
                            edge,
                            tri_mesh: mesh2,
                            tri: tri2,
                        };
                        let new_vertex = *self.new_vertices.entry(intersect).or_insert_with(|| {
                            let vert = self.vertices.len();
                            self.vertices.push(segment.at_time(time));
                            self.vertex_origins.push(VertexOrigin::Intersect(intersect));
                            self.edge_times.insert(vert, time);
                            vert
                        });
                        new_edge.push(new_vertex);
                    }
                }
            }
            let Ok(new_edge) = new_edge.clone().into_inner() else {
                println!("{}", self.input_tris[0][tri1].for_vertices(&self.vertices));
                println!("{}", self.input_tris[1][tri2].for_vertices(&self.vertices));
                panic!("{:?}", new_edge);
            };
            let mut edge = OrderedMeshEdge::from(new_edge);
            let tri1n = self.input_tris[0][tri1]
                .for_vertices(&self.vertices)
                .normal();
            let tri2n = self.input_tris[1][tri2]
                .for_vertices(&self.vertices)
                .normal();
            let edge_dir = edge.for_vertices(&self.vertices).as_ray().dir();
            if tri1n.cross(tri2n).dot(edge_dir) < 0.0 {
                edge.invert();
            }
            self.new_loop_edges.push(edge);
        }
    }
    pub fn build_edge_orders(&mut self) {
        for (int, &v) in &self.new_vertices {
            self.edge_orders.entry(int.edge).or_default().push(v);
        }
        for vs in self.edge_orders.values_mut() {
            vs.sort_by_cached_key(|&v| {
                NotNan::new(*self.edge_times.get(&v).expect("missing time")).unwrap()
            });
        }
    }
    pub fn build_loops(&mut self) {
        for edge in &self.new_loop_edges {
            assert!(
                self.forward_loop_adjacency
                    .insert(edge[0], edge[1])
                    .is_none()
            );
            assert!(
                self.reverse_loop_adjacency
                    .insert(edge[1], edge[0])
                    .is_none()
            );
        }
        let mut loop_adjacency = self.forward_loop_adjacency.clone();
        while let Some(&start) = loop_adjacency.keys().next() {
            let mut seq = vec![];
            let mut next = start;
            loop {
                seq.push(next);
                next = loop_adjacency
                    .remove(&next)
                    .expect("intersection does not form loops.");
                if next == start {
                    break;
                }
            }
            for i in &seq {
                self.loop_ids.insert(*i, self.loops.len());
            }
            self.loops.push(seq);
        }
    }
    pub fn build_new_faces(&mut self) {
        for mesh in 0..2 {
            for tri in self.input_tris[mesh].iter() {
                self.new_faces[mesh].push(NewFace {
                    edge_vertices: [const { vec![] }; 3],
                    internal_vertices: vec![],
                    // polygon_edges: [LoopBuilder::new(), LoopBuilder::new()],
                    // exterior_polygons: vec![],
                    // internal_polygons: vec![],
                    edges: HashSet::new(),
                });
            }
        }
    }
    pub fn collect_face_vertices(&mut self) {
        for (v, vo) in self.vertex_origins.iter().enumerate() {
            match vo {
                VertexOrigin::Mesh(_) => {}
                VertexOrigin::Intersect(int) => {
                    self.new_faces[int.tri_mesh][int.tri]
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
                        NotNan::new(mul * *self.edge_times.get(&v).expect("missing time")).unwrap()
                    });
                }
            }
        }
    }
    fn build_ordered_wings(&mut self) {
        for (int, &v1) in self.new_vertices.iter() {
            let mut result: [usize; 2] = [
                self.reverse_loop_adjacency.get(&v1).unwrap(),
                self.forward_loop_adjacency.get(&v1).unwrap(),
            ]
            .map(|&v2| match self.vertex_origins[v2] {
                VertexOrigin::Mesh(_) => unreachable!(),
                VertexOrigin::Intersect(int2) => {
                    if int.edge_mesh == int2.edge_mesh {
                        for &tri1 in &self.input_wings.get(&int.edge).unwrap().wing_tris {
                            for &tri2 in &self.input_wings.get(&int2.edge).unwrap().wing_tris {
                                if tri1 == tri2 {
                                    return tri1;
                                }
                            }
                        }
                        unreachable!()
                    } else if int.edge_mesh == int2.tri_mesh {
                        int2.tri
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
                    let mut prev = self.input_tris[mesh][tri].vertices()[e];
                    for &next in &face.edge_vertices[e] {
                        face.edges.insert(MeshEdge::new(prev, next));
                        prev = next;
                    }
                    let next = self.input_tris[mesh][tri].vertices()[(e + 1) % 3];
                    face.edges.insert(MeshEdge::new(prev, next));
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
                // for e in 0..3 {
                //     for &nv in &face.edge_vertices[e] {
                //         let ordered_wing = self.ordered_wings.get(&nv).unwrap();
                //         if ordered_wing[0] == tri {
                //             continue;
                //         }
                //         assert_eq!(ordered_wing[1], tri);
                //         let mut prev = nv;
                //         loop {
                //             let next = *self.forward_loop_adjacency.get(&prev).unwrap();
                //             assert_ne!(next, nv);
                //             face.edges.insert(MeshEdge::new(prev, next));
                //             match self.vertex_origins[next] {
                //                 VertexOrigin::Mesh(_) => unreachable!(),
                //                 VertexOrigin::Intersect(int) => {
                //                     if int.edge_mesh == mesh {
                //                         break;
                //                     }
                //                 }
                //             }
                //             prev = next;
                //         }
                //     }
                // }
            }
        }
    }
    pub fn build_triangulation(&mut self) {
        for mesh in 0..2 {
            for (tri, face) in &mut self.new_faces[mesh].iter_mut().enumerate() {
                let ptri = self.input_tris[mesh][tri].for_vertices(&self.vertices);
                let mut triangulation = Triangulation::new();
                for v in self.input_tris[mesh][tri] {
                    triangulation.add_vertex(v, ptri.project(self.vertices[v]));
                    triangulation.add_boundary(v);
                }
                for vs in &face.edge_vertices {
                    for &v in vs {
                        triangulation.add_vertex(v, ptri.project(self.vertices[v]));
                        triangulation.add_boundary(v);
                    }
                }
                for &v in &face.internal_vertices {
                    triangulation.add_vertex(v, ptri.project(self.vertices[v]));
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
                    if self.bvhs[1 - mesh].intersects_point(self.vertices[cc[0].vertices()[0]], rng)
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
        self.build_new_edges();
        self.build_edge_orders();
        self.build_loops();
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
    let mut bimesh = Bimesh::new(&mesh1, &mesh2, &mut rng());
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
    let mut vs = (0..4)
        .map(|_| {
            Vec3::new(
                rng.random_range(0..steps) as f64 / (steps as f64),
                rng.random_range(0..steps) as f64 / (steps as f64),
                rng.random_range(0..steps) as f64 / (steps as f64),
            )
        })
        .collect::<Vec<Vec3>>();
    if (vs[1] - vs[0]).cross(vs[2] - vs[0]).dot(vs[3] - vs[0]) < 0.0 {
        vs.swap(2, 3);
    }
    let mut ts = vec![
        MeshTriangle::new(0, 2, 1),
        MeshTriangle::new(0, 3, 2),
        MeshTriangle::new(0, 1, 3),
        MeshTriangle::new(1, 2, 3),
    ];
    ts.shuffle(rng);
    Mesh::new(vs, ts)
}

#[tokio::test]
async fn test() -> anyhow::Result<()> {
    let dir = PathBuf::from("../")
        .join("target")
        .join("test_outputs")
        .join("test_random");
    tokio::fs::create_dir_all(&dir).await?;

    for seed in 0..1000 {
        for steps in 2..100 {
            println!("{:?}", seed);
            let mut rng = XorShiftRng::seed_from_u64(seed);
            let m1 = rand_tetr(&mut rng, steps);
            let m2 = rand_tetr(&mut rng, steps);
            // write_stl_file(&m1, &dir.join("mesh1.stl")).await?;
            // write_stl_file(&m2, &dir.join("mesh2.stl")).await?;
            let bm = Bimesh::new(&m1, &m2, &mut rng);
            let union = bm.union();
            let inter = bm.intersect();
            // write_stl_file(&union, &dir.join("union.stl")).await?;
            // write_stl_file(&inter, &dir.join("inter.stl")).await?;
            for i in 2..100 {
                let p: Vec3 = rng.random();
                let in_m1 = m1.intersects_point(p, &mut rng);
                let in_m2 = m2.intersects_point(p, &mut rng);
                let in_union = union.intersects_point(p, &mut rng);
                let in_inter = inter.intersects_point(p, &mut rng);
                assert_eq!(in_union, in_m1 || in_m2);
                assert_eq!(in_inter, in_m1 && in_m2);
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
