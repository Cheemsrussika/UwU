pub mod matcher;
pub mod recipe;
pub mod recipes_basic;
pub mod recipes_tools;
pub mod recipes_tools_adv;

pub use matcher::{consume_crafting_grid, get_all_recipes, match_crafting_grid};
pub use recipe::Recipe;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_tools;
