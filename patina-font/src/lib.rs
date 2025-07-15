#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]

#[cfg(test)]
mod test;

use patina_geo::geo2::polygon2::Polygon2;
use patina_mesh::ser::stl::write_test_stl_file;
use patina_vec::vec2::Vec2;
use rusttype::{OutlineBuilder, Point};

pub struct PolygonOutlineBuilder {
    polys: Vec<Polygon2>,
    poly: Vec<Vec2>,
}

impl OutlineBuilder for PolygonOutlineBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        assert!(self.poly.is_empty());
        self.poly.push(Vec2::new(x as f64, y as f64));
    }
    fn line_to(&mut self, x: f32, y: f32) {
        self.poly.push(Vec2::new(x as f64, y as f64));
    }
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        todo!()
    }
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        todo!()
    }
    fn close(&mut self) {
        assert_eq!(self.poly.first(), self.poly.last());
        self.poly.pop();
        let poly = Polygon2::new(self.poly.clone());
        println!("{}", poly);
        poly.check_self_separate().unwrap();
        self.polys.push(poly);
        self.poly.clear();
    }
}

impl PolygonOutlineBuilder {
    pub fn new() -> Self {
        PolygonOutlineBuilder {
            polys: vec![],
            poly: vec![],
        }
    }
    pub fn build(self) -> Vec<Polygon2> {
        assert!(self.poly.is_empty());
        self.polys
    }
}
