use super::Inventory;
use crate::engine::items::ItemType;

pub type SlotArrays = (
    [Option<(ItemType, u32)>; 9],
    [Option<(ItemType, u32)>; 27],
    [Option<(ItemType, u32)>; 46],
);

pub fn extract_inventory_slots(inventory: &Inventory) -> SlotArrays {
    let mut hotbar_items = [None; 9];
    for i in 0..9 {
        if let Some(it) = &inventory.hotbar[i] { hotbar_items[i] = Some((it.item, it.count)); }
    }
    let mut storage_items = [None; 27];
    for i in 0..27 {
        if let Some(it) = &inventory.storage[i] { storage_items[i] = Some((it.item, it.count)); }
    }
    let mut all_slots = [None; 46];
    all_slots[0..27].copy_from_slice(&storage_items);
    all_slots[27..36].copy_from_slice(&hotbar_items);
    for i in 0..4 {
        if let Some(it) = &inventory.armor[i] { all_slots[36 + i] = Some((it.item, it.count)); }
    }
    if let Some(it) = &inventory.offhand { all_slots[40] = Some((it.item, it.count)); }
    for i in 0..4 {
        if let Some(it) = &inventory.craft[i] { all_slots[41 + i] = Some((it.item, it.count)); }
    }
    if let Some(it) = &inventory.result { all_slots[45] = Some((it.item, it.count)); }

    (hotbar_items, storage_items, all_slots)
}
