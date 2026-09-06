use bevy::math::Vec3;
use super::types::Vertex;

pub fn build_hover_overlay_mesh(bx: i32, by: i32, bz: i32) -> (Vec<Vertex>, Vec<u32>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let base = Vec3::new(bx as f32, by as f32, bz as f32);
    let min_off = -0.015;
    let max_off = 1.015;
    let thick = 0.055; // Bold visible outline

    let edges = [
        // Bottom 4 edges
        (Vec3::new(min_off, min_off, min_off), Vec3::new(max_off, min_off + thick, min_off + thick)),
        (Vec3::new(min_off, min_off, max_off - thick), Vec3::new(max_off, min_off + thick, max_off)),
        (Vec3::new(min_off, min_off, min_off), Vec3::new(min_off + thick, min_off + thick, max_off)),
        (Vec3::new(max_off - thick, min_off, min_off), Vec3::new(max_off, min_off + thick, max_off)),
        // Top 4 edges
        (Vec3::new(min_off, max_off - thick, min_off), Vec3::new(max_off, max_off, min_off + thick)),
        (Vec3::new(min_off, max_off - thick, max_off - thick), Vec3::new(max_off, max_off, max_off)),
        (Vec3::new(min_off, max_off - thick, min_off), Vec3::new(min_off + thick, max_off, max_off)),
        (Vec3::new(max_off - thick, max_off - thick, min_off), Vec3::new(max_off, max_off, max_off)),
        // 4 Vertical Pillars
        (Vec3::new(min_off, min_off, min_off), Vec3::new(min_off + thick, max_off, min_off + thick)),
        (Vec3::new(max_off - thick, min_off, min_off), Vec3::new(max_off, max_off, min_off + thick)),
        (Vec3::new(min_off, min_off, max_off - thick), Vec3::new(min_off + thick, max_off, max_off)),
        (Vec3::new(max_off - thick, min_off, max_off - thick), Vec3::new(max_off, max_off, max_off)),
    ];

    let face_indices = [
        (0, 3, 2, 1, -Vec3::Z),
        (4, 5, 6, 7, Vec3::Z),
        (0, 1, 5, 4, -Vec3::Y),
        (3, 7, 6, 2, Vec3::Y),
        (0, 4, 7, 3, -Vec3::X),
        (1, 2, 6, 5, Vec3::X),
    ];

    for (p0_rel, p1_rel) in edges {
        let p0 = base + p0_rel;
        let p1 = base + p1_rel;

        let corners = [
            Vec3::new(p0.x, p0.y, p0.z),
            Vec3::new(p1.x, p0.y, p0.z),
            Vec3::new(p1.x, p1.y, p0.z),
            Vec3::new(p0.x, p1.y, p0.z),
            Vec3::new(p0.x, p0.y, p1.z),
            Vec3::new(p1.x, p0.y, p1.z),
            Vec3::new(p1.x, p1.y, p1.z),
            Vec3::new(p0.x, p1.y, p1.z),
        ];

        for (i0, i1, i2, i3, norm) in face_indices {
            let s = vertices.len() as u32;
            for &idx in &[i0, i1, i2, i3] {
                vertices.push(Vertex {
                    position: corners[idx].to_array(),
                    normal: norm.to_array(),
                    uv: [0.0, 0.0],
                    tex_layer: -1.0,
                });
            }
            indices.extend_from_slice(&[s, s + 1, s + 2, s, s + 2, s + 3]);
        }
    }

    (vertices, indices)
}