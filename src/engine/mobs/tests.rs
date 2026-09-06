#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use bevy::math::Vec3;
    use crate::engine::mobs::*;
    use crate::engine::items::ItemType;

    #[test]
    fn test_animal_drops() {
        let pig = Animal::new(AnimalType::Pig);
        let pig_drops = pig.drops();
        assert_eq!(pig_drops.len(), 1);
        assert_eq!(pig_drops[0].item, ItemType::Porkchop);

        let cow = Animal::new(AnimalType::Cow);
        let cow_drops = cow.drops();
        assert_eq!(cow_drops.len(), 2);
        assert_eq!(cow_drops[0].item, ItemType::Beef);
        assert_eq!(cow_drops[1].item, ItemType::Leather);

        let sheep = Animal::new(AnimalType::Sheep);
        let sheep_drops = sheep.drops();
        assert_eq!(sheep_drops.len(), 2);
        assert_eq!(sheep_drops[0].item, ItemType::Mutton);
        assert_eq!(sheep_drops[1].item, ItemType::WhiteWool);
    }

    #[test]
    fn test_animal_ai_look_at_player() {
        let mut ai = AnimalAi::default();
        let current_pos = Vec3::new(0.0, 0.0, 0.0);
        let player_pos = Vec3::new(2.0, 0.0, 2.0); // dist < 6.0
        let mut yaw = 0.0;

        // First tick should detect player and switch to LookAtPlayer
        let (vel, is_moving) = ai.update(current_pos, player_pos, &mut yaw, 0.1);
        assert_eq!(vel, Vec3::ZERO);
        assert!(!is_moving);

        // Next tick faces player
        ai.update(current_pos, player_pos, &mut yaw, 0.1);
        assert!((yaw - std::f32::consts::FRAC_PI_4).abs() < 0.01);
    }

    #[test]
    fn test_bevy_ecs_animal_spawn() {
        let mut world = World::new();
        let entity = spawn_animal(&mut world, Vec3::new(10.0, 64.0, 10.0), AnimalType::Pig);
        let animal = world.get::<Animal>(entity);
        assert!(animal.is_some());
        assert_eq!(animal.unwrap().animal_type, AnimalType::Pig);
    }
}
