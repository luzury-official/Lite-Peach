use crate::world::World;
use crate::object::Mesh as MyMesh;
use crate::camera::Camera3D as MyCamera;
use macroquad::models::Vertex as MQVertex;
use macroquad::models::Mesh as MQMesh;
use macroquad::prelude::*;

pub struct Renderer;

impl Renderer {
    pub fn new() -> Self {
        Self
    }

    pub fn draw_world(&self, world: &World, cam: &MyCamera) {
        set_camera(&macroquad::prelude::Camera3D {
            position: cam.transform.position,
            target: cam.target,
            up: cam.up,
            fovy: cam.fovy,
            ..Default::default()
        });

        for mesh in &world.objects {
            self.draw_mesh(mesh);
        }

        set_default_camera();
        draw_text("LitePeach Engine v0.1", 10.0, 20.0, 20.0, WHITE);
    }

    fn draw_mesh(&self, mesh: &MyMesh) {
        let vertices: Vec<MQVertex> = mesh.vertices
            .iter()
            .map(|v| {
                let rotated_pos = mesh.transform.rotation.rotate_vector(v.position);
                MQVertex {
                    position: rotated_pos + mesh.transform.position,
                    uv: v.uv,
                    color: v.color.into(),
                    normal: vec4(0.0, 1.0, 0.0, 0.0),
                }
            })
            .collect();

        let indices: Vec<u16> = mesh.indices
            .iter()
            .map(|&i| i as u16)
            .collect();

        let mq_mesh = MQMesh {
            vertices,
            indices,
            texture: None,
        };

        draw_mesh(&mq_mesh);
    }
}