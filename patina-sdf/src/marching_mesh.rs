use itertools::Itertools;
use std::cell::OnceCell;
// use patina_calc::{EvalVisitor, Expr, ExprProgramBuilder, Program, ProgramVisit, Solver};
use crate::octree::{Octree, OctreeBranch, OctreePath, OctreeView, OctreeViewMut};
use crate::sdf::{Sdf, Sdf3};
use crate::transvoxel::cube_edge::{CubeEdge, CubeEdgeSet};
use crate::transvoxel::cube_face::{CubeFace, CubeFaceSet};
use crate::transvoxel::cube_input::CubeInput;
use crate::transvoxel::cube_triangle::CubeTriMesh;
use crate::transvoxel::cube_vertex;
use crate::transvoxel::cube_vertex::{CubeVertex, CubeVertexSet, cube_corners, cube_points};
use inari::DecInterval;
use ordered_float::NotNan;
use parking_lot::Mutex;
use patina_geo::aabb::Aabb;
use patina_geo::geo3::aabb3::Aabb3;
use patina_geo::geo3::triangle3::Triangle3;
use patina_mesh::mesh::Mesh;
use patina_mesh::mesh_triangle::MeshTriangle;
use patina_scalar::deriv::Deriv;
use patina_scalar::newton::Newton;
use patina_vec::vec3::{Vec3, Vector3};
use rayon::iter::ParallelIterator;
use rayon::iter::{IntoParallelRefIterator, IntoParallelRefMutIterator};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::mem;

#[derive(Debug, Default)]
struct MarchingNodeValue {}

#[derive(Debug, Clone)]
enum SdfState {
    Uninit,
    Empty,
    Sdf(Sdf<3>),
}

impl Default for SdfState {
    fn default() -> Self {
        SdfState::Uninit
    }
}

#[derive(Debug, Default)]
struct MarchingNodeKey {
    sdf: SdfState,
}

struct VertexBuilder {
    sdf: Sdf3,
    path: OctreePath,
    v1: CubeVertex,
    v2: CubeVertex,
}
struct MeshBuilder {
    vertex_table: HashMap<(Vector3<NotNan<f64>>, Vector3<NotNan<f64>>), usize>,
    vertices: Vec<VertexBuilder>,
    triangles: Vec<MeshTriangle>,
}

pub struct MarchingMesh {
    min_render_depth: usize,
    max_render_depth: usize,
    subdiv_max_dot: f64,
    aabb: Aabb3,
    mesh_builder: Mutex<MeshBuilder>,
}

type MarchingOctree = Octree<MarchingNodeKey, MarchingNodeValue>;
type MarchingOctreeBranch = OctreeBranch<MarchingNodeKey, MarchingNodeValue>;

