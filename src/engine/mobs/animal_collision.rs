use bevy::math::Vec3;
use super::animal_def::{Animal, AnimalType};
use crate::engine::Player;

pub fn resolve_player_animals_collision(player: &mut Player, animals: &mut [(Animal, Vec3)]) {
    let p_radius = 0.40f32;
    let p_height = player.height;
    let p_pos = player.position;

    for (animal, a_pos) in animals.iter_mut() {
        let (a_radius, a_height) = match animal.animal_type {
            AnimalType::Pig => (0.45, 0.9),
            AnimalType::Cow => (0.50, 1.4),
            AnimalType::Sheep => (0.45, 1.3),
        };

        let dy = p_pos.y - a_pos.y;
        if dy > -p_height && dy < a_height {
            let dx = p_pos.x - a_pos.x;
            let dz = p_pos.z - a_pos.z;
            let dist_sq = dx * dx + dz * dz;
            let min_dist = p_radius + a_radius;
            if dist_sq < min_dist * min_dist && dist_sq > 0.00001 {
                let dist = dist_sq.sqrt();
                let overlap = min_dist - dist;
                let nx = dx / dist;
                let nz = dz / dist;

                // Push player and animal apart
                player.position.x += nx * overlap * 0.7;
                player.position.z += nz * overlap * 0.7;
                a_pos.x -= nx * overlap * 0.3;
                a_pos.z -= nz * overlap * 0.3;

                // Damp relative velocity
                let normal_dot = player.velocity.x * nx + player.velocity.z * nz;
                if normal_dot < 0.0 {
                    player.velocity.x -= nx * normal_dot;
                    player.velocity.z -= nz * normal_dot;
                }
            }
        }
    }
}
