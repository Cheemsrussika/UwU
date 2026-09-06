use bevy::math::Vec3;
use crate::world::VoxelWorld;

pub fn check_player_collision(pos: Vec3, width: f32, height: f32, world: &VoxelWorld) -> bool {
    let (min_x, max_x) = ((pos.x - width).floor() as i32, (pos.x + width - 0.0001).floor() as i32);
    let (min_y, max_y) = (pos.y.floor() as i32, (pos.y + height - 0.0001).floor() as i32);
    let (min_z, max_z) = ((pos.z - width).floor() as i32, (pos.z + width - 0.0001).floor() as i32);

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            for z in min_z..=max_z {
                if world.get_block(x, y, z).is_solid() { return true; }
            }
        }
    }
    false
}
