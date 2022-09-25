use nalgebra as na;
use na::{Point2, Vector2};

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

pub struct Particle {
    position: Point2<f32>,
    velocity: Vector2<f32>,
    acceleration: Vector2<f32>,
    display_radius: i16,
    mass: f32,
}

impl Particle {
    pub fn new(position: Point2<f32>, velocity: Vector2<f32>, acceleration: Vector2<f32>, mass: f32) -> Self {
        Self {
            position,
            velocity,
            acceleration,
            display_radius: 10,
            mass,
        }
    }

    pub fn display(&mut self, canvas: &mut WindowCanvas) {
        let _ = canvas.filled_circle(self.position.x as i16, self.position.y as i16, self.display_radius, Color::RGB(255, 210, 0));
    }

    pub fn update(&mut self) {
        // Semi-implicit Euler Integration
        self.velocity += self.acceleration;

        self.update_x_position();
        self.update_y_position();
    }

    // TODO: Replace with real collision physics
    fn update_x_position(&mut self) {
        let mut new_x_position = self.position.x + self.velocity.x;

        // TODO: Figure out window boundaries based on coordinates
        if new_x_position - (self.display_radius as f32) < 0.0 {
            new_x_position = self.display_radius as f32;
            self.velocity.x = -self.velocity.x;
        } else if new_x_position + (self.display_radius as f32) > 800.00 {
            new_x_position = 800.00 - self.display_radius as f32;
            self.velocity.x = -self.velocity.x;
        }

        self.position.x = new_x_position;
    }

    // TODO: Replace with real collision physics
    fn update_y_position(&mut self) {
        let mut new_y_position = self.position.y + self.velocity.y;

        // TODO: Figure out window boundaries based on coordinates
        if new_y_position - (self.display_radius as f32) < 0.0 {
            new_y_position = self.display_radius as f32;
            self.velocity.y = -self.velocity.y;
        } else if new_y_position + (self.display_radius as f32) > 600.00 {
            new_y_position = 600.00 - self.display_radius as f32;
            self.velocity.y = -self.velocity.y;
        }

        self.position.y = new_y_position;
    }
}