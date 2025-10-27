use macroquad::prelude::*;

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

        // Restart when dead
        if game_over && is_key_pressed(KeyCode::Space) {
            squares.clear();
            bullets.clear();
            circle.x = screen_width() / 2.0;
            circle.y = screen_height() / 2.0;
            game_over = false;
        }

        // Square-bullet collisions
        squares_to_bullets_collision(&mut squares, &mut bullets);

        // Render //
        clear_background(WHITE);

        draw_shape(&mut circle);
        draw_shapes(&mut squares);
        draw_shapes(&mut bullets);

        if game_over {
            draw_text("GAME OVER", screen_width() / 2.0, screen_height() / 2.0, 50.0, RED);
        }

        next_frame().await;
    }
}