impl MarchingMesh {
    pub fn new(aabb: Aabb3) -> Self {
        Self {
            min_render_depth: 6,
            max_render_depth: 10,
            subdiv_max_dot: 0.9,
            aabb,
            mesh_builder: Mutex::new(MeshBuilder {
                vertex_table: HashMap::new(),
                vertices: vec![],
                triangles: vec![],
            }),
        }
    }
    pub fn min_render_depth(&mut self, min_render_depth: usize) -> &mut Self {
        self.min_render_depth = min_render_depth;
        self
    }
    pub fn max_render_depth(&mut self, max_render_depth: usize) -> &mut Self {
        self.max_render_depth = max_render_depth;
        self
    }
    pub fn subdiv_max_dot(&mut self, subdiv_max_dot: f64) -> &mut Self {
        self.subdiv_max_dot = subdiv_max_dot;
        self
    }
    fn find_marching_cube(&self, aabb: &Aabb3, sdf: &Sdf<3>) -> CubeTriMesh {
        let mut result = CubeVertexSet::new();
        for cv in cube_corners() {
            let p = (0..3)
                .map(|axis| {
                    if cv[axis] > 0 {
                        aabb.max()[axis]
                    } else {
                        aabb.min()[axis]
                    }
                })
                .collect();
            let d = sdf.evaluate(p);
            result[cv] = d >= 0.0;
        }
        CubeInput::new(CubeFaceSet::new(), CubeEdgeSet::new(), result).as_mesh()
    }
    fn is_divided(&self, octree: &MarchingOctree, path: &OctreePath) -> bool {
        if let Some((index, path)) = path.view() {
            match octree.view() {
                OctreeView::Leaf(key, value) => false,
                OctreeView::Branch(branch) => self.is_divided(&branch[index], &path),
            }
        } else {
            match octree.view() {
                OctreeView::Leaf(_, _) => false,
                OctreeView::Branch(_) => true,
            }
        }
    }
    fn add_marching_cube_sub(
        &self,
        root: &MarchingOctree,
        octree: &MarchingOctree,
        aabb: &Aabb3,
        sdf: &Sdf<3>,
    ) {
        let mut faces = CubeFaceSet::new();
        for face in CubeFace::all() {
            let mut is_divided;
            if let Some(neighbor) = octree.path().face_adjacent_for(face) {
                is_divided = self.is_divided(root, &neighbor);
            } else {
                is_divided = false;
            }
            faces[face] = is_divided;
        }
        let mut edges = CubeEdgeSet::new();
        for edge in CubeEdge::all() {
            let mut is_divided;
            if let Some(neighbor) = octree.path().edge_adjacent_for(edge) {
                is_divided = self.is_divided(root, &neighbor);
            } else {
                is_divided = false;
            }
            edges[edge] = is_divided;
        }

        let mut to_sample = CubeVertexSet::corners();
        faces.add_samples_to(&mut to_sample);
        edges.add_samples_to(&mut to_sample);
        if to_sample != CubeVertexSet::new() {
            to_sample[cube_vertex::cube_center()] |= true;
        }
        let mut vertices = CubeVertexSet::new();
        for cv in cube_points() {
            if to_sample[cv] {
                let v = self.path_position(octree.path(), cv);
                let eval = sdf.evaluate(v);
                vertices[cv] = eval >= 0.0;
            }
        }
        let transvoxel = CubeInput::new(faces, edges, vertices);
        let mesh2 = transvoxel.as_mesh();

        let ref mut mesh_builder = *self.mesh_builder.lock();
        for tri in mesh2.triangles() {
            let tri = tri
                .vertices()
                .map(|(v1, v2)| self.get_vertex(octree.path(), v1, v2, sdf, mesh_builder));
            mesh_builder.triangles.push(MeshTriangle::from(tri));
        }
    }
    fn path_position(&self, path: &OctreePath, cv: CubeVertex) -> Vec3 {
        let numer = path.position() * 2 + cv.map(|x| x as usize);
        let denom = (2 << path.depth()) as f64;
        let v = numer.map(|x| x as f64) / denom;
        let v = self.aabb.min() + v.mul_elements(self.aabb.dimensions());
        v
    }
    fn get_vertex(
        &self,
        path: &OctreePath,
        mut v1: CubeVertex,
        mut v2: CubeVertex,
        sdf: &Sdf3,
        mesh_builder: &mut MeshBuilder,
    ) -> usize {
        if v2 < v1 {
            mem::swap(&mut v1, &mut v2);
        }
        let v1p = self.path_position(path, v1);
        let v2p = self.path_position(path, v2);
        *mesh_builder
            .vertex_table
            .entry((
                v1p.map(|x| NotNan::new(x).unwrap()),
                v2p.map(|x| NotNan::new(x).unwrap()),
            ))
            .or_insert_with(|| {
                mesh_builder.vertices.push(VertexBuilder {
                    sdf: sdf.clone(),
                    path: *path,
                    v1,
                    v2,
                });
                mesh_builder.vertices.len() - 1
            })
    }
    fn init_sdf(&self, tree: &mut MarchingOctree, sdf: &Sdf3) -> Option<Sdf3> {
        match tree.key().sdf.clone() {
            SdfState::Uninit => {}
            SdfState::Empty => return None,
            SdfState::Sdf(sdf) => return Some(sdf.clone()),
        };
        let aabb = tree.path().aabb_inside(&self.aabb);
        let aabb_intervals: Vector3<DecInterval> = (0..3)
            .map(|axis| DecInterval::try_from((aabb.min()[axis], aabb.max()[axis])).unwrap())
            .collect();
        let (sdf2, range) = sdf.evaluate_constrain(aabb_intervals);
        if !range.contains(0.0) {
            tree.key_mut().sdf = SdfState::Empty;
            return None;
        }
        let sdf = sdf2.unwrap_or(sdf.clone());
        tree.key_mut().sdf = SdfState::Sdf(sdf.clone());
        Some(sdf)
    }
    fn build_octree(&self, tree: &mut MarchingOctree, sdf: &Sdf3) {
        let aabb = tree.path().aabb_inside(&self.aabb);
        let sdf = self.init_sdf(tree, sdf);
        let Some(sdf) = sdf else {
            return;
        };
        if tree.path().depth() < self.min_render_depth {
            self.build_branch(tree, &sdf);
            return;
        }
        if tree.path().depth() >= self.max_render_depth {
            return;
        }
        let mcube = self.find_marching_cube(&aabb, &sdf);
        let vertices = mcube
            .triangles()
            .iter()
            .flat_map(|t| t.vertices().into_iter().cloned())
            .collect::<HashSet<_>>();
        let mut normals = vec![];
        for (v1, v2) in vertices {
            let (_, normal) = self.find_vertex(tree.path(), v1, v2, &sdf);
            normals.push(normal);
        }
        if normals
            .iter()
            .tuple_combinations()
            .any(|(n1, n2)| n1.dot(*n2) < self.subdiv_max_dot)
        {
            self.build_branch(tree, &sdf);
        }
    }
    fn position(&self, aabb: &Aabb3, v: CubeVertex) -> Vec3 {
        (0..3)
            .map(|axis| match v[axis] {
                0 => aabb.min()[axis],
                1 => (aabb.min()[axis] + aabb.max()[axis]) / 2.0,
                2 => aabb.max()[axis],
                _ => unreachable!(),
            })
            .collect()
    }
    fn position_range(&self, aabb: &Aabb3, v1: CubeVertex, v2: CubeVertex) -> (Vec3, Vec3) {
        (self.position(aabb, v1), self.position(aabb, v2))
    }
    fn find_vertex(
        &self,
        path: &OctreePath,
        v1: CubeVertex,
        v2: CubeVertex,
        sdf: &Sdf3,
    ) -> (Vec3, Vec3) {
        let min = self.path_position(path, v1);
        let max = self.path_position(path, v2);
        let range = max - min;
        let lsdf = |t| {
            sdf.evaluate_deriv1(
                min.map(Deriv::constant) + range.map(Deriv::constant) * Deriv::variable(t, 0),
            )
        };
        let t = Newton::new().solve(0.0..1.0, lsdf);
        let t = if let Some(t) = t {
            t.into_inner()
        } else {
            println!("{:?}->{:?}", min, max);
            for x in -1..=11 {
                println!("{:?} {:?}", x, lsdf(x as f64 / 10.0));
            }
            0.5
        };
        assert!(t >= 0.0 && t <= 1.0, "{:?}", t);
        let vertex_position = min + range * t.clamp(0.00001, 0.99999);
        let eval_position = min + range * t;
        let normal: Vec3 = sdf.normal(eval_position);
        (vertex_position, normal)
    }
    fn build_branch(&self, tree: &mut MarchingOctree, sdf: &Sdf3) {
        let depth = tree.path().depth();
        match tree.view_mut() {
            OctreeViewMut::Leaf(_, _) => {
                tree.set_branch(Default::default());
            }
            OctreeViewMut::Branch(_) => {}
        }
        let branch = match tree.view_mut() {
            OctreeViewMut::Leaf(_, _) => unreachable!(),
            OctreeViewMut::Branch(branch) => branch,
        };
        branch.children_flat_mut().par_iter_mut().for_each(|child| {
            self.build_octree(child, sdf);
        });
    }

