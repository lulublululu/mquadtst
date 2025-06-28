use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;

const MOVEMENT_SPEED: f32 = 300.0;
const CIRCLE_SIZE: f32 = 16.0;

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    color: Color
}

#[macroquad::main("BasicShapes")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);

    let mut squares = vec![];
    let mut circle = Shape {
        size: CIRCLE_SIZE,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        color: YELLOW
    };

    loop {
        clear_background(GREEN);

        let delta_time = get_frame_time();
        let mut push = false;

        // MAKE SQUARES
        if rand::gen_range(0, 99) >= 95 {
            let size = rand::gen_range(16.0,64.0);
            squares.push(Shape {
                size,
                speed: rand::gen_range(50.0,150.0),
                x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                y: -size,
                color: *vec![DARKPURPLE,RED,MAGENTA].choose().unwrap()
            })
        }

        // UPDATE SQUARES
        for square in &mut squares {
            square.y += square.speed * delta_time;

            if  circle.x-circle.size < square.x + square.size / 2.0 &&
                circle.x+circle.size > square.x - square.size / 2.0 && 
                circle.y+circle.size > square.y - square.size / 2.0 &&
                circle.y-circle.size < square.y + square.size / 2.0 
            {
                circle.y += square.speed * delta_time;
                push = true
            }
        }

        // CLEAN UP SQUARES
        squares.retain(|square| square.y < screen_height() + square.size);
        
        // DRAW SQUARES
        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                square.color,
            );
        }

        // CIRCLE
        if is_key_down(KeyCode::Right) {
            circle.x += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Left) {
            circle.x -= MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            circle.y += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Up) && !push {
            circle.y -= MOVEMENT_SPEED * delta_time;
        }
        circle.x = clamp(circle.x, circle.size, screen_width()-circle.size);
        circle.y = clamp(circle.y, circle.size, screen_height()-circle.size);

        draw_circle(circle.x, circle.y, circle.size, circle.color);

        next_frame().await
    }
}