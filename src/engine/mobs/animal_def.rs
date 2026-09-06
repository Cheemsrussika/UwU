use bevy::prelude::Component;
use crate::engine::items::{ItemStack, ItemType};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnimalType {
    Pig,
    Cow,
    Sheep,
}

#[derive(Component, Clone, Debug)]
pub struct Animal {
    pub animal_type: AnimalType,
    pub health: f32,
    pub max_health: f32,
    pub yaw: f32,
    pub walk_time: f32,
    pub is_moving: bool,
}

impl Animal {
    pub fn new(animal_type: AnimalType) -> Self {
        let max_hp = match animal_type {
            AnimalType::Pig | AnimalType::Cow => 10.0,
            AnimalType::Sheep => 8.0,
        };
        Self {
            animal_type,
            health: max_hp,
            max_health: max_hp,
            yaw: 0.0,
            walk_time: 0.0,
            is_moving: false,
        }
    }

    pub fn drops(&self) -> Vec<ItemStack> {
        match self.animal_type {
            AnimalType::Pig => vec![ItemStack::new(ItemType::Porkchop, 1)],
            AnimalType::Cow => vec![ItemStack::new(ItemType::Beef, 2), ItemStack::new(ItemType::Leather, 1)],
            AnimalType::Sheep => vec![ItemStack::new(ItemType::Mutton, 1), ItemStack::new(ItemType::WhiteWool, 1)],
        }
    }
}
