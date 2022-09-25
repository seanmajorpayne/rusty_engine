extern crate nalgebra as na;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::time::Duration;
use sdl2::gfx::primitives::DrawRenderer;

mod particle;

const scale: f32 = 6.0;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut p = particle::Particle::new(
        na::Point2::new(200.0, 200.0),
        na::Vector2::new(0.0, 0.0),
        na::Vector2::new(0.0, 9.8 / scale),
        0.0,
    );

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    ..
                } => break 'running,
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        p.display(&mut canvas);
        p.update();

        canvas.present();
    }
}
