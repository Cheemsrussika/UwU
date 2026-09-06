use winit::event::MouseButton;
use crate::engine::{Camera, Inventory, MiningState, Player, TickPriority, TickSystem};
use crate::input::HudZone;
use crate::world::{BlockType, FluidSimulator, VoxelWorld};

pub fn handle_mouse_action(
    button: MouseButton,
    is_pressed: bool,
    aspect: f32,
    mouse_pos: (f32, f32),
    screen_size: (f32, f32),
    is_shift: bool,
    inventory: &mut Inventory,
    mining_state: &mut MiningState,
    world: &mut VoxelWorld,
    camera: &Camera,
    player: &Player,
    tick_system: &mut TickSystem<BlockType>,
    block_tx: &std::sync::mpsc::Sender<crate::network::Packet>,
) {
    let in_hud = HudZone::is_point_in_hud(mouse_pos.0, mouse_pos.1, screen_size.0, screen_size.1, inventory.is_open);
    if in_hud {
        mining_state.is_holding_left = false;
        mining_state.set_mining(None, false);
        if !is_pressed { return; }
        if inventory.is_open {
            if let Some(slot) = HudZone::get_clicked_inventory_slot(mouse_pos.0, mouse_pos.1, screen_size.0, screen_size.1) {
                inventory.click_slot(slot, button == MouseButton::Right, is_shift);
            }
        } else if button == MouseButton::Left {
            if let Some(slot) = HudZone::get_clicked_hotbar_slot(mouse_pos.0, mouse_pos.1, screen_size.0, screen_size.1) {
                inventory.select_slot(slot);
            }
        }
    } else {
        let (ro, rd) = camera.screen_to_ray(player.position, aspect, mouse_pos.0, mouse_pos.1, screen_size.0, screen_size.1);
        let hit = crate::world::raycast_world_precise(world, ro, rd, 150.0);
        match button {
            MouseButton::Left => {
                mining_state.is_holding_left = is_pressed;
                let target = hit.map(|(hit_b, _, _)| hit_b);
                mining_state.set_mining(target, is_pressed);
            }
            MouseButton::Right => {
                if is_pressed {
                    if let Some((_, place_p, _)) = hit {
                        if let Some(selected) = inventory.consume_selected() {
                            world.set_block(place_p.0, place_p.1, place_p.2, selected);
                            let _ = block_tx.send(crate::network::Packet::ClientboundBlockUpdate {
                                x: place_p.0, y: place_p.1, z: place_p.2,
                                block_type: crate::world::block_to_u8(&selected),
                            });
                            if selected.is_fluid() {
                                tick_system.schedule_tick(selected, place_p, FluidSimulator::WATER_TICK_DELAY, TickPriority::Normal);
                            }
                            FluidSimulator::schedule_neighbors(world, tick_system, place_p.0, place_p.1, place_p.2);
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
