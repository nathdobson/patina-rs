// Derived from https://docs.rs/threemf/latest/threemf/

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
pub struct Mesh {
    pub vertices: Vertices,
    pub triangles: Triangles,
}

impl Mesh {
    pub fn new(vertices: Vertices, triangles: Triangles) -> Self {
        Mesh {
            vertices,
            triangles,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
pub struct Vertices {
    #[serde(default)]
    pub vertex: Vec<Vertex>,
}

impl Vertices {
    pub fn new(vertex: Vec<Vertex>) -> Self {
        Vertices { vertex }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
pub struct Triangles {
    #[serde(default)]
    pub triangle: Vec<Triangle>,
}

impl Triangles {
    pub fn new(triangle: Vec<Triangle>) -> Self {
        Triangles { triangle }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
pub struct Vertex {
    #[serde(rename = "@x")]
    pub x: f64,
    #[serde(rename = "@y")]
    pub y: f64,
    #[serde(rename = "@z")]
    pub z: f64,
}

impl Vertex {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vertex { x, y, z }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
pub struct Triangle {
    #[serde(rename = "@v1")]
    pub v1: usize,
    #[serde(rename = "@v2")]
    pub v2: usize,
    #[serde(rename = "@v3")]
    pub v3: usize,
}

impl Triangle {
    pub fn new(v1: usize, v2: usize, v3: usize) -> Self {
        Triangle { v1, v2, v3 }
    }
}
