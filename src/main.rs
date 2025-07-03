use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;

const MOVEMENT_SPEED: f32 = 300.0;
const CIRCLE_SIZE: f32 = 16.0;

enum ShapeType {
    Circle,
    Square,
}

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    color: Color,
    kind: ShapeType,
    collided: bool,
}

impl Shape {
    fn collides_with(&self, other: &Self) -> bool {
        match (&self.kind, &other.kind) {
            (ShapeType::Circle, ShapeType::Circle) => self.circle().overlaps(&other.circle()),
            (ShapeType::Circle, ShapeType::Square) => self.circle().overlaps_rect(&other.rect()),
            (ShapeType::Square, ShapeType::Circle) => other.circle().overlaps_rect(&self.rect()),
            (ShapeType::Square, ShapeType::Square) => self.rect().overlaps(&other.rect()),
        }
    }

    fn rect(&self) -> Rect {
        Rect {
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
            w: self.size,
            h: self.size,
        }
    }

    fn circle(&self) -> Circle {
        Circle {
            x: self.x,
            y: self.y,
            r: self.size,
        }
    }
}

#[macroquad::main("BasicShapes")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);

    let mut gameover = false;

    let mut squares = vec![];
    let mut bullets: Vec<Shape> = vec![];
    let mut circle = Shape {
        size: CIRCLE_SIZE,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        color: YELLOW,
        kind: ShapeType::Circle,
        collided: false,
    };

    loop {
        clear_background(GREEN);

        let delta_time = get_frame_time();

        // MAKE SQUARES
        if rand::gen_range(0, 99) >= 95 {
            let size = rand::gen_range(16.0, 64.0);
            squares.push(Shape {
                size,
                speed: rand::gen_range(50.0, 150.0),
                x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                y: -size,
                color: *vec![DARKPURPLE, RED, MAGENTA].choose().unwrap(),
                kind: ShapeType::Square,
                collided: false,
            })
        }

        if !gameover {
            // UPDATE SQUARES
            for square in &mut squares {
                square.y += square.speed * delta_time;
            }
            // UPDATE BULLETS
            for bullet in &mut bullets {
                bullet.y -= bullet.speed * delta_time;
            }
            bullets.retain(|bullet| bullet.y > 0.0 - bullet.size / 2.0);
        }

        // CHECK BULLET COLLISION
        for square in squares.iter_mut() {
            for bullet in bullets.iter_mut() {
                if bullet.collides_with(square) {
                    bullet.collided = true;
                    square.collided = true;
                }
            }
        }

        // CLEAN UP SQUARES AND BULLETS
        squares.retain(|square| square.y < screen_height() + square.size);
        squares.retain(|square| !square.collided);
        bullets.retain(|bullet| !bullet.collided);

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
        if !gameover {
            if is_key_down(KeyCode::Right) {
                circle.x += MOVEMENT_SPEED * delta_time;
            }
            if is_key_down(KeyCode::Left) {
                circle.x -= MOVEMENT_SPEED * delta_time;
            }
            if is_key_down(KeyCode::Down) {
                circle.y += MOVEMENT_SPEED * delta_time;
            }
            if is_key_down(KeyCode::Up) {
                circle.y -= MOVEMENT_SPEED * delta_time;
            }
            if is_key_pressed(KeyCode::Space) {
                bullets.push(Shape {
                    x: circle.x,
                    y: circle.y,
                    speed: circle.speed * 2.0,
                    size: 5.0,
                    collided: false,
                    color: RED,
                    kind: ShapeType::Circle,
                });
            }

            // CHECK SQUARE COLLISION
            if squares.iter().any(|square| circle.collides_with(square)) {
                gameover = true
            }
        } else {
            if is_key_pressed(KeyCode::Space) {
                squares.clear();
                bullets.clear();
                circle.x = screen_width() / 2.0;
                circle.y = screen_height() / 2.0;
                gameover = false;
            }
        }
        circle.x = clamp(circle.x, circle.size, screen_width() - circle.size);
        circle.y = clamp(circle.y, circle.size, screen_height() - circle.size);

        // DRAW BULLETS
        for bullet in &bullets {
            draw_circle(bullet.x, bullet.y, bullet.size / 2.0, bullet.color);
        }

        // DRAW CIRCLE
        draw_circle(circle.x, circle.y, circle.size, circle.color);

        if gameover {
            let text = "GAME OVER!";
            let text_dimensions = measure_text(text, None, 50, 1.0);
            draw_text(
                text,
                screen_width() / 2.0 - text_dimensions.width / 2.0,
                screen_height() / 2.0 - text_dimensions.offset_y,
                50.0,
                RED,
            );
        }

        next_frame().await
    }
}
