use macroquad::prelude::*;
use crate::particles::{Particle};
use crate::sim_tools::{Tool, PanTool};

pub struct AppState {
    pub bodies : Vec<Box<dyn Particle>>,
    pub view_state : ViewState,
    pub previous_tool_id : i32,
    pub active_tool: Box<dyn Tool>,
    pub drag_start: Option<Vec2>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            bodies: Vec::new(),
            view_state: ViewState {
                offset: vec2(0.0, 0.0),
                zoom: 1.0
            },
            previous_tool_id: 0,
            active_tool: Box::new(PanTool::new()),
            drag_start: None
        }
    }
}

pub struct ViewState {
    pub offset: Vec2,
    pub zoom: f32,
}