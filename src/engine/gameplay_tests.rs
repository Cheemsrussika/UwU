#[cfg(test)]
mod tests {
    use bevy::math::Vec3;
    use crate::engine::{Inventory, ItemStack, ItemType, MiningState, Player};
    use crate::world::block::BlockType;
    use crate::world::VoxelWorld;

    #[test]
    fn test_inventory_interaction_pickup_and_split() {
        let mut inv = Inventory::new();
        // Clear hotbar and storage
        for s in &mut inv.hotbar { *s = None; }
        for s in &mut inv.storage { *s = None; }

        inv.storage[0] = Some(ItemStack::new(ItemType::Apple, 10));

        // Left click to pick up
        inv.click_slot(0, false, false);
        assert!(inv.storage[0].is_none());
        assert_eq!(inv.carried_item.as_ref().unwrap().count, 10);

        // Left click on empty slot 1
        inv.click_slot(1, false, false);
        assert_eq!(inv.storage[1].as_ref().unwrap().count, 10);
        assert!(inv.carried_item.is_none());

        // Right click to split stack in half (10 -> 5 in slot, 5 carried)
        inv.click_slot(1, true, false);
        assert_eq!(inv.storage[1].as_ref().unwrap().count, 5);
        assert_eq!(inv.carried_item.as_ref().unwrap().count, 5);

        // Shift click on slot 1 moves it to hotbar
        inv.click_slot(1, false, true);
        assert!(inv.storage[1].is_none());
        assert_eq!(inv.hotbar[0].as_ref().unwrap().count, 5);
    }

    #[test]
    fn test_survival_mining_speed() {
        let mut mining = MiningState::new();
        mining.set_mining(Some((10, 5, 10)), true);

        // Mining stone by hand (hardness 1.5, tool_speed 1.0, modifier 100)
        let dt = 0.05;
        let broke = mining.update(dt, BlockType::Stone, None);
        assert!(!broke);
        let progress_hand = mining.progress;

        // Mining stone with Diamond Pickaxe (tool_speed 8.0, modifier 30)
        let mut mining_pick = MiningState::new();
        mining_pick.set_mining(Some((10, 5, 10)), true);
        mining_pick.update(dt, BlockType::Stone, Some(ItemType::DiamondPickaxe));
        let progress_pick = mining_pick.progress;

        // Diamond Pickaxe must be over 10x faster than bare hands
        assert!(progress_pick > progress_hand * 10.0);

        // Verify pickaxe completes breaking block within 0.4s
        assert!(mining_pick.update(0.4, BlockType::Stone, Some(ItemType::DiamondPickaxe)));
    }

    #[test]
    fn test_crouch_mechanics_and_aim_priority() {
        let world = VoxelWorld::new_flat(1);
        let mut player = Player::new(0.0, 5.0, 0.0);

        // 1. Crouch height
        player.update_physics(Vec3::ZERO, false, true, false, false, None, 0.05, &world);
        assert!(player.is_sneaking);
        assert_eq!(player.height, 1.5);

        // 2. Aim yaw vs move input priority
        // Stationary with aim_dir -> body turns to aim
        let aim = 1.25f32;
        let aim_dir = Vec3::new(aim.sin(), 0.0, aim.cos());
        player.update_physics(Vec3::ZERO, false, false, false, false, Some(aim_dir), 0.5, &world);
        assert!((player.yaw - aim).abs() < 0.2);
        assert!((player.head_yaw - aim).abs() < 0.2);

        // Head looks down at a block below -> head_pitch becomes positive
        player.update_physics(Vec3::ZERO, false, false, false, false, Some(Vec3::new(0.0, -1.0, 0.0001)), 0.5, &world);
        assert!(player.head_pitch > 0.5, "head must pitch down when aiming at a block below");

        // Moving towards Vec3::X -> body AND head turn towards movement direction, not the mouse
        let move_dir = Vec3::new(1.0, 0.0, 0.0);
        let expected_yaw = move_dir.x.atan2(move_dir.z);
        for _ in 0..10 {
            player.update_physics(move_dir, false, false, false, false, Some(aim_dir), 0.1, &world);
        }
        assert_eq!(player.yaw, expected_yaw);
        assert!((player.head_yaw - expected_yaw).abs() < 0.2, "head must turn to movement direction when moving");
    }
}
