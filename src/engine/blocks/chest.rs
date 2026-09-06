use bevy::prelude::Component;
use crate::engine::items::ItemStack;

pub const CHEST_SLOTS: usize = 27;

#[derive(Component, Clone, Debug)]
pub struct Chest {
    pub items: [Option<ItemStack>; CHEST_SLOTS],
}

impl Default for Chest {
    fn default() -> Self {
        Self { items: [None; CHEST_SLOTS] }
    }
}

impl Chest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, idx: usize) -> Option<&ItemStack> {
        if idx < CHEST_SLOTS { self.items[idx].as_ref() } else { None }
    }

    pub fn put(&mut self, idx: usize, stack: Option<ItemStack>) -> Option<ItemStack> {
        if idx < CHEST_SLOTS {
            let old = self.items[idx].take();
            self.items[idx] = stack;
            old
        } else {
            stack
        }
    }

    pub fn insert_auto(&mut self, mut stack: ItemStack) -> Option<ItemStack> {
        for slot in &mut self.items {
            if let Some(s) = slot {
                if s.item == stack.item {
                    let space = s.item.max_stack_size().saturating_sub(s.count);
                    let take = stack.count.min(space);
                    s.count += take;
                    stack.count -= take;
                    if stack.count == 0 { return None; }
                }
            }
        }
        for slot in &mut self.items {
            if slot.is_none() {
                *slot = Some(stack);
                return None;
            }
        }
        Some(stack)
    }
}
