use itertools::Itertools;
// use patina_calc::{EvalVisitor, Expr, ExprProgramBuilder, Program, ProgramVisit, Solver};
use crate::octree::{Octree, OctreeBranch, OctreePath, OctreeView, OctreeViewMut};
use crate::sdf::Sdf;
use crate::transvoxel::cube_edge::{CubeEdge, CubeEdgeSet};
use crate::transvoxel::cube_face::{CubeFace, CubeFaceSet};
use crate::transvoxel::cube_input::CubeInput;
use crate::transvoxel::cube_triangle::CubeTriMesh;
use crate::transvoxel::cube_vertex;
use crate::transvoxel::cube_vertex::{CubeVertex, CubeVertexSet, cube_corners, cube_points};
use inari::DecInterval;
use ordered_float::NotNan;
use patina_geo::geo3::aabb::Aabb;
use patina_mesh::mesh::Mesh;
use patina_mesh::mesh_triangle::MeshTriangle;
use patina_scalar::deriv::Deriv;
use patina_scalar::newton::Newton;
use patina_vec::vec3::Vec3;
use patina_vec::vector3::Vector3;
use std::collections::{HashMap, HashSet};
use std::mem;

#[derive(Debug)]
struct MarchingNode {}
pub struct MarchingMesh {
    min_render_depth: usize,
    max_render_depth: usize,
    subdiv_max_dot: f64,
    aabb: Aabb,
    vertices: Vec<Vec3>,
    vertex_table: HashMap<Vector3<NotNan<f64>>, usize>,
    triangles: Vec<MeshTriangle>,
}

impl Default for MarchingNode {
    fn default() -> Self {
        MarchingNode {}
    }
}

impl MarchingNode {}

