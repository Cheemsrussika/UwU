use bevy::prelude::*;
use bevy::math::Vec3;
use crate::engine::items::ItemStack;

#[derive(Component, Debug, Clone)]
pub struct ItemEntityId(pub i32);

#[derive(Component, Debug, Clone)]
pub struct Position(pub Vec3);

#[derive(Component, Debug, Clone)]
pub struct Velocity(pub Vec3);

#[derive(Component, Debug, Clone)]
pub struct ItemPayload(pub ItemStack);

#[derive(Component, Debug, Clone)]
pub struct ItemAge(pub f32);

#[derive(Component, Debug, Clone)]
pub struct PickupDelay(pub f32);

#[derive(Component, Debug, Clone)]
pub struct Grounded(pub bool);

#[derive(Component, Debug, Clone)]
pub struct BobOffset(pub f32);
