#[cfg(test)]
mod tests {
    use crate::engine::TickSystem;
    use crate::world::chunk_save::{load_chunk_file, save_chunk_file};
    use crate::world::{schedule_camera_chunk_fluids, BlockType, Chunk, VoxelWorld};

    #[test]
    fn test_chunk_save_and_load_roundtrip() {
        let temp_dir = std::env::temp_dir().join("zomboid_test_save");
        let _ = std::fs::create_dir_all(&temp_dir);
        let path = temp_dir.join("c.1.0.2.dat");

        let mut chunk = Chunk::new((1, 0, 2));
        chunk.set_block(0, 1, 0, BlockType::Stone);
        chunk.set_block(5, 2, 7, BlockType::Grass);
        chunk.set_block(15, 3, 15, BlockType::WaterSource);

        assert!(save_chunk_file(&path, (1, 0, 2), &chunk).is_ok());
        let (coords, loaded) = load_chunk_file(&path).expect("Failed to load chunk");
        assert_eq!(coords, (1, 0, 2));
        assert_eq!(loaded.get_block(0, 1, 0), BlockType::Stone);
        assert_eq!(loaded.get_block(5, 2, 7), BlockType::Grass);
        assert_eq!(loaded.get_block(15, 3, 15), BlockType::WaterSource);
        assert_eq!(loaded.get_block(0, 0, 0), BlockType::Air);

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn test_fluid_camera_scheduling_and_border_dirty() {
        let mut world = VoxelWorld::new_flat(1);
        world.set_block(0, 5, 0, BlockType::WaterSource);

        let mut tick_system: TickSystem<BlockType> = TickSystem::new(20.0);
        schedule_camera_chunk_fluids(&world, &mut tick_system, (0, 0), 1);
        tick_system.advance(0.26);

        let ticks = tick_system.drain_current_ticks();
        assert!(ticks.iter().any(|t| t.pos == (0, 5, 0) && t.target == BlockType::WaterSource));

        // Border dirty test
        world.rebuild_all_dirty_chunks();
        world.set_block(0, 4, 0, BlockType::Air);
        let is_neighbor_dirty = world.storage.get_chunk_read(&(-1, 0, 0), |c| c.is_dirty).unwrap_or(false);
        assert!(is_neighbor_dirty);
    }
}
