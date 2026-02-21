use macroquad::prelude::*;
mod particles;
use particles::Body;
mod physics;
use physics::*;
mod sim_tools;
use sim_tools::{PlaceTool, PanTool};

mod app_state;
use app_state::AppState;

#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn get_gravity_value() -> f32;
    // fn get_gravity_sign() -> f32;
    fn get_spawning_mass() -> f32;
    fn should_reset_simulation() -> f32;
    fn get_active_tool() -> i32;
    fn update_ui_view_pos(x: f32, y: f32, zoom: f32);
}


#[macroquad::main("Gravity Simulation")]
async fn main() {
    let mut app = AppState::new();
    
    // Create a few random particles
    for _ in 0..1000 {
        app.bodies.push(Box::new(Body {
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

        if app.view.changed { // Only sync when moved to save performance
            unsafe {
                #[cfg(target_arch = "wasm32")]
                update_ui_view_pos(app.view.offset.x, app.view.offset.y, app.view.zoom);
            }
        }
        app.view.changed = false;
        
        if app.previous_tool_id != tool_id {
            app.previous_tool_id = tool_id;
            match tool_id {
                0 => {app.active_tool = Box::new(PanTool::new());},
                1 => {app.active_tool = Box::new(PlaceTool::new());},
                _ => {}
            }
        }
        
        if should_reset == 1.0 {
            app.bodies = Vec::new();
            app.view.offset = vec2(0.0, 0.0);
            app.view.zoom = 1.0;

            for _ in 0..1000 {
                app.bodies.push(Box::new( Body {
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
        tick_physics(&mut app.bodies, g_force, dt);

        // --- Update and Draw ---
        for b in app.bodies.iter_mut() {
            b.update(dt);
            b.draw(&app.view);
        }

        let mouse_pos = mouse_position().into();

        if is_mouse_button_pressed(MouseButton::Left) {
            app.drag_start = Some(mouse_pos);
            let mut tool = std::mem::replace(&mut app.active_tool, Box::new(PlaceTool::new()));
            
            tool.on_click(mouse_pos, &mut app);
            
            app.active_tool = tool;
            
        }

        if is_mouse_button_down(MouseButton::Left) {
            if let Some(start) = app.drag_start {
                let mut tool = std::mem::replace(&mut app.active_tool, Box::new(PlaceTool::new()));
            
                tool.on_drag(start, mouse_pos, &mut app);
            
                app.active_tool = tool;

            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            if let Some(start) = app.drag_start {
                let mut tool = std::mem::replace(&mut app.active_tool, Box::new(PlaceTool::new()));
            
                tool.on_release(start, mouse_pos, &mut app, spawning_mass);
            
                app.active_tool = tool;
                app.drag_start = None;
            }
        }

        // Drawing
        if let Some(start) = app.drag_start {
            let mut tool = std::mem::replace(&mut app.active_tool, Box::new(PlaceTool::new()));
            
            tool.draw_preview(start, mouse_pos, &app);
            
            app.active_tool = tool;
        }

        next_frame().await
    }
}