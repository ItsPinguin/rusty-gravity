use macroquad::prelude::*;
use crate::particles::{Particle, Body};
use crate::sim_tools::Tool;
use crate::app_state::*;

pub struct PlaceTool;

// Change 'impl Tool' to 'impl PlaceTool'
impl PlaceTool {
    pub fn new() -> Self { 
        PlaceTool // Use the struct name to create the instance
    }
}

impl Tool for PlaceTool {
    fn on_click(&mut self, _pos: Vec2, _app: &mut AppState) {}
    
    fn on_drag(&mut self, _start: Vec2, _current: Vec2, _app: &mut AppState) {}

    fn on_release(&mut self, start: Vec2, end: Vec2, app: &mut AppState, mass: f32) {
        let initial_vel = (start - end) * 0.5;
        app.bodies.push(Box::new(Body {
            pos: start - app.view.offset,
            vel: initial_vel,
            mass,
        }));
    }

    fn draw_preview(&self, start: Vec2, current: Vec2, _app: &AppState) {
        draw_line(start.x, start.y, current.x, current.y, 2.0, WHITE);
    }
}