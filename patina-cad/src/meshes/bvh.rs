use crate::geo2::triangle2::Triangle2;
use crate::geo3::aabb::AABB;
use crate::geo3::ray3::Ray3;
use crate::geo3::sphere;
use crate::geo3::sphere::Sphere;
use crate::geo3::triangle3::Triangle3;
use crate::math::float_bool::{Epsilon, FloatBool};
use patina_vec::vec3::Vec3;
use crate::meshes::intersect_bvh_bvh::{IntersectBvhBvh, MeshIntersect};
use crate::meshes::intersect_bvh_ray::{IntersectBvhRay, MeshRayIntersection};
use crate::meshes::mesh::Mesh;
use crate::meshes::mesh_triangle::MeshTriangle;
use crate::sat::sat_intersects;
use crate::util::scan::ScanIteratorExt;
use ordered_float::{NotNan, OrderedFloat};
use rand::Rng;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::Add;

#[derive(Debug)]
pub struct BvhNode {
    aabb: AABB,
    nodes: Vec<usize>,
    leaves: Vec<usize>,
}

pub struct BvhNodeView<'a> {
    tris: &'a [MeshTriangle],
    vertices: &'a [Vec3],
    bvh: &'a Bvh,
    node: usize,
}

pub struct BvhTriangleView<'a> {
    index: usize,
    triangle: &'a MeshTriangle,
    vertices: &'a [Vec3],
}

pub struct Bvh {
    root: usize,
    nodes: Vec<BvhNode>,
}

