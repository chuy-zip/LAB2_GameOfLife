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
                    if x < cells.width && y < cells.height {
                        cells.set_dead(x, y);
                    }
                    x += 1;
                }
                run_count = 0;
            }
            'o' => {
                let count = if run_count == 0 { 1 } else { run_count };
                for _ in 0..count {
                    if x < cells.width && y < cells.height {
                        cells.set_alive(x, y);
                    }
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
            ' ' => continue, // Ignorar espacios
            _ => (),
        }
    }
}


fn main() {
    let window_width = 300;
    let window_height = 300;

    let framebuffer_width = 200;
    let framebuffer_height = 200;

    let frame_delay = Duration::from_millis(100);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut cells = Cells::new(framebuffer_width, framebuffer_height);

    // Draw RLE pattern
    let rle_expression = "bo$2bo$3o!";
    draw_cells_from_rle(rle_expression, 3, 3, &mut cells);

    let rle_expression2 = "44b2o4b$44b2o4b9$41b2obob2o2b2$41bo5bo2b2$42b2ob2o3b$44bo5b3$38b2o6bo3b$37bobo5bobo2b$12bo26bo4bo3bob$13bo30b5ob$11b3o29b2o3b2o$44b5ob$45b3o2b$46bo3b$24b2o4b3o17b$24b2o6bo17b$31bo18b5$23b2o25b$22bobo21b2o2b$24bo21b2o2b$13bo36b$12b4o34b$11b2obobo6bobo24b$2o8b3obo2bo3bo3bo24b$2o9b2obobo4bo28b$12b4o4bo4bo24b$13bo7bo28b$21bo3bo6b2o16b$23bobo6bobo15b$34bo15b$34b2o!";
    draw_cells_from_rle(rle_expression2, 50, 47, &mut cells);

    let rle_expression3 = "2b3o3b3o2b2$o4bobo4bo$o4bobo4bo$o4bobo4bo$2b3o3b3o2b2$2b3o3b3o2b$o4bobo4bo$o4bobo4bo$o4bobo4bo2$2b3o3b3o!";
    draw_cells_from_rle(rle_expression3, 13, 13, &mut cells);

    let rle_expression4 = "obo$b2o$bo17$20bo$21b2o$20b2o5$51b2o$52bo$52bobo$42bo10b2o$40b3o$39bo$39b2o$24b2o48b2o$25bo48b2o$25bob2o$26bo2bo23bo$27b2o24bo$42b2o9b3o$42b2o11bo4$51bo3b2o$50bobo3bo$49bobo3bo$45b2obobo3bo$45b2obo2b4obo5b2o$49bobo3bobo5bo$45b2ob2o2bo2bobo2b3o$46bobo2b2o3bo3bo$34b2o10bobo$34b2o11bo!";
    draw_cells_from_rle(rle_expression4, 100,120, &mut cells);
    draw_cells_from_rle(rle_expression4, 100,50, &mut cells);
    draw_cells_from_rle(rle_expression4, 40,120, &mut cells);

    let rle_expression5 = "6b2o$2b2o6b2o$bo10bo$bobo6bobo$2obob4obob2o$3bob4obo$2obo6bob2o$bobo2b2o2bobo$bo2bob2obo2bo$2bob2o2b2obo$b2ob2o2b2ob2o2$b2o2b4o2b2o$obo2bo2bo2bobo$bo10bo!";
    draw_cells_from_rle(&rle_expression5, 10, 90, &mut cells);

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
        
        cells.update();

        // render
        render(&mut framebuffer, &cells);

        // Update the window with the framebuffer contents
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
