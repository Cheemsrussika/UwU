use std::path::PathBuf;
use super::chunk_save::{load_chunk_file, save_chunk_file};
use super::voxel::VoxelWorld;

pub fn get_world_save_dir() -> PathBuf {
    PathBuf::from("saves").join("world").join("chunks")
}

pub fn save_world_to_disk(world: &VoxelWorld) -> usize {
    let dir = get_world_save_dir();
    let _ = std::fs::create_dir_all(&dir);
    let mut count = 0;
    world.for_each_chunk(|&coords, chunk| {
        let filename = format!("c.{}.{}.{}.dat", coords.0, coords.1, coords.2);
        let path = dir.join(filename);
        if save_chunk_file(&path, coords, chunk).is_ok() {
            count += 1;
        }
    });
    count
}

pub fn save_chunk_at(world: &VoxelWorld, coords: (i32, i32, i32)) {
    let dir = get_world_save_dir();
    let filename = format!("c.{}.{}.{}.dat", coords.0, coords.1, coords.2);
    let path = dir.join(filename);
    world.storage.get_chunk_read(&coords, |chunk| {
        let _ = save_chunk_file(&path, coords, chunk);
    });
}

pub fn load_world_from_disk(world: &mut VoxelWorld) -> bool {
    let dir = get_world_save_dir();
    let entries = match std::fs::read_dir(&dir) {
        Ok(e) => e,
        Err(_) => return false,
    };

    let mut loaded = 0;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("dat") {
            if let Ok((coords, chunk)) = load_chunk_file(&path) {
                world.storage.insert(coords, chunk);
                loaded += 1;
            }
        }
    }

    if loaded > 0 {
        world.needs_mesh_rebuild = true;
        world.rebuild_all_dirty_chunks();
        true
    } else {
        false
    }
}
