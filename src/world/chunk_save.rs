use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Result, Write};
use std::path::Path;
use super::block::BlockType;
use super::block_conv::{block_from_u8, block_to_u8};
use super::chunk::Chunk;

const CHUNK_MAGIC: u32 = 0x4D43484B; // 'MCHK'

pub fn save_chunk_file(path: &Path, coords: (i32, i32, i32), chunk: &Chunk) -> Result<()> {
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let file = File::create(path)?;
    let mut w = BufWriter::new(file);

    w.write_all(&CHUNK_MAGIC.to_be_bytes())?;
    w.write_all(&coords.0.to_be_bytes())?;
    w.write_all(&coords.1.to_be_bytes())?;
    w.write_all(&coords.2.to_be_bytes())?;

    let mut non_air = Vec::new();
    for lx in 0..16 {
        for ly in 0..16 {
            for lz in 0..16 {
                let b = chunk.get_block(lx, ly, lz);
                if b != BlockType::Air {
                    non_air.push((lx as u8, ly as u8, lz as u8, block_to_u8(&b)));
                }
            }
        }
    }

    w.write_all(&(non_air.len() as u32).to_be_bytes())?;
    for (x, y, z, bt) in non_air {
        w.write_all(&[x, y, z, bt])?;
    }
    w.flush()
}

pub fn load_chunk_file(path: &Path) -> Result<((i32, i32, i32), Chunk)> {
    let file = File::open(path)?;
    let mut r = BufReader::new(file);

    let mut magic_buf = [0u8; 4];
    r.read_exact(&mut magic_buf)?;
    if u32::from_be_bytes(magic_buf) != CHUNK_MAGIC {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Bad chunk magic"));
    }

    let mut c_buf = [0u8; 12];
    r.read_exact(&mut c_buf)?;
    let cx = i32::from_be_bytes(c_buf[0..4].try_into().unwrap());
    let cy = i32::from_be_bytes(c_buf[4..8].try_into().unwrap());
    let cz = i32::from_be_bytes(c_buf[8..12].try_into().unwrap());

    let mut chunk = Chunk::new((cx, cy, cz));
    let mut count_buf = [0u8; 4];
    r.read_exact(&mut count_buf)?;
    let count = u32::from_be_bytes(count_buf) as usize;

    let mut entry = [0u8; 4];
    for _ in 0..count {
        r.read_exact(&mut entry)?;
        let (lx, ly, lz, bt) = (entry[0] as usize, entry[1] as usize, entry[2] as usize, entry[3]);
        chunk.set_block(lx, ly, lz, block_from_u8(bt));
    }
    Ok(((cx, cy, cz), chunk))
}
