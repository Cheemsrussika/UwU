use bevy::math::{Mat4, Vec3};
use crate::engine::items::ItemType;
use crate::render::types::Vertex;

pub fn add_held_tool(
    v: &mut Vec<Vertex>,
    idx: &mut Vec<u32>,
    tool: ItemType,
    shoulder: Vec3,
    pitch: f32,
    yaw: f32,
    player_pos: Vec3,
) {
    let (t_item, r_item, s_item) = if tool.is_tool() {
        (
            Vec3::new(0.0, 4.0 / 16.0, 0.5 / 16.0),
            Mat4::from_rotation_y((-90.0f32).to_radians())
                * Mat4::from_rotation_z(45.0f32.to_radians()),
            Vec3::splat(0.80),
        )
    } else {
        (
            Vec3::new(0.0, 3.0 / 16.0, 1.0 / 16.0),
            Mat4::from_rotation_x((-10.0f32).to_radians()),
            Vec3::splat(0.52),
        )
    };

    let m_item = Mat4::from_translation(t_item)
        * r_item
        * Mat4::from_scale(s_item)
        * Mat4::from_translation(Vec3::splat(-0.5));

    let m_layer = Mat4::from_rotation_x((-90.0f32).to_radians())
        * Mat4::from_rotation_y(180.0f32.to_radians())
        * Mat4::from_translation(Vec3::new(0.0, 2.0 / 16.0, -10.0 / 16.0));

    let m_arm_to_item = Mat4::from_scale(Vec3::new(1.0, -1.0, -1.0)) * (m_layer * m_item);

    let world_mat = Mat4::from_translation(player_pos)
        * Mat4::from_rotation_y(yaw)
        * Mat4::from_translation(shoulder)
        * Mat4::from_rotation_x(pitch)
        * m_arm_to_item;

    let quads = crate::render::item_model::get_item_model(tool);
    let layer = tool.tex_layer();
    for quad in quads {
        let base = v.len() as u32;
        let r_norm = (world_mat.transform_vector3(quad.normal)).normalize();
        for (i, p) in quad.corners.iter().enumerate() {
            v.push(Vertex {
                position: world_mat.transform_point3(*p).to_array(),
                normal: r_norm.to_array(),
                uv: quad.uvs[i],
                tex_layer: layer,
            });
        }
        idx.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }
}
