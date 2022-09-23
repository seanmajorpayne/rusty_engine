extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::camera::{ArcBall, FirstPerson};
use na::{Vector3, UnitQuaternion, Point1, Point3, Translation3, Isometry3};
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::text::Font;
use kiss3d::renderer::{LineRenderer, Renderer};

use nalgebra::geometry::Translation;

use std::path::Path;

mod particle;

fn main() {
    let mut window = Window::new("Kiss3d: Window");

    let position = Isometry3::new(
        Vector3::new(0.01, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0)
    );
    let velocity = Isometry3::new(
        Vector3::new(0.01, -0.00001, -0.0001), Vector3::new(0.0, 0.0, 0.0)
    );
    let acceleration = Isometry3::new(
        Vector3::new(0.0, -0.001, 0.0), Vector3::new(0.0, 0.0, 0.0)
    );
    let mass = 1.0;

    let mut p = particle::Particle::new(
        position,
        velocity,
        acceleration,
        mass,
    );

    p.display(&mut window, 0.01);

    window.set_light(Light::StickToCamera);

    let mut camera = FirstPerson::new(Point3::new(0.0, 0.0, -1.0), Point3::origin());

    while window.render_with_camera(&mut camera) {
        let window_size = window.size();

        // TODO: Scale based on camera & move to separate function
        let a = &Point3::new(0.5 * 2.0, 0.3 * 2.0, 1.0);
        let b = &Point3::new(0.5 * 2.0, -0.3 * 2.0, 1.0);
        let c = &Point3::new(-0.5 * 2.0, 0.3 * 2.0, 1.0);
        let d = &Point3::new(-0.5 * 2.0, -0.3 * 2.0, 1.0);
        window.draw_line(a, b, &Point3::new(1.0, 0.0, 0.0));
        window.draw_line(a, c, &Point3::new(1.0, 0.0, 0.0));
        window.draw_line(b, d, &Point3::new(1.0, 0.0, 0.0));
        window.draw_line(c, d, &Point3::new(1.0, 0.0, 0.0));
        // TODO end

        p.update(&window_size);
    }
}
