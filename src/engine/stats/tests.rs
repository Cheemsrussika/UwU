#[cfg(test)]
mod tests {
    use crate::engine::stats::*;
    use crate::engine::items::ItemType;

    #[test]
    fn test_health_and_fall_damage() {
        let mut hp = Health::default();
        assert_eq!(hp.current, 20.0);
        assert_eq!(hp.max, 20.0);
        hp.damage(5.0);
        assert_eq!(hp.current, 15.0);
        hp.heal(10.0);
        assert_eq!(hp.current, 20.0);

        assert_eq!(calculate_fall_damage(2.5), 0.0);
        assert_eq!(calculate_fall_damage(3.0), 0.0);
        assert_eq!(calculate_fall_damage(5.0), 2.0);
        assert_eq!(calculate_fall_damage(8.2), 5.0);
    }

    #[test]
    fn test_hunger_and_saturation() {
        let mut hunger = Hunger::default();
        assert_eq!(hunger.food_level, 20.0);
        assert_eq!(hunger.saturation, 5.0);

        hunger.add_exhaustion(4.0);
        assert_eq!(hunger.saturation, 4.0);
        assert_eq!(hunger.food_level, 20.0);

        hunger.add_exhaustion(16.0);
        assert_eq!(hunger.saturation, 0.0);
        assert_eq!(hunger.food_level, 20.0);

        hunger.add_exhaustion(4.0);
        assert_eq!(hunger.food_level, 19.0);

        let eaten = hunger.eat(1.0, 2.0);
        assert!(eaten);
        assert_eq!(hunger.food_level, 20.0);
    }

    #[test]
    fn test_experience_java_leveling() {
        let mut xp = Experience::new();
        assert_eq!(xp.level, 0);
        assert_eq!(Experience::xp_needed_for_level(0), 7);
        assert_eq!(Experience::xp_needed_for_level(15), 37);
        assert_eq!(Experience::xp_needed_for_level(16), 42);
        assert_eq!(Experience::xp_needed_for_level(30), 112);
        assert_eq!(Experience::xp_needed_for_level(31), 121);

        xp.add_xp(5);
        assert_eq!(xp.level, 0);
        assert_eq!(xp.points, 5);
        assert!((xp.progress() - 5.0 / 7.0).abs() < 0.001);

        xp.add_xp(2);
        assert_eq!(xp.level, 1);
        assert_eq!(xp.points, 0);
    }

    #[test]
    fn test_food_values() {
        let (apple_food, apple_sat) = get_food_values(ItemType::Apple).unwrap();
        assert_eq!(apple_food, 4.0);
        assert_eq!(apple_sat, 2.4);

        let (steak_food, steak_sat) = get_food_values(ItemType::CookedBeef).unwrap();
        assert_eq!(steak_food, 8.0);
        assert_eq!(steak_sat, 12.8);
    }
}
