use super::item_type::ItemType;

#[derive(bevy::prelude::Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ItemStack {
    pub item: ItemType,
    pub count: u32,
}

impl ItemStack {
    pub fn new(item: ItemType, count: u32) -> Self {
        Self { item, count }
    }

    pub fn can_stack_with(&self, other: &ItemStack) -> bool {
        self.item == other.item && self.count < self.item.max_stack_size()
    }

    pub fn add(&mut self, amount: u32) -> u32 {
        let max = self.item.max_stack_size();
        let space = max.saturating_sub(self.count);
        let to_add = amount.min(space);
        self.count += to_add;
        amount - to_add
    }
}
