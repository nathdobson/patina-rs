use crate::mesh::Mesh;
use crate::mesh_triangle::MeshTriangle;
use patina_geo::aabb::Aabb;

impl Mesh {
    pub fn from_aabb(aabb: Aabb<3>) -> Mesh {
        Mesh::new(
            aabb.vertices().to_vec(),
            vec![
                MeshTriangle::new(0b000, 0b001, 0b011),
                MeshTriangle::new(0b000, 0b011, 0b010),
                MeshTriangle::new(0b100, 0b111, 0b101),
                MeshTriangle::new(0b100, 0b110, 0b111),
                //
                MeshTriangle::new(0b000, 0b101, 0b001),
                MeshTriangle::new(0b000, 0b100, 0b101),
                MeshTriangle::new(0b010, 0b011, 0b111),
                MeshTriangle::new(0b010, 0b111, 0b110),
                //
                MeshTriangle::new(0b000, 0b010, 0b110),
                MeshTriangle::new(0b000, 0b110, 0b100),
                MeshTriangle::new(0b001, 0b111, 0b011),
                MeshTriangle::new(0b001, 0b101, 0b111),
                //
            ],
        )
    }
}
