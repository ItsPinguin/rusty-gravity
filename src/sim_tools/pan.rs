use macroquad::prelude::*;
use crate::particles::{Particle};
use crate::sim_tools::Tool;
use crate::app_state::ViewState;

pub struct PanTool {
    initial_offset: Vec2,
}

impl PanTool {
    pub fn new() -> Self { Self { initial_offset: Vec2::ZERO } }
}

impl Tool for PanTool {
    fn on_click(&mut self, _pos: Vec2, _bodies: &mut Vec<Box<dyn Particle>>, view: &mut ViewState) {
        // Remember where the camera was when we started clicking
        self.initial_offset = view.offset;
    }

    fn on_drag(&mut self, start: Vec2, current: Vec2, view: &mut ViewState) {
        // Calculate the difference and update the view
        let diff = current - start;
        view.offset = self.initial_offset + diff;
    }

    fn on_release(&mut self, _start: Vec2, _end: Vec2, _bodies: &mut Vec<Box<dyn Particle>>, _view: &mut ViewState, _mass: f32) {}

    fn draw_preview(&self, _start: Vec2, _current: Vec2, _view: &ViewState) {
    }
}