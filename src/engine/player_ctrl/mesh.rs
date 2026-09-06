use bevy::math::Vec3;
use super::def::Player;
use super::steve_box::add_steve_box;
use super::steve_uv::*;
use crate::render::types::Vertex;

pub fn build_player_mesh(p: &Player) -> (Vec<Vertex>, Vec<u32>) {
    let mut v = Vec::with_capacity(144);
    let mut idx = Vec::with_capacity(216);

    let leg_swing = (p.walk_time * 0.6662).cos() * 1.4 * p.walk_speed;
    let arm_swing = (p.walk_time * 0.6662).cos() * 1.0 * p.walk_speed;

    let c_y = if p.is_sneaking { -0.125 } else { 0.0 };
    let head_pivot = Vec3::new(0.0, 1.40 + c_y - if p.is_sneaking { 0.2625 } else { 0.0 }, 0.0);
    let torso_pivot = Vec3::new(0.0, 1.40 + c_y - if p.is_sneaking { 0.20 } else { 0.0 }, 0.0);
    let torso_pitch = if p.is_sneaking { 0.5 } else { 0.0 };

    let arm_y = 1.38 + c_y - if p.is_sneaking { 0.20 } else { 0.0 };
    let arm_pitch_extra = if p.is_sneaking { 0.4 } else { 0.0 };
    let leg_z = if p.is_sneaking { -0.25 } else { 0.0 };
    let hip_y = 1.40 + c_y - 0.72;

    add_steve_box(&mut v, &mut idx, head_pivot, Vec3::new(0.5, 0.5, 0.5), Vec3::new(0.0, 0.25, 0.0), 0.0, p.yaw, p.position, head_uvs());
    add_steve_box(&mut v, &mut idx, torso_pivot, Vec3::new(0.5, 0.72, 0.25), Vec3::new(0.0, -0.36, 0.0), torso_pitch, p.yaw, p.position, body_uvs());

    let r_shoulder = Vec3::new(-0.375, arm_y, 0.0);
    let swing_val = (p.mining_swing * std::f32::consts::PI).sin();
    let r_pitch = arm_swing + arm_pitch_extra - swing_val * 1.2;
    add_steve_box(&mut v, &mut idx, r_shoulder, Vec3::new(0.24, 0.72, 0.24), Vec3::new(0.0, -0.36, 0.0), r_pitch, p.yaw, p.position, right_arm_uvs());

    let l_shoulder = Vec3::new(0.375, arm_y, 0.0);
    let l_pitch = -arm_swing + arm_pitch_extra;
    add_steve_box(&mut v, &mut idx, l_shoulder, Vec3::new(0.24, 0.72, 0.24), Vec3::new(0.0, -0.36, 0.0), l_pitch, p.yaw, p.position, left_arm_uvs());

    add_steve_box(&mut v, &mut idx, Vec3::new(-0.125, hip_y, leg_z), Vec3::new(0.24, 0.68, 0.24), Vec3::new(0.0, -0.34, 0.0), -leg_swing, p.yaw, p.position, right_leg_uvs());
    add_steve_box(&mut v, &mut idx, Vec3::new(0.125, hip_y, leg_z), Vec3::new(0.24, 0.68, 0.24), Vec3::new(0.0, -0.34, 0.0), leg_swing, p.yaw, p.position, left_leg_uvs());

    if let Some(item) = p.held_item {
        super::hand_item::add_held_item(&mut v, &mut idx, item, r_shoulder, r_pitch, p.yaw, p.position);
    }

    (v, idx)
}