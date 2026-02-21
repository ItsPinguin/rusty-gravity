use crate::{Vec2, app_state::ViewState};

pub trait Particle {
    fn get_type(&self) -> &str; // Changed to &self (no need for &mut)
    fn get_pos(&self) -> Vec2;
    fn get_mass(&self) -> f32;
    fn get_vel(&self) -> Vec2;          // Added
    fn set_vel(&mut self, vel: Vec2);   // Added
    fn set_mass(&mut self, mass: f32);  // Added
    
    fn react_to_other(&mut self, other: &mut dyn Particle, g_force: f32, dt: f32) -> i8;
    fn update(&mut self, dt: f32);
    fn draw(&self, view_state: &ViewState);
}

pub mod body;
pub mod planet;

// This makes "Body" available as "particles::Body" instead of "particles::body::Body"
pub use body::Body;
pub use planet::Planet;