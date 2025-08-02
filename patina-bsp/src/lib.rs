#![deny(unused_must_use)]
#![allow(dead_code)]

use patina_vec::vec3::Vec3;

pub enum Bsp {
    Leaf(BspLeaf),
    Branch(BspBranch),
}

pub struct BspBranch {
    normal: Vec3,
    offset: f64,
    children: [Box<Bsp>; 2],
}

pub struct BspLeaf {
    inside: bool,
}

#[test]
fn test() {

}
