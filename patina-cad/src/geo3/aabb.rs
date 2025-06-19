use crate::math::interval::Interval;
use crate::math::vec3::Vec3;
use crate::meshes::mesh::Mesh;
use crate::meshes::mesh_triangle::MeshTriangle;
use crate::sat::ConvexPoly;

#[derive(Copy, Clone, Debug)]
pub struct AABB {
    min: Vec3,
    max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        AABB { min, max }
    }
    pub fn from_point(p: Vec3) -> Self {
        Self::new(p, p)
    }
    pub fn empty() -> Self {
        Self::new(Vec3::splat(f64::INFINITY), Vec3::splat(-f64::INFINITY))
    }
    pub fn union(&self, other: &Self) -> Self {
        Self::new(self.min.min(other.min), self.max.max(other.max))
    }
    pub fn min(&self) -> Vec3 {
        self.min
    }
    pub fn max(&self) -> Vec3 {
        self.max
    }
    pub fn surface_area(&self) -> f64 {
        let d = self.max - self.min;
        let d = d.max(Vec3::splat(0.0));
        d.x() * d.y() + d.x() * d.z() + d.y() * d.z()
    }
    pub fn intersect(&self, other: &Self) -> Self {
        Self::new(self.min.max(other.min), self.max.min(other.max))
    }
    pub fn dimensions(&self) -> Vec3 {
        (self.max - self.min).max(Vec3::zero())
    }
    pub fn intersects(&self, other: &Self) -> bool {
        self.intersect(other)
            .dimensions()
            .into_iter()
            .all(|x| x >= 0.0)
    }
    pub fn vertices(&self) -> [Vec3; 8] {
        let min = self.min;
        let max = self.max;
        [
            Vec3::new(min.x(), min.y(), min.z()),
            Vec3::new(min.x(), min.y(), max.z()),
            Vec3::new(min.x(), max.y(), min.z()),
            Vec3::new(min.x(), max.y(), max.z()),
            Vec3::new(max.x(), min.y(), min.z()),
            Vec3::new(max.x(), min.y(), max.z()),
            Vec3::new(max.x(), max.y(), min.z()),
            Vec3::new(max.x(), max.y(), max.z()),
        ]
    }
    pub fn as_mesh(&self) -> Mesh {
        Mesh::new(
            self.vertices().to_vec(),
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

impl ConvexPoly for AABB {
    fn normals(&self) -> impl AsRef<[Vec3]> {
        [Vec3::axis_x(), Vec3::axis_y(), Vec3::axis_z()]
    }

    fn project_onto(&self, vector: Vec3) -> Interval {
        let mut result = Interval::empty();
        for x in self.vertices() {
            result = result.union(Interval::from(x.dot(vector)));
        }
        result
    }
}
