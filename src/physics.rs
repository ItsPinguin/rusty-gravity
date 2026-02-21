use crate::particles::Particle;

pub fn tick_physics(bodies: &mut Vec<Box<dyn Particle>>, g_force: f32, dt: f32) {
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
            if signal == 1 || signal == 3 {
                to_remove.push(i);
            }
            if signal == 2 || signal == 3 {
                to_remove.push(j);
            }
        }
    }
    to_remove.sort();
    to_remove.dedup();
    for &idx in to_remove.iter().rev() {
        if idx < bodies.len() {
            bodies.remove(idx);
        }
    }
}