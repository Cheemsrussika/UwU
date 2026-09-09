use crate::engine::{Camera, MiningState, Player};
use crate::input::HudZone;
use crate::world::VoxelWorld;

pub fn handle_update_cursor(
    aspect: f32,
    mouse_pos: (f32, f32),
    screen_size: (f32, f32),
    aspect_out: &mut f32,
    mouse_ndc: &mut (f32, f32),
    inventory_open: bool,
    current_hovered: &mut Option<(i32, i32, i32)>,
    mining_state: &mut MiningState,
    camera: &Camera,
    player: &Player,
    world: &VoxelWorld,
    aim_yaw: &mut Option<f32>,
) {
    *aspect_out = aspect;
    *mouse_ndc = ((mouse_pos.0 / screen_size.0) * 2.0 - 1.0, 1.0 - (mouse_pos.1 / screen_size.1) * 2.0);
    if HudZone::is_point_in_hud(mouse_pos.0, mouse_pos.1, screen_size.0, screen_size.1, inventory_open) {
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
