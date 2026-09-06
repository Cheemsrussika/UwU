pub mod block_entity_registry;
pub mod chest;
pub mod crafting_table;
pub mod furnace;
pub mod furnace_fuel;
pub mod furnace_recipe;

pub use block_entity_registry::BlockEntityManager;
pub use chest::{Chest, CHEST_SLOTS};
pub use crafting_table::CraftingTable;
pub use furnace::Furnace;
pub use furnace_fuel::get_fuel_burn_time;
pub use furnace_recipe::get_smelting_result;

#[cfg(test)]
mod tests;
