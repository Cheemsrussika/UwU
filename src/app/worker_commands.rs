use bevy::math::Vec3;
use crate::engine::{Camera, Inventory, Player, TickSystem};
use crate::input::{HudZone, LogicCommand};
use crate::world::{block_from_u8, apply_chunk_runs, BlockType, Chunk, VoxelWorld};

pub fn handle_worker_command(
    cmd: LogicCommand,
    world: &mut VoxelWorld,
    player: &Player,
    camera: &mut Camera,
    inventory: &mut Inventory,
    items: &mut crate::engine::ItemEntityManager,
    move_input: &mut Vec3,
    jump_input: &mut bool,
    sneak_input: &mut bool,
    sprint_input: &mut bool,
    aim_yaw: &mut Option<f32>,
    mouse_ndc: &mut (f32, f32),
    aspect_out: &mut f32,
    current_hovered: &mut Option<(i32, i32, i32)>,
    mining_state: &mut crate::engine::MiningState,
    tick_system: &mut TickSystem<BlockType>,
    show_chunk_borders: &mut bool,
    piechart_state: &mut crate::engine::ProfilerPieChartState,
    profiler: &crate::engine::Profiler,
    block_tx: &std::sync::mpsc::Sender<crate::network::Packet>,
) {
    match cmd {
        LogicCommand::TogglePieChart => piechart_state.toggle(),
        LogicCommand::ToggleChunkBorders => *show_chunk_borders = !*show_chunk_borders,
        LogicCommand::ProfilerNavigate(idx) => {
            let times = profiler.get_times(&piechart_state.current_path);
            piechart_state.handle_key_press(idx, &times);
        }
        LogicCommand::MoveInput(dir) => *move_input = dir,
        LogicCommand::Jump(j) => *jump_input = j,
        LogicCommand::Sneak(s) => *sneak_input = s,
        LogicCommand::Sprint(s) => *sprint_input = s,
        LogicCommand::RotateCamera(amt) => camera.rotation_angle += amt,
        LogicCommand::ZoomCamera(amt) => camera.distance = (camera.distance + amt).clamp(6.0, 80.0),
        LogicCommand::ToggleInventory => inventory.toggle_open(),
        LogicCommand::SelectSlot(slot) => inventory.select_slot(slot),
        LogicCommand::NextSlot => inventory.next_slot(),
        LogicCommand::PrevSlot => inventory.prev_slot(),
        LogicCommand::UpdateCursor { aspect, mouse_pos, screen_size } => {
            *aspect_out = aspect;
            *mouse_ndc = ((mouse_pos.0 / screen_size.0) * 2.0 - 1.0, 1.0 - (mouse_pos.1 / screen_size.1) * 2.0);
            if HudZone::is_point_in_hud(mouse_pos.0, mouse_pos.1, screen_size.0, screen_size.1, inventory.is_open) {
                *current_hovered = None;
                if mining_state.is_holding_left { mining_state.set_mining(None, false); }
            } else {
                let (ro, rd) = camera.screen_to_ray(player.position, aspect, mouse_pos.0, mouse_pos.1, screen_size.0, screen_size.1);
                let hit = crate::world::raycast_world_precise(world, ro, rd, 150.0);
                *current_hovered = hit.map(|(b, _, _)| b);
                *aim_yaw = Some(hit.map(|(b, _, _)| (b.0 as f32 + 0.5 - player.position.x).atan2(b.2 as f32 + 0.5 - player.position.z)).unwrap_or_else(|| rd.x.atan2(rd.z)));
                if mining_state.is_holding_left && mining_state.target != *current_hovered {
                    mining_state.set_mining(*current_hovered, current_hovered.is_some());
                }
            }
        }
        LogicCommand::MouseAction { button, is_pressed, aspect, mouse_pos, screen_size, is_shift } => {
            super::mouse_actions::handle_mouse_action(
                button, is_pressed, aspect, mouse_pos, screen_size, is_shift,
                inventory, mining_state, world, camera, player, tick_system, block_tx,
            );
        }
        LogicCommand::RemoteBlockChange { x, y, z, block_type } => {
            let bt = block_from_u8(block_type);
            world.set_block(x, y, z, bt);
        }
        LogicCommand::RemoteChunkData { x, y, z, runs } => {
            let coords = (x, y, z);
            let exists = world.storage.get_chunk_write(&coords, |chunk| {
                apply_chunk_runs(chunk, &runs);
                chunk.disk_dirty = false;
                chunk.is_dirty = true;
            }).is_some();
            if !exists {
                let mut chunk = Chunk::new(coords);
                apply_chunk_runs(&mut chunk, &runs);
                chunk.disk_dirty = false;
                chunk.is_dirty = true;
                world.storage.insert(coords, chunk);
            }
            world.needs_mesh_rebuild = true;
        }
        LogicCommand::RemoteSpawnItem { entity_id, pos, item_type, count } => {
            if let Some(it) = crate::engine::items::item_from_u8(item_type) {
                items.spawn_with_id(entity_id, pos, Vec3::ZERO, crate::engine::ItemStack::new(it, count));
            }
        }
        LogicCommand::RemoteRemoveEntities { entity_ids } => {
            items.remove_many_by_id(&entity_ids);
        }
    }
}