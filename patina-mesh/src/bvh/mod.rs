mod edge_bvh2;

use crate::edge_mesh2::EdgeMesh2;
use crate::mesh::Mesh;
use crate::mesh_triangle::MeshTriangle;
use crate::util::scan::ScanIteratorExt;
use ordered_float::NotNan;
use patina_geo::aabb::Aabb;
use patina_geo::geo3::triangle3::Triangle3;
use patina_vec::vec::Vector;
use patina_vec::vec3::{Vec3, Vector3};
use rand::Rng;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use std::sync::Arc;
use patina_geo::geo2::segment2::Segment2;

#[derive(Debug)]
pub struct BvhNode<const N: usize, V> {
    aabb: Aabb<N>,
    nodes: Vec<usize>,
    leaves: Vec<V>,
}

pub struct BvhNodeView<'a, const N: usize, M, V> {
    bvh: &'a Bvh<N, M, V>,
    node: usize,
}

pub struct BvhLeafView<'a, const N: usize, M, V> {
    mesh: &'a M,
    leaf: &'a V,
}

pub struct Bvh<const N: usize, M, V> {
    mesh: Arc<M>,
    root: usize,
    nodes: Vec<BvhNode<N, V>>,
}

pub struct BvhBuilder<const N: usize, V> {
    nodes: Vec<BvhNode<N, V>>,
    max_split_size: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct BvhLeafBuilder<const N: usize, V> {
    leaf: V,
    midpoint: Vector<f64, N>,
    aabb: Aabb<N>,
}

impl<const N: usize, V: Clone> BvhBuilder<N, V> {
    pub fn new() -> Self {
        BvhBuilder {
            nodes: vec![],
            max_split_size: 4,
        }
    }
    fn build<M>(self, root: usize, mesh: Arc<M>) -> Bvh<N, M, V> {
        Bvh {
            mesh,
            root,
            nodes: self.nodes,
        }
    }
    pub fn add_leaf(&mut self, leaves: &[BvhLeafBuilder<N, V>]) -> usize {
        let aabb = leaves
            .iter()
            .fold(Aabb::empty(), |aabb, t| aabb.union(&t.aabb));
        self.nodes.push(BvhNode {
            aabb,
            nodes: vec![],
            leaves: leaves.iter().map(|t| t.leaf.clone()).collect(),
        });
        self.nodes.len() - 1
    }
    pub fn add_node(
        &mut self,
        left: &[BvhLeafBuilder<N, V>],
        right: &[BvhLeafBuilder<N, V>],
    ) -> usize {
        let aabb = left
            .iter()
            .chain(right.iter())
            .fold(Aabb::empty(), |aabb, t| aabb.union(&t.aabb));
        let left = self.add_leaves(left);
        let right = self.add_leaves(right);
        self.nodes.push(BvhNode {
            aabb,
            nodes: vec![left, right],
            leaves: vec![],
        });
        self.nodes.len() - 1
    }
    pub fn add_leaves(&mut self, tris: &[BvhLeafBuilder<N, V>]) -> usize {
        if tris.len() < self.max_split_size {
            return self.add_leaf(tris);
        }
        let (left, right, _) = (0..3)
            .map(|axis| {
                let mut by_axis = tris.to_vec();
                by_axis.sort_by_key(|x| NotNan::new(x.midpoint[axis]).unwrap());
                let forward_surface_area: Vec<_> = by_axis
                    .iter()
                    .scan_full(Aabb::empty(), |b, t| b.union(&t.aabb))
                    .map(|x| x.surface_measure())
                    .collect();
                let mut reverse_surface_area: Vec<_> = by_axis
                    .iter()
                    .rev()
                    .scan_full(Aabb::empty(), |b, t| b.union(&t.aabb))
                    .map(|x| x.surface_measure())
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

impl<const N: usize, V: Clone> BvhLeafBuilder<N, V> {
    pub fn new(leaf: V, midpoint: Vector<f64, N>, aabb: Aabb<N>) -> Self {
        BvhLeafBuilder {
            leaf,
            midpoint,
            aabb,
        }
    }
}

impl BvhLeafBuilder<3, usize> {
    pub fn from_triangle(index: usize, tri: Triangle3) -> Self {
        BvhLeafBuilder::new(
            index,
            tri.points().iter().sum::<Vec3>() / 3.0,
            tri.points().iter().cloned().collect(),
        )
    }
}

impl BvhLeafBuilder<2, usize> {
    pub fn from_segment(index: usize, seg: Segment2) -> Self {
        BvhLeafBuilder::new(
            index,
            seg.midpoint(),
            seg.points().iter().cloned().collect(),
        )
    }
}

impl Bvh<3, Mesh, usize> {
    pub fn from_mesh(mesh: Arc<Mesh>) -> Self {
        Bvh::new(
            &mesh
                .triangles()
                .iter()
                .enumerate()
                .map(|(index, t)| {
                    BvhLeafBuilder::from_triangle(index, t.for_vertices(mesh.vertices()))
                })
                .collect::<Vec<_>>(),
            mesh,
        )
    }
}

impl<const N: usize, M, V> Bvh<N, M, V> {
    pub fn new(leaves: &[BvhLeafBuilder<N, V>], mesh: Arc<M>) -> Self
    where
        V: Clone,
    {
        let mut builder = BvhBuilder::new();
        let root = builder.add_leaves(&leaves);
        builder.build(root, mesh)
    }

    pub fn root_view(&self) -> BvhNodeView<'_, N, M, V> {
        BvhNodeView {
            bvh: self,
            node: self.root,
        }
    }
}

impl<'a, const N: usize, M, V> BvhNodeView<'a, N, M, V> {
    pub fn index(&self) -> usize {
        self.node
    }
    pub fn aabb(&self) -> &Aabb<N> {
        &self.bvh.nodes[self.node].aabb
    }
    pub fn leaves(&self) -> impl Iterator<Item = BvhLeafView<'a, N, M, V>> {
        self.bvh.nodes[self.node]
            .leaves
            .iter()
            .map(|index| BvhLeafView {
                mesh: &*self.bvh.mesh,
                leaf: index,
            })
    }
    pub fn nodes<'b>(&'b self) -> impl Iterator<Item = BvhNodeView<'a, N, M, V>> {
        self.bvh.nodes[self.node]
            .nodes
            .iter()
            .map(|child| BvhNodeView {
                bvh: self.bvh,
                node: *child,
            })
    }
}

impl<'a, const N: usize, M, V: Debug> Debug for BvhNodeView<'a, N, M, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut f = f.debug_struct("BvhNode");
        f.field("area", &self.aabb().surface_measure());
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

impl<const N: usize, M, V: Debug> Debug for Bvh<N, M, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.root_view().fmt(f)
    }
}

impl<'a, const N: usize, M, V> BvhLeafView<'a, N, M, V> {
    pub fn leaf(&self) -> &'a V {
        self.leaf
    }
}

impl<'a, const N: usize, M, V: Debug> Debug for BvhLeafView<'a, N, M, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("BvhTriangleView")
            .field("leaf", &self.leaf)
            .finish()
    }
}
