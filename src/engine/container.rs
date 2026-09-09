use crate::engine::blocks::CHEST_SLOTS;
use crate::engine::Inventory;
use crate::engine::items::{ItemStack, ItemType};

pub use super::container_backend::{read_backend, write_backend};
pub use super::container_click::apply_container_click;
pub use super::container_insert::quick_insert;
pub use super::container_slots::{container_slot_positions, get_clicked_container_slot};

#[derive(Clone, Debug)]
pub struct ContainerSnapshot {
    pub kind: ContainerKind,
    pub items: Vec<Option<(ItemType, u32)>>,
    pub prog_a: f32,
    pub prog_b: f32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ContainerKind { Chest, CraftingTable, Furnace }

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ContainerRef {
    pub kind: ContainerKind,
    pub pos: (i32, i32, i32),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SlotTarget {
    Backend(usize),
    Player(usize),
}

impl ContainerKind {
    pub fn backend_size(self) -> usize {
        match self {
            ContainerKind::Chest => CHEST_SLOTS,
            ContainerKind::CraftingTable => 10,
            ContainerKind::Furnace => 3,
        }
    }
}

pub fn container_kind_from_block(block: crate::world::BlockType) -> Option<ContainerKind> {
    use crate::world::BlockType;
    match block {
        BlockType::Chest => Some(ContainerKind::Chest),
        BlockType::CraftingTable => Some(ContainerKind::CraftingTable),
        BlockType::Furnace => Some(ContainerKind::Furnace),
        _ => None,
    }
}

pub fn player_slot_mut<'a>(inv: &'a mut Inventory, idx: usize) -> &'a mut Option<ItemStack> {
    if idx < 27 { &mut inv.storage[idx] } else if idx < 36 { &mut inv.hotbar[idx - 27] }
    else { &mut inv.hotbar[0] }
}
