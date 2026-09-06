use glam::Vec3;
use crate::render::types::Vertex;
use super::steve_uv::FaceUvs;

pub fn add_steve_box(
    v: &mut Vec<Vertex>,
    idx: &mut Vec<u32>,
    pivot: Vec3,
    size: Vec3,
    offset: Vec3,
    pitch: f32,
    yaw: f32,
    player_pos: Vec3,
    uvs: FaceUvs,
) {
    let (cp, sp) = (pitch.cos(), pitch.sin());
    let (cy, sy) = (yaw.cos(), yaw.sin());
    let rot_pitch = |p: Vec3| Vec3::new(p.x, p.y * cp - p.z * sp, p.y * sp + p.z * cp);
    let rot_yaw = |p: Vec3| Vec3::new(p.x * cy + p.z * sy, p.y, -p.x * sy + p.z * cy);
    let xf = |local: Vec3| player_pos + rot_yaw(pivot + rot_pitch(offset + local));
    let xf_norm = |n: Vec3| rot_yaw(rot_pitch(n));

    let (hx, hy, hz) = (size.x * 0.5, size.y * 0.5, size.z * 0.5);
    let faces: [(Vec3, [Vec3; 4], [f32; 4]); 6] = [
        (Vec3::Y, [Vec3::new(-hx, hy, hz), Vec3::new(hx, hy, hz), Vec3::new(hx, hy, -hz), Vec3::new(-hx, hy, -hz)], uvs[0]),
        (-Vec3::Y, [Vec3::new(-hx, -hy, -hz), Vec3::new(hx, -hy, -hz), Vec3::new(hx, -hy, hz), Vec3::new(-hx, -hy, hz)], uvs[1]),
        (Vec3::Z, [Vec3::new(-hx, -hy, hz), Vec3::new(hx, -hy, hz), Vec3::new(hx, hy, hz), Vec3::new(-hx, hy, hz)], uvs[2]),
        (-Vec3::Z, [Vec3::new(hx, -hy, -hz), Vec3::new(-hx, -hy, -hz), Vec3::new(-hx, hy, -hz), Vec3::new(hx, hy, -hz)], uvs[3]),
        (Vec3::X, [Vec3::new(hx, -hy, hz), Vec3::new(hx, -hy, -hz), Vec3::new(hx, hy, -hz), Vec3::new(hx, hy, hz)], uvs[4]),
        (-Vec3::X, [Vec3::new(-hx, -hy, -hz), Vec3::new(-hx, -hy, hz), Vec3::new(-hx, hy, hz), Vec3::new(-hx, hy, -hz)], uvs[5]),
    ];

    for (norm, corners, uv) in faces {
        let s = v.len() as u32;
        let r_norm = xf_norm(norm);
        let uv_coords = [[uv[0], uv[3]], [uv[2], uv[3]], [uv[2], uv[1]], [uv[0], uv[1]]];
        for (i, p) in corners.iter().enumerate() {
            v.push(Vertex {
                position: xf(*p).to_array(),
                normal: r_norm.to_array(),
                uv: uv_coords[i],
                tex_layer: 6.0,
            });
        }
        idx.extend_from_slice(&[s, s + 1, s + 2, s, s + 2, s + 3]);
    }
}
