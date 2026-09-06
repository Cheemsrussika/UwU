use crate::engine::items::ItemType;
use crate::world::block::BlockType;

pub fn get_fuel_burn_time(item: ItemType) -> Option<f32> {
    match item {
        ItemType::Coal => Some(80.0),
        ItemType::Block(BlockType::OakLog) => Some(15.0),
        ItemType::Block(BlockType::OakPlanks) | ItemType::Block(BlockType::Wood) => Some(15.0),
        ItemType::Stick => Some(5.0),
        ItemType::WoodenPickaxe | ItemType::WoodenSword => Some(10.0),
        _ => None,
    }
}
