use nalgebra as na;
use na::{Point2, Vector2};
use sdl2::pixels::Color;

use rand::Rng;

// TODO: Changeable world variable
const gravity_enabled: bool = false;

pub struct Particle {
    position: Point2<f32>,
    velocity: Vector2<f32>,
    acceleration: Vector2<f32>,
    sum_of_forces: Vector2<f32>,
    display_radius: i16,
    inv_mass: f32,
}

impl Particle {
    pub fn new(position: Point2<f32>, velocity: Vector2<f32>, acceleration: Vector2<f32>, mass: f32) -> Self {
        Self {
            position,
            velocity,
            acceleration,
            sum_of_forces: Vector2::new(0.0, 0.0),
            display_radius: 10,
            inv_mass: 1.0 / mass,
        }
    }

    pub fn new_random(x: f32, y: f32) -> Self {
        let mut rng = rand::thread_rng();

        let position = Point2::new(x, y);
        let velocity = Vector2::new(rng.gen_range(-100.0..100.0), rng.gen_range(-100.0..100.0));
        let acceleration = Vector2::new(rng.gen_range(-100.0..100.0), rng.gen_range(-100.0..100.0));
        let display_radius = rng.gen_range(1..25);
        let inv_mass = 1.0 / rng.gen_range(1.0..100.0);

        Self {
            position,
            velocity,
            acceleration,
            sum_of_forces: Vector2::new(0.0, 0.0),
            display_radius,
            inv_mass,
        }
    }

    pub fn position(&self) -> Point2<f32> { self.position }
    pub fn radius(&self) -> i16 { self.display_radius }
    pub fn velocity(&self) -> na::Vector2<f32> { self.velocity }

    pub fn add_force(&mut self, force: Vector2<f32>) {
        self.sum_of_forces += force;
    }

    fn clear_forces(&mut self) {
        self.sum_of_forces.x = 0.0;
        self.sum_of_forces.y = 0.0;
    }

    pub fn update(&mut self, delta_time: f32) {
        self.acceleration = self.sum_of_forces * self.inv_mass;
        
        if gravity_enabled {
            let gravity_constant = 9.8;
            self.acceleration += Vector2::new(0.0, gravity_constant * 50.0);
        }

        // Semi-implicit Euler Integration
        self.velocity += self.acceleration * delta_time;

        self.update_x_position(delta_time);
        self.update_y_position(delta_time);
        self.clear_forces();
    }

    // TODO: Replace with real collision physics
    fn update_x_position(&mut self, delta_time: f32) {
        let mut new_x_position = self.position.x + (self.velocity.x * delta_time);

        if new_x_position - (self.display_radius as f32) < 0.0 {
            new_x_position = self.display_radius as f32;
            self.velocity.x *= -0.9;
        } else if new_x_position + (self.display_radius as f32) > 800.00 {
            new_x_position = 800.00 - self.display_radius as f32;
            self.velocity.x *= -0.9;
        }

        self.position.x = new_x_position;
    }

    // TODO: Replace with real collision physics
    fn update_y_position(&mut self, delta_time: f32) {
        let mut new_y_position = self.position.y + (self.velocity.y * delta_time);

        if new_y_position - (self.display_radius as f32) <= 0.0 {
            new_y_position = self.display_radius as f32;
            self.velocity.y *= -0.9;
        } else if new_y_position + (self.display_radius as f32) >= 600.00 {
            new_y_position = 600.00 - self.display_radius as f32;
            self.velocity.y *= -0.9;
        }

        self.position.y = new_y_position;
    }
}