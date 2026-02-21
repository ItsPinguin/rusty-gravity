use macroquad::prelude::*;
use crate::particles::{Particle, Body};
use crate::sim_tools::Tool;

pub struct PlacementTool;

// Change 'impl Tool' to 'impl PlacementTool'
impl PlacementTool {
    pub fn new() -> Self { 
        PlacementTool // Use the struct name to create the instance
    }
}

impl Tool for PlacementTool {
    fn on_click(&mut self, _pos: Vec2, _bodies: &mut Vec<Box<dyn Particle>>) {}
    
    fn on_drag(&mut self, _start: Vec2, _current: Vec2) {}

    fn on_release(&mut self, start: Vec2, end: Vec2, bodies: &mut Vec<Box<dyn Particle>>, mass: f32) {
        let initial_vel = (start - end) * 0.5;
        bodies.push(Box::new(Body {
            pos: start,
            vel: initial_vel,
            mass,
        }));
    }

    fn draw_preview(&self, start: Vec2, current: Vec2) {
        draw_line(start.x, start.y, current.x, current.y, 2.0, WHITE);
    }
}