use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const SQUARE_SIZE: usize = 20;

#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
}

fn main() {
    let mut window = Window::new(
        "Mini Engine with Rust",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut position = Position { x: 100.0, y: 100.0 };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Handle input
        let move_speed = 5.0;
        let mut moved = false;

        if window.is_key_down(Key::Up) {
            position.y = (position.y - move_speed).max(0.0);
            moved = true;
        }
        if window.is_key_down(Key::Down) {
            position.y = (position.y + move_speed).min((HEIGHT - SQUARE_SIZE) as f32);
            moved = true;
        }
        if window.is_key_down(Key::Left) {
            position.x = (position.x - move_speed).max(0.0);
            moved = true;
        }
        if window.is_key_down(Key::Right) {
            position.x = (position.x + move_speed).min((WIDTH - SQUARE_SIZE) as f32);
            moved = true;
        }

        if moved {
            println!("Position: {:?}", position);
        }

        // Clear the buffer (black background)
        for pixel in buffer.iter_mut() {
            *pixel = 0;
        }

        // Draw the red square
        let square_x = position.x as usize;
        let square_y = position.y as usize;

        for y in square_y..square_y + SQUARE_SIZE {
            for x in square_x..square_x + SQUARE_SIZE {
                if x < WIDTH && y < HEIGHT {
                    let index = y * WIDTH + x;
                    buffer[index] = 0xFF0000; // Red color (RGB: FF0000)
                }
            }
        }

        // Update the window with the buffer
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }

    println!("Closing...");
}