    fn build_mesh(&self, root: &MarchingOctree, tree: &MarchingOctree, root_sdf: &Sdf3) {
        match tree.view() {
            OctreeView::Leaf(leaf, _) => match &leaf.sdf {
                SdfState::Uninit => unreachable!(),
                SdfState::Empty => {}
                SdfState::Sdf(sdf) => {
                    self.build_mesh_leaf(root, tree, &sdf);
                }
            },
            OctreeView::Branch(branch) => {
                self.build_mesh_branch(root, branch, root_sdf);
            }
        }
    }

    fn build_mesh_branch(
        &self,
        root: &MarchingOctree,
        branch: &MarchingOctreeBranch,
        root_sdf: &Sdf3,
    ) {
        branch.children_flat().par_iter().for_each(|child| {
            self.build_mesh(root, child, root_sdf);
        });
    }

    fn build_mesh_leaf(&self, root: &MarchingOctree, tree: &MarchingOctree, sdf: &Sdf3) {
        let aabb = tree.path().aabb_inside(&self.aabb);
        self.add_marching_cube_sub(root, tree, &aabb, sdf);
    }

    fn get_neighbors(
        &mut self,
        octree: &MarchingOctree,
        depth: usize,
        neighbors: &mut HashSet<OctreePath>,
    ) {
        match octree.view() {
            OctreeView::Leaf(_, _) => {
                if octree.path().depth() == depth {
                    for neighbor in octree.path().face_adjacent() {
                        neighbors.insert(neighbor);
                    }
                    for neighbor in octree.path().edge_adjacent() {
                        neighbors.insert(neighbor);
                    }
                }
            }
            OctreeView::Branch(branch) => {
                for child in branch.children_flat() {
                    self.get_neighbors(child, depth, neighbors);
                }
            }
        }
    }
    fn refine_path(&mut self, octree: &mut MarchingOctree, path: OctreePath, sdf: Option<&Sdf3>) {
        if path.depth() == 1 {
            return;
        }
        if let Some((index, path)) = path.view() {
            match octree.view_mut() {
                OctreeViewMut::Leaf(_, _) => octree.set_branch(Default::default()),
                OctreeViewMut::Branch(branch) => {}
            }
            match octree.view_mut() {
                OctreeViewMut::Leaf(_, _) => unreachable!(),
                OctreeViewMut::Branch(branch) => {
                    for subtree in branch.children_flat_mut() {
                        if let Some(sdf) = sdf {
                            self.init_sdf(subtree, sdf);
                        } else {
                            subtree.key_mut().sdf = SdfState::Empty;
                        }
                    }
                    let sdf = match &branch.child(index).key().sdf {
                        SdfState::Uninit => unreachable!(),
                        SdfState::Empty => None,
                        SdfState::Sdf(sdf) => Some(sdf.clone()),
                    };
                    self.refine_path(branch.child_mut(index), path, sdf.as_ref())
                }
            }
        }
    }

