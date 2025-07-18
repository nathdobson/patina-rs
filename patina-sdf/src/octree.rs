use crate::exact::Exact;
use crate::transvoxel::cube_edge::CubeEdge;
use crate::transvoxel::cube_face::CubeFace;
use arrayvec::ArrayVec;
use inari::DecInterval;
use itertools::Itertools;
use ordered_float::NotNan;
use patina_geo::aabb::Aabb;
use patina_scalar::Scalar;
use patina_vec::vec3::{Vec3, Vector3};
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Index, IndexMut};

#[derive(Debug)]
pub struct Octree<V> {
    path: OctreePath,
    node: OctreeNode<V>,
}

#[derive(Debug)]
enum OctreeNode<V> {
    Leaf(V),
    Branch(Box<OctreeBranch<V>>),
}

pub enum OctreeView<'a, V> {
    Leaf(&'a V),
    Branch(&'a OctreeBranch<V>),
}

pub enum OctreeViewMut<'a, V> {
    Leaf(&'a mut V),
    Branch(&'a mut OctreeBranch<V>),
}

#[derive(Debug)]
pub struct OctreeBranch<V> {
    children: [[[Octree<V>; 2]; 2]; 2],
}

pub struct OctreeIndex([bool; 3]);

#[derive(Copy, Clone, Hash, Ord, Eq, PartialEq, PartialOrd)]
pub struct OctreePath {
    depth: usize,
    position: [usize; 3],
}

impl From<[bool; 3]> for OctreeIndex {
    fn from(value: [bool; 3]) -> Self {
        OctreeIndex(value)
    }
}

impl From<OctreeIndex> for [bool; 3] {
    fn from(value: OctreeIndex) -> Self {
        value.0
    }
}

impl OctreePath {
    pub fn new_root() -> Self {
        OctreePath {
            depth: 0,
            position: [0; 3],
        }
    }
    pub fn push_back(&self, index: OctreeIndex) -> Self {
        OctreePath {
            depth: self.depth + 1,
            position: self
                .position
                .map_with_index(|axis, p| (p << 1) + index.0[axis] as usize),
        }
    }
    pub fn view(&self) -> Option<(OctreeIndex, Self)> {
        if self.depth == 0 {
            return None;
        }
        let index = OctreeIndex(
            (0..3)
                .map(|axis| (self.position[axis] & (1 << (self.depth - 1))) != 0)
                .collect_array()
                .unwrap(),
        );
        let subpath = OctreePath {
            depth: self.depth - 1,
            position: self.position.map(|x| x & ((1 << (self.depth - 1)) - 1)),
        };
        Some((index, subpath))
    }
    pub fn aabb(&self) -> Aabb {
        Aabb::new(
            self.position
                .map(|x| (x as f64) / ((1 << self.depth) as f64))
                .into(),
            self.position
                .map(|x| ((x + 1) as f64) / ((1 << self.depth) as f64))
                .into(),
        )
    }
    pub fn aabb_inside(&self, root: &Aabb) -> Aabb {
        let fract = self.aabb();
        let base = root.min();
        let delta = root.dimensions();
        Aabb::new(
            fract.min().mul_elements(delta) + base,
            fract.max().mul_elements(delta) + base,
        )
    }
    pub fn depth(&self) -> usize {
        self.depth
    }
    pub fn face_adjacent_for(&self, face: CubeFace) -> Option<Self> {
        let mut position2 = self.position;
        let mut coord = &mut position2[face.axis() as usize];
        if !face.side() && *coord > 0 {
            *coord -= 1;
            Some(OctreePath {
                depth: self.depth,
                position: position2,
            })
        } else if face.side() && *coord < 1 << self.depth {
            *coord += 1;
            Some(OctreePath {
                depth: self.depth,
                position: position2,
            })
        } else {
            None
        }
    }
    pub fn edge_adjacent_for(&self, edge: CubeEdge) -> Option<Self> {
        let mut position2 = self.position;
        let mut coord = &mut position2[edge.axis1() as usize];
        if !edge.side1() && *coord > 0 {
            *coord -= 1;
        } else if edge.side1() && *coord < 1 << self.depth {
            *coord += 1;
        } else {
            return None;
        }
        let mut coord = &mut position2[edge.axis2() as usize];
        if !edge.side2() && *coord > 0 {
            *coord -= 1;
        } else if edge.side2() && *coord < 1 << self.depth {
            *coord += 1;
        } else {
            return None;
        }
        Some(OctreePath {
            depth: self.depth,
            position: position2,
        })
    }
    pub fn face_adjacent(&self) -> ArrayVec<Self, 6> {
        let mut result = ArrayVec::new();
        for face in CubeFace::all() {
            if let Some(adj) = self.face_adjacent_for(face) {
                result.push(adj);
            }
        }
        result
    }
    pub fn edge_adjacent(&self) -> ArrayVec<Self, 12> {
        let mut result = ArrayVec::new();
        for edge in CubeEdge::all() {
            if let Some(adj) = self.edge_adjacent_for(edge) {
                result.push(adj);
            }
        }
        result
    }
    pub fn position(&self) -> Vector3<usize> {
        Vector3::from(self.position)
    }
}

impl Debug for OctreeIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{}{}{}>",
            self.0[0] as u8, self.0[1] as u8, self.0[2] as u8
        )
    }
}

