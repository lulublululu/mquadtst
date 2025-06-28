use macroquad::prelude::*;

const MOVEMENT_SPEED: f32 = 300.0;
const CIRCLE_SIZE: f32 = 16.0;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    loop {
        clear_background(GREEN);

        let delta_time = get_frame_time();

        if is_key_down(KeyCode::Right) {
            x += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Left) {
            x -= MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            y += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            y -= MOVEMENT_SPEED * delta_time;
        }
        x = clamp(x, CIRCLE_SIZE, screen_width()-CIRCLE_SIZE);
        y = clamp(y, CIRCLE_SIZE, screen_height()-CIRCLE_SIZE);

        draw_circle(x, y, CIRCLE_SIZE, YELLOW);

        next_frame().await
    }
}