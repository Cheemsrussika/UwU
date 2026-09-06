use crate::engine::items::ItemType;
use crate::world::block::BlockType;

pub fn get_smelting_result(input: ItemType) -> Option<ItemType> {
    match input {
        ItemType::Block(BlockType::Cobblestone) => Some(ItemType::Block(BlockType::Stone)),
        ItemType::Block(BlockType::OakLog) => Some(ItemType::Coal),
        ItemType::Porkchop => Some(ItemType::CookedPorkchop),
        ItemType::Beef => Some(ItemType::CookedBeef),
        ItemType::Mutton => Some(ItemType::CookedMutton),
        _ => None,
    }
}
