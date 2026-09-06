use bevy::prelude::*;
use bevy::math::Vec3;
use super::components::*;
use crate::engine::Inventory;

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
