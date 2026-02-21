use macroquad::prelude::*;
use crate::particles::{Particle, Body};

pub trait Tool {
    // Called when mouse is pressed
    fn on_click(&mut self, pos: Vec2, bodies: &mut Vec<Box<dyn Particle>>);
    
    // Called while mouse is held down
    fn on_drag(&mut self, start: Vec2, current: Vec2);
    
    // Called when mouse is released
    fn on_release(&mut self, start: Vec2, end: Vec2, bodies: &mut Vec<Box<dyn Particle>>, mass: f32);

    // Optional: Draw a preview (like the line or a ghost planet)
    fn draw_preview(&self, start: Vec2, current: Vec2);
}

pub mod placement_tool;
pub use placement_tool::PlacementTool;