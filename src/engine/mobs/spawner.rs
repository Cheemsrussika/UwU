use bevy::prelude::*;
use bevy::math::Vec3;
use super::animal_def::{Animal, AnimalType};
use super::manager::spawn_animal;
use crate::world::{BlockType, VoxelWorld};

pub const MAX_ANIMALS: usize = 12;

pub fn maybe_spawn_animals(
    ecs_world: &mut World,
    voxel_world: &VoxelWorld,
    player_pos: Vec3,
    timer: &mut f32,
    dt: f32,
) {
    *timer += dt;
    if *timer < 3.0 { return; }
    *timer = 0.0;

    let current_count = {
        let mut query = ecs_world.query::<&Animal>();
        query.iter(ecs_world).count()
    };
    if current_count >= MAX_ANIMALS { return; }

    let px = player_pos.x.floor() as i32;
    let py = player_pos.y.floor() as i32;
    let pz = player_pos.z.floor() as i32;

    let offsets = [(12, 12), (-12, 14), (16, -10), (-14, -14), (8, -16), (-18, 6)];
    let anim_types = [AnimalType::Pig, AnimalType::Cow, AnimalType::Sheep];

    for &(dx, dz) in &offsets {
        let x = px + dx;
        let z = pz + dz;
        for dy in -4..=6 {
            let y = py + dy;
            if voxel_world.get_block(x, y, z) == BlockType::Grass
                && !voxel_world.get_block(x, y + 1, z).is_solid()
                && !voxel_world.get_block(x, y + 2, z).is_solid()
            {
                let atype = anim_types[((x * 7 + z * 13).abs() as usize) % 3];
                let spawn_pos = Vec3::new(x as f32 + 0.5, (y + 1) as f32, z as f32 + 0.5);
                spawn_animal(ecs_world, spawn_pos, atype);
                // Also spawn a companion
                let companion_pos = Vec3::new(x as f32 + 1.2, (y + 1) as f32, z as f32 + 0.5);
                spawn_animal(ecs_world, companion_pos, atype);
                return;
            }
        }
    }
}
