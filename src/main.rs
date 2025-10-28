use macroquad::prelude::*;
use std::fs::{self};
use macroquad_particles::{Emitter};

mod particle;

mod shader;
use shader::*;

mod gamestate;
use gamestate::*;

mod shape;
use shape::*;

#[macroquad::main("rakey")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);

    let mut direction_modifier: f32 = 0.0;

    let render_target = render_target(320, 150);
    render_target.texture.set_filter(FilterMode::Nearest);

    let material = initialise_shader();

    const PLAYER_SPEED: f32 = 150.0;

    let mut bullets: Vec<Shape> = vec![];
    let mut squares: Vec<Shape> = vec![];
    let mut explosions: Vec<(Emitter, Vec2)> = vec![];

    let mut circle = Shape {
        size: 32.0,
        speed: 150.0,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        collided: false,
        t: ShapeType::SHAPECIRCLE
    };

    let mut game_state: GameState = GameState::MainMenu;

    // Scoring
    let mut score: u32 = 0;
    let mut high_score = fs::read_to_string("highscore.dat")
        .map_or(Ok(0), |i| i.parse::<u32>())
        .unwrap_or(0);

    loop {
        clear_background(WHITE);

        material.set_uniform("iResolution", (screen_width(), screen_height()));
        material.set_uniform("direction_modifier", direction_modifier);
        gl_use_material(&material);
        draw_texture_ex(
            &render_target.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        gl_use_default_material();

        match game_state {
            GameState::MainMenu => {
                if is_key_pressed(KeyCode::Escape) {
                    break;
                }

                if is_key_pressed(KeyCode::Space) {
                    squares.clear();
                    bullets.clear();
                    circle.x = screen_width() / 2.0;
                    circle.y = screen_height() / 2.0;
                    score = 0;
                    game_state = GameState::Playing;
                }

                let text = "Press Space";
                let text_dim = measure_text(text, None, 50, 1.0);
                draw_text(text, screen_width() / 2.0 - text_dim.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    WHITE
                );
            }

            GameState::Paused => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Playing;
                }

                let text = "Paused";
                let text_dim = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dim.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    WHITE,
                );
            }

            GameState::GameOver => {
                let text = "GAME OVER!";
                let text_dim = measure_text(text, None, 50, 1.0); 
                draw_text("GAME OVER",
                screen_width() / 2.0 - text_dim.width / 2.0,
                screen_height() / 2.0,
                50.0,
                WHITE);
            
                // Restart when dead
                if is_key_pressed(KeyCode::Space) {
                    squares.clear();
                    bullets.clear();
                    explosions.clear();
                    score = 0;
                    circle.x = screen_width() / 2.0;
                    circle.y = screen_height() / 2.0;
                    game_state = GameState::Playing;
                }
            }

            GameState::Playing => {
                let dt = get_frame_time(); // Get delta time

                gen_rects(&mut squares);
        
                // Remove squares and bullets outside the screen
                update_squares(&mut squares, dt);
                update_bullets(&mut bullets, dt);
        
                if is_key_down(KeyCode::Up) {
                    circle.y -= PLAYER_SPEED * dt;
                }
                if is_key_down(KeyCode::Down) {
                    circle.y += PLAYER_SPEED * dt;
                }
                if is_key_down(KeyCode::Left) {
                    circle.x -= PLAYER_SPEED * dt;
                    direction_modifier -= 0.05 * dt;
                }
                if is_key_down(KeyCode::Right) {
                    circle.x += PLAYER_SPEED * dt;
                    direction_modifier += 0.05 * dt;
                }
                if is_key_down(KeyCode::E) {
                    break; // Loop exit
                }
                if is_key_pressed(KeyCode::Q) {
                    let mut s = Shape::new_shape(5.314, circle.speed * 2.0, circle.x, circle.y);
                    s.t = ShapeType::SHAPECIRCLE; 
                    bullets.push(s);
                }
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Paused;
                }
        
                // Limit the circles movement
                circle.x = clamp(circle.x, 0.0, screen_width());
                circle.y = clamp(circle.y, 0.0, screen_height());
        
                // Player-square collision
                if squares.iter().any(|square| circle.collides_with(square)) {
                    game_state = GameState::GameOver;
                }
        
                // Square-bullet collisions
                // Return the produced score
                score += squares_to_bullets_collision(&mut squares, &mut bullets, &mut explosions);
                high_score = high_score.max(score);
                // Write the new highscore
                if score == high_score {
                    fs::write("highscore.dat", high_score.to_string()).ok();
                }

                // Retain valid particles
                explosions.retain(|(explosion, _)| explosion.config.emitting);

                draw_shape(&mut circle);
                draw_shapes(&mut squares);
                draw_shapes(&mut bullets);

                for (explosion, coords) in explosions.iter_mut() {
                    explosion.draw(*coords);
                }

                draw_text(
                    format!("Score: {}", score).as_str(),
                    10.0,
                    35.0,
                    25.0,
                    WHITE
                );

                draw_text(
                    format!("High score: {}", high_score).as_str(), 
                    screen_width() - measure_text(
                                                format!("High score: {}", high_score).as_str(),
                                                None,
                                                25,
                                                1.0)
                                                .width - 10.0,
                    35.0,
                    25.0,
                    WHITE
                );
            }
        }

        next_frame().await;
    }
}
