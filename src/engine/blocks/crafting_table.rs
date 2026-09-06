use bevy::prelude::Component;
use crate::engine::items::ItemStack;
use crate::engine::crafting::{consume_crafting_grid, match_crafting_grid};

#[derive(Component, Clone, Debug, Default)]
pub struct CraftingTable {
    pub grid: [Option<ItemStack>; 9],
    pub result: Option<ItemStack>,
}

impl CraftingTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_result(&mut self) {
        self.result = match_crafting_grid(&self.grid, 3, 3);
    }

    pub fn take_result(&mut self) -> Option<ItemStack> {
        let res = self.result.take()?;
        consume_crafting_grid(&mut self.grid);
        self.update_result();
        Some(res)
    }

    pub fn set_slot(&mut self, idx: usize, stack: Option<ItemStack>) {
        if idx < 9 {
            self.grid[idx] = stack;
            self.update_result();
        }
    }
}
