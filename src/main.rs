use macroquad::prelude::*;
mod particles;
use particles::{Particle, Planet, Body};

#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn get_gravity_value() -> f32;
    // fn get_gravity_sign() -> f32;
    fn get_spawning_mass() -> f32;
    fn should_reset_simulation() -> f32;
}


#[macroquad::main("Gravity Simulation")]
async fn main() {
    let mut bodies : Vec<Box<dyn Particle>> = Vec::new();
    
    // Create a few random particles
    for _ in 0..1000 {
        bodies.push(Box::new(Body {
            pos: vec2(rand::gen_range(100.0, 1500.0), rand::gen_range(100.0, 900.0)),
            vel: vec2(rand::gen_range(1.0, 5.0), rand::gen_range(-1.0, 1.0)),
            mass: rand::gen_range(1.0, 2.0)
        }));
    }
    let mut drag_start = None;

    loop {
        // Inside your loop:
        #[cfg(target_arch = "wasm32")]
        let g_force = unsafe { get_gravity_value() };
        // #[cfg(target_arch = "wasm32")]
        // let g_sign = unsafe { get_gravity_sign() };
        #[cfg(target_arch = "wasm32")]
        let spawning_mass = unsafe { get_spawning_mass() };
        #[cfg(target_arch = "wasm32")]
        let should_reset = unsafe { should_reset_simulation() };
        unsafe {

        }
        if should_reset == 1.0 {
            bodies = Vec::new();

            for _ in 0..1000 {
                bodies.push(Box::new( Body {
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
        let mut to_remove = Vec::new();

        for i in 0..bodies.len() {
            for j in 0..bodies.len() {
                if i == j { continue; }


                let signal = if i < j {
                    let (left, right) = bodies.split_at_mut(j);
                    let b_i = &mut *left[i];
                    let b_j = &mut *right[0];
                    b_i.react_to_other(b_j, g_force, dt)
                } else {
                    let (left, right) = bodies.split_at_mut(i);
                    let b_j = &mut *left[j];
                    let b_i = &mut *right[0];
                    b_i.react_to_other(b_j, g_force, dt)
                };
            
                // if signal == 2 {
                //     to_remove.push(j);
                // }

                // let the_way = i < j;
                // let (left, right) = if the_way {bodies.split_at_mut(j)} else {bodies.split_at_mut(i)};
                // let body_i = if the_way {&mut*left[i]} else {&mut*right[0]};
                // let body_j = if the_way {&mut*right[0]} else {&mut*left[j]};

                // let other_snapshot = bodies[j].get_snapshot();
                // let other = ParticleProxy { 
                //     pos: other_snapshot.0,
                //     mass: other_snapshot.1,
                //     vel: other_snapshot.2,
                //     p_type: other_snapshot.3.to_string(),
                //     };
                // let signal = body_i.react_to_other(body_j, g_force, dt);
                if signal == 1 || signal == 3 {
                    to_remove.push(i);
                }
                if signal == 2 || signal == 3 {
                    to_remove.push(j);
                }
                // bodies[j] = to_delete.1
            }
        }
        to_remove.sort();
        to_remove.dedup();
        for &idx in to_remove.iter().rev() {
            if idx < bodies.len() {
                bodies.remove(idx);
            }
        }

        // --- Update and Draw ---
        for b in bodies.iter_mut() {
            b.update(dt);
        
            b.draw();
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            drag_start = Some(mouse_position());
        }

        if is_mouse_button_released(MouseButton::Left) {
            if let Some((sx, sy)) = drag_start {
                let (ex, ey) = mouse_position();

                // The velocity is the vector from where you started to where you released
                // We multiply by a small factor (0.1) so it's not too fast
                let initial_vel = vec2(sx - ex, sy - ey) * 0.5;

                bodies.push(Box::new (Body {
                    pos: vec2(sx, sy),
                    vel: initial_vel,
                    mass: spawning_mass
                    // sign: g_sign
                }));
                drag_start = None;
            }
        }

        // Visual feedback: Draw a line while dragging
        if let Some((sx, sy)) = drag_start {
            let (ex, ey) = mouse_position();
            draw_line(sx, sy, ex, ey, 2.0, WHITE);
        }

        // draw_text(&format!("FPS: {}", get_fps()), 20.0, 20.0, 20.0, GREEN);
        // draw_text(&format!("g_s: {}", g_sign), 200.0, 20.0, 20.0, GREEN);
        next_frame().await
    }
}