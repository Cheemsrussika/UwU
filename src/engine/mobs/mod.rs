pub mod animal_ai;
pub mod animal_box;
pub mod animal_collision;
pub mod animal_def;
pub mod animal_mesh;
pub mod animal_physics;
pub mod manager;
pub mod spawner;

pub use animal_ai::{AiState, AnimalAi};
pub use animal_collision::resolve_player_animals_collision;
pub use animal_def::{Animal, AnimalType};
pub use animal_mesh::build_animal_mesh;
pub use animal_physics::update_animal_physics;
pub use manager::{render_animals, spawn_animal, tick_animals};
pub use spawner::maybe_spawn_animals;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_mesh;
