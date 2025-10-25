use macroquad::prelude::*;

// Falling shape
struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32
}

impl Shape {
    fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }

    fn rect(&self) -> Rect {
        Rect {
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
            w: self.size,
            h: self.size
        }
    }
}

#[macroquad::main("rakey")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);

    const PLAYER_SPEED: f32 = 150.0;

    let mut squares= vec![];
    let mut circle = Shape {
        size: 32.0,
        speed: 150.0,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0
    };

    let mut game_over = false;

    loop {
        // Update
        let dt = get_frame_time(); // Get delta time

        if rand::gen_range(0, 99) >= 95 {
            let size = rand::gen_range(16.0, 64.0);
            squares.push(Shape{
                size,
                speed: rand::gen_range(50.0, 150.0),
                x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                y: -size
            });
        }

        for square in &mut squares {
            square.y += square.speed * dt;
        }

        /* Remove squares outside teh screen */
        squares.retain(|square| square.y < screen_height() + square.size);

        if !game_over {
            if is_key_down(KeyCode::Up) {
                circle.y -= PLAYER_SPEED * dt;
            }
            if is_key_down(KeyCode::Down) {
                circle.y += PLAYER_SPEED * dt;
            }
            if is_key_down(KeyCode::Left) {
                circle.x -= PLAYER_SPEED * dt;
            }
            if is_key_down(KeyCode::Right) {
                circle.x += PLAYER_SPEED * dt;
            }
        }

        // Collision
        if squares.iter().any(|square| circle.collides_with(square)) {
            game_over = true;
        }

        // Restart when dead
        if game_over && is_key_pressed(KeyCode::Space) {
            squares.clear();
            circle.x = screen_width() / 2.0;
            circle.y = screen_height() / 2.0;
            game_over = false;
        }

        // Exit loop
        if is_key_down(KeyCode::E) {
            break;
        }

        /* Limit the circle's movement */
        circle.x = clamp(circle.x, 0.0, screen_width());
        circle.y = clamp(circle.y, 0.0, screen_height());

        clear_background(WHITE);

        draw_circle(circle.x, circle.y, circle.size, RED);

        for square in &squares {
            draw_rectangle(square.x,
                square.y,
                square.size,
                square.size,
                GREEN
            );
        }

        if game_over {
            draw_text("GAME OVER", screen_width() / 2.0, screen_height() / 2.0, 50.0, RED);
        }

        next_frame().await;
    }
}
