use crate::Vec2;

pub trait Particle {
    fn get_type(&self) -> &str; // Changed to &self (no need for &mut)
    fn get_pos(&self) -> Vec2;
    fn get_mass(&self) -> f32;
    fn get_vel(&self) -> Vec2;          // Added
    fn set_vel(&mut self, vel: Vec2);   // Added
    fn add_mass(&mut self, mass: f32);  // Added
    
    fn react_to_other(&mut self, other: &dyn Particle, g_force: f32, dt: f32) -> i8;
    fn update(&mut self, dt: f32);
    fn draw(&self);
    fn get_snapshot(&self) -> (Vec2, f32, Vec2, &str);
}

pub struct ParticleProxy {
    pub pos: Vec2,
    pub mass: f32,
    pub vel: Vec2,
    pub p_type: String,
}

pub mod body;
pub mod planet;

// This makes "Body" available as "particles::Body" instead of "particles::body::Body"
pub use body::Body;
pub use planet::Planet;