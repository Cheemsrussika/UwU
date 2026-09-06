use bevy::math::Vec3;
use crate::render::types::Vertex;

pub fn add_animal_box(
    v: &mut Vec<Vertex>,
    idx: &mut Vec<u32>,
    origin: Vec3,
    sz: Vec3,
    tex_u: f32,
    tex_v: f32,
    dx: f32,
    dy: f32,
    dz: f32,
    tex_layer: f32,
) {
    let (x0, y0, z0) = (origin.x, origin.y, origin.z);
    let (x1, y1, z1) = (x0 + sz.x, y0 + sz.y, z0 + sz.z);

    // Standard Java UV boxes on 64x64 skin
    let s = 1.0 / 64.0;
    let faces: [(Vec3, [[f32; 3]; 4], [f32; 4]); 6] = [
        // Top (+Y)
        (Vec3::Y, [[x0, y1, z1], [x1, y1, z1], [x1, y1, z0], [x0, y1, z0]], [(tex_u + dz) * s, tex_v * s, (tex_u + dz + dx) * s, (tex_v + dz) * s]),
        // Bottom (-Y)
        (-Vec3::Y, [[x0, y0, z0], [x1, y0, z0], [x1, y0, z1], [x0, y0, z1]], [(tex_u + dz + dx) * s, tex_v * s, (tex_u + dz + 2.0 * dx) * s, (tex_v + dz) * s]),
        // Front (+Z)
        (Vec3::Z, [[x0, y0, z1], [x1, y0, z1], [x1, y1, z1], [x0, y1, z1]], [(tex_u + dz) * s, (tex_v + dz + dy) * s, (tex_u + dz + dx) * s, (tex_v + dz) * s]),
        // Back (-Z)
        (-Vec3::Z, [[x1, y0, z0], [x0, y0, z0], [x0, y1, z0], [x1, y1, z0]], [(tex_u + 2.0 * dz + dx) * s, (tex_v + dz + dy) * s, (tex_u + 2.0 * (dz + dx)) * s, (tex_v + dz) * s]),
        // Left (-X)
        (-Vec3::X, [[x0, y0, z0], [x0, y0, z1], [x0, y1, z1], [x0, y1, z0]], [tex_u * s, (tex_v + dz + dy) * s, (tex_u + dz) * s, (tex_v + dz) * s]),
        // Right (+X)
        (Vec3::X, [[x1, y0, z1], [x1, y0, z0], [x1, y1, z0], [x1, y1, z1]], [(tex_u + dz + dx) * s, (tex_v + dz + dy) * s, (tex_u + 2.0 * dz + dx) * s, (tex_v + dz) * s]),
    ];

    for (norm, corners, uv) in faces {
        let base = v.len() as u32;
        let uvs = [[uv[0], uv[1]], [uv[2], uv[1]], [uv[2], uv[3]], [uv[0], uv[3]]];
        for i in 0..4 {
            v.push(Vertex {
                position: corners[i],
                normal: norm.to_array(),
                uv: uvs[i],
                tex_layer,
            });
        }
        idx.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }
}
