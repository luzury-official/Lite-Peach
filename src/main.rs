mod math;
mod object;
mod world;
mod render;
mod camera;

use macroquad::prelude::*;
use crate::world::World;
use crate::render::Renderer;
use crate::camera::Camera3D;

#[macroquad::main("My 3D Engine")]
async fn main() {
    let mut world = World::new();
    let renderer = Renderer::new();

    let mut player = Camera3D::new(
        vec3(5.0, 5.0, 5.0),
        vec3(0.0, 0.0, 0.0),
    );

    loop {
        player.rotate_camera('y', 0.5);
        renderer.draw_world(&world, &player);
        next_frame().await
    }
}