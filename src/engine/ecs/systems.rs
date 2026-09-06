use bevy::prelude::*;
use bevy::math::Vec3;
use super::components::*;
use crate::engine::Inventory;
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
        if !grounded.0 {
            vel.0.y -= 12.0 * dt;
        }
        vel.0.x *= (1.0 - 2.0 * dt).max(0.0);
        vel.0.z *= (1.0 - 2.0 * dt).max(0.0);

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
        if age.0 >= 300.0 {
            expired.push(entity);
        }
    }
    for entity in expired {
        let _ = ecs_world.despawn(entity);
    }
}

pub fn process_item_pickup(
    ecs_world: &mut World,
    player_pos: Vec3,
    inv: &mut Inventory,
) -> Vec<i32> {
    let mut picked_up_ids = Vec::new();
    let mut to_despawn = Vec::new();

    let mut query = ecs_world.query::<(
        Entity,
        &ItemEntityId,
        &Position,
        &mut ItemPayload,
        &PickupDelay,
    )>();

    for (entity, id, pos, mut payload, delay) in query.iter_mut(ecs_world) {
        if delay.0 > 0.0 {
            continue;
        }
        let d = pos.0.distance(player_pos + Vec3::new(0.0, 0.9, 0.0));
        if d < 1.75 {
            let remaining = inv.add_stack(&payload.0);
            if remaining == 0 {
                picked_up_ids.push(id.0);
                to_despawn.push(entity);
            } else {
                payload.0.count = remaining;
            }
        }
    }

    for entity in to_despawn {
        let _ = ecs_world.despawn(entity);
    }
    picked_up_ids
}
