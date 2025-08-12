mod physics;
mod shapes;

use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(BLACK);

        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, WHITE);

        next_frame().await
    }
}
