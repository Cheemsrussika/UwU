use bevy::math::Vec3;
use super::*;
use crate::engine::Inventory;
use crate::world::block::BlockType;
use crate::world::VoxelWorld;

#[test]
fn test_item_stack_limits() {
    let mut s1 = ItemStack::new(ItemType::Block(BlockType::Dirt), 60);
    assert_eq!(s1.item.max_stack_size(), 64);
    let rem = s1.add(10);
    assert_eq!(rem, 6);
    assert_eq!(s1.count, 64);

    let sword = ItemStack::new(ItemType::DiamondSword, 1);
    assert_eq!(sword.item.max_stack_size(), 1);

    let bucket = ItemStack::new(ItemType::Bucket, 1);
    assert_eq!(bucket.item.max_stack_size(), 16);
}

#[test]
fn test_dropped_item_physics_and_pickup() {
    let world = VoxelWorld::new_flat(1);
    let mut manager = ItemEntityManager::new();
    let mut inv = Inventory::new();

    for slot in &mut inv.hotbar { *slot = None; }

    let stack = ItemStack::new(ItemType::Apple, 5);
    manager.spawn(Vec3::new(0.0, 5.0, 0.0), Vec3::ZERO, stack);
    assert_eq!(manager.items.len(), 1);

    for _ in 0..60 {
        manager.update(0.05, &world, Vec3::ZERO);
    }
    assert!(manager.items[0].on_ground);
    let item_pos = manager.items[0].position;

    // Too far -> no pickup
    manager.try_pickup(Vec3::new(10.0, 5.0, 10.0), &mut inv);
    assert_eq!(manager.items.len(), 1);

    // Close -> picked up
    manager.try_pickup(item_pos, &mut inv);
    assert_eq!(manager.items.len(), 0);
    assert_eq!(inv.hotbar[0].as_ref().unwrap().item, ItemType::Apple);
    assert_eq!(inv.hotbar[0].as_ref().unwrap().count, 5);
}

#[test]
fn test_steve_player_mesh_generation() {
    let p = crate::engine::Player::new(0.0, 5.0, 0.0);
    let (verts, indices) = p.mesh();
    assert_eq!(verts.len(), 144);
    assert_eq!(indices.len(), 216);

    for v in &verts {
        assert_eq!(v.tex_layer, 6.0);
        let n = Vec3::from_array(v.normal);
        assert!((n.length() - 1.0).abs() < 0.01);
    }

    let mut p2 = crate::engine::Player::new(0.0, 5.0, 0.0);
    p2.held_item = Some(ItemType::Block(BlockType::Wood));
    let (vb, _) = p2.mesh();
    assert_eq!(vb.len(), 144 + 24);

    p2.held_item = Some(ItemType::DiamondSword);
    let (vs, _) = p2.mesh();
    assert_eq!(vs.len(), 1136); // 144 body + 992 crisp extruded tool vertices
}

#[test]
fn test_spawn_with_duplicate_id_ignored() {
    let mut manager = ItemEntityManager::new();
    let stack = ItemStack::new(ItemType::Apple, 1);
    manager.spawn_with_id(42, Vec3::new(1.0, 2.0, 3.0), Vec3::ZERO, stack.clone());
    assert_eq!(manager.items.len(), 1);

    // Attempting to spawn same ID again (network echo) must be ignored
    manager.spawn_with_id(42, Vec3::new(1.0, 2.0, 3.0), Vec3::ZERO, stack);
    assert_eq!(manager.items.len(), 1);
}
