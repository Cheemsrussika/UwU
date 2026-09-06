#[cfg(test)]
mod tests {
    use crate::engine::player_ctrl::def::Player;
    use crate::world::VoxelWorld;
    use glam::Vec3;

    #[test]
    fn test_sprinting_speed_boost_and_stamina() {
        let world = VoxelWorld::new_flat(1);
        let mut p_walk = Player::new(0.0, 5.0, 0.0);
        let mut p_sprint = Player::new(0.0, 5.0, 0.0);

        let dt = 0.1;
        let move_dir = Vec3::new(0.0, 0.0, 1.0);

        // Walking
        p_walk.update_physics(move_dir, false, false, false, false, None, dt, &world);
        // Sprinting
        p_sprint.update_physics(move_dir, false, false, true, false, None, dt, &world);

        // Java Parity: SPRINTING_SPEED_BOOST = 0.30 -> walk_speed is ~1.30x higher
        assert!(p_sprint.is_sprinting);
        assert!(!p_walk.is_sprinting);
        assert!(p_sprint.velocity.z > p_walk.velocity.z * 1.25);

        // Stamina drain during sprinting
        assert!(p_sprint.stamina < p_walk.stamina);
        let expected_drain = 12.0 * dt;
        assert!((100.0 - p_sprint.stamina - expected_drain).abs() < 0.1);
    }

    #[test]
    fn test_mining_arm_swing_animation() {
        let world = VoxelWorld::new_flat(1);
        let mut player = Player::new(0.0, 5.0, 0.0);
        assert_eq!(player.mining_swing, 0.0);

        // Advance physics with is_mining = true
        let dt = 0.1;
        player.update_physics(Vec3::ZERO, false, false, false, true, None, dt, &world);

        // Java Parity: 6-tick rate (3.33 / sec)
        assert!(player.mining_swing > 0.0);
        let expected_swing = 0.1 * 3.33;
        assert!((player.mining_swing - expected_swing).abs() < 0.05);

        // Generating mesh with arm swing should produce valid vertices
        let (v, i) = player.mesh();
        assert!(!v.is_empty());
        assert!(!i.is_empty());
    }
}
