use crate::meshes::mesh::Mesh;

pub fn remove_small_edges(mesh:&Mesh,threshold:f64)->Mesh{
    
    for tri in mesh.triangles(){
        for edge in tri.edges(){
            if edge.for_vertices(mesh.vertices()).as_ray().dir().length()<threshold{
                
            }
        }
    }
    todo!();
}