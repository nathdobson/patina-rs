use patina_vec::vec3::Vec3;
use crate::meshes::mesh::Mesh;
use crate::meshes::mesh_triangle::MeshTriangle;
use crate::meshes::subdivision::subdivide;

pub struct Sphere {
    start: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(start: Vec3, radius: f64) -> Sphere {
        Sphere { start, radius }
    }
    pub fn start(&self) -> Vec3 {
        self.start
    }
    pub fn radius(&self) -> f64 {
        self.radius
    }
    pub fn as_mesh(&self, detail: usize) -> Mesh {
        let mut mesh = icosphere(detail);
        for v in mesh.vertices_mut() {
            *v = *v * self.radius + self.start();
        }
        mesh
    }
}

fn icosahedron() -> Mesh {
    let mut vertices = vec![];
    let phi = (1.0 + 5.0f64.sqrt()) / 2.0;
    for rot in 0..3 {
        for b in [-phi, phi] {
            for a in [-1.0, 1.0] {
                let mut p = Vec3::splat(0.0);
                p[(rot + 1) % 3] = a;
                p[(rot + 2) % 3] = b;
                p = p.normalize();
                vertices.push(p);
            }
        }
    }
    let mut triangles = vec![];
    for rot in 0..3 {
        for a in 0..2 {
            for b in 0..2 {
                let mut t = MeshTriangle::new(
                    rot * 4 + a * 2,
                    rot * 4 + a * 2 + 1,
                    ((rot + 1) % 3) * 4 + b * 2 + a,
                );
                if (a == 0) != (b == 1) {
                    t.invert();
                }
                triangles.push(t);
            }
        }
    }
    for x in 0..2 {
        for y in 0..2 {
            for z in 0..2 {
                let mut t = MeshTriangle::new(
                    y + z * 2,
                    4 + z + x * 2,
                    8 + x + y * 2, //
                );
                if (x + y + z) % 2 == 0 {
                    t.invert();
                }
                triangles.push(t);
            }
        }
    }
    Mesh::new(vertices, triangles)
}

fn spherify(mesh: &mut Mesh) {
    for vertex in mesh.vertices_mut() {
        *vertex = vertex.normalize();
    }
}

fn icosphere(detail: usize) -> Mesh {
    let mut mesh = icosahedron();
    spherify(&mut mesh);
    for i in 0..detail {
        mesh = subdivide(&mesh);
        spherify(&mut mesh);
    }
    mesh
}
