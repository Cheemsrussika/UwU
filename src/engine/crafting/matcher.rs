use super::recipe::Recipe;
use super::recipes_basic::get_basic_recipes;
use super::recipes_tools::get_tool_recipes;
use super::recipes_tools_adv::get_advanced_tool_recipes;
use crate::engine::items::ItemStack;

pub fn get_all_recipes() -> Vec<Recipe> {
    let mut r = get_basic_recipes();
    r.extend(get_tool_recipes());
    r.extend(get_advanced_tool_recipes());
    r
}

pub fn match_crafting_grid(grid: &[Option<ItemStack>], width: usize, height: usize) -> Option<ItemStack> {
    let recipes = get_all_recipes();
    for r in &recipes {
        if r.matches(grid, width, height) {
            return Some(r.result);
        }
    }
    None
}

pub fn consume_crafting_grid(grid: &mut [Option<ItemStack>]) {
    for slot in grid.iter_mut() {
        if let Some(s) = slot {
            if s.count <= 1 {
                *slot = None;
            } else {
                s.count -= 1;
            }
        }
    }
}
