use crate::math::{Vertex, Triangle, Quaternion};
use macroquad::prelude::Vec3;

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub transform: Transform,
}

impl Mesh {
    pub fn new(pos: Vec3) -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            transform: Transform::new(pos),
        }
    }

    pub fn add_vertex(&mut self, v: Vertex) {
        self.vertices.push(v);
    }

    pub fn add_triangle(&mut self, i1: u32, i2: u32, i3: u32) {
        self.indices.push(i1);
        self.indices.push(i2);
        self.indices.push(i3);
    }
}

pub struct Transform {
    pub position: Vec3,
    pub rotation: Quaternion,
    pub scale: f32,
}

impl Transform {
    pub fn new(pos: Vec3) -> Self {
        Self {
            position: pos,
            rotation: Quaternion::identity(),
            scale: 1.0,
        }
    }

    pub fn rotate(&mut self, axis: char, angle: f32) {
        let axis_vec = match axis {
            'x' => {
                let forward = self.rotation.rotate_vector(Vec3::new(0.0, 0.0, -1.0));
                let right = forward.cross(Vec3::new(0.0, 1.0, 0.0)).normalize();
                right
            }
            'y' => Vec3::new(0.0, 1.0, 0.0),
            'z' => Vec3::new(0.0, 0.0, -1.0),
            _ => return,
        };

        let delta_rot = Quaternion::from_axis_angle(axis_vec, angle.to_radians());
        self.rotation = Quaternion::mul(&delta_rot, &self.rotation);
    }

    pub fn move_transform(&mut self, direction: Vec3, speed: f32) {
        let move_vec = self.rotation.rotate_vector(direction);
        self.position += move_vec * speed;
    }
}