use crate::{Vec2, app_state::ViewState};
use macroquad::prelude::*;
use crate::particles::Particle;

pub struct Body {
    pub pos: Vec2,
    pub vel: Vec2,
    pub mass: f32,
}

impl Particle for Body {
    fn get_type(&self) -> &str {
        return "body"
    }

    fn react_to_other(&mut self, other: &mut dyn Particle, g_force: f32, dt: f32) -> i8 {
        let p1 = self.pos;
        let p2 = other.get_pos();
        let m1 = self.mass;
        let m2 = other.get_mass();

        if (other.get_type() == self.get_type() && ((m1 / 3.14).sqrt() + (m2 / 3.14).sqrt()) * 0.8 >= (p1 - p2).length() && m1.signum() == m2.signum()) {
            // let kept_body : &mut dyn Particle = if m1 >= m2 {self} else {other};
            // let might_remove : &mut dyn Particle = if m1 < m2 {self} else {other};
            let (kept_body, might_remove) = if m1 >= m2 {
                (self as &mut dyn Particle, other)
            } else {
                (other, self as &mut dyn Particle)
            };

            kept_body.set_vel((kept_body.get_vel() * kept_body.get_mass() 
                + might_remove.get_vel() * might_remove.get_mass() * 0.5) 
                / (kept_body.get_mass() + might_remove.get_mass() * 0.5)
            );
            kept_body.set_mass(kept_body.get_mass() + might_remove.get_mass() * 0.5);
            might_remove.set_mass(might_remove.get_mass() * 0.5);

            if might_remove.get_mass().abs() < 1.0 {
                kept_body.set_mass(kept_body.get_mass() + might_remove.get_mass());
                return if m1 >= m2 {2} else {1};
            } else {
                return 0
            }
                
        } else {
            
            let dir = p2 - p1;
            let dist_sq = dir.length_squared().max(100.0); // "Softening" to prevent glitches
            let force_mag = (g_force * (m1 / m1.abs()) * m2) / (dist_sq + 0.001); // G constant set to 100.0 for visibility
            let accel = dir.normalize() * force_mag;
            
            self.vel += accel * dt;
            return 0
        }
    }

    fn update(&mut self, dt: f32) {
        self.pos += self.vel * dt;
    }

    fn draw(&self, view_state: &ViewState) {
        // 1. Calculate speed
        let speed = self.vel.length();
    
        // 2. Map speed to a 0.0 - 1.0 range (Adjust 200.0 based on your sim's scale)
        let t = (speed / 200.0).clamp(0.0, 1.0);
    
        // 3. Create a color: Blue (slow) to Red (fast)
        // Low 't' = more Blue, High 't' = more Red
        let color = Color::new(t, 0.2, 1.0 - t, 1.0);
    
        draw_circle(self.pos.x + view_state.offset.x, self.pos.y + view_state.offset.y, (self.mass / 3.14).sqrt(), color);
    }

    // Must declare return types!
    fn get_pos(&self) -> Vec2 {
        self.pos
    }

    fn get_mass(&self) -> f32 {
        self.mass
    }

    fn set_mass(&mut self, mass: f32) {
        self.mass = mass
    }
    fn get_vel(&self) -> Vec2 {
        return self.vel;
    }
    fn set_vel(&mut self, vel: Vec2) {
        self.vel = vel
    }
}