pub struct BvhBuilder {
    nodes: Vec<BvhNode>,
    max_split_size: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct BvhTriangleBuilder {
    index: usize,
    triangle: Triangle3,
    midpoint: Vec3,
    aabb: AABB,
}

fn tri_bounds(slice: &[BvhTriangleBuilder]) -> AABB {
    let mut union = AABB::empty();
    for tri in slice {
        for p in tri.triangle.points() {
            union = union.union(&AABB::from_point(*p));
        }
    }
    union
}

impl BvhBuilder {
    pub fn new() -> Self {
        BvhBuilder {
            nodes: vec![],
            max_split_size: 4,
        }
    }
    fn build(self, root: usize) -> Bvh {
        Bvh {
            root,
            nodes: self.nodes,
        }
    }
    pub fn add_leaf(&mut self, tris: &[BvhTriangleBuilder]) -> usize {
        let aabb = tris
            .iter()
            .fold(AABB::empty(), |aabb, t| aabb.union(&t.aabb));
        self.nodes.push(BvhNode {
            aabb,
            nodes: vec![],
            leaves: tris.iter().map(|t| t.index).collect(),
        });
        self.nodes.len() - 1
    }
    pub fn add_node(&mut self, left: &[BvhTriangleBuilder], right: &[BvhTriangleBuilder]) -> usize {
        let aabb = left
            .iter()
            .chain(right.iter())
            .fold(AABB::empty(), |aabb, t| aabb.union(&t.aabb));
        let left = self.add_triangles(left);
        let right = self.add_triangles(right);
        self.nodes.push(BvhNode {
            aabb,
            nodes: vec![left, right],
            leaves: vec![],
        });
        self.nodes.len() - 1
    }
    pub fn add_triangles(&mut self, tris: &[BvhTriangleBuilder]) -> usize {
        if tris.len() < self.max_split_size {
            return self.add_leaf(tris);
        }
        let (left, right, _) = (0..3)
            .map(|axis| {
                let mut by_axis = tris.to_vec();
                by_axis.sort_by_key(|x| NotNan::new(x.midpoint[axis]).unwrap());
                let forward_surface_area: Vec<_> = by_axis
                    .iter()
                    .scan_full(AABB::empty(), |b, t| b.union(&t.aabb))
                    .map(|x| x.surface_area())
                    .collect();
                let mut reverse_surface_area: Vec<_> = by_axis
                    .iter()
                    .rev()
                    .scan_full(AABB::empty(), |b, t| b.union(&t.aabb))
                    .map(|x| x.surface_area())
                    .collect();
                reverse_surface_area.reverse();
                let (split, area) = forward_surface_area
                    .into_iter()
                    .zip(reverse_surface_area.into_iter())
                    .enumerate()
                    .map(|(i, (l, r))| {
                        assert!(l.is_finite());
                        assert!(r.is_finite());
                        let cost = (i as f64) * l + ((tris.len() - i) as f64) * r;
                        assert!(cost.is_finite());
                        (i, cost)
                    })
                    .min_by_key(|(i, a)| NotNan::new(*a).unwrap())
                    .unwrap();
                let right = by_axis.split_off(split);
                let left = by_axis;
                (left, right, area)
            })
            .min_by_key(|(left, right, area)| NotNan::new(*area).unwrap())
            .unwrap();
        if left.is_empty() {
            self.add_leaf(&right)
        } else if right.is_empty() {
            self.add_leaf(&left)
        } else {
            self.add_node(&left, &right)
        }
    }
}

impl BvhTriangleBuilder {
    pub fn new(index: usize, triangle: Triangle3) -> Self {
        BvhTriangleBuilder {
            index,
            triangle,
            midpoint: triangle.points().iter().sum::<Vec3>() / 3.0,
            aabb: triangle
                .points()
                .iter()
                .map(|x| AABB::from_point(*x))
                .fold(AABB::empty(), |a, b| a.union(&b)),
        }
    }
}

//impl BvhTriangle {
// pub fn intersect_leaf(&self, other: &Self, result: &mut Vec<(usize, usize)>) {
//     if self.triangle.intersects(&other.triangle) {
//         result.push((self.index, other.index));
//     }
// }
// pub fn intersect_node(&self, other: &BvhNodeView, result: &mut Vec<(usize, usize)>) {
//     let in_bounds = sat_intersects(&self.triangle, other.aabb());
//     let len = result.len();
//     for node2 in other.nodes() {
//         self.intersect_node(&node2, result);
//     }
//     for leaf2 in other.leaves() {
//         self.intersect_leaf(&leaf2, result);
//     }
//     if result.len() > len && !in_bounds {
//         dbg!(self);
//         dbg!(other);
//         dbg!(result.last());
//         panic!();
//     }
// }
// pub fn intersect_ray(&self, ray: &Ray3, eps: Epsilon, result: &mut Vec<RayMeshIntersection>) {
//     let (truth, time) = self.triangle.intersect_ray(ray, eps);
//     if truth.maybe() {
//         result.push(RayMeshIntersection {
//             index: self.index,
//             time,
//             truth,
//         })
//     }
// }
// }

impl Bvh {
    pub fn new(triangles: &[BvhTriangleBuilder]) -> Self {
        let mut builder = BvhBuilder::new();
        let root = builder.add_triangles(&triangles);
        builder.build(root)
    }
    pub fn from_mesh(mesh: &Mesh) -> Self {
        let mut triangles = vec![];
        for (index, t) in mesh.triangles().iter().enumerate() {
            let points = t.vertices().map(|v| mesh.vertices()[v]);
            triangles.push(BvhTriangleBuilder::new(index, Triangle3::new(points)));
        }
        Bvh::new(&triangles)
    }
    pub fn root_view<'a>(
        &'a self,
        tris: &'a [MeshTriangle],
        vertices: &'a [Vec3],
    ) -> BvhNodeView<'a> {
        BvhNodeView {
            tris,
            vertices,
            bvh: self,
            node: self.root,
        }
    }
    // pub fn intersect_bvh(&self, other: &Bvh) -> Vec<(usize, usize)> {
    //     let mut result = vec![];
    //     self.root_view()
    //         .intersect_node(&other.root_view(), &mut result);
    //     result
    // }
}

