use crate::{Vec2, particles::ParticleProxy};
use macroquad::prelude::*;
use crate::particles::Particle;

pub struct Body {
    pub pos: Vec2,
    pub vel: Vec2,
    pub mass: f32,
}

impl Particle for Body {
    fn get_snapshot(&self) -> (Vec2, f32, Vec2, &str) {
        return (self.pos, self.mass, self.vel, self.get_type())
    }
    fn get_type(&self) -> &str {
        return "body"
    }

    fn react_to_other(&mut self, other: &dyn Particle, g_force: f32, dt: f32) -> i8 {
        let p1 = self.pos;
        let p2 = other.get_pos();
        let m1 = self.mass;
        let m2 = other.get_mass();

        // 1. Check for Collision/Fusion
        let radius_sum = ((m1.abs() / 3.14).sqrt() + (m2.abs() / 3.14).sqrt()) * 0.8;
        if (p1 - p2).length() < radius_sum && m1.signum() == m2.signum() {
            if m1 >= m2 {
                // Self is the "winner". We calculate new velocity.
                // Note: We can only change OURSELF here. 
                // We return '2' to tell the main loop to kill 'other'.
                let combined_mass = m1 + m2;
                self.vel = (self.vel * m1 + other.get_vel() * m2) / combined_mass;
                self.mass = combined_mass;
                return 2
            }
            return 0
        } 

        // 2. Standard Gravity
        let dir = p2 - p1;
        let dist_sq = dir.length_squared().max(100.0);
        let force_mag = (g_force * m1.signum() * m2) / (dist_sq + 0.001);
        let accel = dir.normalize() * force_mag;
        
        // We modify OUR velocity
        self.vel += accel * dt;
        0
    }

    // fn react_to_other(&mut self, other: &dyn Particle, dt: f32) -> i8 {
    //     let p1 = self.pos;
    //     let p2 = other.get_pos();
    //     let m1 = self.mass;
    //     let m2 = other.get_mass();

    //     if other.get_type() != self.get_type() {
    //         return 0
    //     }

    //     if ((m1 / 3.14).sqrt() + (m2 / 3.14).sqrt()) * 0.8 >= (p1 - p2).length() && m1.signum() == m2.signum() {
    //         let kept_body : &mut Body = if m1 >= m2 {self} else {other};
    //         let might_remove : &dyn Particle = if m1 < m2 {self} else {other};

    //         kept_body.vel = (kept_body.vel * kept_body.mass 
    //             + might_remove.vel * might_remove.mass * 0.5) 
    //             / (kept_body.mass + might_remove.mass * 0.5);
    //         kept_body.mass += might_remove.mass * 0.5;
    //         might_remove.mass *= 0.5;

    //         if might_remove.mass.abs() < 1.0 {
    //             kept_body.mass += kept_body.mass;
    //             return 2;
    //         } else {
    //             return 0
    //         }
                
    //     } else {
            
    //         let dir = p2 - p1;
    //         let dist_sq = dir.length_squared().max(100.0); // "Softening" to prevent glitches
    //         let force_mag = (g_force * (m1 / m1.abs()) * m2) / (dist_sq + 0.001); // G constant set to 100.0 for visibility
    //         let accel = dir.normalize() * force_mag;
            
    //         bodies[i].vel += accel * dt;
    //         return 0
    //     }
    // }

    fn update(&mut self, dt: f32) {
        self.pos += self.vel * dt;
    }

    fn draw(&self) {
        let speed = self.vel.length();
        let t = (speed / 200.0).clamp(0.0, 1.0);
        let color = Color::new(t, 0.2, 1.0 - t, 1.0);
        draw_circle(self.pos.x, self.pos.y, self.mass / 10.0, color);
    }

    // Must declare return types!
    fn get_pos(&self) -> Vec2 {
        self.pos
    }

    fn get_mass(&self) -> f32 {
        self.mass
    }

    fn add_mass(&mut self, mass: f32) {
        self.mass += mass
    }
    fn get_vel(&self) -> Vec2 {
        return self.vel;
    }
    fn set_vel(&mut self, vel: Vec2) {
        self.vel = vel
    }
}