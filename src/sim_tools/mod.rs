use macroquad::prelude::*;
use crate::particles::{Particle, Body};
use crate::ViewState;

pub trait Tool {
    fn on_click(&mut self, pos: Vec2, bodies: &mut Vec<Box<dyn Particle>>, view: &mut ViewState);
    fn on_drag(&mut self, start: Vec2, current: Vec2, view: &mut ViewState);
    fn on_release(&mut self, start: Vec2, end: Vec2, bodies: &mut Vec<Box<dyn Particle>>, view: &mut ViewState, mass: f32);
    fn draw_preview(&self, start: Vec2, current: Vec2, view: &ViewState);
}

pub mod place;
pub use place::PlaceTool;

pub mod pan;
pub use pan::PanTool;