impl<'a> BvhNodeView<'a> {
    pub fn index(&self) -> usize {
        self.node
    }
    pub fn aabb(&self) -> &AABB {
        &self.bvh.nodes[self.node].aabb
    }
    pub fn leaves(&self) -> impl Iterator<Item = BvhTriangleView<'a>> {
        self.bvh.nodes[self.node]
            .leaves
            .iter()
            .map(|&index| BvhTriangleView {
                index,
                triangle: &self.tris[index],
                vertices: self.vertices,
            })
    }
    pub fn nodes<'b>(&'b self) -> impl Iterator<Item = BvhNodeView<'a>> {
        self.bvh.nodes[self.node]
            .nodes
            .iter()
            .map(|child| BvhNodeView {
                tris: self.tris,
                vertices: self.vertices,
                bvh: self.bvh,
                node: *child,
            })
    }
    pub fn intersect_ray(&self, ray: &Ray3, eps: Epsilon) -> Vec<MeshRayIntersection> {
        let mut intersect = IntersectBvhRay::new(eps);
        intersect.intersect_node_ray(self, &ray);
        intersect.build()
    }
    pub fn intersects_point(&self, point: Vec3, eps: Epsilon, rng: &mut impl Rng) -> FloatBool {
        let ints = self.intersect_ray(&Ray3::new(point, Vec3::random_unit(rng)), eps);
        let mut result = FloatBool::from(false);
        for int in &ints {
            result = result.xor(int.truth);
        }
        println!("result={:?}", ints);
        result
    }
    // pub fn intersect_node(&self, other: &BvhNodeView, result: &mut Vec<(usize, usize)>) {
    //     if !self.aabb().intersects(other.aabb()) {
    //         return;
    //     }
    //     for leaf1 in self.leaves() {
    //         for leaf2 in other.leaves() {
    //             leaf1.intersect_leaf(leaf2, result);
    //         }
    //         for node2 in other.nodes() {
    //             leaf1.intersect_node(&node2, result);
    //         }
    //     }
    //     for node1 in self.nodes() {
    //         for leaf2 in other.leaves() {
    //             node1.intersect_leaf(&leaf2, result);
    //         }
    //         for node2 in other.nodes() {
    //             node1.intersect_node(&node2, result);
    //         }
    //     }
    // }
    // pub fn intersect_leaf(&self, other: &BvhTriangle, result: &mut Vec<(usize, usize)>) {
    //     if sat_intersects(self.aabb(), &other.triangle) {
    //         for node1 in self.nodes() {
    //             node1.intersect_leaf(&other, result);
    //         }
    //         for leaf1 in self.leaves() {
    //             leaf1.intersect_leaf(&other, result);
    //         }
    //     }
    // }
    // pub fn intersect_ray(&self, ray: &Ray3, result: &mut Vec<RayMeshIntersection>) {
    //     if ray.intersect_aabb(self.aabb()).is_some() {
    //         for leaf in self.leaves() {
    //             leaf.intersect_ray(ray, result);
    //         }
    //         for node in self.nodes() {
    //             node.intersect_ray(&ray, result);
    //         }
    //     }
    // }
}

impl<'a> Debug for BvhNodeView<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut f = f.debug_struct("BvhNode");
        f.field("area", &self.aabb().surface_area());
        f.field("aabb", &self.aabb());
        for leaf in self.leaves() {
            f.field("leaf", &leaf);
        }
        for node in self.nodes() {
            f.field("node", &node);
        }
        f.finish()
    }
}

impl Debug for Bvh {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "todo")
        // self.root_view().fmt(f)
    }
}

impl<'a> BvhTriangleView<'a> {
    pub fn index(&self) -> usize {
        self.index
    }
    pub fn triangle(&self) -> &'a MeshTriangle {
        self.triangle
    }
    pub fn triangle3(&self) -> Triangle3 {
        self.triangle.for_vertices(self.vertices)
    }
    pub fn vertices(&self) -> &[Vec3] {
        self.vertices
    }
}

impl<'a> Debug for BvhTriangleView<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("BvhTriangleView")
            .field("index", &self.index)
            .field("mesh_triangle", self.triangle)
            .field("triangle", &self.triangle)
            .finish()
    }
}

// impl Debug for BvhTriangle {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         f.debug_struct("BvhTriangle")
//             .field("index", &self.index)
//             .finish()
//     }
// }

#[test]
fn test() {
    let sphere = Sphere::new(Vec3::zero(), 1.0).as_mesh(0);
    let bvh = Bvh::from_mesh(&sphere);
}
