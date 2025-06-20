use crate::geo3::aabb::AABB;
use crate::geo3::ray3::Ray3;
use crate::geo3::sphere;
use crate::geo3::sphere::Sphere;
use crate::geo3::triangle::Triangle;
use crate::math::vec3::Vec3;
use crate::meshes::mesh::Mesh;
use crate::sat::sat_intersects;
use crate::util::scan::ScanIteratorExt;
use ordered_float::{NotNan, OrderedFloat};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::Add;

#[derive(Debug)]
pub struct BvhNode {
    aabb: AABB,
    nodes: Vec<usize>,
    leaves: Vec<BvhTriangle>,
}

pub struct BvhNodeView<'a> {
    bvh: &'a Bvh,
    node: usize,
}

pub struct BvhTriangle {
    index: usize,
    triangle: Triangle,
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
    triangle: Triangle,
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

static IGNORE_AABB1: bool = true;
static IGNORE_AABB2: bool = false;
static IGNORE_AABB3: bool = false;
static IGNORE_AABB4: bool = false;

// struct NodeCandidate {
//     left: Vec<BvhTriangleBuilder>,
//     right: Vec<BvhTriangleBuilder>,
//     left_aabb: AABB,
//     right_aabb: AABB,
//     surface_area: f64,
// }
//
// impl NodeCandidate {
//     pub fn new(tris: &[BvhTriangleBuilder], axis: usize) -> NodeCandidate {
//         let mut new_tris = tris.to_vec();
//         new_tris.sort_by_key(|x| NotNan::new(x.midpoint[axis]).unwrap());
//         let left = new_tris.split_off(new_tris.len() / 2);
//         let right = new_tris;
//         let left_aabb = tri_bounds(&left);
//         let right_aabb = tri_bounds(&right);
//         NodeCandidate {
//             left,
//             right,
//             left_aabb,
//             right_aabb,
//             surface_area: left_aabb.surface_area() + right_aabb.surface_area(),
//         }
//     }
// }

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
            leaves: tris
                .iter()
                .map(|t| BvhTriangle {
                    index: t.index,
                    triangle: t.triangle,
                })
                .collect(),
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
    pub fn new(index: usize, triangle: Triangle) -> Self {
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

impl BvhTriangle {
    pub fn intersect_leaf(&self, other: &Self, result: &mut Vec<(usize, usize)>) {
        if self.triangle.intersects(&other.triangle) {
            println!("intersect");
            result.push((self.index, other.index));
        }
    }
    pub fn intersect_node(&self, other: &BvhNodeView, result: &mut Vec<(usize, usize)>) {
        let in_bounds = sat_intersects(&self.triangle, other.aabb());
        let len = result.len();
        for node2 in other.nodes() {
            self.intersect_node(&node2, result);
        }
        for leaf2 in other.leaves() {
            self.intersect_leaf(&leaf2, result);
        }
        if result.len() > len && !in_bounds {
            dbg!(self);
            dbg!(other);
            dbg!(result.last());
            panic!();
        }
    }
    pub fn intersect_ray(&self, ray: &Ray3, result: &mut Vec<RayMeshIntersection>) {
        if let Some(time) = self.triangle.intersect_ray(ray) {
            result.push(RayMeshIntersection {
                index: self.index,
                time,
            })
        }
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub struct RayMeshIntersection {
    index: usize,
    time: f64,
}

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
            triangles.push(BvhTriangleBuilder::new(index, Triangle::new(points)));
        }
        Bvh::new(&triangles)
    }
    pub fn root_view(&self) -> BvhNodeView<'_> {
        BvhNodeView {
            bvh: self,
            node: self.root,
        }
    }
    pub fn intersect_bvh(&self, other: &Bvh) -> Vec<(usize, usize)> {
        let mut result = vec![];
        self.root_view()
            .intersect_node(&other.root_view(), &mut result);
        result
    }
    pub fn intersect_ray(&self, ray: &Ray3) -> Vec<RayMeshIntersection> {
        let mut result = vec![];
        self.root_view().intersect_ray(ray, &mut result);
        println!("result = {:?}", result);
        result
    }
}

impl<'a> BvhNodeView<'a> {
    pub fn aabb(&self) -> &AABB {
        &self.bvh.nodes[self.node].aabb
    }
    pub fn leaves(&self) -> &[BvhTriangle] {
        &self.bvh.nodes[self.node].leaves
    }
    pub fn nodes<'b>(&'b self) -> impl Iterator<Item = BvhNodeView<'a>> {
        self.bvh.nodes[self.node]
            .nodes
            .iter()
            .map(|child| BvhNodeView {
                bvh: self.bvh,
                node: *child,
            })
    }
    pub fn intersect_node(&self, other: &BvhNodeView, result: &mut Vec<(usize, usize)>) {
        if !IGNORE_AABB2 && !self.aabb().intersects(other.aabb()) {
            return;
        }
        for leaf1 in self.leaves() {
            for leaf2 in other.leaves() {
                leaf1.intersect_leaf(leaf2, result);
            }
            for node2 in other.nodes() {
                leaf1.intersect_node(&node2, result);
            }
        }
        for node1 in self.nodes() {
            for leaf2 in other.leaves() {
                node1.intersect_leaf(&leaf2, result);
            }
            for node2 in other.nodes() {
                node1.intersect_node(&node2, result);
            }
        }
    }
    pub fn intersect_leaf(&self, other: &BvhTriangle, result: &mut Vec<(usize, usize)>) {
        if IGNORE_AABB3 || sat_intersects(self.aabb(), &other.triangle) {
            for node1 in self.nodes() {
                node1.intersect_leaf(&other, result);
            }
            for leaf1 in self.leaves() {
                leaf1.intersect_leaf(&other, result);
            }
        }
    }
    pub fn intersect_ray(&self, ray: &Ray3, result: &mut Vec<RayMeshIntersection>) {
        if IGNORE_AABB4 || ray.intersect_aabb(self.aabb()).is_some() {
            for leaf in self.leaves() {
                leaf.intersect_ray(ray, result);
            }
            for node in self.nodes() {
                node.intersect_ray(&ray, result);
            }
        }
    }
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
        self.root_view().fmt(f)
    }
}

impl Debug for BvhTriangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("BvhTriangle")
            .field("index", &self.index)
            .field("vertices", &self.triangle)
            .finish()
    }
}

#[test]
fn test() {
    let sphere = Sphere::new(Vec3::zero(), 1.0).as_mesh(0);
    let bvh = Bvh::from_mesh(&sphere);
}
