use crate::{Vec2, ViewState};
use macroquad::prelude::*;
use crate::particles::Particle;

pub struct Planet {
    pub pos: Vec2,
    pub vel: Vec2,
    pub mass: f32,
}

impl Particle for Planet {

    fn get_type(&self) -> &str {
        return "planet"
    }

    fn react_to_other(&mut self, other: &mut dyn Particle, g_force: f32, dt: f32) -> i8 {
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
                return 2;
            }
            return 0;
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

    fn update(&mut self, dt: f32) {
        self.pos += self.vel * dt;
    }

    fn draw(&self, _view_state: &ViewState) {
        draw_circle(self.pos.x, self.pos.y, 10.0, PURPLE);
        draw_circle_lines(self.pos.x, self.pos.y, 15.0, 2.0, WHITE);
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