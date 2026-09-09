use super::atlas::HudAtlas;
use super::bars::{add_hud_quad, add_textured_quad};
use super::renderer::HudVertex;
use super::slot_render::draw_slot_item_and_count;
use crate::engine::container::{ContainerSnapshot, SlotTarget, container_slot_positions};
use crate::engine::items::ItemType;

pub fn draw_container_window(
    v: &mut Vec<HudVertex>,
    i: &mut Vec<u32>,
    snapshot: &ContainerSnapshot,
    player_all_slots: &[Option<(ItemType, u32)>; 46],
    carried: Option<(ItemType, u32)>,
    mouse_ndc: (f32, f32),
    aspect: f32,
) {
    let white = [1.0, 1.0, 1.0, 1.0];
    let s = 0.0065f32;
    let (win_w, win_h) = (176.0 * s / aspect, 166.0 * s);
    let (win_x, win_y) = (-win_w * 0.5, -win_h * 0.5);

    // Background (reuse inventory bg; its top area will be masked)
    add_textured_quad(v, i, win_x, win_y, win_w, win_h, HudAtlas::INVENTORY_BG, white);

    // Mask the top region with a dark quad (covers armor/craft area from INVENTORY_BG)
    let mask_h = 84.0 * s;
    add_hud_quad(v, i, win_x, win_y + win_h - mask_h, win_w, mask_h, [0.12, 0.12, 0.13, 0.95]);

    let (icon_w, icon_h) = (16.0 * s / aspect, 16.0 * s);

    // Draw backend slots
    for (gx, gy, target) in container_slot_positions(snapshot.kind) {
        if let SlotTarget::Backend(b) = target {
            // Slot background
            let sx = win_x + (gx + 1.0) * s / aspect;
            let sy = win_y + (166.0 - gy - 18.0 + 1.0) * s;
            add_textured_quad(v, i, sx, sy, icon_w, icon_h, HudAtlas::SLOT, white);
            // Item icon + count
            if let Some((item, count)) = snapshot.items.get(b).and_then(|s| *s) {
                draw_slot_item_and_count(v, i, sx, sy, icon_w, icon_h, item, count, aspect);
            }
        }
    }

    // Draw player storage slots (0..27) + hotbar (27..36)
    for idx in 0..36 {
        if let Some((item, count)) = player_all_slots[idx] {
            if let Some((gx, gy)) = super::slot_coords::get_slot_pos(idx) {
                let sx = win_x + (gx + 1.0) * s / aspect;
                let sy = win_y + (166.0 - gy - 18.0 + 1.0) * s;
                draw_slot_item_and_count(v, i, sx, sy, icon_w, icon_h, item, count, aspect);
            }
        }
    }

    // Furnace progress bars
    if snapshot.kind == crate::engine::container::ContainerKind::Furnace {
        let bar_x = win_x + 72.0 * s / aspect;
        let bar_w = 14.0 * s / aspect;
        let bar_h = 14.0 * s;
        // Flame background (burn progress)
        let flame_y = win_y + (166.0 - 50.0 - 14.0) * s;
        add_hud_quad(v, i, bar_x, flame_y, bar_w, bar_h, [0.15, 0.15, 0.15, 0.8]);
        let flame_h = bar_h * snapshot.prog_a;
        add_hud_quad(v, i, bar_x, flame_y + bar_h - flame_h, bar_w, flame_h, [0.95, 0.65, 0.0, 0.9]);
        // Arrow background (cook progress)
        let arrow_x = win_x + 82.0 * s / aspect;
        let arrow_w = 22.0 * s / aspect;
        let arrow_h = 16.0 * s;
        let arrow_y = win_y + (166.0 - 56.0 - 16.0) * s;
        add_hud_quad(v, i, arrow_x, arrow_y, arrow_w, arrow_h, [0.15, 0.15, 0.15, 0.8]);
        let cook_w = arrow_w * snapshot.prog_b;
        add_hud_quad(v, i, arrow_x, arrow_y, cook_w, arrow_h, [0.7, 0.7, 0.7, 0.9]);
    }

    // Carried item at mouse
    if let Some((item, count)) = carried {
        draw_slot_item_and_count(v, i, mouse_ndc.0 - icon_w * 0.5, mouse_ndc.1 - icon_h * 0.5, icon_w, icon_h, item, count, aspect);
    }
}
