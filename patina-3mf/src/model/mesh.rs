// Derived from https://docs.rs/threemf/latest/threemf/

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
pub struct ModelMesh {
    pub vertices: ModelVertices,
    pub triangles: ModelTriangles,
}

impl ModelMesh {
    pub fn new(vertices: ModelVertices, triangles: ModelTriangles) -> Self {
        ModelMesh {
            vertices,
            triangles,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
pub struct ModelVertices {
    #[serde(default)]
    pub vertex: Vec<ModelVertex>,
}

impl ModelVertices {
    pub fn new(vertex: Vec<ModelVertex>) -> Self {
        ModelVertices { vertex }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
pub struct ModelTriangles {
    #[serde(default)]
    pub triangle: Vec<ModelTriangle>,
}

impl ModelTriangles {
    pub fn new(triangle: Vec<ModelTriangle>) -> Self {
        ModelTriangles { triangle }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
pub struct ModelVertex {
    #[serde(rename = "@x")]
    pub x: f64,
    #[serde(rename = "@y")]
    pub y: f64,
    #[serde(rename = "@z")]
    pub z: f64,
}

impl ModelVertex {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        ModelVertex { x, y, z }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[non_exhaustive]
pub struct ModelTriangle {
    #[serde(rename = "@v1")]
    pub v1: usize,
    #[serde(rename = "@v2")]
    pub v2: usize,
    #[serde(rename = "@v3")]
    pub v3: usize,
}

impl ModelTriangle {
    pub fn new(v1: usize, v2: usize, v3: usize) -> Self {
        ModelTriangle { v1, v2, v3 }
    }
}
