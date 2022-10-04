use nalgebra as na;

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