    fn refine_neighbors(&mut self, octree: &mut MarchingOctree, sdf: &Sdf3) {
        for depth in (0..=self.max_render_depth).rev() {
            let mut to_refine = HashSet::new();
            self.get_neighbors(octree, depth, &mut to_refine);
            for path in to_refine {
                self.refine_path(octree, path, Some(sdf));
            }
        }
    }

    fn collect_mesh(&mut self) -> Mesh {
        let vertices = self
            .mesh_builder
            .lock()
            .vertices
            .par_iter()
            .map(|x| self.find_vertex(&x.path, x.v1, x.v2, &x.sdf).0)
            .collect();
        Mesh::new(vertices, self.mesh_builder.get_mut().triangles.clone())
    }

    pub fn build(mut self, sdf: &Sdf3) -> Mesh {
        let mut octree = MarchingOctree::new_root();
        self.build_octree(&mut octree, sdf);
        self.refine_neighbors(&mut octree, sdf);
        let mut comp = Complexity::new();
        comp.add_tree(&octree);
        println!("{:#?}", comp);
        self.build_mesh(&octree, &octree, sdf);
        self.collect_mesh()
    }
}

#[derive(Debug)]
struct Complexity {
    depth_to_complexity_to_count: BTreeMap<usize, BTreeMap<usize, usize>>,
}

impl Complexity {
    pub fn new() -> Self {
        Complexity {
            depth_to_complexity_to_count: BTreeMap::new(),
        }
    }
    pub fn add_tree(&mut self, tree: &MarchingOctree) {
        let comp = match &tree.key().sdf {
            SdfState::Uninit => unreachable!(),
            SdfState::Empty => 0,
            SdfState::Sdf(sdf) => sdf.complexity(),
        };
        *self
            .depth_to_complexity_to_count
            .entry(tree.path().depth())
            .or_default()
            .entry(comp)
            .or_default() += 1;
        match tree.view() {
            OctreeView::Leaf(_, _) => {}
            OctreeView::Branch(branch) => {
                for subtree in branch.children_flat() {
                    self.add_tree(subtree);
                }
            }
        }
    }
}
