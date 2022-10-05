use num;
use nalgebra as na;
use crate::particle;

// generate_drag_force for a particle
// k represents a constant (fluid density * coefficient of drag * cross-sectional area)
// cross-sectional area is assumed to be 1.0 for now
// TODO: Handle mass
pub fn generate_drag_force(velocity: na::Vector2<f32>, k: f32) -> na::Vector2<f32> {
    let mut drag = na::Vector2::new(0.0, 0.0);

    if velocity.norm() != 0.0 {
        drag = k * velocity.norm() * (-velocity / velocity.norm());
    }

    drag
}

// generate_friction_force
// temporarily using k to represent the Normal Force * coefficient of friction
// TODO: Include normal force based on object
pub fn generate_friction_force(velocity: na::Vector2<f32>, k: f32) -> na::Vector2<f32> {
    velocity * -1.0 * k
}

// generate_gravitational_force
pub fn generate_gravitational_force(particle_one: &particle::Particle, particle_two: &particle::Particle, gravitational_constant: f32) -> (na::Vector2<f32>, na::Vector2<f32>) {
    let difference = particle_two.position() - particle_one.position();

    // Accommodating for visual effect. Pixel values squared create too large of a divisor
    let distance_squared = num::clamp(difference.norm_squared(), 5.0, 100.0);

    let direction = difference / difference.norm();

    let attraction_force = gravitational_constant * (particle_one.mass() * particle_two.mass() / distance_squared) * direction;

    (attraction_force, -attraction_force)
}