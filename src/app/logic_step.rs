use glam::Vec3;
use crate::engine::{Inventory, ItemEntityManager, ItemStack, ItemType, MiningState, Player, TickSystem};
use crate::world::{BlockType, FluidSimulator, VoxelWorld};

pub fn step_physics_and_mining(
    player: &mut Player,
    world: &mut VoxelWorld,
    items: &mut ItemEntityManager,
    inventory: &mut Inventory,
    mining_state: &mut MiningState,
    tick_system: &mut TickSystem<BlockType>,
    move_input: Vec3,
    jump_input: bool,
    sneak_input: bool,
    sprint_input: bool,
    aim_yaw: Option<f32>,
    dt: f32,
    block_tx: &std::sync::mpsc::Sender<crate::network::Packet>,
) {
    let is_mining = mining_state.is_active;
    player.update_physics(move_input, jump_input, sneak_input, sprint_input, is_mining, aim_yaw, dt, world);
    items.update(dt, world);
    items.try_pickup(player.position, inventory);

    if let Some(target) = mining_state.target {
        let b = world.get_block(target.0, target.1, target.2);
        if mining_state.update(dt, b, player.held_item) {
            let drop_pos = Vec3::new(target.0 as f32 + 0.5, target.1 as f32 + 0.5, target.2 as f32 + 0.5);
            if MiningState::can_harvest(b, player.held_item) {
                let drop_block = match b { BlockType::Grass => BlockType::Dirt, other => other };
                items.spawn(drop_pos, Vec3::new(0.0, 2.5, 0.0), ItemStack::new(ItemType::Block(drop_block), 1));
            }
            world.remove_block(target.0, target.1, target.2);
            let _ = block_tx.send(crate::network::Packet::ClientboundBlockUpdate {
                x: target.0, y: target.1, z: target.2, block_type: 0,
            });
            crate::world::save_chunk_at(world, VoxelWorld::world_to_chunk(target.0, target.1, target.2).0);
            FluidSimulator::schedule_neighbors(world, tick_system, target.0, target.1, target.2);
            mining_state.target = None;
        }
    }
}
