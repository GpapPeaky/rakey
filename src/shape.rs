use macroquad::prelude::*;

// Shape types
#[derive(PartialEq)]
pub enum ShapeType {
    SHAPENIL,
    SHAPECIRCLE,
    SHAPERECT
}

// Falling shape
pub struct Shape {
    pub size: f32,
    pub speed: f32,
    pub x: f32,
    pub y: f32,
    pub collided: bool,
    pub t: ShapeType
}

impl Shape {
    // Shape constructor 
    pub fn new_shape(size: f32, speed: f32, x: f32, y: f32) -> Self {
        Self{
            size: size,
            speed: speed,
            x: x,
            y: y,
            collided: false,
            t: ShapeType::SHAPENIL
        }
    }

    // Assume a shape is a rectangle for collision detection
    // The &self as a parameter, in a call: obj.func() is the 'obj' object
    pub fn as_rect(&self) -> Rect {
        Rect {
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
            w: self.size,
            h: self.size,   
        }
    }

    // Collision constructor
    pub fn collides_with(&self, other: &Self) -> bool {
        self.as_rect().overlaps(&other.as_rect())
    }
}

pub fn add_shape(vector: &mut Vec<Shape>, size: f32, speed: f32, x: f32, y: f32, t: ShapeType) {
    let mut s = Shape::new_shape(size, speed, x, y);
    s.t = t;
    vector.push(s);
}

pub fn gen_rects(squares: &mut Vec<Shape>) {
    if rand::gen_range(0, 99) >= 95 {
        let size = rand::gen_range(16.0, 64.0);
        let speed = rand::gen_range(25.0, 105.0);
        let x = rand::gen_range(0.0, screen_height());

        add_shape(squares, size, speed, x, -size, ShapeType::SHAPERECT);
    }
}

pub fn update_squares(squares: &mut Vec<Shape>, dt: f32) {
    // Move down
    for square in squares.iter_mut() { // Use iter_mut() as to avoid moving the vector and borrowing it here.
        square.y += square.speed * dt;
    }

    // Keep whatever is inside the screen
    squares.retain(|square| square.y < screen_height() + square.size);
}

pub fn update_bullets(bullets: &mut Vec<Shape>, dt: f32) {
    // Move up
    for bullet in  bullets.iter_mut() {
        bullet.y -= bullet.speed * dt;
    }

    // Keep whatever is inside the screen
    bullets.retain(|bullet| bullet.y > 0.0 - bullet.size / 2.0);
}

pub fn squares_to_bullets_collision(squares: &mut Vec<Shape>,bullets: &mut Vec<Shape>) {
    for square in squares.iter_mut() {
        for bullet in bullets.iter_mut() {
            if bullet.collides_with(square) {
                square.collided = true;
                bullet.collided = true;
            }
        }   
    }

    squares.retain(|square| !square.collided);
    bullets.retain(|bullet| !bullet.collided);
}

pub fn draw_shape(s: &mut Shape) {
    if s.t == ShapeType::SHAPECIRCLE {
        draw_circle(s.x, s.y, s.size / 2.0, RED);
    } else if s.t == ShapeType::SHAPERECT {
        draw_rectangle(s.x, s.y, s.size, s.size, GREEN);
    }
}

pub fn draw_shapes(v: &mut Vec<Shape>) {
    for s in v {
        draw_shape(s);
    }
}
