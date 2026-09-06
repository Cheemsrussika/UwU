use super::block::BlockType;
use super::chunk::Chunk;
use super::water_slope::compute_water_corner_heights;
use crate::render::types::Vertex;
use bevy::math::Vec3;

pub fn compute_chunk_mesh<F>(chunk: &Chunk, mut get_world_block: F) -> (Vec<Vertex>, Vec<u32>)
where
    F: FnMut(i32, i32, i32) -> BlockType,
{
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let uvs = [[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]];

    let faces: [(Vec3, [Vec3; 4], (i32, i32, i32), [usize; 4]); 6] = [
        (Vec3::Y, [Vec3::new(0.0, 1.0, 1.0), Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, 1.0, 0.0), Vec3::new(0.0, 1.0, 0.0)], (0, 1, 0), [3, 2, 1, 0]),
        (-Vec3::Y, [Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 1.0), Vec3::new(0.0, 0.0, 1.0)], (0, -1, 0), [0, 1, 2, 3]),
        (Vec3::Z, [Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 1.0), Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.0, 1.0, 1.0)], (0, 0, 1), [3, 2, 2, 3]),
        (-Vec3::Z, [Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(1.0, 1.0, 0.0)], (0, 0, -1), [1, 0, 0, 1]),
        (Vec3::X, [Vec3::new(1.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 0.0), Vec3::new(1.0, 1.0, 1.0)], (1, 0, 0), [2, 1, 1, 2]),
        (-Vec3::X, [Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 1.0, 1.0), Vec3::new(0.0, 1.0, 0.0)], (-1, 0, 0), [0, 3, 3, 0]),
    ];

    let (bx, by, bz) = (chunk.coords.0 * 16, chunk.coords.1 * 16, chunk.coords.2 * 16);

    for lx in 0..16 {
        for ly in 0..16 {
            for lz in 0..16 {
                let block = chunk.get_block(lx, ly, lz);
                if block == BlockType::Air { continue; }
                let (gx, gy, gz) = (bx + lx as i32, by + ly as i32, bz + lz as i32);
                let base = Vec3::new(gx as f32, gy as f32, gz as f32);
                if super::custom_block_mesh::append_custom_block_mesh(block, base, &mut vertices, &mut indices) {
                    continue;
                }
                let is_water = block.is_fluid();
                let tex_layer = block.tex_layer();
                let ch = if is_water { compute_water_corner_heights(gx, gy, gz, &mut get_world_block) } else { [1.0; 4] };

                let (water_uvs, water_top_tex) = if is_water {
                    let (fx, fz) = super::water_flow_dir::compute_water_flow_vector(gx, gy, gz, &mut get_world_block);
                    super::water_flow_dir::compute_water_top_uvs(fx, fz)
                } else {
                    (uvs, tex_layer)
                };

                for (normal, corners, offset, c_map) in &faces {
                    let neighbor = get_world_block(gx + offset.0, gy + offset.1, gz + offset.2);
                    let draw = if is_water { !neighbor.is_fluid() } else { neighbor.is_transparent() };
                    if draw {
                        let face_tex = super::face_texture::compute_face_texture(block, *offset, is_water, water_top_tex, tex_layer);
                        let face_uvs = if is_water && offset.1 == 1 { water_uvs } else { uvs };
                        let s = vertices.len() as u32;
                        for (i, corner) in corners.iter().enumerate() {
                            let mut p = *corner;
                            if is_water && p.y > 0.0 { p.y = ch[c_map[i]]; }
                            vertices.push(Vertex {
                                position: (base + p).to_array(),
                                normal: normal.to_array(),
                                uv: face_uvs[i],
                                tex_layer: face_tex,
                            });
                        }
                        indices.extend_from_slice(&[s, s + 1, s + 2, s, s + 2, s + 3]);
                    }
                }
            }
        }
    }
    (vertices, indices)
}