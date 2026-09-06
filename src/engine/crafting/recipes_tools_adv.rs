use super::recipe::Recipe;
use crate::engine::items::{ItemStack, ItemType};

fn add_tool_set(mat: ItemType, axe: ItemType, shovel: ItemType, hoe: ItemType, r: &mut Vec<Recipe>) {
    let stick = ItemType::Stick;
    // Shovel 1x3
    r.push(Recipe::shaped(1, 3, vec![Some(mat), Some(stick), Some(stick)], ItemStack::new(shovel, 1)));

    // Axe 2x3 (Right & Left orientations)
    r.push(Recipe::shaped(2, 3, vec![
        Some(mat), Some(mat),
        Some(mat), Some(stick),
        None,      Some(stick),
    ], ItemStack::new(axe, 1)));
    r.push(Recipe::shaped(2, 3, vec![
        Some(mat),   Some(mat),
        Some(stick), Some(mat),
        Some(stick), None,
    ], ItemStack::new(axe, 1)));

    // Hoe 2x3 (Right & Left orientations)
    r.push(Recipe::shaped(2, 3, vec![
        Some(mat), Some(mat),
        None,      Some(stick),
        None,      Some(stick),
    ], ItemStack::new(hoe, 1)));
    r.push(Recipe::shaped(2, 3, vec![
        Some(mat),   Some(mat),
        Some(stick), None,
        Some(stick), None,
    ], ItemStack::new(hoe, 1)));
}

pub fn get_advanced_tool_recipes() -> Vec<Recipe> {
    let mut r = Vec::new();
    let wood = ItemType::Block(crate::world::block::BlockType::OakPlanks);
    let stone = ItemType::Block(crate::world::block::BlockType::Cobblestone);
    let iron = ItemType::IronIngot;
    let dia = ItemType::Diamond;

    add_tool_set(wood, ItemType::WoodenAxe, ItemType::WoodenShovel, ItemType::WoodenHoe, &mut r);
    add_tool_set(stone, ItemType::StoneAxe, ItemType::StoneShovel, ItemType::StoneHoe, &mut r);
    add_tool_set(iron, ItemType::IronAxe, ItemType::IronShovel, ItemType::IronHoe, &mut r);
    add_tool_set(dia, ItemType::DiamondAxe, ItemType::DiamondShovel, ItemType::DiamondHoe, &mut r);
    r
}
