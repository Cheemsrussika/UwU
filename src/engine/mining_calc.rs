use super::items::ItemType;
use crate::world::block::BlockType;

pub fn get_block_hardness(block: BlockType) -> f32 {
    match block {
        BlockType::Grass => 0.6,
        BlockType::Dirt => 0.5,
        BlockType::Stone => 1.5,
        BlockType::Wood | BlockType::OakLog | BlockType::OakPlanks => 2.0,
        BlockType::OakLeaves => 0.2,
        BlockType::Cobblestone => 2.0,
        BlockType::Furnace => 3.5,
        BlockType::CraftingTable | BlockType::Chest => 2.5,
        BlockType::Torch => 0.05,
        BlockType::IronBlock | BlockType::CoalBlock | BlockType::DiamondBlock => 5.0,
        _ => 1.0,
    }
}

pub fn can_harvest_block(block: BlockType, held: Option<ItemType>) -> bool {
    match block {
        BlockType::Stone | BlockType::Cobblestone | BlockType::Furnace
        | BlockType::IronBlock | BlockType::CoalBlock | BlockType::DiamondBlock => {
            matches!(
                held,
                Some(ItemType::WoodenPickaxe | ItemType::StonePickaxe | ItemType::IronPickaxe | ItemType::DiamondPickaxe)
            )
        }
        _ => true,
    }
}

pub fn get_tool_speed(block: BlockType, held: Option<ItemType>) -> f32 {
    let Some(item) = held else { return 1.0; };
    match block {
        BlockType::Stone | BlockType::Cobblestone | BlockType::Furnace
        | BlockType::IronBlock | BlockType::CoalBlock | BlockType::DiamondBlock => match item {
            ItemType::DiamondPickaxe => 8.0,
            ItemType::IronPickaxe => 6.0,
            ItemType::StonePickaxe => 4.0,
            ItemType::WoodenPickaxe => 2.0,
            _ => 1.0,
        },
        BlockType::Dirt | BlockType::Grass => match item {
            ItemType::DiamondShovel => 8.0,
            ItemType::IronShovel => 6.0,
            ItemType::StoneShovel => 4.0,
            ItemType::WoodenShovel => 2.0,
            _ => 1.0,
        },
        BlockType::Wood | BlockType::OakLog | BlockType::OakPlanks
        | BlockType::CraftingTable | BlockType::Chest => match item {
            ItemType::DiamondAxe => 8.0,
            ItemType::IronAxe => 6.0,
            ItemType::StoneAxe => 4.0,
            ItemType::WoodenAxe => 2.0,
            _ => 1.0,
        },
        BlockType::OakLeaves => match item {
            ItemType::DiamondSword | ItemType::IronSword | ItemType::StoneSword | ItemType::WoodenSword => 1.5,
            _ => 1.0,
        },
        _ => 1.0,
    }
}
