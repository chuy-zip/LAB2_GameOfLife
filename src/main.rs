extern crate nalgebra_glm as glm;

use glm::Vec3;
use minifb::{Window, WindowOptions};
use std::time::Duration;

mod framebuffer;
mod line;
mod polygon;

use crate::framebuffer::Framebuffer;
use crate::polygon::Polygon;

fn main() {
    let window_width = 800;
    let window_height = 600;

    let framebuffer_width = window_width;
    let framebuffer_height = window_height;

    let close_delay = Duration::from_secs(10);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Hello window :o",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    let max_y = framebuffer.height as f32;

    let flip_y = |y: f32| max_y - y;

    let points = vec![
        Vec3::new(100.0, flip_y(100.0), 0.0),
        Vec3::new(200.0, flip_y(100.0), 0.0),
        Vec3::new(150.0, flip_y(200.0), 0.0),
    ];

    framebuffer.filled_polygon_with_outline(&points, 0xFFFF00, 0xFFFFFF, 2);

    window
        .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
        .unwrap();

    std::thread::sleep(close_delay);
}
