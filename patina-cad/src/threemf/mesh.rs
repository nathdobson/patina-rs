// Derived from https://docs.rs/threemf/latest/threemf/

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Mesh {
    pub vertices: Vertices,
    pub triangles: Triangles,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Vertices {
    #[serde(default)]
    pub vertex: Vec<Vertex>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Triangles {
    #[serde(default)]
    pub triangle: Vec<Triangle>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Vertex {
    #[serde(rename = "@x")]
    pub x: f64,
    #[serde(rename = "@y")]
    pub y: f64,
    #[serde(rename = "@z")]
    pub z: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Triangle {
    #[serde(rename = "@v1")]
    pub v1: usize,
    #[serde(rename = "@v2")]
    pub v2: usize,
    #[serde(rename = "@v3")]
    pub v3: usize,
}
