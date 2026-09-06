pub mod experience;
pub mod food_data;
pub mod health;
pub mod hunger;

pub use experience::Experience;
pub use food_data::get_food_values;
pub use health::{calculate_fall_damage, Health};
pub use hunger::Hunger;

#[cfg(test)]
mod tests;
