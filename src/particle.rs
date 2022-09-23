use kiss3d::scene::SceneNode;
use nalgebra as na;
use kiss3d::window::Window;
use na::{Isometry3, Point3};

pub struct Particle {
    position: Isometry3<f32>,
    velocity: Isometry3<f32>,
    acceleration: Isometry3<f32>,
    scene_node: Option<SceneNode>,
    display_radius: Option<f32>,
    mass: f32,
}

impl Particle {
    pub fn new(position: Isometry3<f32>, velocity: Isometry3<f32>, acceleration: Isometry3<f32>, mass: f32) -> Self {
        Self {
            position,
            velocity,
            acceleration,
            scene_node: None,
            display_radius: None,
            mass,
        }
    }

    pub fn display(&mut self, window: &mut Window, display_radius: f32) {
        let mut scene_node      = window.add_sphere(display_radius);
        scene_node.set_color(1.0, 0.0, 0.0);
        self.scene_node = Some(scene_node);
        self.display_radius = Some(display_radius);
    }

    pub fn update(&mut self, window_size: &na::base::Vector2<u32>) {
        // Semi-implicit Euler Integration
        self.velocity.append_translation_mut(&self.acceleration.translation);

        self.update_x_position();
        self.update_y_position();

        match self.scene_node.as_mut() {
            Some(n) => {
                n.set_local_translation(self.position.translation);
            },
            None => {},
        }
    }

    // TODO: Replace with real collision physics
    fn update_x_position(&mut self) {
        let mut new_x_position = self.velocity.translation.x + self.position.translation.x;

        let display_radius = self.display_radius.unwrap();

        // TODO: Figure out window boundaries based on coordinates
        if new_x_position + display_radius >= 0.5 {
            new_x_position = 0.5 - display_radius;
            self.velocity.translation.x = -self.velocity.translation.x;
        }
        else if new_x_position - display_radius <= -0.5 {
            new_x_position = -0.5 + display_radius;
            self.velocity.translation.x = -self.velocity.translation.x;
        }

        self.position.translation.x = new_x_position;
    }

    // TODO: Replace with real collision physics
    fn update_y_position(&mut self) {
        let mut new_y_position = self.velocity.translation.y + self.position.translation.y;

        let display_radius = self.display_radius.unwrap();

        if new_y_position + display_radius > 0.3 {
            new_y_position = 0.3 + display_radius;
            self.velocity.translation.y = -self.velocity.translation.y;
        }
        else if new_y_position - display_radius <= -0.3 {
            new_y_position = -0.3 + display_radius;
            self.acceleration.translation.x = 0.0;
            self.velocity.translation.y = -self.velocity.translation.y;        }

        self.position.translation.y = new_y_position;
    }
}