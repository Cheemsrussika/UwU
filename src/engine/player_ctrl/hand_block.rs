use bevy::math::{Mat4, Vec3};
use crate::render::types::Vertex;
use crate::world::block::BlockType;

pub fn add_held_block(
    v: &mut Vec<Vertex>,
    idx: &mut Vec<u32>,
    b: BlockType,
    shoulder: Vec3,
    pitch: f32,
    yaw: f32,
    player_pos: Vec3,
) {
    let r_block = Mat4::from_rotation_x(65.0f32.to_radians())
        * Mat4::from_rotation_y(45.0f32.to_radians());
    let m_block = Mat4::from_translation(Vec3::new(0.0, 2.5 / 16.0, 0.0))
        * r_block
        * Mat4::from_scale(Vec3::splat(0.30))
        * Mat4::from_translation(Vec3::splat(-0.5));

    let m_layer = Mat4::from_rotation_x((-90.0f32).to_radians())
        * Mat4::from_rotation_y(180.0f32.to_radians())
        * Mat4::from_translation(Vec3::new(0.0, 2.0 / 16.0, -10.0 / 16.0));

    let m_arm_to_item = Mat4::from_scale(Vec3::new(1.0, -1.0, -1.0)) * (m_layer * m_block);

    let world_mat = Mat4::from_translation(player_pos)
        * Mat4::from_rotation_y(yaw)
        * Mat4::from_translation(shoulder)
        * Mat4::from_rotation_x(pitch)
        * m_arm_to_item;

    let faces: [(Vec3, [Vec3; 4]); 6] = [
        (Vec3::Y, [Vec3::new(0.0, 1.0, 1.0), Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, 1.0, 0.0), Vec3::new(0.0, 1.0, 0.0)]),
        (-Vec3::Y, [Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 1.0), Vec3::new(0.0, 0.0, 1.0)]),
        (Vec3::Z, [Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 1.0), Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.0, 1.0, 1.0)]),
        (-Vec3::Z, [Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(1.0, 1.0, 0.0)]),
        (Vec3::X, [Vec3::new(1.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 0.0), Vec3::new(1.0, 1.0, 1.0)]),
        (-Vec3::X, [Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 1.0, 1.0), Vec3::new(0.0, 1.0, 0.0)]),
    ];
    let uvs = [[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]];

    for (f_i, (norm, corners)) in faces.iter().enumerate() {
        let base_idx = v.len() as u32;
        let r_norm = (world_mat.transform_vector3(*norm)).normalize();
        let layer = if b == BlockType::Grass {
            if f_i == 0 { 0.0 } else if f_i == 1 { 1.0 } else { 7.0 }
        } else if b == BlockType::OakLog {
            if f_i == 0 || f_i == 1 { 15.0 } else { 14.0 }
        } else {
            b.tex_layer()
        };
        for (i, p) in corners.iter().enumerate() {
            v.push(Vertex {
                position: world_mat.transform_point3(*p).to_array(),
                normal: r_norm.to_array(),
                uv: uvs[i],
                tex_layer: layer,
            });
        }
        idx.extend_from_slice(&[base_idx, base_idx + 1, base_idx + 2, base_idx, base_idx + 2, base_idx + 3]);
    }
}
