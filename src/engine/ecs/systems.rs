use bevy::prelude::*;
use super::components::*;
use crate::world::VoxelWorld;

pub fn update_item_physics_and_lifetime(
    ecs_world: &mut World,
    voxel_world: &VoxelWorld,
    dt: f32,
) {
    let mut query = ecs_world.query::<(
        &mut Position,
        &mut Velocity,
        &mut ItemAge,
        &mut PickupDelay,
        &mut Grounded,
    )>();

    for (mut pos, mut vel, mut age, mut pickup_delay, mut grounded) in query.iter_mut(ecs_world) {
        age.0 += dt;
        if pickup_delay.0 > 0.0 {
            pickup_delay.0 = (pickup_delay.0 - dt).max(0.0);
        }
        let in_water = voxel_world.get_block(pos.0.x.floor() as i32, pos.0.y.floor() as i32, pos.0.z.floor() as i32).is_fluid();
        if in_water {
            vel.0.y = (vel.0.y + 7.0 * dt).min(1.5);
            vel.0.x *= (1.0 - 4.0 * dt).max(0.0);
            vel.0.z *= (1.0 - 4.0 * dt).max(0.0);
let (fx, fz) = crate::world::compute_water_flow_vector(pos.0.x.floor() as i32, pos.0.y.floor() as i32, pos.0.z.floor() as i32, |x, y, z| voxel_world.get_block(x, y, z));
            let fl = (fx * fx + fz * fz).sqrt();
            if fl > 0.01 { vel.0.x += fx / fl * 5.6 * dt; vel.0.z += fz / fl * 5.6 * dt; }
        } else {
            if !grounded.0 { vel.0.y -= 12.0 * dt; }
            vel.0.x *= (1.0 - 2.0 * dt).max(0.0);
            vel.0.z *= (1.0 - 2.0 * dt).max(0.0);
        }

        let next_pos = pos.0 + vel.0 * dt;
        let bx = next_pos.x.floor() as i32;
        let by = (next_pos.y - 0.1).floor() as i32;
        let bz = next_pos.z.floor() as i32;

        if voxel_world.get_block(bx, by, bz).is_solid() {
            pos.0.x = next_pos.x;
            pos.0.y = (by + 1) as f32;
            pos.0.z = next_pos.z;
            vel.0.y = 0.0;
            grounded.0 = true;
        } else {
            pos.0 = next_pos;
            grounded.0 = false;
        }
    }

    let mut expired = Vec::new();
    let mut age_query = ecs_world.query::<(Entity, &ItemAge)>();
    for (entity, age) in age_query.iter(ecs_world) {
        if age.0 >= 300.0 { expired.push(entity); }
    }
    for entity in expired { let _ = ecs_world.despawn(entity); }
}
