use bevy::math::Vec3;
use crate::world::VoxelWorld;

pub fn update_animal_physics(
    pos: &mut Vec3,
    vel: &mut Vec3,
    desired_vel: Vec3,
    is_moving: bool,
    dt: f32,
    world: &VoxelWorld,
) {
    if is_moving {
        vel.x = desired_vel.x;
        vel.z = desired_vel.z;
    } else {
        vel.x *= (1.0 - 5.0 * dt).max(0.0);
        vel.z *= (1.0 - 5.0 * dt).max(0.0);
    }

    vel.y -= 18.0 * dt;

    let nx = *pos + Vec3::new(vel.x * dt, 0.0, 0.0);
    if check_obstacle(nx, world) {
        if can_step_up(nx, world) { vel.y = 4.5; }
        else { vel.x = 0.0; }
    } else {
        pos.x = nx.x;
    }

    let nz = *pos + Vec3::new(0.0, 0.0, vel.z * dt);
    if check_obstacle(nz, world) {
        if can_step_up(nz, world) { vel.y = 4.5; }
        else { vel.z = 0.0; }
    } else {
        pos.z = nz.z;
    }

    let ny = *pos + Vec3::new(0.0, vel.y * dt, 0.0);
    let by = (ny.y).floor() as i32;
    let bx = pos.x.floor() as i32;
    let bz = pos.z.floor() as i32;
    if world.get_block(bx, by, bz).is_solid() {
        pos.y = (by + 1) as f32;
        vel.y = 0.0;
    } else {
        pos.y = ny.y;
    }
}

fn check_obstacle(pos: Vec3, world: &VoxelWorld) -> bool {
    let bx = pos.x.floor() as i32;
    let by = pos.y.floor() as i32;
    let bz = pos.z.floor() as i32;
    world.get_block(bx, by, bz).is_solid()
}

fn can_step_up(pos: Vec3, world: &VoxelWorld) -> bool {
    let bx = pos.x.floor() as i32;
    let by = pos.y.floor() as i32 + 1;
    let bz = pos.z.floor() as i32;
    !world.get_block(bx, by, bz).is_solid()
}
