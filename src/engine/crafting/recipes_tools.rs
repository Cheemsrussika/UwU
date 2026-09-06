use super::recipe::Recipe;
use crate::engine::items::{ItemStack, ItemType};
use crate::world::block::BlockType;

pub fn get_tool_recipes() -> Vec<Recipe> {
    let planks = ItemType::Block(BlockType::OakPlanks);
    let cobble = ItemType::Block(BlockType::Cobblestone);
    let iron = ItemType::IronIngot;
    let stick = ItemType::Stick;

    vec![
        // Wooden Pickaxe
        Recipe::shaped(3, 3, vec![
            Some(planks), Some(planks), Some(planks),
            None,         Some(stick),  None,
            None,         Some(stick),  None,
        ], ItemStack::new(ItemType::WoodenPickaxe, 1)),
        // Stone Pickaxe
        Recipe::shaped(3, 3, vec![
            Some(cobble), Some(cobble), Some(cobble),
            None,         Some(stick),  None,
            None,         Some(stick),  None,
        ], ItemStack::new(ItemType::StonePickaxe, 1)),
        // Iron Pickaxe
        Recipe::shaped(3, 3, vec![
            Some(iron), Some(iron),   Some(iron),
            None,       Some(stick),  None,
            None,       Some(stick),  None,
        ], ItemStack::new(ItemType::IronPickaxe, 1)),
        // Diamond Pickaxe
        Recipe::shaped(3, 3, vec![
            Some(ItemType::Diamond), Some(ItemType::Diamond), Some(ItemType::Diamond),
            None,                    Some(stick),             None,
            None,                    Some(stick),             None,
        ], ItemStack::new(ItemType::DiamondPickaxe, 1)),
        // Wooden Sword
        Recipe::shaped(1, 3, vec![Some(planks), Some(planks), Some(stick)], ItemStack::new(ItemType::WoodenSword, 1)),
        // Stone Sword
        Recipe::shaped(1, 3, vec![Some(cobble), Some(cobble), Some(stick)], ItemStack::new(ItemType::StoneSword, 1)),
        // Iron Sword
        Recipe::shaped(1, 3, vec![Some(iron), Some(iron), Some(stick)], ItemStack::new(ItemType::IronSword, 1)),
        // Diamond Sword
        Recipe::shaped(1, 3, vec![Some(ItemType::Diamond), Some(ItemType::Diamond), Some(stick)], ItemStack::new(ItemType::DiamondSword, 1)),
    ]
}
