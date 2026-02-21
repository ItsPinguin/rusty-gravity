use macroquad::prelude::*;
use crate::particles::{Particle, Body};
use crate::app_state::{AppState, ViewState};

pub trait Tool {
    fn on_click(&mut self, pos: Vec2, app: &mut AppState);
    fn on_drag(&mut self, start: Vec2, current: Vec2, app: &mut AppState);
    fn on_release(&mut self, start: Vec2, end: Vec2, app: &mut AppState, mass: f32);
    fn draw_preview(&self, start: Vec2, current: Vec2, app: &AppState);
}

pub mod place;
pub use place::PlaceTool;

pub mod pan;
pub use pan::PanTool;