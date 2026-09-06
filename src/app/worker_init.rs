use bevy::math::Vec3;
use crate::engine::{Camera, Inventory, ItemEntityManager, Player, TickSystem};
use crate::engine::mobs::{spawn_animal, AnimalType};
use crate::world::{BlockType, FluidSimulator, VoxelWorld};

pub fn init_logic_entities(world: &VoxelWorld) -> (Player, Camera, Inventory, ItemEntityManager, TickSystem<BlockType>) {
    let player = Player::new(0.0, 6.0, 0.0);
    let camera = Camera::new();
    let inventory = Inventory::new();
    let mut items = ItemEntityManager::new();
    let mut tick_system = TickSystem::new(20.0);
    FluidSimulator::schedule_initial_water(world, &mut tick_system);

    // Initial animals spawn nearby
    spawn_animal(&mut items.ecs_world, Vec3::new(6.0, 4.0, 6.0), AnimalType::Pig);
    spawn_animal(&mut items.ecs_world, Vec3::new(8.0, 4.0, 5.0), AnimalType::Pig);
    spawn_animal(&mut items.ecs_world, Vec3::new(-6.0, 4.0, 7.0), AnimalType::Cow);
    spawn_animal(&mut items.ecs_world, Vec3::new(-8.0, 4.0, 8.0), AnimalType::Cow);
    spawn_animal(&mut items.ecs_world, Vec3::new(5.0, 4.0, -8.0), AnimalType::Sheep);
    spawn_animal(&mut items.ecs_world, Vec3::new(7.0, 4.0, -7.0), AnimalType::Sheep);
    items.sync_items();

    (player, camera, inventory, items, tick_system)
}
