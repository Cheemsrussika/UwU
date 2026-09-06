use bevy::math::Vec3;
use crate::render::types::Vertex;
use super::animal_box::add_animal_box;
use super::animal_def::{Animal, AnimalType};

pub fn build_animal_mesh(animal: &Animal, pos: Vec3, v: &mut Vec<Vertex>, idx: &mut Vec<u32>) {
    let leg_swing = if animal.is_moving { (animal.walk_time * 8.0).sin() * 0.15 } else { 0.0 };
    match animal.animal_type {
        AnimalType::Pig => render_pig(pos, leg_swing, v, idx),
        AnimalType::Cow => render_cow(pos, leg_swing, v, idx),
        AnimalType::Sheep => render_sheep(pos, leg_swing, v, idx),
    }
}

fn render_pig(p: Vec3, ls: f32, v: &mut Vec<Vertex>, idx: &mut Vec<u32>) {
    let t = 36.0;
    // Head (8x8x8) & Snout (4x3x1)
    add_animal_box(v, idx, p + Vec3::new(-0.25, 0.45, 0.35), Vec3::new(0.5, 0.5, 0.5), 0.0, 0.0, 8.0, 8.0, 8.0, t);
    add_animal_box(v, idx, p + Vec3::new(-0.125, 0.45, 0.60), Vec3::new(0.25, 0.1875, 0.0625), 16.0, 16.0, 4.0, 3.0, 1.0, t);
    // Body (10x16x8)
    add_animal_box(v, idx, p + Vec3::new(-0.3125, 0.35, -0.45), Vec3::new(0.625, 0.5, 0.9), 28.0, 8.0, 10.0, 8.0, 16.0, t);
    // 4 Legs (4x6x4)
    let leg_sz = Vec3::new(0.25, 0.375, 0.25);
    add_animal_box(v, idx, p + Vec3::new(0.0625, 0.0, 0.25 + ls), leg_sz, 0.0, 16.0, 4.0, 6.0, 4.0, t);
    add_animal_box(v, idx, p + Vec3::new(-0.3125, 0.0, 0.25 - ls), leg_sz, 0.0, 16.0, 4.0, 6.0, 4.0, t);
    add_animal_box(v, idx, p + Vec3::new(0.0625, 0.0, -0.35 - ls), leg_sz, 0.0, 16.0, 4.0, 6.0, 4.0, t);
    add_animal_box(v, idx, p + Vec3::new(-0.3125, 0.0, -0.35 + ls), leg_sz, 0.0, 16.0, 4.0, 6.0, 4.0, t);
}

fn render_cow(p: Vec3, ls: f32, v: &mut Vec<Vertex>, idx: &mut Vec<u32>) {
    let t = 37.0;
    // Head (8x8x6) & Horns (1x3x1)
    add_animal_box(v, idx, p + Vec3::new(-0.25, 0.7, 0.35), Vec3::new(0.5, 0.5, 0.375), 0.0, 0.0, 8.0, 8.0, 6.0, t);
    add_animal_box(v, idx, p + Vec3::new(-0.3125, 0.9, 0.40), Vec3::new(0.0625, 0.1875, 0.0625), 22.0, 0.0, 1.0, 3.0, 1.0, t);
    add_animal_box(v, idx, p + Vec3::new(0.25, 0.9, 0.40), Vec3::new(0.0625, 0.1875, 0.0625), 22.0, 0.0, 1.0, 3.0, 1.0, t);
    // Body (12x18x10) & Udder (4x6x1)
    add_animal_box(v, idx, p + Vec3::new(-0.375, 0.55, -0.55), Vec3::new(0.75, 0.625, 1.05), 18.0, 4.0, 12.0, 10.0, 18.0, t);
    add_animal_box(v, idx, p + Vec3::new(-0.125, 0.45, -0.35), Vec3::new(0.25, 0.1, 0.35), 52.0, 0.0, 4.0, 1.0, 6.0, t);
    // 4 Legs (4x12x4)
    let leg_sz = Vec3::new(0.25, 0.65, 0.25);
    add_animal_box(v, idx, p + Vec3::new(0.125, 0.0, 0.3 + ls), leg_sz, 0.0, 16.0, 4.0, 12.0, 4.0, t);
    add_animal_box(v, idx, p + Vec3::new(-0.375, 0.0, 0.3 - ls), leg_sz, 0.0, 16.0, 4.0, 12.0, 4.0, t);
    add_animal_box(v, idx, p + Vec3::new(0.125, 0.0, -0.45 - ls), leg_sz, 0.0, 16.0, 4.0, 12.0, 4.0, t);
    add_animal_box(v, idx, p + Vec3::new(-0.375, 0.0, -0.45 + ls), leg_sz, 0.0, 16.0, 4.0, 12.0, 4.0, t);
}

fn render_sheep(p: Vec3, ls: f32, v: &mut Vec<Vertex>, idx: &mut Vec<u32>) {
    let t = 38.0;
    // Head (6x6x8) & Body (8x6x16)
    add_animal_box(v, idx, p + Vec3::new(-0.1875, 0.65, 0.35), Vec3::new(0.375, 0.375, 0.5), 0.0, 0.0, 6.0, 6.0, 8.0, t);
    add_animal_box(v, idx, p + Vec3::new(-0.35, 0.55, -0.5), Vec3::new(0.7, 0.65, 1.0), 28.0, 8.0, 8.0, 6.0, 16.0, t);
    // 4 Legs (4x12x4)
    let leg_sz = Vec3::new(0.25, 0.65, 0.25);
    add_animal_box(v, idx, p + Vec3::new(0.1, 0.0, 0.25 + ls), leg_sz, 0.0, 16.0, 4.0, 12.0, 4.0, t);
    add_animal_box(v, idx, p + Vec3::new(-0.35, 0.0, 0.25 - ls), leg_sz, 0.0, 16.0, 4.0, 12.0, 4.0, t);
    add_animal_box(v, idx, p + Vec3::new(0.1, 0.0, -0.4 - ls), leg_sz, 0.0, 16.0, 4.0, 12.0, 4.0, t);
    add_animal_box(v, idx, p + Vec3::new(-0.35, 0.0, -0.4 + ls), leg_sz, 0.0, 16.0, 4.0, 12.0, 4.0, t);
}
