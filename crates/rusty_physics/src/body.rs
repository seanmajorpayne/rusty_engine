use std::f32::consts::PI;
use nalgebra as na;
use na::{Point2, Vector2};
use sdl2::pixels::Color;

use rand::{
    Rng,
    distributions::{Distribution, Standard},
};

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::render::WindowCanvas;

// TODO: Changeable world variable
const gravity_enabled: bool = false;

pub trait Shape {
    fn render(&self, canvas: &mut WindowCanvas);
    fn rotate(&mut self, angle: f32);
    fn update_x(&mut self, x: f32);
    fn update_y(&mut self, y: f32);
    fn moment_of_inertia(&self) -> f32;
}

#[derive(Clone)]
pub struct Circle {
    x: f32,
    y: f32,
    radius: f32,
    angle: f32,
    color: Color,
}

impl Circle {
    pub fn new(x: f32, y: f32, radius: f32, color: Color) -> Self {
        Circle { x, y, radius, angle: 0.0, color }
    }

    fn new_rand(x: f32, y: f32) -> Self {
        let mut rng = rand::thread_rng();
        let radius = rng.gen_range(0.1..=10.0);

        let color_range = rng.gen_range(0..=255);
        let color = Color::RGB(color_range, color_range, color_range);

        Circle::new(x, y, radius, color)
    }
}

impl Shape for Circle {
    fn render(&self, canvas: &mut WindowCanvas) {
        let _ = canvas.circle(self.x as i16, self.y as i16, self.radius as i16, self.color);

        // TODO: Double check
        let point_x = self.radius * (PI * 2.0 * self.angle/360.0).cos();
        let point_y = self.radius * (PI * 2.0 * self.angle/360.0).sin();

        let _ = canvas.draw_line((self.x as i32, self.y as i32), ((self.x + point_x) as i32, (self.y + point_y) as i32));
    }

    fn rotate(&mut self, angle: f32) {
        self.angle += angle;
    }

    fn update_x(&mut self, x: f32) {
        self.x = x;
    }

    fn update_y(&mut self, y: f32) {
        self.y = y;
    }

    fn moment_of_inertia(&self) -> f32 {
        0.5 * self.radius * self.radius
    }
}

#[derive(Clone)]
struct Polygon {
    x: f32,
    y: f32,
    vertices: Vec<Vector2<f32>>,
}

impl Polygon {
    fn new(vertices: Vec<Vector2<f32>>) -> Polygon {
        Polygon { x: 5.0, y: 5.0, vertices }
    }

    // TODO: Complex polygons
    // TODO: Move to proper random impl
    fn new_rand(x: f32, y: f32) -> Self {
        let mut rng = rand::thread_rng();
        let half_size = rng.gen_range(0.0..=5.0);

        Self {
            x: 5.0,
            y: 5.0,
            vertices: vec![
                Vector2::new(x - half_size, y + half_size),
                Vector2::new(x + half_size, y + half_size),
                Vector2::new(x + half_size, y - half_size),
                Vector2::new(x - half_size, y - half_size),
            ]
        }
    }
}

impl Shape for Polygon {
    fn render(&self, canvas: &mut WindowCanvas) {
        // TODO: ???

        // let mut iterable_vertices = self.vertices.into_iter().peekable();
        //
        // let (mut vx, mut vy): (Vec<f32>, Vec<f32>) = (vec![], vec![]);
        // while iterable_vertices.peek().is_some() {
        //     match iterable_vertices.next() {
        //         Some(vertex) => {
        //             vx.push(vertex.x);
        //             vy.push(vertex.y);
        //         }
        //         _ => ()
        //     }
        // }
        //
        // let vx: [i16; 4] = vx.into_iter().map(|e| e as i16).collect();
        // let vy: [i16; 4] = vy.into_iter().map(|e| e as i16).collect();
        //
        // let result = canvas.filled_polygon(
        //     &vx,
        //     &vy,
        //     Color::RGB(255, 210, 0),
        // );
        //
        // // TODO: Is this appropriate error handling?
        // match result {
        //     Ok(()) => (),
        //     _ => Err("How do I handle this"),
        // }
    }

    fn rotate(&mut self, angle: f32) {
        // TODO
    }

