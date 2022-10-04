use sdl2::Sdl;
use sdl2::pixels::Color;
use std::{thread, time};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use std::time::Duration;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;

use rusty_physics::particle;
use nalgebra as na;

// TODO: Eventually replace SDL2 with WGPU custom functions
pub struct Renderer {
    canvas: WindowCanvas,
}

impl Renderer {
    pub fn new(sdl_context: &mut Sdl) -> Self {
        let video_subsystem = match sdl_context.video() {
            Ok(v) => v,
            Err(e) => panic!("Unable to start video subsystem: {:?}", e),
        };

        // TODO: Adjustable window
        let window = match video_subsystem
            .window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build() {
                Ok(w) => w,
                Err(e) => panic!("Unable to create window: {:?}", e),
        };

        let canvas = match window.into_canvas().build() {
            Ok(c) => c,
            Err(e) => panic!("Unable to create canvas: {:?}", e),
        };

        Self {
            canvas,
        }
    }

    // TODO: Render queue instead of individually passed objects
    pub fn render(&mut self, particles: &Vec<particle::Particle>, liquid: &Rect) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGB(0, 0, 255));
        let _ = self.canvas.fill_rect(*liquid);

        for p in particles {
            self.draw_filled_circle(
                p.position().x as i16,
                p.position().y as i16,
                p.radius(),
                Color::RGB(255, 210, 0)
            );
        }

        self.canvas.present();
    }

    pub fn draw_filled_circle(&mut self, x: i16, y: i16, radius: i16, color: Color) {
        // TODO: Handle result
        let _ = self.canvas.filled_circle(x, y, radius, color);
    }
}