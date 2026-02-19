use macroquad::prelude::*;

struct Body {
    pos: Vec2,
    vel: Vec2,
    mass: f32
}

#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn get_gravity_value() -> f32;
    // fn get_gravity_sign() -> f32;
    fn get_spawning_mass() -> f32;
    fn should_reset_simulation() -> f32;
}


#[macroquad::main("Gravity Simulation")]
async fn main() {
    let mut bodies = Vec::new();
    
    // Create a few random particles
    for _ in 0..1000 {
        bodies.push(Body {
            pos: vec2(rand::gen_range(100.0, 1500.0), rand::gen_range(100.0, 900.0)),
            vel: vec2(rand::gen_range(1.0, 5.0), rand::gen_range(-1.0, 1.0)),
            mass: rand::gen_range(1.0, 2.0)
        });
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

        if should_reset == 1.0 {
            bodies = Vec::new();

            for _ in 0..1000 {
                bodies.push(Body {
                pos: vec2(rand::gen_range(100.0, 1500.0), rand::gen_range(100.0, 900.0)),
                vel: vec2(rand::gen_range(1.0, 5.0), rand::gen_range(-1.0, 1.0)),
                mass: rand::gen_range(1.0, 2.0)
            });
    }
        }

        clear_background(BLACK);
        let dt = get_frame_time(); // Get time elapsed (around 0.016s for 60fps)

        // --- Physics Logic ---
        // We use a simple O(N^2) loop to calculate gravity between all pairs
        let mut to_remove = Vec::new();

        for i in 0..bodies.len() {
            for j in 0..bodies.len() {
                let p1 = bodies[i].pos;
                let p2 = bodies[j].pos;
                let m1 = bodies[i].mass;
                let m2 = bodies[j].mass;
                if i == j { continue; }

                if ((m1 / 3.14).sqrt() + (m2 / 3.14).sqrt()) * 0.8 >= (p1 - p2).length() && m1.signum() == m2.signum() {
                    let kept_body = if m1 >= m2 {i} else {j};
                    let might_remove = if kept_body != i {i} else {j};

                    bodies[kept_body].vel = (bodies[kept_body].vel * bodies[kept_body].mass 
                        + bodies[might_remove].vel * bodies[might_remove].mass * 0.5) 
                        / (bodies[kept_body].mass + bodies[might_remove].mass * 0.5);
                    bodies[kept_body].mass += bodies[might_remove].mass * 0.5;
                    bodies[might_remove].mass *= 0.5;

                    if bodies[might_remove].mass.abs() < 1.0 {
                        bodies[kept_body].mass += bodies[might_remove].mass;
                        to_remove.push(might_remove);
                    }
                        
                } else {
                    
                    let dir = p2 - p1;
                    let dist_sq = dir.length_squared().max(100.0); // "Softening" to prevent glitches
                    let force_mag = (g_force * (m1 / m1.abs()) * m2) / (dist_sq + 0.001); // G constant set to 100.0 for visibility
                    let accel = dir.normalize() * force_mag;
                    
                    bodies[i].vel += accel * dt;
                }
            }
        }
        to_remove.sort();
        to_remove.dedup();
        for &idx in to_remove.iter().rev() {
            bodies.remove(idx);
        }

        // --- Update and Draw ---
        for b in bodies.iter_mut() {
            b.pos += b.vel * dt;
        
            // 1. Calculate speed
            let speed = b.vel.length();
        
            // 2. Map speed to a 0.0 - 1.0 range (Adjust 200.0 based on your sim's scale)
            let t = (speed / 200.0).clamp(0.0, 1.0);
        
            // 3. Create a color: Blue (slow) to Red (fast)
            // Low 't' = more Blue, High 't' = more Red
            let color = Color::new(t, 0.2, 1.0 - t, 1.0);
        
            draw_circle(b.pos.x, b.pos.y, (b.mass / 3.14).sqrt(), color);
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

                bodies.push(Body {
                    pos: vec2(sx, sy),
                    vel: initial_vel,
                    mass: spawning_mass
                    // sign: g_sign
                });
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