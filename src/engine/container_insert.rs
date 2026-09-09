use crate::engine::blocks::BlockEntityManager;
use crate::engine::items::ItemStack;
use super::container::{ContainerKind, ContainerRef};

pub fn quick_insert(be: &mut BlockEntityManager, c: &ContainerRef, stack: ItemStack) -> Option<ItemStack> {
    match c.kind {
        ContainerKind::Chest => insert_into_chest(be, c.pos, stack),
        ContainerKind::CraftingTable => {
            let entry = be.crafting_tables.entry(c.pos).or_default();
            insert_into_grid(&mut entry.grid, stack)
        }
        ContainerKind::Furnace => {
            let mut entry = be.furnaces.entry(c.pos).or_default();
            insert_into_furnace(&mut entry, stack)
        }
    }
}

fn insert_into_chest(be: &mut BlockEntityManager, pos: (i32, i32, i32), stack: ItemStack) -> Option<ItemStack> {
    be.chests.entry(pos).or_default().insert_auto(stack)
}

fn insert_into_grid(grid: &mut [Option<ItemStack>; 9], stack: ItemStack) -> Option<ItemStack> {
    let mut rem = stack;
    for slot in grid.iter_mut() {
        if let Some(s) = slot.as_mut() {
            if s.item == rem.item {
                let before = rem.count;
                rem.count = s.add(rem.count);
                if rem.count == 0 { return None; }
                if rem.count != before { continue; }
            }
        }
    }
    for slot in grid.iter_mut() {
        if slot.is_none() {
            *slot = Some(rem);
            return None;
        }
    }
    Some(rem)
}

fn insert_into_furnace(furnace: &mut crate::engine::blocks::Furnace, stack: ItemStack) -> Option<ItemStack> {
    if furnace.input.is_none() || furnace.input.as_ref().map_or(false, |s| s.item == stack.item && s.count < s.item.max_stack_size()) {
        if let Some(s) = furnace.input.as_mut() {
            let _ = s.add(stack.count);
            return None;
        } else {
            furnace.input = Some(stack);
            return None;
        }
    }
    if furnace.fuel.is_none() || furnace.fuel.as_ref().map_or(false, |s| s.item == stack.item && s.count < s.item.max_stack_size()) {
        if let Some(s) = furnace.fuel.as_mut() {
            let _ = s.add(stack.count);
            return None;
        } else {
            furnace.fuel = Some(stack);
            return None;
        }
    }
    Some(stack)
}
