use bevy::math::Vec3;
use super::def::Player;
use crate::world::VoxelWorld;

pub fn update_player_movement(p: &mut Player, move_input: Vec3, jump: bool, sprint: bool, aim_yaw: Option<f32>, dt: f32, world: &VoxelWorld) {
    let foot_block = world.get_block(p.position.x.floor() as i32, p.position.y.floor() as i32, p.position.z.floor() as i32);
    p.in_water = foot_block.is_fluid();

    let is_moving = move_input.length_squared() > 0.01;
    if sprint && is_moving && !p.is_sneaking && p.stamina > 5.0 && !p.in_water {
        p.is_sprinting = true;
    }
    if !is_moving || p.is_sneaking || p.stamina <= 0.0 {
        p.is_sprinting = false;
    }
    if p.is_sprinting {
        p.stamina = (p.stamina - 12.0 * dt).max(0.0);
    }

    let gravity = if p.in_water { -6.0 } else { -26.0 };
    let base_speed = if p.in_water { 4.0 } else { 8.5 };
    let walk_speed = if p.is_flying { 14.0 } else if p.is_sprinting { base_speed * 1.30 } else if p.is_sneaking { base_speed * 0.3 } else { base_speed };
    let jump_force = if p.in_water { 4.5 } else { 8.8 };

    if p.is_flying {
        super::flight::update_flight_movement(p, jump, p.is_sneaking, dt);
    } else {
        if jump && (p.on_ground || p.in_water) && !p.is_sneaking {
            p.velocity.y = jump_force;
            p.on_ground = false;
        }
        p.velocity.y += gravity * dt;
        let max_fall = if p.in_water { -5.0 } else { -35.0 };
        if p.velocity.y < max_fall { p.velocity.y = max_fall; }
    }

    let target_vx = move_input.x * walk_speed;
    let target_vz = move_input.z * walk_speed;
    let accel = 18.0 * dt;
    p.velocity.x += (target_vx - p.velocity.x) * accel.min(1.0);
    p.velocity.z += (target_vz - p.velocity.z) * accel.min(1.0);

    if p.in_water {
        let (fx, fz) = crate::world::compute_water_flow_vector(
            p.position.x.floor() as i32,
            p.position.y.floor() as i32,
            p.position.z.floor() as i32,
            |x, y, z| world.get_block(x, y, z),
        );
        p.velocity.x += fx * 4.0 * dt;
        p.velocity.z += fz * 4.0 * dt;
    }

    // Keyboard movement direction takes priority, otherwise smooth rotate to mouse aim
    if move_input.length_squared() > 0.01 {
        p.yaw = move_input.x.atan2(move_input.z);
    } else if let Some(target_yaw) = aim_yaw {
        let diff = (target_yaw - p.yaw + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU) - std::f32::consts::PI;
        p.yaw += diff * (10.0 * dt).min(1.0);
    }

    let h_speed = (p.velocity.x * p.velocity.x + p.velocity.z * p.velocity.z).sqrt();
    if h_speed > 0.1 && p.on_ground {
        p.walk_time += dt * h_speed * 1.5;
        p.walk_speed = (p.walk_speed + (h_speed / walk_speed - p.walk_speed) * 10.0 * dt).clamp(0.0, 1.0);
    } else {
        p.walk_speed = (p.walk_speed - 8.0 * dt).max(0.0);
    }

    // X axis with sneak edge protection
    let nx = Vec3::new(p.position.x + p.velocity.x * dt, p.position.y, p.position.z);
    let edge_x = p.is_sneaking && p.on_ground && !world.get_block(nx.x.floor() as i32, (p.position.y - 0.5).floor() as i32, p.position.z.floor() as i32).is_solid();
    if !edge_x && !super::def::check_player_collision(nx, p.width, p.height, world) { p.position.x = nx.x; } else { p.velocity.x = 0.0; }

    // Z axis with sneak edge protection
    let nz = Vec3::new(p.position.x, p.position.y, p.position.z + p.velocity.z * dt);
    let edge_z = p.is_sneaking && p.on_ground && !world.get_block(p.position.x.floor() as i32, (p.position.y - 0.5).floor() as i32, nz.z.floor() as i32).is_solid();
    if !edge_z && !super::def::check_player_collision(nz, p.width, p.height, world) { p.position.z = nz.z; } else { p.velocity.z = 0.0; }

    // Y axis
    let ny = Vec3::new(p.position.x, p.position.y + p.velocity.y * dt, p.position.z);
    if !super::def::check_player_collision(ny, p.width, p.height, world) {
        p.position.y = ny.y;
        p.on_ground = false;
    } else {
        if p.velocity.y < 0.0 {
            p.on_ground = true;
            p.position.y = p.position.y.floor();
            while super::def::check_player_collision(p.position, p.width, p.height, world) { p.position.y += 0.05; }
        } else if p.velocity.y > 0.0 {
            p.position.y = p.position.y.ceil() - p.height - 0.001;
        }
        p.velocity.y = 0.0;
    }

    if p.position.y < -30.0 { p.position = Vec3::new(0.0, 7.0, 0.0); p.velocity = Vec3::ZERO; }
}