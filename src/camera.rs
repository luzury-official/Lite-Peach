use macroquad::prelude::Vec3;
use crate::math::Quaternion;
use crate::object::Transform;

pub struct Camera3D {
    pub transform: Transform,
    pub target: Vec3,
    pub up: Vec3,
    pub fovy: f32,
}

#[derive(Clone, Copy)]
pub enum MoveMode {
    Free,
    Grounded,
    Global,
}

impl Camera3D {
    pub fn new(pos: Vec3) -> Self {
        Self {
            transform: Transform::new(pos),
            target: pos + Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            fovy: 45.0,
        }
    }

    pub fn set_position(&mut self, pos: Vec3) {
        self.transform.position = pos;
        self.update_target();
    }

    fn update_target(&mut self) {
        let forward = self.transform.rotation.rotate_vector(Vec3::new(0.0, 0.0, -1.0));
        self.target = self.transform.position + forward;
    }

    pub fn rotate_camera(&mut self, axis: char, angle: f32) {
        self.transform.rotate(axis, angle);
        self.update_target();
    }

    pub fn move_camera(&mut self, direction: Vec3, speed: f32) {
        self.transform.move_transform(direction, speed);
        self.update_target();
    }
}