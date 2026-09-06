use crate::engine::items::ItemStack;
use crate::engine::Inventory;

pub fn handle_slot_click(inv: &mut Inventory, slot_idx: usize, is_right: bool, is_shift: bool) {
    if slot_idx >= 46 { return; }
    if slot_idx == 45 && inv.result.is_some() {
        handle_result_pickup(inv);
        return;
    }
    if is_shift { handle_shift_click(inv, slot_idx); }
    else if is_right { handle_right_click(inv, slot_idx); }
    else { handle_left_click(inv, slot_idx); }
    update_crafting(inv);
}

fn get_slot<'a>(inv: &'a mut Inventory, idx: usize) -> &'a mut Option<ItemStack> {
    if idx < 27 { &mut inv.storage[idx] } else if idx < 36 { &mut inv.hotbar[idx - 27] }
    else if idx < 40 { &mut inv.armor[idx - 36] } else if idx == 40 { &mut inv.offhand }
    else if idx < 45 { &mut inv.craft[idx - 41] } else { &mut inv.result }
}

fn handle_shift_click(inv: &mut Inventory, idx: usize) {
    let stack = match get_slot(inv, idx).take() { Some(s) => s, None => return };
    let target = if idx < 27 { &mut inv.hotbar[..] } else { &mut inv.storage[..] };
    for slot in target { if slot.is_none() { *slot = Some(stack); return; } }
    *get_slot(inv, idx) = Some(stack);
}

fn handle_left_click(inv: &mut Inventory, idx: usize) {
    let slot = get_slot(inv, idx);
    match (slot.take(), inv.carried_item.take()) {
        (Some(s), None) => inv.carried_item = Some(s),
        (None, Some(c)) => *get_slot(inv, idx) = Some(c),
        (Some(mut s), Some(mut c)) => {
            if s.item == c.item {
                let space = s.item.max_stack_size().saturating_sub(s.count);
                let take = c.count.min(space);
                s.count += take; c.count -= take;
                *get_slot(inv, idx) = Some(s);
                inv.carried_item = if c.count > 0 { Some(c) } else { None };
            } else {
                *get_slot(inv, idx) = Some(c); inv.carried_item = Some(s);
            }
        }
        _ => {}
    }
}

fn handle_right_click(inv: &mut Inventory, idx: usize) {
    let slot = get_slot(inv, idx);
    match (slot.take(), inv.carried_item.take()) {
        (Some(mut s), None) => {
            let half = s.count / 2; s.count -= half;
            inv.carried_item = if half > 0 { Some(ItemStack::new(s.item, half)) } else { None };
            *get_slot(inv, idx) = Some(s);
        }
        (None, Some(mut c)) => {
            *get_slot(inv, idx) = Some(ItemStack::new(c.item, 1));
            c.count -= 1;
            inv.carried_item = if c.count > 0 { Some(c) } else { None };
        }
        (Some(mut s), Some(mut c)) => {
            if s.item == c.item && s.count < s.item.max_stack_size() {
                s.count += 1; c.count -= 1;
                *get_slot(inv, idx) = Some(s);
                inv.carried_item = if c.count > 0 { Some(c) } else { None };
            } else {
                *get_slot(inv, idx) = Some(s); inv.carried_item = Some(c);
            }
        }
        _ => {}
    }
}

fn handle_result_pickup(inv: &mut Inventory) {
    if let Some(res) = inv.result.take() {
        if inv.carried_item.is_none() {
            inv.carried_item = Some(res);
            crate::engine::crafting::consume_crafting_grid(&mut inv.craft);
        } else {
            inv.result = Some(res);
        }
    }
}

fn update_crafting(inv: &mut Inventory) {
    inv.result = crate::engine::crafting::match_crafting_grid(&inv.craft, 2, 2);
}