impl MarchingMesh {
    pub fn new(aabb: Aabb) -> Self {
        Self {
            min_render_depth: 2,
            max_render_depth: 10,
            subdiv_max_dot: 0.9,
            aabb,
            vertices: vec![],
            vertex_table: HashMap::new(),
            triangles: vec![],
        }
    }
    fn find_marching_cube(&self, aabb: &Aabb, sdf: &Sdf) -> CubeTriMesh {
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
    fn is_divided(&self, octree: &Octree<MarchingNode>, path: &OctreePath) -> bool {
        if let Some((index, path)) = path.view() {
            match octree.view() {
                OctreeView::Leaf(leaf) => false,
                OctreeView::Branch(branch) => self.is_divided(&branch[index], &path),
            }
        } else {
            match octree.view() {
                OctreeView::Leaf(_) => false,
                OctreeView::Branch(_) => true,
            }
        }
    }
    fn add_marching_cube_sub(
        &mut self,
        root: &Octree<MarchingNode>,
        octree: &Octree<MarchingNode>,
        aabb: &Aabb,
        sdf: &Sdf,
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

        for tri in mesh2.triangles() {
            let tri_verts = tri
                .vertices()
                .map(|(v1, v2)| self.add_vertex(octree.path(), v1, v2, sdf));
            let tri2 = MeshTriangle::from(tri_verts);
            self.triangles.push(tri2);
        }
    }
    fn path_position(&self, path: &OctreePath, cv: CubeVertex) -> Vec3 {
        let numer = path.position() * 2 + cv.map(|x| x as usize);
        let denom = (2 << path.depth()) as f64;
        let v = numer.map(|x| x as f64) / denom;
        let v = self.aabb.min() + v.mul_elements(self.aabb.dimensions());
        v
    }
    fn add_vertex(
        &mut self,
        path: &OctreePath,
        mut v1: CubeVertex,
        mut v2: CubeVertex,
        sdf: &Sdf,
    ) -> usize {
        if v2 < v1 {
            mem::swap(&mut v1, &mut v2);
        }
        let v1p = self.path_position(path, v1);
        let v2p = self.path_position(path, v2);
        let vp = (v1p + v2p) / 2.0;
        let vnn = vp.map(|x| NotNan::new(x).unwrap());
        if let Some(v) = self.vertex_table.get(&vnn) {
            return *v;
        }
        let p = self.find_vertex(path, v1, v2, sdf).0;
        let index = self.vertices.len();
        self.vertices.push(p);
        self.vertex_table.insert(vnn, index);
        index
    }
    fn build_octree(&mut self, tree: &mut Octree<MarchingNode>, sdf: &Sdf) {
        let aabb = tree.path().aabb_inside(&self.aabb);
        let aabb_intervals: Vector3<DecInterval> = (0..3)
            .map(|axis| DecInterval::try_from((aabb.min()[axis], aabb.max()[axis])).unwrap())
            .collect();
        let (sdf2, range) = sdf.evaluate_constrain(aabb_intervals);
        let sdf = sdf2.unwrap_or(sdf.clone());
        if !range.contains(0.0) {
            return;
        }
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
    fn position(&self, aabb: &Aabb, v: CubeVertex) -> Vec3 {
        (0..3)
            .map(|axis| match v[axis] {
                0 => aabb.min()[axis],
                1 => (aabb.min()[axis] + aabb.max()[axis]) / 2.0,
                2 => aabb.max()[axis],
                _ => unreachable!(),
            })
            .collect()
    }
    fn position_range(&self, aabb: &Aabb, v1: CubeVertex, v2: CubeVertex) -> (Vec3, Vec3) {
        (self.position(aabb, v1), self.position(aabb, v2))
    }
    fn find_vertex(
        &self,
        path: &OctreePath,
        v1: CubeVertex,
        v2: CubeVertex,
        sdf: &Sdf,
    ) -> (Vec3, Vec3) {
        let min = self.path_position(path, v1);
        let max = self.path_position(path, v2);
        let range = max - min;
        let lsdf = |t| {
            sdf.evaluate_deriv1(
                min.map(Deriv::constant) + range.map(Deriv::constant) * Deriv::variable(t, 0),
            )
        };
        let t = Newton::new().solve(0.0..1.0, lsdf).unwrap().into_inner();
        assert!(t >= -1.0 && t <= 1.0, "{:?}", t);
        let vertex_position = min + range * t.clamp(-0.99, 0.99);
        let eval_position = min + range * t;
        let normal: Vec3 = sdf.normal(eval_position);
        (vertex_position, normal)
    }
    fn build_branch(&mut self, tree: &mut Octree<MarchingNode>, sdf: &Sdf) {
        match tree.view_mut() {
            OctreeViewMut::Leaf(_) => {
                tree.set_branch(Default::default());
            }
            OctreeViewMut::Branch(_) => {}
        }
        let branch = match tree.view_mut() {
            OctreeViewMut::Leaf(_) => unreachable!(),
            OctreeViewMut::Branch(branch) => branch,
        };
        for child in branch.children_flat_mut() {
            self.build_octree(child, sdf);
        }
    }

    fn build_mesh(&mut self, root: &Octree<MarchingNode>, tree: &Octree<MarchingNode>, sdf: &Sdf) {
        match tree.view() {
            OctreeView::Leaf(_) => {
                self.build_mesh_leaf(root, tree, sdf);
            }
            OctreeView::Branch(branch) => {
                self.build_mesh_branch(root, branch, sdf);
            }
        }
    }

    fn build_mesh_branch(
        &mut self,
        root: &Octree<MarchingNode>,
        branch: &OctreeBranch<MarchingNode>,
        sdf: &Sdf,
    ) {
        for child in branch.children_flat() {
            self.build_mesh(root, child, sdf);
        }
    }

    fn build_mesh_leaf(
        &mut self,
        root: &Octree<MarchingNode>,
        tree: &Octree<MarchingNode>,
        sdf: &Sdf,
    ) {
        let aabb = tree.path().aabb_inside(&self.aabb);
        self.add_marching_cube_sub(root, tree, &aabb, sdf);
    }

    fn get_neighbors(
        &mut self,
        octree: &Octree<MarchingNode>,
        depth: usize,
        neighbors: &mut HashSet<OctreePath>,
    ) {
        match octree.view() {
            OctreeView::Leaf(_) => {
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
    fn refine_path(&mut self, octree: &mut Octree<MarchingNode>, path: OctreePath) {
        if path.depth() == 1 {
            return;
        }
        if let Some((index, path)) = path.view() {
            match octree.view_mut() {
                OctreeViewMut::Leaf(_) => octree.set_branch(Default::default()),
                OctreeViewMut::Branch(branch) => {}
            }
            match octree.view_mut() {
                OctreeViewMut::Leaf(_) => unreachable!(),
                OctreeViewMut::Branch(branch) => self.refine_path(branch.child_mut(index), path),
            }
        }
    }

    fn refine_neighbors(&mut self, octree: &mut Octree<MarchingNode>) {
        for depth in (0..=self.max_render_depth).rev() {
            let mut to_refine = HashSet::new();
            self.get_neighbors(octree, depth, &mut to_refine);
            for path in to_refine {
                self.refine_path(octree, path);
            }
        }
    }

    pub fn build(mut self, sdf: &Sdf) -> Mesh {
        let mut octree = Octree::<MarchingNode>::new_root(MarchingNode::default());
        self.build_octree(&mut octree, sdf);
        self.refine_neighbors(&mut octree);
        self.build_mesh(&octree, &octree, sdf);
        Mesh::new(self.vertices, self.triangles)
    }
}
