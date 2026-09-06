use super::recipe::Recipe;
use crate::engine::items::{ItemStack, ItemType};
use crate::world::block::BlockType;

pub fn get_basic_recipes() -> Vec<Recipe> {
    let planks = ItemType::Block(BlockType::OakPlanks);
    let log = ItemType::Block(BlockType::OakLog);
    let cobble = ItemType::Block(BlockType::Cobblestone);
    let table = ItemType::Block(BlockType::CraftingTable);
    let furnace = ItemType::Block(BlockType::Furnace);
    let chest = ItemType::Block(BlockType::Chest);

    vec![
        // Log -> 4 Planks
        Recipe::shapeless(vec![log], ItemStack::new(planks, 4)),
        // 2 Planks vertical -> 4 Sticks
        Recipe::shaped(1, 2, vec![Some(planks), Some(planks)], ItemStack::new(ItemType::Stick, 4)),
        // 4 Planks 2x2 -> Crafting Table
        Recipe::shaped(2, 2, vec![Some(planks), Some(planks), Some(planks), Some(planks)], ItemStack::new(table, 1)),
        // 8 Cobblestone hollow 3x3 -> Furnace
        Recipe::shaped(3, 3, vec![
            Some(cobble), Some(cobble), Some(cobble),
            Some(cobble), None,         Some(cobble),
            Some(cobble), Some(cobble), Some(cobble),
        ], ItemStack::new(furnace, 1)),
        // 8 Planks hollow 3x3 -> Chest
        Recipe::shaped(3, 3, vec![
            Some(planks), Some(planks), Some(planks),
            Some(planks), None,         Some(planks),
            Some(planks), Some(planks), Some(planks),
        ], ItemStack::new(chest, 1)),
        // 3 IronIngot V-shape -> Bucket
        Recipe::shaped(3, 2, vec![
            Some(ItemType::IronIngot), None,                      Some(ItemType::IronIngot),
            None,                      Some(ItemType::IronIngot), None,
        ], ItemStack::new(ItemType::Bucket, 1)),
        // Wood -> 4 Planks
        Recipe::shapeless(vec![ItemType::Block(BlockType::Wood)], ItemStack::new(planks, 4)),
        // Coal + Stick -> 4 Torches
        Recipe::shaped(1, 2, vec![Some(ItemType::Coal), Some(ItemType::Stick)], ItemStack::new(ItemType::Block(BlockType::Torch), 4)),
        // 9 IronIngot -> IronBlock & uncrafting
        Recipe::shaped(3, 3, vec![Some(ItemType::IronIngot); 9], ItemStack::new(ItemType::Block(BlockType::IronBlock), 1)),
        Recipe::shapeless(vec![ItemType::Block(BlockType::IronBlock)], ItemStack::new(ItemType::IronIngot, 9)),
        // 9 Coal -> CoalBlock & uncrafting
        Recipe::shaped(3, 3, vec![Some(ItemType::Coal); 9], ItemStack::new(ItemType::Block(BlockType::CoalBlock), 1)),
        Recipe::shapeless(vec![ItemType::Block(BlockType::CoalBlock)], ItemStack::new(ItemType::Coal, 9)),
        // 9 Diamond -> DiamondBlock & uncrafting
        Recipe::shaped(3, 3, vec![Some(ItemType::Diamond); 9], ItemStack::new(ItemType::Block(BlockType::DiamondBlock), 1)),
        Recipe::shapeless(vec![ItemType::Block(BlockType::DiamondBlock)], ItemStack::new(ItemType::Diamond, 9)),
    ]
}
