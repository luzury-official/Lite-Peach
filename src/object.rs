use crate::math::{Vertex, Triangle};
use macroquad::prelude::Vec3;

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub position: Vec3,
    pub rotation: Vec3, // That vector describes where object is rotated
}

impl Mesh {
    pub fn new(pos: Vec3) -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            position: pos,
            rotation: Vec3::ZERO,
        }
    }

    pub fn add_vertex(&mut self,v: Vertex) {
        self.vertices.push(v);
    }

    pub fn add_triangle(&mut self,i1: u32, i2: u32, i3: u32) {
        self.indices.push(i1);
        self.indices.push(i2);
        self.indices.push(i3);
    }
}