impl Debug for OctreePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        for axis in 0..3 {
            if self.depth > 0 {
                write!(f, "{:0width$b}", self.position[axis], width = self.depth)?;
            } else {
                write!(f, "-")?;
            }
            if axis < 2 {
                write!(f, ", ")?;
            }
        }
        write!(f, ")")?;
        Ok(())
    }
}

impl<V> OctreeBranch<V> {
    pub fn children_flat(&self) -> [&Octree<V>; 8] {
        self.children
            .iter()
            .flatten()
            .flatten()
            .collect_array()
            .unwrap()
    }
    pub fn children_flat_mut(&mut self) -> [&mut Octree<V>; 8] {
        self.children
            .iter_mut()
            .flatten()
            .flatten()
            .collect_array()
            .unwrap()
    }
    pub fn child(&self, index: OctreeIndex) -> &Octree<V> {
        &self.children[index.0[0] as usize][index.0[1] as usize][index.0[2] as usize]
    }
    pub fn child_mut(&mut self, index: OctreeIndex) -> &mut Octree<V> {
        &mut self.children[index.0[0] as usize][index.0[1] as usize][index.0[2] as usize]
    }
}

impl<V> Octree<V> {
    pub fn new_root(value: V) -> Self {
        Octree {
            path: OctreePath::new_root(),
            node: OctreeNode::Leaf(value),
        }
    }
    pub fn new_leaf(path: OctreePath, value: V) -> Self {
        Octree {
            path,
            node: OctreeNode::Leaf(value),
        }
    }
    pub fn path(&self) -> &OctreePath {
        &self.path
    }
    pub fn view(&self) -> OctreeView<'_, V> {
        match &self.node {
            OctreeNode::Leaf(leaf) => OctreeView::Leaf(leaf),
            OctreeNode::Branch(branch) => OctreeView::Branch(branch),
        }
    }
    pub fn view_mut(&mut self) -> OctreeViewMut<'_, V> {
        match &mut self.node {
            OctreeNode::Leaf(leaf) => OctreeViewMut::Leaf(leaf),
            OctreeNode::Branch(branch) => OctreeViewMut::Branch(branch),
        }
    }
    pub fn set_branch(&mut self, values: [[[V; 2]; 2]; 2]) {
        let children = values.map_with_index(|x, values| {
            values.map_with_index(|y, values| {
                values.map_with_index(|z, value| {
                    Octree::new_leaf(
                        self.path
                            .push_back(OctreeIndex::from([x != 0, y != 0, z != 0])),
                        value,
                    )
                })
            })
        });
        self.node = OctreeNode::Branch(Box::new(OctreeBranch { children }));
    }
    // fn depth(&self, path: OctreePath) -> usize {
    //     if let Some((index, path)) = path.view() {
    //
    //     }
    //     match self.view() {
    //         OctreeView::Leaf(_) => {}
    //         OctreeView::Branch(_) => {}
    //     }
    // }
}

impl<V> Index<OctreeIndex> for OctreeBranch<V> {
    type Output = Octree<V>;
    fn index(&self, index: OctreeIndex) -> &Self::Output {
        &self.children[index.0[0] as usize][index.0[1] as usize][index.0[2] as usize]
    }
}

impl<V> IndexMut<OctreeIndex> for OctreeBranch<V> {
    fn index_mut(&mut self, index: OctreeIndex) -> &mut Self::Output {
        &mut self.children[index.0[0] as usize][index.0[1] as usize][index.0[2] as usize]
    }
}

pub trait MapWithIndex<const N: usize, T1, T2> {
    fn map_with_index<F: FnMut(usize, T1) -> T2>(self, f: F) -> [T2; N];
}

impl<const N: usize, T1, T2> MapWithIndex<N, T1, T2> for [T1; N] {
    fn map_with_index<F: FnMut(usize, T1) -> T2>(self, mut f: F) -> [T2; N] {
        let mut result = ArrayVec::<T2, N>::new();
        for (i, x) in self.into_iter().enumerate() {
            result.push(f(i, x));
        }
        result.into_inner().ok().unwrap()
    }
}

#[test]
fn test_octree_path() {
    let path = OctreePath::new_root();
    assert_eq!("(-, -, -)", format!("{:?}", path));
    let path = path.push_back(OctreeIndex::from([true, false, false]));
    assert_eq!("(1, 0, 0)", format!("{:?}", path));
    let path = path.push_back(OctreeIndex::from([false, true, false]));
    assert_eq!("(10, 01, 00)", format!("{:?}", path));
    let path = path.push_back(OctreeIndex::from([false, false, true]));
    assert_eq!("(100, 010, 001)", format!("{:?}", path));
    let (index, path) = path.view().unwrap();
    assert_eq!("<100>", format!("{:?}", index));
    assert_eq!("(00, 10, 01)", format!("{:?}", path));
    let (index, path) = path.view().unwrap();
    assert_eq!("<010>", format!("{:?}", index));
    assert_eq!("(0, 0, 1)", format!("{:?}", path));
    let (index, path) = path.view().unwrap();
    assert_eq!("<001>", format!("{:?}", index));
    assert_eq!("(-, -, -)", format!("{:?}", path));
    assert!(path.view().is_none());
}
