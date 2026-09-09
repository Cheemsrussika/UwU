use crate::engine::items::ItemStack;

pub fn handle_right_click(slot: &mut Option<ItemStack>, carried: &mut Option<ItemStack>) {
    match (slot.take(), carried.take()) {
        (Some(mut s), None) => {
            let half = s.count / 2;
            s.count -= half;
            *carried = if half > 0 { Some(ItemStack::new(s.item, half)) } else { None };
            *slot = Some(s);
        }
        (None, Some(mut c)) => {
            *slot = Some(ItemStack::new(c.item, 1));
            c.count -= 1;
            *carried = if c.count > 0 { Some(c) } else { None };
        }
        (Some(mut s), Some(mut c)) => {
            if s.item == c.item && s.count < s.item.max_stack_size() {
                s.count += 1; c.count -= 1;
                *slot = Some(s);
                *carried = if c.count > 0 { Some(c) } else { None };
            } else {
                *slot = Some(c); *carried = Some(s);
            }
        }
        _ => {}
    }
}

pub fn handle_left_click(slot: &mut Option<ItemStack>, carried: &mut Option<ItemStack>) {
    match (slot.take(), carried.take()) {
        (Some(s), None) => { *carried = Some(s); }
        (None, Some(c)) => { *slot = Some(c); }
        (Some(mut s), Some(mut c)) => {
            if s.item == c.item {
                let space = s.item.max_stack_size().saturating_sub(s.count);
                let take = c.count.min(space);
                s.count += take; c.count -= take;
                *slot = Some(s);
                *carried = if c.count > 0 { Some(c) } else { None };
            } else {
                *slot = Some(c); *carried = Some(s);
            }
        }
        _ => {}
    }
}
