use macroquad::prelude::*;

struct Body {
    pos: Vec2,
    vel: Vec2,
    mass: f32,
}

#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn get_gravity_value() -> f32;
}


#[macroquad::main("Gravity Simulation")]
async fn main() {
    let mut bodies = Vec::new();
    
    // Create a few random particles
    for _ in 0..1000 {
        bodies.push(Body {
            pos: vec2(rand::gen_range(100.0, 1500.0), rand::gen_range(100.0, 900.0)),
            vel: vec2(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0)),
            mass: rand::gen_range(1.0, 10.0),
        });
    }

    loop {
        // Inside your loop:
        #[cfg(target_arch = "wasm32")]
        let g_force = unsafe { get_gravity_value() };

        clear_background(BLACK);
        let dt = get_frame_time(); // Get time elapsed (around 0.016s for 60fps)

        // --- Physics Logic ---
        // We use a simple O(N^2) loop to calculate gravity between all pairs
        for i in 0..bodies.len() {
            for j in 0..bodies.len() {
                if i == j { continue; }
                
                let p1 = bodies[i].pos;
                let p2 = bodies[j].pos;
                let m2 = bodies[j].mass;
                
                let dir = p2 - p1;
                let dist_sq = dir.length_squared().max(100.0); // "Softening" to prevent glitches
                let force_mag = (g_force * m2) / dist_sq; // G constant set to 100.0 for visibility
                let accel = dir.normalize() * force_mag;
                
                bodies[i].vel += accel * dt;
            }
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
        
            draw_circle(b.pos.x, b.pos.y, b.mass / 10.0, color);
        }

        draw_text(&format!("FPS: {}", get_fps()), 20.0, 20.0, 20.0, GREEN);
        next_frame().await
    }
}