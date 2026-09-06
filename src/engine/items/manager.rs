use bevy::ecs::prelude::*;
use bevy::math::Vec3;
use super::dropped_item::DroppedItem;
use super::item_mesh::build_dropped_item_mesh;
use super::item_stack::ItemStack;
use crate::engine::ecs::*;
use crate::engine::Inventory;
use crate::render::types::Vertex;
use crate::world::VoxelWorld;

#[derive(Resource)]
pub struct ItemEntityManager {
    pub ecs_world: World,
    pub next_id: i32,
    pub items: Vec<DroppedItem>,
}

impl Default for ItemEntityManager {
    fn default() -> Self { Self::new() }
}

impl ItemEntityManager {
    pub fn new() -> Self {
        let base = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis()).unwrap_or(1000) as i32 & 0x3FFFFFFF).max(1000);
        Self { ecs_world: World::new(), next_id: base, items: Vec::new() }
    }

    pub fn spawn(&mut self, pos: Vec3, vel: Vec3, item: ItemStack) -> i32 {
        let id = self.next_id;
        self.next_id += 1;
        self.spawn_with_id(id, pos, vel, item);
        id
    }

    pub fn spawn_with_id(&mut self, id: i32, pos: Vec3, vel: Vec3, item: ItemStack) {
        let mut query = self.ecs_world.query::<&ItemEntityId>();
        if query.iter(&self.ecs_world).any(|ent_id| ent_id.0 == id) {
            return;
        }
        let bob_offset = (pos.x * 13.0 + pos.z * 17.0).sin().abs() * 6.28;
        self.ecs_world.spawn((
            ItemEntityId(id), Position(pos), Velocity(vel), ItemPayload(item),
            ItemAge(0.0), PickupDelay(0.5), Grounded(false), BobOffset(bob_offset),
        ));
        self.sync_items();
    }

    pub fn remove_many_by_id(&mut self, ids: &[i32]) {
        let mut to_despawn = Vec::new();
        let mut query = self.ecs_world.query::<(Entity, &ItemEntityId)>();
        for (e, ent_id) in query.iter(&self.ecs_world) {
            if ids.contains(&ent_id.0) { to_despawn.push(e); }
        }
        for e in to_despawn { let _ = self.ecs_world.despawn(e); }
        self.sync_items();
    }

    pub fn update(&mut self, dt: f32, world: &VoxelWorld) {
        update_item_physics_and_lifetime(&mut self.ecs_world, world, dt);
        self.sync_items();
    }

    pub fn try_pickup(&mut self, player_pos: Vec3, inv: &mut Inventory) -> Vec<i32> {
        let picked = process_item_pickup(&mut self.ecs_world, player_pos, inv);
        if !picked.is_empty() { self.sync_items(); }
        picked
    }

    pub fn sync_items(&mut self) {
        self.items.clear();
        let mut query = self.ecs_world.query::<(
            &ItemEntityId, &Position, &Velocity, &ItemPayload,
            &ItemAge, &BobOffset, &PickupDelay, &Grounded,
        )>();
        for (id, pos, vel, payload, age, bob, delay, grounded) in query.iter(&self.ecs_world) {
            self.items.push(DroppedItem {
                id: id.0, position: pos.0, velocity: vel.0, item: payload.0.clone(),
                age: age.0, bob_offset: bob.0, pickup_delay: delay.0, on_ground: grounded.0,
            });
        }
    }

    pub fn build_mesh(&self, v: &mut Vec<Vertex>, idx: &mut Vec<u32>) {
        for item in &self.items { build_dropped_item_mesh(item, v, idx); }
    }
}
