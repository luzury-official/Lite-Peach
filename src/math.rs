use macroquad::prelude::{Vec3, Vec2, Color};

pub struct Vertex {
    pub position: Vec3,
    pub color: Color,
    pub uv: Vec2,
}

impl Vertex {
    pub fn new(pos: [f32; 3], color: Color, uv: [f32; 2]) -> Self {
        Self {
            position: Vec3::from_array(pos),
            color,
            uv: Vec2::from_array(uv),
        }
    }
}

pub struct Triangle {
    pub vertices: [Vertex; 3],
}

impl Triangle {
    pub fn new(v1: Vertex, v2: Vertex, v3: Vertex) -> Self {
        Self {
            vertices: [v1, v2, v3],
        }
    }

    pub fn get_center(&self) -> Vec3 {
        (self.vertices[0].position + self.vertices[1].position + self.vertices[2].position) / 3.0
    }
}

pub struct Quaternion {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Quaternion {
    pub fn identity() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }
    }

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn get_norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalize(&mut self) {
        let n = self.get_norm();
        if n > 0.0 {
            self.x /= n;
            self.y /= n;
            self.z /= n;
            self.w /= n;
        }
    }

    pub fn conjugate(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }

    pub fn mul(q1: &Quaternion, q2: &Quaternion) -> Quaternion {
        Quaternion {
            w: q1.w * q2.w - q1.x * q2.x - q1.y * q2.y - q1.z * q2.z,
            x: q1.w * q2.x + q1.x * q2.w + q1.y * q2.z - q1.z * q2.y,
            y: q1.w * q2.y - q1.x * q2.z + q1.y * q2.w + q1.z * q2.x,
            z: q1.w * q2.z + q1.x * q2.y - q1.y * q2.x + q1.z * q2.w,
        }
    }

    pub fn from_axis_angle(axis: Vec3, angle_rad: f32) -> Self {
        let axis = axis.normalize();
        
        let (sin_half, cos_half) = (angle_rad * 0.5).sin_cos();

        Self {
            x: axis.x * sin_half,
            y: axis.y * sin_half,
            z: axis.z * sin_half,
            w: cos_half,
        }
    }

    pub fn rotate_vector(&self, v: Vec3) -> Vec3 {
        let u = Vec3::new(self.x, self.y, self.z);
        
        let t = u.cross(v) * 2.0;
        
        v + t * self.w + u.cross(t)
    }
}