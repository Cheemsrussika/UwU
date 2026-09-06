use super::ItemType;
use crate::world::block::BlockType;

pub fn item_to_u8(it: &ItemType) -> u8 {
    match it {
        ItemType::Block(BlockType::Grass) => 0,
        ItemType::Block(BlockType::Dirt) => 1,
        ItemType::Block(BlockType::Wood) => 2,
        ItemType::Block(BlockType::Stone) => 3,
        ItemType::Apple => 4,
        ItemType::Bread => 5,
        ItemType::Bucket => 6,
        ItemType::WaterBucket => 7,
        ItemType::DiamondSword => 8,
        ItemType::DiamondPickaxe => 9,
        _ => 255,
    }
}

pub fn item_from_u8(v: u8) -> Option<ItemType> {
    match v {
        0 => Some(ItemType::Block(BlockType::Grass)),
        1 => Some(ItemType::Block(BlockType::Dirt)),
        2 => Some(ItemType::Block(BlockType::Wood)),
        3 => Some(ItemType::Block(BlockType::Stone)),
        4 => Some(ItemType::Apple),
        5 => Some(ItemType::Bread),
        6 => Some(ItemType::Bucket),
        7 => Some(ItemType::WaterBucket),
        8 => Some(ItemType::DiamondSword),
        9 => Some(ItemType::DiamondPickaxe),
        _ => None,
    }
}
