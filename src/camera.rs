use macroquad::prelude::Vec3;
use crate::math::Quaternion;

pub struct Camera3D {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fovy: f32,
}

impl Camera3D {
    pub fn new(pos: Vec3, tar: Vec3) -> Self {
        Self {
            position: pos,
            target: tar,
            up: Vec3::new(0.0, 1.0, 0.0),
            fovy: 45.0,
        }
    }

    pub fn set_position(&mut self, pos: Vec3, tar: Vec3) {
        self.position = pos;
        self.target = tar;
    }

    pub fn rotate_camera(&mut self, axis: char, angle: f32) {
        let axis_vec = match axis {
            'x' => Vec3::new(1.0, 0.0, 0.0),
            'y' => Vec3::new(0.0, 1.0, 0.0),
            'z' => Vec3::new(0.0, 0.0, 1.0),
            _ => {
                println!("This axis doesn't exist!");
                return;
            }
        };

        let rotation = Quaternion::from_axis_angle(axis_vec, angle.to_radians());

        let relative_pos = self.position - self.target;

        let rotated_pos = rotation.rotate_vector(relative_pos);

        self.position = self.target + rotated_pos;
    }
}