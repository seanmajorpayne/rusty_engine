use nalgebra as na;

use sdl2::{EventPump, TimerSubsystem};
use std::{thread, time};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use rusty_graphics::render;
use rusty_physics::{body, forces};
use rusty_physics::forces::{generate_drag_force, generate_friction_force, generate_gravitational_force};

pub struct Application<'a> {
    renderer: render::Renderer,
    event_pump: EventPump,
    timer: TimerSubsystem,
    fps: u32,
    milliseconds_per_frame: u32,
    time_previous_frame: u32,
    delta_time: f32,
    is_running: bool,
    // TODO: World crate
    bodies: Vec<body::Body<'a>>,
    liquid: Rect,
}

macro_rules! vec2 {
    ($x:expr, $y:expr) => (na::Vector2::new($x, $y));
}

impl Application<'_> {
    pub fn new() -> Self {
        let mut sdl_context = match sdl2::init() {
            Ok(c) => c,
            Err(e) => panic!("Failed to initialize SDL: {:?}", e),
        };

        let timer = match sdl_context.timer() {
            Ok(t) => t,
            Err(e) => panic!("Failed to create timer subsystem: {:?}", e),
        };

        let event_pump = match sdl_context.event_pump() {
            Ok(p) => p,
            Err(e) => panic!("Failed to create event pump: {:?}", e),
        };

        let renderer = render::Renderer::new(&mut sdl_context);

        let fps = 60;

        let shape = body::Circle::new(200.0, 200.0, 20.0, Color::RGB(255, 255, 0));

        // TODO: temp
        let p = body::Body::new(
            na::Point2::new(200.0, 300.0),
            na::Vector2::new(0.0, 0.0),
            na::Vector2::new(0.0, 0.0),  // TODO: Global constant for pixels_per_meter
            Box::new(shape),
            10.0,
        );

        let shape2 = body::Circle::new(300.0, 200.0, 20.0, Color::RGB(255, 255, 0));
        let p2 = body::Body::new(
            na::Point2::new(300.0, 200.0),
            na::Vector2::new(0.0, 0.0),
            na::Vector2::new(0.0, 0.0),  // TODO: Global constant for pixels_per_meter
            Box::new(shape2),
            1.0,
        );

        // TODO: Based on window size
        let liquid = Rect::new(
            0,
            300,
            800,
            300,
        );

        let bodies = vec![p, p2];

        Self {
            renderer,
            event_pump,
            timer,
            fps,
            milliseconds_per_frame: (1000 / fps) as u32,
            time_previous_frame: 0,
            delta_time: 0.0,
            is_running: true,
            bodies,
            liquid,
        }
    }

    pub fn run(&mut self) {
        self.time_previous_frame = self.timer.ticks();

        while self.is_running {
            self.handle_events();
            self.control_framerate();
            self.update_engine_state();
            self.renderer.render(&self.bodies, &self.liquid);
        }
    }

    fn handle_events(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.is_running = false,

                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => self.bodies[0].add_force(vec2!(-500.0, 0.0)),

                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    self.bodies[0].add_force(vec2!(500.0, 0.0));
                },

                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => self.bodies[0].add_force(vec2!(0.0, -500.0)),

                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => self.bodies[0].add_force(vec2!(0.0, 500.0)),

                Event::KeyUp {
                    keycode: Some(Keycode::Left) | Some(Keycode::Right) | Some(Keycode::Up) | Some(Keycode::Down),
                    ..
                } => self.bodies[0].add_force(vec2!(0.0, 0.0)),

                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    x,
                    y,
                    ..
                } => {
                    // spawn a random Body
                    let p = body::Body::new_random(x as f32, y as f32);

                    &mut self.bodies.push(p);
                },

                _ => {}
            }
        }
    }

    fn control_framerate(&mut self) {
        let time_to_wait = self.milliseconds_per_frame as i32 - (self.timer.ticks() - self.time_previous_frame) as i32;
        if time_to_wait > 0 {
            let dur = Duration::from_millis(time_to_wait as u64);
            thread::sleep(dur);
        }

        self.delta_time = (self.timer.ticks() - self.time_previous_frame) as f32 / 1000.0;

        if self.delta_time > 0.016 {
            self.delta_time = 0.016;    // Protect against high values during debug
        }

        self.time_previous_frame = self.timer.ticks();
    }

    fn update_engine_state(&mut self) {
        let body_one = &self.bodies[0];
        let body_two = &self.bodies[1];
        let (force_one, force_two) = generate_gravitational_force(body_one, body_two, 100.0);
        (&mut self.bodies[0]).add_force(force_one);
        (&mut self.bodies[1]).add_force(force_two);
        (&mut self.bodies[0]).update(self.delta_time);
        (&mut self.bodies[1]).update(self.delta_time);

        // for i in 0..self.bodies.len() {
        //     let body_velocity = self.bodies[i].velocity();
        //     (&mut self.bodies[i]).add_force(generate_friction_force(body_velocity, 0.1 * 50.0));
        //
        //     if self.bodies[i].position().y >= (self.liquid.y() as f32) {
        //         (&mut self.bodies[i]).add_force(generate_drag_force(body_velocity, 2.0));
        //     }
        //
        //     (&mut self.bodies[i]).update(self.delta_time);
        // }
    }
}