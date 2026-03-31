mod math;
mod object;
mod world;
mod render;
mod camera;

use macroquad::prelude::*;
use crate::world::World;
use crate::render::Renderer;
use crate::camera::Camera3D;

#[macroquad::main("LitePeach")]
async fn main() {
    let world = World::new();
    let _renderer = Renderer::new();
    
    let mut cam = Camera3D::new(vec3(10.0, 10.0, 10.0));

    let speed = 0.2;
    let rot_speed = 1.5;

    loop {
        clear_background(BLACK);

        if is_key_down(KeyCode::W) { cam.move_camera(vec3(0.0, 0.0, -1.0), speed); }
        if is_key_down(KeyCode::S) { cam.move_camera(vec3(0.0, 0.0, 1.0), speed); }
        if is_key_down(KeyCode::A) { cam.move_camera(vec3(-1.0, 0.0, 0.0), speed); }
        if is_key_down(KeyCode::D) { cam.move_camera(vec3(1.0, 0.0, 0.0), speed); }

        if is_key_down(KeyCode::Left) { cam.rotate_camera('y', rot_speed); }
        if is_key_down(KeyCode::Right) { cam.rotate_camera('y', -rot_speed); }
        if is_key_down(KeyCode::Up) { cam.rotate_camera('x', rot_speed); }
        if is_key_down(KeyCode::Down) { cam.rotate_camera('x', -rot_speed); }

        set_camera(&macroquad::camera::Camera3D {
            position: cam.transform.position,
            target: cam.target,
            up: cam.up,
            fovy: cam.fovy,
            ..Default::default()
        });

        draw_grid(50, 1.0, WHITE, GRAY);
        draw_cube(vec3(0.0, 0.0, 0.0), vec3(2.0, 2.0, 2.0), None, RED);
        draw_cube(vec3(5.0, 0.0, 0.0), vec3(1.0, 1.0, 1.0), None, GREEN);

        set_default_camera();

        next_frame().await
    }
}