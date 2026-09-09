use super::container::{ContainerKind, SlotTarget};

pub fn container_slot_positions(kind: ContainerKind) -> Vec<(f32, f32, SlotTarget)> {
    let mut out = Vec::new();
    match kind {
        ContainerKind::Chest => {
            for r in 0..3usize {
                for c in 0..9usize {
                    out.push((8.0 + c as f32 * 18.0, 18.0 + r as f32 * 18.0, SlotTarget::Backend(r * 9 + c)));
                }
            }
        }
        ContainerKind::CraftingTable => {
            for r in 0..3usize {
                for c in 0..3usize {
                    out.push((30.0 + c as f32 * 18.0, 18.0 + r as f32 * 18.0, SlotTarget::Backend(r * 3 + c)));
                }
            }
            out.push((124.0, 36.0, SlotTarget::Backend(9)));
        }
        ContainerKind::Furnace => {
            out.push((56.0, 34.0, SlotTarget::Backend(0)));
            out.push((56.0, 70.0, SlotTarget::Backend(1)));
            out.push((116.0, 54.0, SlotTarget::Backend(2)));
        }
    }
    for r in 0..3usize {
        for c in 0..9usize {
            out.push((8.0 + c as f32 * 18.0, 84.0 + r as f32 * 18.0, SlotTarget::Player(r * 9 + c)));
        }
    }
    for c in 0..9usize {
        out.push((8.0 + c as f32 * 18.0, 142.0, SlotTarget::Player(27 + c)));
    }
    out
}

pub fn get_clicked_container_slot(
    mx: f32, my: f32, sw: f32, sh: f32,
    kind: ContainerKind,
) -> Option<SlotTarget> {
    let aspect = sw / sh;
    let ndc_x = (mx / sw) * 2.0 - 1.0;
    let ndc_y = 1.0 - (my / sh) * 2.0;

    let s = 0.0065f32;
    let win_w = 176.0 * s / aspect;
    let win_x = -win_w * 0.5;
    let win_y = -166.0 * s * 0.5;

    for (gx, gy, target) in container_slot_positions(kind) {
        let sx = win_x + gx * s / aspect;
        let sy = win_y + (166.0 - gy - 18.0) * s;
        if ndc_x >= sx && ndc_x <= sx + 18.0 * s / aspect && ndc_y >= sy && ndc_y <= sy + 18.0 * s {
            return Some(target);
        }
    }
    None
}
