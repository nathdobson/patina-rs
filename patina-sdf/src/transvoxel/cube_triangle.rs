use patina_mesh::mesh::Mesh;
use patina_mesh::mesh_triangle::MeshTriangle;
use crate::transvoxel::cube_vertex::CubeVertex;

#[derive(Copy, Clone, Debug)]
pub struct CubeTri([(CubeVertex, CubeVertex); 3]);

#[derive(Clone, Debug)]
pub struct CubeTriMesh(Vec<CubeTri>);

impl CubeTri {
    pub fn new(vs: [(CubeVertex, CubeVertex); 3]) -> Self {
        CubeTri(vs)
    }
    pub fn vertices(&self) -> &[(CubeVertex, CubeVertex); 3] {
        &self.0
    }
}

impl CubeTriMesh {
    pub fn new(tris: Vec<CubeTri>) -> Self {
        CubeTriMesh(tris)
    }
    pub fn triangles(&self) -> &[CubeTri] {
        &self.0
    }
    pub fn into_mesh(&self) -> Mesh {
        let mut vertices = vec![];
        let mut tris = vec![];
        for tri in self.triangles() {
            let mut index = vertices.len();
            for &(v1, v2) in tri.vertices() {
                vertices.push((v1 + v2).map(|x| x as f64));
            }
            tris.push(MeshTriangle::new(index, index + 1, index + 2));
        }
        Mesh::new(vertices, tris)
    }
}