    fn update_x(&mut self, x: f32) {
        self.x = x;
    }

    fn update_y(&mut self, y: f32) {
        self.y = y;
    }

    fn moment_of_inertia(&self) -> f32 {
        todo!()
    }
}

pub struct Body<'a> {
    position: Point2<f32>,
    velocity: Vector2<f32>,
    acceleration: Vector2<f32>,
    sum_of_forces: Vector2<f32>,
    shape: Box<dyn Shape + 'a>,
    display_radius: i16,
    mass: f32,
    inv_mass: f32,
    rotation: f32,
    angular_velocity: f32,
    angular_acceleration: f32,
    sum_of_torque: f32,
}

impl<'a> Body<'a> {
    pub fn new(position: Point2<f32>, velocity: Vector2<f32>, acceleration: Vector2<f32>, shape: Box<dyn Shape + 'a>, mass: f32) -> Self {
        let sum_of_forces = Vector2::new(0.0, 0.0);
        let inv_mass = 1.0 / mass;

        let rotation = 0.0;
        let angular_velocity = 0.0;
        let angular_acceleration = 0.0;
        let sum_of_torque = 0.0;

        Self {
            position,
            velocity,
            acceleration,
            sum_of_forces,
            shape,
            display_radius: 20,
            mass,
            inv_mass,
            rotation,
            angular_velocity,
            angular_acceleration,
            sum_of_torque
        }
    }

    // TODO: Move to random generator
    pub fn new_random(x: f32, y: f32) -> Self {
        let mut rng = rand::thread_rng();

        let position = Point2::new(x, y);
        let velocity = Vector2::new(rng.gen_range(-100.0..100.0), rng.gen_range(-100.0..100.0));
        let acceleration = Vector2::new(rng.gen_range(-100.0..100.0), rng.gen_range(-100.0..100.0));
        let sum_of_forces = Vector2::new(0.0, 0.0);

        let shape: Box<dyn Shape + 'a> = match rng.gen_range(0..=1) {
            0 => Box::new(Circle::new_rand(x, y)),
            _ => Box::new(Polygon::new_rand(x, y)),
        };

        let mass = rng.gen_range(1.0..100.0);
        let inv_mass = 1.0 / mass;
        let rotation = rng.gen_range(1.0..100.0);
        let angular_velocity = rng.gen_range(1.0..100.0);
        let angular_acceleration = rng.gen_range(1.0..100.0);
        let sum_of_torque = rng.gen_range(1.0..100.0);

        Self {
            position,
            velocity,
            acceleration,
            sum_of_forces,
            shape,
            display_radius: 5,
            mass,
            inv_mass,
            rotation,
            angular_velocity,
            angular_acceleration,
            sum_of_torque
        }
    }

    pub fn position(&self) -> Point2<f32> { self.position }
    pub fn radius(&self) -> i16 { self.display_radius }
    pub fn velocity(&self) -> na::Vector2<f32> { self.velocity }
    pub fn shape(&self) -> &Box<dyn Shape + 'a> { &self.shape }
    pub fn mass(&self) -> f32 { self.mass }

    pub fn add_force(&mut self, force: Vector2<f32>) {
        self.sum_of_forces += force;
    }

    fn clear_forces(&mut self) {
        self.sum_of_forces.x = 0.0;
        self.sum_of_forces.y = 0.0;
    }

    pub fn add_torque(&mut self, torque: f32) {
        self.sum_of_torque += torque;
    }

    fn clear_torque(&mut self) {
        self.sum_of_torque = 0.0;
    }

    pub fn update(&mut self, delta_time: f32) {
        self.integrate_linear(delta_time);
        self.integrate_angular(delta_time);
    }

    fn integrate_linear(&mut self, delta_time: f32) {
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

    fn integrate_angular(&mut self, delta_time: f32) {
        self.angular_acceleration = self.sum_of_torque * self.inv_intertia;
        self.angular_velocity += self.angular_acceleration * delta_time;
        self.rotation += self.angular_velocity * delta_time;
        self.shape.rotate(self.rotation);
        self.clear_torque();
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
        self.shape.update_x(new_x_position);
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
        self.shape.update_y(new_y_position)
    }

    fn moment_of_inertia(&mut self) {
        self.shape.moment_of_inertia * self.mass
    }
}