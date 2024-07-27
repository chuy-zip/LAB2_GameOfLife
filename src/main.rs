use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

mod framebuffer;
mod cells;

use framebuffer::Framebuffer;
use cells::Cells;

fn render(framebuffer: &mut Framebuffer, cells: &Cells) {
    // Clear the framebuffer
    framebuffer.set_background_color(0x333355);
    framebuffer.clear();

    // Draw cells based on their state
    framebuffer.set_current_color(0xFFDDDD);
    for y in 0..cells.height {
        for x in 0..cells.width {
            if cells.is_alive(x, y) {
                framebuffer.point(x, y);
            }
        }
    }
}

fn main() {
    let window_width = 200;
    let window_height = 200;

    let framebuffer_width = 80;
    let framebuffer_height = 60;

    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut cells = Cells::new(framebuffer_width, framebuffer_height);

    // Ejemplo: establecer algunas células vivas
    cells.set_alive(10, 10);
    cells.set_alive(11, 10);
    cells.set_alive(12, 10);

    let mut window = Window::new(
        "Rust Graphics - Render Loop Example",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    while window.is_open() {
        // listen to inputs
        if window.is_key_down(Key::Escape) {
            break;
        }

        // render
        render(&mut framebuffer, &cells);

        // Update the window with the framebuffer contents
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
