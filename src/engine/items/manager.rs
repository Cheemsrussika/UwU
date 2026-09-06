use glam::Vec3;
use super::dropped_item::DroppedItem;
use super::item_mesh::build_dropped_item_mesh;
use super::item_stack::ItemStack;
use crate::engine::Inventory;
use crate::render::types::Vertex;
use crate::world::VoxelWorld;

#[derive(Default)]
pub struct ItemEntityManager {
    pub items: Vec<DroppedItem>,
}

impl ItemEntityManager {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn spawn(&mut self, pos: Vec3, vel: Vec3, item: ItemStack) {
        self.items.push(DroppedItem::new(pos, vel, item));
    }

    pub fn update(&mut self, dt: f32, world: &VoxelWorld) {
        for item in &mut self.items {
            item.update(dt, world);
        }
        self.items.retain(|it| it.age < 300.0);
    }

    pub fn try_pickup(&mut self, player_pos: Vec3, inv: &mut Inventory) {
        self.items.retain_mut(|it| {
            if it.pickup_delay > 0.0 {
                return true;
            }
            let d = it.position.distance(player_pos + Vec3::new(0.0, 0.9, 0.0));
            if d < 1.75 {
                let remaining = inv.add_stack(&it.item);
                if remaining == 0 {
                    return false; // fully picked up
                }
                it.item.count = remaining;
            }
            true
        });
    }

    pub fn build_mesh(&self, v: &mut Vec<Vertex>, idx: &mut Vec<u32>) {
        for item in &self.items {
            build_dropped_item_mesh(item, v, idx);
        }
    }
}
