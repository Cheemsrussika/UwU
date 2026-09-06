use bevy::prelude::*;
use bevy::math::Vec3;
use super::animal_def::{Animal, AnimalType};
use super::animal_ai::AnimalAi;
use super::animal_physics::update_animal_physics;
use super::animal_mesh::build_animal_mesh;
use crate::engine::ecs::components::{Position, Velocity};
use crate::render::types::Vertex;
use crate::world::VoxelWorld;

pub fn spawn_animal(world: &mut World, pos: Vec3, animal_type: AnimalType) -> Entity {
    world.spawn((
        Animal::new(animal_type),
        AnimalAi::default(),
        Position(pos),
        Velocity(Vec3::ZERO),
    )).id()
}

pub fn tick_animals(
    ecs_world: &mut World,
    voxel_world: &VoxelWorld,
    player_pos: Vec3,
    dt: f32,
) {
    let mut query = ecs_world.query::<(
        &mut Animal,
        &mut AnimalAi,
        &mut Position,
        &mut Velocity,
    )>();

    for (mut animal, mut ai, mut pos, mut vel) in query.iter_mut(ecs_world) {
        let (desired_vel, is_moving) = ai.update(pos.0, player_pos, &mut animal.yaw, dt);
        animal.is_moving = is_moving;
        if is_moving {
            animal.walk_time += dt;
        } else {
            animal.walk_time = 0.0;
        }
        update_animal_physics(&mut pos.0, &mut vel.0, desired_vel, is_moving, dt, voxel_world);
    }
}

pub fn render_animals(
    ecs_world: &mut World,
    v: &mut Vec<Vertex>,
    idx: &mut Vec<u32>,
) {
    let mut query = ecs_world.query::<(&Animal, &Position)>();
    for (animal, pos) in query.iter(ecs_world) {
        build_animal_mesh(animal, pos.0, v, idx);
    }
}
