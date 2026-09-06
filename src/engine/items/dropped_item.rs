use glam::Vec3;
use super::item_stack::ItemStack;
use crate::world::VoxelWorld;

#[derive(Clone, Debug)]
pub struct DroppedItem {
    pub position: Vec3,
    pub velocity: Vec3,
    pub item: ItemStack,
    pub age: f32,
    pub bob_offset: f32,
    pub pickup_delay: f32,
    pub on_ground: bool,
}

impl DroppedItem {
    pub fn new(position: Vec3, velocity: Vec3, item: ItemStack) -> Self {
        let bob_offset = (position.x * 13.0 + position.z * 17.0).sin().abs() * 6.28;
        Self {
            position,
            velocity,
            item,
            age: 0.0,
            bob_offset,
            pickup_delay: 0.5,
            on_ground: false,
        }
    }

    pub fn update(&mut self, dt: f32, world: &VoxelWorld) {
        self.age += dt;
        if self.pickup_delay > 0.0 {
            self.pickup_delay = (self.pickup_delay - dt).max(0.0);
        }

        if !self.on_ground {
            self.velocity.y -= 12.0 * dt;
        }

        self.velocity.x *= (1.0 - 2.0 * dt).max(0.0);
        self.velocity.z *= (1.0 - 2.0 * dt).max(0.0);

        let next_pos = self.position + self.velocity * dt;
        let bx = next_pos.x.floor() as i32;
        let by = (next_pos.y - 0.1).floor() as i32;
        let bz = next_pos.z.floor() as i32;

        if world.get_block(bx, by, bz).is_solid() {
            self.position.x = next_pos.x;
            self.position.y = (by + 1) as f32;
            self.position.z = next_pos.z;
            self.velocity.y = 0.0;
            self.on_ground = true;
        } else {
            self.position = next_pos;
            self.on_ground = false;
        }
    }

    pub fn get_render_offset_and_rot(&self) -> (f32, f32) {
        let ticks = self.age * 20.0;
        let bob = (ticks / 10.0 + self.bob_offset).sin() * 0.1 + 0.1;
        let spin = ticks / 20.0 + self.bob_offset;
        (bob, spin)
    }
}
