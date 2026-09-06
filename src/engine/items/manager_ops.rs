use bevy::ecs::prelude::*;
use bevy::math::Vec3;
use super::item_stack::ItemStack;
use crate::engine::ecs::*;

pub fn spawn_dropped_item(
    ecs_world: &mut World,
    id: i32,
    pos: Vec3,
    vel: Vec3,
    item: ItemStack,
) {
    let mut query = ecs_world.query::<&ItemEntityId>();
    if query.iter(ecs_world).any(|ent_id| ent_id.0 == id) {
        return;
    }
    let bob_offset = (pos.x * 13.0 + pos.z * 17.0).sin().abs() * 6.28;
    ecs_world.spawn((
        ItemEntityId(id), Position(pos), Velocity(vel), ItemPayload(item),
        ItemAge(0.0), PickupDelay(0.5), Grounded(false), BobOffset(bob_offset),
    ));
}

pub fn remove_dropped_items(ecs_world: &mut World, ids: &[i32]) {
    let mut to_despawn = Vec::new();
    let mut query = ecs_world.query::<(Entity, &ItemEntityId)>();
    for (e, ent_id) in query.iter(ecs_world) {
        if ids.contains(&ent_id.0) { to_despawn.push(e); }
    }
    for e in to_despawn { let _ = ecs_world.despawn(e); }
}
