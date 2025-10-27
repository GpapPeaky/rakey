use macroquad::prelude::*;
use std::fs::{self};

mod shape;
use shape::*;

#[macroquad::main("rakey")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);

    const PLAYER_SPEED: f32 = 150.0;

    let mut bullets: Vec<Shape> = vec![];

    let mut squares: Vec<Shape>  = vec![];
    let mut circle = Shape {
        size: 32.0,
        speed: 150.0,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        collided: false,
        t: ShapeType::SHAPECIRCLE
    };

    let mut game_over = false;

    // Scoring
    let mut score: u32 = 0;
    let mut high_score = fs::read_to_string("highscore.dat")
        .map_or(Ok(0), |i| i.parse::<u32>())
        .unwrap_or(0);

    loop {
        // Updates //
        let dt = get_frame_time(); // Get delta time

        gen_rects(&mut squares);

        // Remove squares and bullets outside the screen
        update_squares(&mut squares, dt);
        update_bullets(&mut bullets, dt);

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
            if is_key_down(KeyCode::E) {
                break; // Loop exit
            }
            if is_key_pressed(KeyCode::Q) {
                let mut s = Shape::new_shape(20.0, circle.speed * 2.0, circle.x, circle.y);
                s.t = ShapeType::SHAPECIRCLE; 
                bullets.push(s);
            }
        }

        // Limit the circles movement
        circle.x = clamp(circle.x, 0.0, screen_width());
        circle.y = clamp(circle.y, 0.0, screen_height());

        // Player-square collision
        if squares.iter().any(|square| circle.collides_with(square)) {
            game_over = true;
        }

        // Square-bullet collisions
        // Return the produced score
        score += squares_to_bullets_collision(&mut squares, &mut bullets);
        high_score = high_score.max(score);

        // Write the new highscore
        if score == high_score {
            fs::write("highscore.dat", high_score.to_string()).ok();
        }

        if game_over {
            draw_text("GAME OVER", screen_width() / 2.0, screen_height() / 2.0, 50.0, RED);
            
            // Restart when dead
            if is_key_pressed(KeyCode::Space) {
                squares.clear();
                bullets.clear();
                score = 0;
                circle.x = screen_width() / 2.0;
                circle.y = screen_height() / 2.0;
                game_over = false;
            }
        }

        // Render //
        clear_background(WHITE);

        draw_shape(&mut circle);
        draw_shapes(&mut squares);
        draw_shapes(&mut bullets);

        draw_text(&score.to_string(), 20.0, 20.0, 24.5, BLUE);

        next_frame().await;
    }
}
