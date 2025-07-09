use crate::transvoxel::cube_edge::CubeEdgeSet;
use crate::transvoxel::cube_face::CubeFaceSet;
use crate::transvoxel::cube_tetr::CubeTetrMesh;
use crate::transvoxel::cube_triangle::CubeTriMesh;
use crate::transvoxel::cube_vertex::CubeVertexSet;
use arrayvec::ArrayVec;
use std::collections::HashSet;

pub struct CubeInput {
    faces: CubeFaceSet,
    edges: CubeEdgeSet,
    vertices: CubeVertexSet,
}

impl CubeInput {
    pub fn new(faces: CubeFaceSet, edges: CubeEdgeSet, vertices: CubeVertexSet) -> Self {
        CubeInput {
            faces,
            edges,
            vertices,
        }
    }
    pub fn as_mesh(&self) -> CubeTriMesh {
        let mut tris = vec![];
        for tetr in CubeTetrMesh::divided_cube(&self.faces, &self.edges)
            .tetrs()
            .iter()
        {
            let mut inside = ArrayVec::<_, 4>::new();
            let mut outside = ArrayVec::<_, 4>::new();
            for &v in tetr.vertices() {
                if self.vertices[v] {
                    inside.push(v);
                } else {
                    outside.push(v);
                }
            }
            let tetr_mesh = tetr.as_mesh(&inside, &outside);
            for tri in tetr_mesh.triangles() {
                tris.push(*tri);
            }
        }
        let mut tri_set = HashSet::new();
        for tri in &tris {
            let mut vs = tri.vertices().map(|(v1, v2)| v1 + v2);
            vs.sort();
            assert!(tri_set.insert(vs));
        }
        CubeTriMesh::new(tris)
    }
}
