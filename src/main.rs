use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

mod cells;
mod framebuffer;

use cells::Cells;
use framebuffer::Framebuffer;

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

pub fn draw_cells_from_rle(expression: &str, start_x: usize, start_y: usize, cells: &mut Cells) {
    let mut x = start_x;
    let mut y = start_y;
    let mut run_count = 0;

    for c in expression.chars() {
        match c {
            'b' => {
                let count = if run_count == 0 { 1 } else { run_count };
                for _ in 0..count {
                    cells.set_dead(x, y);
                    x += 1;
                }
                run_count = 0;
            }
            'o' => {
                let count = if run_count == 0 { 1 } else { run_count };
                for _ in 0..count {
                    cells.set_alive(x, y);
                    x += 1;
                }
                run_count = 0;
            }
            '$' => {
                x = start_x;
                y += 1;
            }
            '0'..='9' => {
                run_count = run_count * 10 + (c as usize - '0' as usize);
            }
            '!' => break,
            _ => (),
        }
    }
}

fn main() {
    let window_width = 600;
    let window_height = 500;

    let framebuffer_width = 160;
    let framebuffer_height = 140;

    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut cells = Cells::new(framebuffer_width, framebuffer_height);

    // Draw RLE pattern
    let rle_expression = "bo$2bo$3o!";
    let start_x = 3;
    let start_y = 3;

    draw_cells_from_rle(rle_expression, start_x, start_y, &mut cells);

    let rle_expression2 = "44b2o4b$44b2o4b9$41b2obob2o2b2$41bo5bo2b2$42b2ob2o3b$44bo5b3$38b2o6bo3b$37bobo5bobo2b$12bo26bo4bo3bob$13bo30b5ob$11b3o29b2o3b2o$44b5ob$45b3o2b$46bo3b$24b2o4b3o17b$24b2o6bo17b$31bo18b5$23b2o25b$22bobo21b2o2b$24bo21b2o2b$13bo36b$12b4o34b$11b2obobo6bobo24b$2o8b3obo2bo3bo3bo24b$2o9b2obobo4bo28b$12b4o4bo4bo24b$13bo7bo28b$21bo3bo6b2o16b$23bobo6bobo15b$34bo15b$34b2o!";

    let start_x2 = 50;
    let start_y2 = 47;

    draw_cells_from_rle(rle_expression2, start_x2, start_y2, &mut cells);

    let rle_expression3 = "2b3o3b3o2b2$o4bobo4bo$o4bobo4bo$o4bobo4bo$2b3o3b3o2b2$2b3o3b3o2b$o4bobo4bo$o4bobo4bo$o4bobo4bo2$2b3o3b3o!";

    let start_x3 = 13;
    let start_y3 = 13;
    draw_cells_from_rle(rle_expression3, start_x3, start_y3, &mut cells);

    let mut window = Window::new(
        "Rust Graphics - Game of Life",
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
