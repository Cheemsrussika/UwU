#[cfg(test)]
mod tests {
    use bevy::math::Vec3;
    use crate::engine::mobs::*;
    use crate::engine::Player;

    #[test]
    fn test_animal_mesh_generation_and_valid_uvs() {
        let pig = Animal::new(AnimalType::Pig);
        let mut v_pig = Vec::new();
        let mut i_pig = Vec::new();
        build_animal_mesh(&pig, Vec3::ZERO, &mut v_pig, &mut i_pig);

        assert!(!v_pig.is_empty());
        assert!(!i_pig.is_empty());
        assert_eq!(v_pig[0].tex_layer, 36.0);

        // Verify UVs are non-zero (not dummy 0.0 which causes alpha discard)
        let has_nonzero_uv = v_pig.iter().any(|vert| vert.uv[0] > 0.0 || vert.uv[1] > 0.0);
        assert!(has_nonzero_uv);

        // Verify Cow and Sheep meshes
        let cow = Animal::new(AnimalType::Cow);
        let mut v_cow = Vec::new();
        let mut i_cow = Vec::new();
        build_animal_mesh(&cow, Vec3::ZERO, &mut v_cow, &mut i_cow);
        assert_eq!(v_cow[0].tex_layer, 37.0);

        let sheep = Animal::new(AnimalType::Sheep);
        let mut v_sheep = Vec::new();
        let mut i_sheep = Vec::new();
        build_animal_mesh(&sheep, Vec3::ZERO, &mut v_sheep, &mut i_sheep);
        assert_eq!(v_sheep[0].tex_layer, 38.0);
    }

    #[test]
    fn test_player_animal_collision_push() {
        let mut player = Player::new(0.0, 0.0, 0.0);
        let pig = Animal::new(AnimalType::Pig);
        let mut animals = vec![(pig, Vec3::new(0.1, 0.0, 0.0))];

        resolve_player_animals_collision(&mut player, &mut animals);

        // Player must be pushed back along negative X
        assert!(player.position.x < 0.0);
        // Pig must be pushed along positive X
        assert!(animals[0].1.x > 0.1);
    }
}
