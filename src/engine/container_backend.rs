use crate::engine::blocks::{BlockEntityManager, CHEST_SLOTS};
use crate::engine::items::ItemStack;
use super::container::{ContainerKind, ContainerRef};

pub fn read_backend(be: &BlockEntityManager, c: &ContainerRef) -> Vec<Option<ItemStack>> {
    match c.kind {
        ContainerKind::Chest => be.chests.get(&c.pos)
            .map(|ch| ch.items.to_vec())
            .unwrap_or_else(|| vec![None; CHEST_SLOTS]),
        ContainerKind::CraftingTable => {
            let mut ws = vec![None; 10];
            if let Some(ct) = be.crafting_tables.get(&c.pos) {
                for i in 0..9 { ws[i] = ct.grid[i]; }
                ws[9] = ct.result;
            }
            ws
        }
        ContainerKind::Furnace => {
            let f = be.furnaces.get(&c.pos);
            vec![
                f.and_then(|f| f.input),
                f.and_then(|f| f.fuel),
                f.and_then(|f| f.output),
            ]
        }
    }
}

pub fn write_backend(be: &mut BlockEntityManager, c: &ContainerRef, ws: &[Option<ItemStack>]) {
    match c.kind {
        ContainerKind::Chest => {
            let entry = be.chests.entry(c.pos).or_default();
            for (i, s) in ws.iter().enumerate().take(CHEST_SLOTS) { entry.items[i] = *s; }
        }
        ContainerKind::CraftingTable => {
            let entry = be.crafting_tables.entry(c.pos).or_default();
            for i in 0..9 { entry.grid[i] = ws.get(i).copied().unwrap_or(None); }
            entry.update_result();
        }
        ContainerKind::Furnace => {
            let entry = be.furnaces.entry(c.pos).or_default();
            entry.input = ws.get(0).copied().flatten();
            entry.fuel = ws.get(1).copied().flatten();
            entry.output = ws.get(2).copied().flatten();
        }
    }
}
