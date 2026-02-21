use macroquad::prelude::*;
use crate::particles::{Particle, Body};
use crate::sim_tools::Tool;
use crate::ViewState;

pub struct PlaceTool;

// Change 'impl Tool' to 'impl PlaceTool'
impl PlaceTool {
    pub fn new() -> Self { 
        PlaceTool // Use the struct name to create the instance
    }
}

impl Tool for PlaceTool {
    fn on_click(&mut self, _pos: Vec2, _bodies: &mut Vec<Box<dyn Particle>>, _view: &mut ViewState) {}
    
    fn on_drag(&mut self, _start: Vec2, _current: Vec2, _view: &mut ViewState) {}

    fn on_release(&mut self, start: Vec2, end: Vec2, bodies: &mut Vec<Box<dyn Particle>>, view: &mut ViewState, mass: f32) {
        let initial_vel = (start - end) * 0.5;
        bodies.push(Box::new(Body {
            pos: start - view.offset,
            vel: initial_vel,
            mass,
        }));
    }

    fn draw_preview(&self, start: Vec2, current: Vec2, _view: &ViewState) {
        draw_line(start.x, start.y, current.x, current.y, 2.0, WHITE);
    }
}