use crate::bvh::Bvh;
use crate::geo2::segment2::Segment2;
use crate::math::vec3::Vec3;
use crate::meshes::mesh::Mesh;
use crate::meshes::mesh_edge::MeshEdge;
use crate::meshes::mesh_triangle::MeshTriangle;
use crate::stl::write_stl_file;
use crate::util::sorted_pair::SortedPair;
use arrayvec::ArrayVec;
use itertools::Itertools;
use ordered_float::NotNan;
use std::collections::{BTreeMap, HashMap, HashSet, hash_set};
use std::path::PathBuf;

pub struct BimeshTriangle {
    source: usize,
    inside: bool,
    triangle: MeshTriangle,
}

pub struct Bimesh {
    vertices: Vec<Vec3>,
    tris: Vec<BimeshTriangle>,
}

impl BimeshTriangle {
    pub fn source(&self) -> usize {
        self.source
    }
    pub fn inside(&self) -> bool {
        self.inside
    }
    pub fn triangle(&self) -> &MeshTriangle {
        &self.triangle
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct VertexType {
    edge: Option<SortedPair<usize>>,
}

struct VertexBuilder {
    vertices: Vec<Vec3>,
    vertex_types: Vec<VertexType>,
}

struct MeshBuilder<'a> {
    mesh: &'a Mesh,
    // mesh_index: usize,
    tris: Vec<MeshTriangle>,
    bvh: Bvh,
    new_vertices: HashMap<SortedPair<usize>, HashMap<usize, Option<usize>>>,
    new_edges: HashMap<usize, HashSet<SortedPair<usize>>>,
    out_tris: Vec<MeshTriangle>,
}

impl VertexBuilder {
    pub fn new(mesh1: &Mesh, mesh2: &Mesh) -> Self {
        let mut vertices: Vec<Vec3> = mesh1
            .vertices()
            .iter()
            .cloned()
            .chain(mesh2.vertices().iter().cloned())
            .collect();
        let vertex_types = vec![VertexType { edge: None }; vertices.len()];
        VertexBuilder {
            vertices,
            vertex_types,
        }
    }
    pub fn build_partial_edge(
        &mut self,
        mesh1: &mut MeshBuilder,
        mesh2: &mut MeshBuilder,
        t1: usize,
        t2: usize,
        result: &mut ArrayVec<usize, 2>,
    ) {
        for e1 in mesh1.tris[t1].edges() {
            if let Some(v) = mesh1
                .new_vertices
                .entry(e1.sorted())
                .or_default()
                .entry(t2)
                .or_insert_with(|| {
                    let e1s = e1.for_vertices(&self.vertices);
                    mesh2.tris[t2]
                        .for_vertices(&self.vertices)
                        .intersect_segment(&e1s)
                        .map(|time| {
                            self.vertices.push(e1s.at_time(time));
                            self.vertex_types.push(VertexType {
                                edge: Some(e1.sorted()),
                            });
                            self.vertices.len() - 1
                        })
                })
            {
                result.push(*v)
            }
        }
    }
}

impl<'a> MeshBuilder<'a> {
    pub fn new(mesh: &'a Mesh, offset: usize) -> Self {
        let tris: Vec<MeshTriangle> = mesh
            .triangles()
            .iter()
            .map(|t| t.vertices().map(|v| v + offset).into())
            .collect();
        let bvh = Bvh::from_mesh(mesh);
        MeshBuilder {
            mesh,
            tris,
            bvh,
            new_vertices: HashMap::new(),
            new_edges: HashMap::new(),
            out_tris: vec![],
        }
    }
    pub fn add_edge(&mut self, t: usize, e: SortedPair<usize>) {
        self.new_edges.entry(t).or_default().insert(e);
    }
    pub fn build_tris(&mut self, vertices: &VertexBuilder) {
        for (ti, mt) in self.tris.iter().enumerate() {
            let edges = self.new_edges.entry(ti).or_default();
            for edge in self.tris[ti].edges() {
                let mut edge_seq = edge.vertices().to_vec();
                if let Some(new) = self.new_vertices.get(&edge.sorted()) {
                    for vo in new.values() {
                        if let Some(v) = vo {
                            edge_seq.push(*v);
                        }
                    }
                }
                let proj = edge.for_vertices(&vertices.vertices).as_ray();
                edge_seq.sort_by_cached_key(|v| {
                    NotNan::new(proj.project(vertices.vertices[*v])).unwrap()
                });
                for e in edge_seq.array_windows::<2>() {
                    edges.insert(SortedPair::from(*e));
                }
            }
            let tri = mt.for_vertices(&vertices.vertices);
            let mut projections = HashMap::new();
            for edge in edges.iter() {
                for v in edge.into_inner() {
                    projections
                        .entry(v)
                        .or_insert_with(|| tri.project(vertices.vertices[v]));
                }
            }
            let mut missing_edges = HashSet::new();

            for (&v1, &p1) in projections.iter() {
                for (&v2, &p2) in projections.iter() {
                    let e = SortedPair::new(v1, v2);
                    if v1 != v2
                        && (vertices.vertex_types[v1].edge != vertices.vertex_types[v2].edge)
                        && !edges.contains(&e)
                    {
                        missing_edges.insert(e);
                    }
                }
            }
            let mut missing_edges = missing_edges.into_iter().collect::<Vec<_>>();
            missing_edges.sort_by_cached_key(|e| {
                let d =
                    NotNan::new(projections[e.first()].distance(projections[e.second()])).unwrap();
                (d, *e.first(), *e.second())
            });
            for missing in missing_edges {
                let s1 = Segment2::new(projections[missing.first()], projections[missing.second()]);
                if !edges.iter().any(|extant| {
                    let s2 =
                        Segment2::new(projections[extant.first()], projections[extant.second()]);
                    if missing.first() == extant.first()
                        || missing.first() == extant.second()
                        || missing.second() == extant.first()
                        || missing.second() == extant.second()
                    {
                        return false;
                    }
                    s1.intersect_time(s2).is_some()
                }) {
                    edges.insert(missing);
                }
            }
            let mut tris: HashSet<[usize; 3]> = HashSet::new();
            let mut adjacency_map = BTreeMap::<usize, HashSet<usize>>::new();
            for e in edges.iter() {
                adjacency_map
                    .entry(*e.first())
                    .or_default()
                    .insert(*e.second());
                adjacency_map
                    .entry(*e.second())
                    .or_default()
                    .insert(*e.first());
            }
            for (&v1, v2s) in adjacency_map.iter() {
                let mut v2v = v2s.iter().cloned().collect::<Vec<usize>>();
                v2v.sort_by_cached_key(|&v2| {
                    NotNan::new((projections[&v2] - projections[&v1]).angle()).unwrap()
                });
                for i in 0..v2v.len() {
                    let v2 = v2v[i];
                    let v3 = v2v[(i + 1) % v2v.len()];
                    if adjacency_map.get(&v2).unwrap().contains(&v3) {
                        let mut t = [v1, v2, v3];
                        t.sort();
                        tris.insert(t);
                    }
                }
            }
            if tris.len() > 1 {
                let mut major = mt.vertices();
                major.sort();
                tris.remove(&major);
            }
            for [v1, v2, v3] in tris {
                let p1 = *projections.get(&v1).unwrap();
                let p2 = *projections.get(&v2).unwrap();
                let p3 = *projections.get(&v3).unwrap();
                let mut t = MeshTriangle::new(v1, v2, v3);
                if (p2 - p1).cross(p3 - p1) < 0.0 {
                    t.invert();
                }
                self.out_tris.push(t);
            }
        }
    }
}

impl Bimesh {
    pub fn new(mesh1: &Mesh, mesh2: &Mesh) -> Self {
        let mut vertices = VertexBuilder::new(mesh1, mesh2);
        let mut mesh1 = MeshBuilder::new(mesh1, 0);
        let mut mesh2 = MeshBuilder::new(mesh2, mesh1.tris.len());
        let intersections = mesh1.bvh.intersect_bvh(&mesh2.bvh);
        for &(t1, t2) in &intersections {
            let mut vs = ArrayVec::<usize, 2>::new();
            vertices.build_partial_edge(&mut mesh1, &mut mesh2, t1, t2, &mut vs);
            vertices.build_partial_edge(&mut mesh2, &mut mesh1, t2, t1, &mut vs);
            let vs = MeshEdge::from(vs.into_inner().unwrap()).sorted();
            mesh1.add_edge(t1, vs);
            mesh2.add_edge(t2, vs);
        }
        mesh1.build_tris(&vertices);
        mesh2.build_tris(&vertices);
        let mut tris = vec![];
        for tri in mesh1.out_tris {
            tris.push(BimeshTriangle {
                source: 0,
                inside: mesh2
                    .bvh
                    .intersects_point(tri.for_vertices(&vertices.vertices).midpoint()),
                triangle: tri,
            });
        }
        for tri in mesh2.out_tris {
            tris.push(BimeshTriangle {
                source: 1,
                inside: mesh1
                    .bvh
                    .intersects_point(tri.for_vertices(&vertices.vertices).midpoint()),
                triangle: tri,
            });
        }
        Bimesh {
            vertices: vertices.vertices,
            tris,
        }
    }
    pub fn mesh_part(&self, source: usize, inside: bool) -> Mesh {
        Mesh::new(
            self.vertices.clone(),
            self.tris
                .iter()
                .filter_map(|t| (t.source == source && t.inside == inside).then_some(t.triangle))
                .collect(),
        )
    }
}

#[tokio::test]
async fn test_bimesh() -> anyhow::Result<()> {
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
    let bimesh = Bimesh::new(&mesh1, &mesh2);
    let dir = PathBuf::from("../")
        .join("target")
        .join("test_outputs")
        .join("test_bimesh");
    tokio::fs::create_dir_all(&dir).await?;
    write_stl_file(&bimesh.mesh_part(0, true), &dir.join("mesh1_inside.stl")).await?;
    write_stl_file(&bimesh.mesh_part(0, false), &dir.join("mesh1_outside.stl")).await?;
    write_stl_file(&bimesh.mesh_part(1, true), &dir.join("mesh2_inside.stl")).await?;
    write_stl_file(&bimesh.mesh_part(1, false), &dir.join("mesh2_outside.stl")).await?;
    Ok(())
}
