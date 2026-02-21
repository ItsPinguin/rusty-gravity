use macroquad::prelude::*;
mod particles;
use particles::{Particle, Body};
mod physics;
use physics::*;
mod sim_tools;
use sim_tools::{Tool, PlaceTool, PanTool};

mod app_state;
use app_state::AppState;

#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn get_gravity_value() -> f32;
    // fn get_gravity_sign() -> f32;
    fn get_spawning_mass() -> f32;
    fn should_reset_simulation() -> f32;
    fn get_active_tool() -> i32;
}


#[macroquad::main("Gravity Simulation")]
async fn main() {
    let mut app_state = AppState::new();
    // let mut bodies : Vec<Box<dyn Particle>> = Vec::new();
    // let mut view_state = ViewState {
    //     offset: vec2(0.0, 0.0),
    //     zoom: 1.0
    // };

    // let mut previous_tool_id = 0;
    // let mut active_tool: Box<dyn Tool> = Box::new(PanTool::new());
    // let mut drag_start: Option<Vec2> = None;
    
    // Create a few random particles
    for _ in 0..1000 {
        app_state.bodies.push(Box::new(Body {
            pos: vec2(rand::gen_range(100.0, 1500.0), rand::gen_range(100.0, 900.0)),
            vel: vec2(rand::gen_range(1.0, 5.0), rand::gen_range(-1.0, 1.0)),
            mass: rand::gen_range(1.0, 2.0)
        }));
    }

    loop {
        #[cfg(target_arch = "wasm32")]
        let g_force = unsafe { get_gravity_value() };
        #[cfg(target_arch = "wasm32")]
        let spawning_mass = unsafe { get_spawning_mass() };
        #[cfg(target_arch = "wasm32")]
        let should_reset = unsafe { should_reset_simulation() };
        let tool_id = unsafe { 
            #[cfg(target_arch = "wasm32")]
            { get_active_tool() }
            #[cfg(not(target_arch = "wasm32"))]
            { 0 }
        };
        if app_state.previous_tool_id != tool_id {
            app_state.previous_tool_id = tool_id;
            match tool_id {
                0 => {app_state.active_tool = Box::new(PanTool::new());},
                1 => {app_state.active_tool = Box::new(PlaceTool::new());},
                _ => {}
            }
        }
        
        if should_reset == 1.0 {
            app_state.bodies = Vec::new();
            app_state.view_state.offset = vec2(0.0, 0.0);
            app_state.view_state.zoom = 1.0;

            for _ in 0..1000 {
                app_state.bodies.push(Box::new( Body {
                pos: vec2(rand::gen_range(100.0, 1500.0), rand::gen_range(100.0, 900.0)),
                vel: vec2(rand::gen_range(1.0, 5.0), rand::gen_range(-1.0, 1.0)),
                mass: rand::gen_range(1.0, 2.0)
            }));
            }
        }

        clear_background(BLACK);
        let dt = get_frame_time(); // Get time elapsed (around 0.016s for 60fps)

        // --- Physics Logic ---
        // We use a simple O(N^2) loop to calculate gravity between all pairs
        tick_physics(&mut app_state.bodies, g_force, dt);

        // --- Update and Draw ---
        for b in app_state.bodies.iter_mut() {
            b.update(dt);
            b.draw(&app_state.view_state);
        }

        let mouse_pos = mouse_position().into();

        if is_mouse_button_pressed(MouseButton::Left) {
            app_state.drag_start = Some(mouse_pos);
            app_state.active_tool.on_click(mouse_pos, &mut app_state.bodies, &mut app_state.view_state);
        }

        if is_mouse_button_down(MouseButton::Left) {
            if let Some(start) = app_state.drag_start {
                app_state.active_tool.on_drag(start, mouse_pos, &mut app_state.view_state);
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            if let Some(start) = app_state.drag_start {
                app_state.active_tool.on_release(start, mouse_pos, &mut app_state.bodies, &mut app_state.view_state, spawning_mass);
                app_state.drag_start = None;
            }
        }

        // Drawing
        if let Some(start) = app_state.drag_start {
            app_state.active_tool.draw_preview(start, mouse_pos, &app_state.view_state);
        }

        next_frame().await
    }
}

pub struct ViewState {
    pub offset: Vec2,
    pub zoom: f32,
}