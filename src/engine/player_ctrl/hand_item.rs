use glam::Vec3;
use crate::engine::items::ItemType;
use crate::render::types::Vertex;

pub fn add_held_item(
    v: &mut Vec<Vertex>,
    idx: &mut Vec<u32>,
    item: ItemType,
    shoulder: Vec3,
    arm_pitch: f32,
    yaw: f32,
    player_pos: Vec3,
) {
    match item {
        ItemType::Block(b) => super::hand_block::add_held_block(v, idx, b, shoulder, arm_pitch, yaw, player_pos),
        other => super::hand_tool::add_held_tool(v, idx, other, shoulder, arm_pitch, yaw, player_pos),
    }
}
