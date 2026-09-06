use bevy::math::Vec3;
use super::dropped_item::DroppedItem;
use super::item_type::ItemType;
use crate::render::types::Vertex;
use crate::world::block::BlockType;

pub fn build_dropped_item_mesh(item: &DroppedItem, vertices: &mut Vec<Vertex>, indices: &mut Vec<u32>) {
    let (bob, spin) = item.get_render_offset_and_rot();
    let center = item.position + Vec3::new(0.0, bob + 0.15, 0.0);
    let (c, s) = (spin.cos(), spin.sin());
    let rot = |v: Vec3| Vec3::new(v.x * c - v.z * s, v.y, v.x * s + v.z * c);

    match item.item.item {
        ItemType::Block(BlockType::Torch) => build_flat_item(item.item.item, center, rot, vertices, indices),
        ItemType::Block(b) => build_mini_block(b, center, rot, vertices, indices),
        other => build_flat_item(other, center, rot, vertices, indices),
    }
}

fn build_mini_block<F: Fn(Vec3) -> Vec3>(
    b: BlockType, center: Vec3, rot: F, v: &mut Vec<Vertex>, idx: &mut Vec<u32>,
) {
    let h = 0.13;
    let faces: [(Vec3, [Vec3; 4]); 6] = [
        (Vec3::Y, [Vec3::new(-h, h, h), Vec3::new(h, h, h), Vec3::new(h, h, -h), Vec3::new(-h, h, -h)]),
        (-Vec3::Y, [Vec3::new(-h, -h, -h), Vec3::new(h, -h, -h), Vec3::new(h, -h, h), Vec3::new(-h, -h, h)]),
        (Vec3::Z, [Vec3::new(-h, -h, h), Vec3::new(h, -h, h), Vec3::new(h, h, h), Vec3::new(-h, h, h)]),
        (-Vec3::Z, [Vec3::new(h, -h, -h), Vec3::new(-h, -h, -h), Vec3::new(-h, h, -h), Vec3::new(h, h, -h)]),
        (Vec3::X, [Vec3::new(h, -h, h), Vec3::new(h, -h, -h), Vec3::new(h, h, -h), Vec3::new(h, h, h)]),
        (-Vec3::X, [Vec3::new(-h, -h, -h), Vec3::new(-h, -h, h), Vec3::new(-h, h, h), Vec3::new(-h, h, -h)]),
    ];
    let uvs = [[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]];

    for (norm, corners) in faces.iter() {
        let base_idx = v.len() as u32;
        let r_norm = rot(*norm);
        let offset = (norm.x as i32, norm.y as i32, norm.z as i32);
        let layer = crate::world::face_texture::compute_face_texture(b, offset, false, 4.0, b.tex_layer());
        for (i, p) in corners.iter().enumerate() {
            v.push(Vertex {
                position: (center + rot(*p)).to_array(),
                normal: r_norm.to_array(),
                uv: uvs[i],
                tex_layer: layer,
            });
        }
        idx.extend_from_slice(&[base_idx, base_idx + 1, base_idx + 2, base_idx, base_idx + 2, base_idx + 3]);
    }
}

fn build_flat_item<F: Fn(Vec3) -> Vec3>(
    item: ItemType, center: Vec3, rot: F, v: &mut Vec<Vertex>, idx: &mut Vec<u32>,
) {
    let quads = crate::render::item_model::get_item_model(item);
    let layer = item.tex_layer();
    for quad in quads {
        let base = v.len() as u32;
        let r_norm = rot(quad.normal);
        for (i, p) in quad.corners.iter().enumerate() {
            let p_centered = (*p - Vec3::splat(0.5)) * 0.38;
            v.push(Vertex {
                position: (center + rot(p_centered)).to_array(),
                normal: r_norm.to_array(),
                uv: quad.uvs[i],
                tex_layer: layer,
            });
        }
        idx.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }
}
