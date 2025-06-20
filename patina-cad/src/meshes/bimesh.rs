use crate::geo2::segment2::Segment2;
use crate::geo2::triangle2::Triangle2;
use crate::geo3::ray3::Ray3;
use crate::math::vec3::Vec3;
use crate::meshes::bvh::Bvh;
use crate::meshes::mesh::Mesh;
use crate::meshes::mesh_edge::MeshEdge;
use crate::meshes::mesh_triangle::MeshTriangle;
use crate::ser::stl::write_stl_file;
use crate::util::sorted_pair::SortedPair;
use arrayvec::ArrayVec;
use itertools::Itertools;
use ordered_float::NotNan;
use rand::Rng;
use std::collections::{BTreeMap, HashMap, HashSet, hash_set};
use std::iter;
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

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct VertexType {
    edge: Option<SortedPair<usize>>,
}

struct VertexBuilder {
    vertices: Vec<Vec3>,
    vertex_types: Vec<VertexType>,
}

struct BimeshBuilder<'a> {
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
        mesh1: &mut BimeshBuilder,
        mesh2: &mut BimeshBuilder,
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

impl<'a> BimeshBuilder<'a> {
    pub fn new(mesh: &'a Mesh, offset: usize) -> Self {
        let tris: Vec<MeshTriangle> = mesh
            .triangles()
            .iter()
            .map(|t| t.vertices().map(|v| v + offset).into())
            .collect();
        let bvh = Bvh::from_mesh(mesh);
        BimeshBuilder {
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
    pub fn build_tris(
        &mut self,
        vertices: &VertexBuilder,
        source: usize,
        mesh2: &Self,
        bimesh_tris: &mut Vec<BimeshTriangle>,
    ) {
        for (ti, mt) in self.tris.iter().enumerate() {
            let edges = self.new_edges.entry(ti).or_default();
            let orig_edges = edges.clone();
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
                    if v1 == v2 {
                        continue;
                    }
                    match (
                        vertices.vertex_types[v1].edge,
                        vertices.vertex_types[v2].edge,
                    ) {
                        (None, None) => continue,
                        (Some(e1), None) => {
                            if *e1.first() == v2 || *e1.second() == v2 {
                                continue;
                            }
                        }
                        (None, Some(e2)) => {
                            if *e2.first() == v1 || *e2.second() == v1 {
                                continue;
                            }
                        }
                        (Some(e1), Some(e2)) => {
                            if e1 == e2 {
                                continue;
                            }
                        }
                    }
                    if edges.contains(&e) {
                        continue;
                    }
                    missing_edges.insert(e);
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
                        let t2 =
                            Triangle2::new([projections[&v1], projections[&v2], projections[&v3]]);
                        if projections.iter().any(|(&vx, &p)| {
                            vx != v1 && vx != v2 && vx != v3 && t2.intersects_point(p)
                        }) {
                            continue;
                        }
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
            // println!("{:?}", tris);
            assert!(tris.len() > 0);
            let expected_area = mt.for_vertices(&vertices.vertices).area();
            let mut actual_area = 0.0;
            for &[v1, v2, v3] in &tris {
                let p1 = *projections.get(&v1).unwrap();
                let p2 = *projections.get(&v2).unwrap();
                let p3 = *projections.get(&v3).unwrap();
                let mut t = MeshTriangle::new(v1, v2, v3);
                if (p2 - p1).cross(p3 - p1) < 0.0 {
                    t.invert();
                }
                let area = t.for_vertices(&vertices.vertices).area();
                if area < 1e-15 {
                    println!("{:?}", mt);
                    println!("{:?}", mt.for_vertices(&vertices.vertices).area());
                    for &v in adjacency_map.keys() {
                        println!("{:?} -> {:?}", v, vertices.vertex_types[v]);
                    }
                    println!("{:?}", orig_edges);
                    println!("{:?}", edges);
                    println!("{:?}", adjacency_map);
                    println!("{:?}", tris);
                    panic!(
                        "Empty triangle {:?} {:?} {:?} {:?}",
                        [p1, p2, p3, p2 - p1, p3 - p1],
                        (p2 - p1).cross(p3 - p1),
                        t.for_vertices(&vertices.vertices),
                        area
                    );
                }
                actual_area += area;
                self.out_tris.push(t);
            }
            println!("area diff {:?}", expected_area - actual_area);
        }
        for tri in &self.out_tris {
            let tri_vs = tri.for_vertices(&vertices.vertices);
            let ray = Ray3::new(tri_vs.midpoint(), Vec3::new(0.123, 0.333, 0.11));
            bimesh_tris.push(BimeshTriangle {
                source,
                inside: mesh2.bvh.intersect_ray(&ray).len() % 2 == 1,
                triangle: *tri,
            });
        }
    }
}

impl Bimesh {
    pub fn new(mesh1: &Mesh, mesh2: &Mesh) -> Self {
        mesh1.check_manifold().unwrap();
        mesh2.check_manifold().unwrap();
        let mut vertices = VertexBuilder::new(mesh1, mesh2);
        let offset = mesh1.vertices().len();
        let mut mesh1 = BimeshBuilder::new(mesh1, 0);
        let mut mesh2 = BimeshBuilder::new(mesh2, offset);
        let intersections = mesh1.bvh.intersect_bvh(&mesh2.bvh);
        for &(t1, t2) in &intersections {
            let mut vs = ArrayVec::<usize, 2>::new();
            vertices.build_partial_edge(&mut mesh1, &mut mesh2, t1, t2, &mut vs);
            vertices.build_partial_edge(&mut mesh2, &mut mesh1, t2, t1, &mut vs);
            if let Ok(vs) = vs.into_inner() {
                let vs = MeshEdge::from(vs).sorted();
                mesh1.add_edge(t1, vs);
                mesh2.add_edge(t2, vs);
            }
        }
        let mut tris = vec![];
        mesh1.build_tris(&vertices, 0, &mesh2, &mut tris);
        mesh2.build_tris(&vertices, 1, &mesh1, &mut tris);
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
    pub fn mesh_part_all(&self, source: usize) -> Mesh {
        Mesh::new(
            self.vertices.clone(),
            self.tris
                .iter()
                .filter_map(|t| (t.source == source).then_some(t.triangle))
                .collect(),
        )
    }
    fn binop(&self, inside1: bool, inside2: bool, invert1: bool, invert2: bool) -> Mesh {
        let mesh = Mesh::new(
            self.vertices.clone(),
            self.tris
                .iter()
                .filter_map(|t| {
                    let inside;
                    let invert;
                    if t.source == 0 {
                        inside = inside1;
                        invert = invert1;
                    } else {
                        inside = inside2;
                        invert = invert2;
                    }
                    let mut tri = *t.triangle();
                    if invert {
                        tri.invert()
                    }
                    (inside == t.inside).then_some(tri)
                })
                .collect(),
        );
        let mesh = mesh.without_dead_vertices();
        for i1 in 0..mesh.vertices().len() {
            for i2 in i1 + 1..mesh.vertices().len() {
                let d = mesh.vertices()[i1].distance(mesh.vertices()[i2]);
                if d < 1e-10 {
                    println!("similar vertex {} ~= {} ({})", i1, i2, d);
                }
            }
        }
        println!("{:#?}", mesh);
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

#[cfg(test)]
#[tokio::test]
async fn test_triangle() -> anyhow::Result<()> {
    let mut mesh1 = Mesh::new(
        vec![Vec3::axis_x(), Vec3::axis_y(), Vec3::axis_z()],
        vec![MeshTriangle::new(0, 1, 2)],
    );
    let mut mesh2 = Mesh::new(
        vec![
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(0.2, 0.1, 0.5),
            Vec3::new(0.1, 0.2, 0.5),
        ],
        vec![MeshTriangle::new(0, 1, 2)],
    );
    let bimesh = Bimesh::new(&mesh1, &mesh2);
    let dir = PathBuf::from("../")
        .join("target")
        .join("test_outputs")
        .join("test_triangle");
    tokio::fs::create_dir_all(&dir).await?;
    write_stl_file(&bimesh.mesh_part_all(0), &dir.join("mesh1.stl")).await?;
    write_stl_file(&bimesh.mesh_part_all(1), &dir.join("mesh2.stl")).await?;
    Ok(())
}

#[cfg(test)]
#[tokio::test]
async fn test_triangle1() -> anyhow::Result<()> {
    let mut mesh1 = Mesh::new(
        vec![Vec3::axis_x(), Vec3::axis_y(), Vec3::axis_z()],
        vec![MeshTriangle::new(0, 1, 2)],
    );
    let mut mesh2 = Mesh::new(
        vec![
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(0.2, 0.1, 0.5),
            Vec3::new(0.1, 0.2, 0.5),
        ],
        vec![MeshTriangle::new(0, 1, 2)],
    );
    let bimesh = Bimesh::new(&mesh1, &mesh2);
    let dir = PathBuf::from("../")
        .join("target")
        .join("test_outputs")
        .join("test_triangle1");
    tokio::fs::create_dir_all(&dir).await?;
    write_stl_file(&bimesh.mesh_part_all(0), &dir.join("mesh1.stl")).await?;
    write_stl_file(&bimesh.mesh_part_all(1), &dir.join("mesh2.stl")).await?;
    Ok(())
}

#[cfg(test)]
#[tokio::test]
async fn test_triangle2() -> anyhow::Result<()> {
    let mut mesh1 = Mesh::new(
        vec![Vec3::axis_x(), Vec3::axis_y(), Vec3::axis_z()],
        vec![MeshTriangle::new(0, 1, 2)],
    );
    let mut mesh2 = Mesh::new(
        vec![
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(0.2, 0.1, 0.5),
            Vec3::new(1.0, -1.0, 0.5),
        ],
        vec![MeshTriangle::new(0, 1, 2)],
    );
    let bimesh = Bimesh::new(&mesh1, &mesh2);
    let dir = PathBuf::from("../")
        .join("target")
        .join("test_outputs")
        .join("test_triangle2");
    tokio::fs::create_dir_all(&dir).await?;
    write_stl_file(&bimesh.mesh_part_all(0), &dir.join("mesh1.stl")).await?;
    write_stl_file(&bimesh.mesh_part_all(1), &dir.join("mesh2.stl")).await?;
    Ok(())
}

#[cfg(test)]
#[tokio::test]
async fn test_tetrahedron_random() -> anyhow::Result<()> {
    use rand::SeedableRng;
    use rand_xorshift::XorShiftRng;

    for seed in 0..100000 {
        println!("seed {:?}", seed);
        let mut rng = XorShiftRng::seed_from_u64(seed);
        let max = 1000000.0;
        let range = 0.0f64..max;
        let meshes = iter::repeat_with(|| {
            let mut vs: Vec<Vec3> = iter::repeat_with(|| {
                Vec3::new(
                    rng.random_range(range.clone()).round() / max,
                    rng.random_range(range.clone()).round() / max,
                    rng.random_range(range.clone()).round() / max,
                )
            })
            .take(4)
            .collect();
            let d1 = vs[1] - vs[0];
            let d2 = vs[2] - vs[0];
            let d3 = vs[3] - vs[0];
            if d1.cross(d2).dot(d3) < 0.0 {
                vs.swap(2, 3);
            }
            Mesh::new(
                vs,
                vec![
                    MeshTriangle::new(0, 2, 1),
                    MeshTriangle::new(0, 3, 2),
                    MeshTriangle::new(0, 1, 3),
                    MeshTriangle::new(1, 2, 3),
                ],
            )
        })
        .take(2)
        .collect::<Vec<_>>();
        let mesh1 = &meshes[0];
        let mesh2 = &meshes[1];
        let dir = PathBuf::from("../")
            .join("target")
            .join("test_outputs")
            .join("test_tetrahedron_random");
        tokio::fs::create_dir_all(&dir).await?;
        let bimesh = Bimesh::new(mesh1, mesh2);
        if seed == 123123123123 {
            write_stl_file(&mesh1, &dir.join("mesh1.stl")).await?;
            write_stl_file(&mesh2, &dir.join("mesh2.stl")).await?;
            write_stl_file(&bimesh.mesh_part(0, true), &dir.join("mesh1_inside.stl")).await?;
            write_stl_file(&bimesh.mesh_part(0, false), &dir.join("mesh1_outside.stl")).await?;
            write_stl_file(&bimesh.mesh_part(1, true), &dir.join("mesh2_inside.stl")).await?;
            write_stl_file(&bimesh.mesh_part(1, false), &dir.join("mesh2_outside.stl")).await?;
        }
        bimesh.intersect();
        bimesh.union();
    }
    Ok(())
}
