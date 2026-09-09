use bevy::math::Vec3;
use bevy::prelude::Component;
use crate::render::types::Vertex;
use crate::world::VoxelWorld;

pub use super::collision::check_player_collision;

#[derive(Component, Clone, Debug)]
pub struct Player {
    pub position: Vec3,
    pub velocity: Vec3,
    pub yaw: f32,
    pub head_yaw: f32,
    pub head_pitch: f32,
    pub on_ground: bool,
    pub in_water: bool,
    pub width: f32,
    pub height: f32,
    pub health: f32,
    pub stamina: f32,
    pub hunger: f32,
    pub saturation: f32,
    pub exhaustion: f32,
    pub hunger_timer: f32,
    pub xp_level: u32,
    pub xp_points: u32,
    pub xp_total: u32,
    pub fall_distance: f32,
    pub walk_time: f32,
    pub walk_speed: f32,
    pub is_sneaking: bool,
    pub is_sprinting: bool,
    pub mining_swing: f32,
    pub held_item: Option<crate::engine::items::ItemType>,
    pub game_mode: crate::engine::GameMode,
    pub is_flying: bool,
}

impl Player {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: Vec3::new(x, y, z), velocity: Vec3::ZERO, yaw: 0.0,
            head_yaw: 0.0, head_pitch: 0.0,
            on_ground: false, in_water: false, width: 0.35, height: 1.8,
            health: 20.0, stamina: 100.0, hunger: 20.0, saturation: 5.0,
            exhaustion: 0.0, hunger_timer: 0.0, xp_level: 0, xp_points: 0,
            xp_total: 0, fall_distance: 0.0, walk_time: 0.0, walk_speed: 0.0,
            is_sneaking: false, is_sprinting: false, mining_swing: 0.0,
            held_item: None, game_mode: crate::engine::GameMode::Survival,
            is_flying: false,
        }
    }

    pub fn update_physics(
        &mut self, move_input: Vec3, jump: bool, sneak: bool, sprint: bool,
        is_mining: bool, aim_dir: Option<Vec3>, dt: f32, world: &VoxelWorld,
    ) {
        self.is_sneaking = sneak;
        self.height = if sneak { 1.5 } else { 1.8 };
        if is_mining {
            self.mining_swing = (self.mining_swing + dt * 3.33).rem_euclid(1.0);
        } else {
            self.mining_swing = 0.0;
        }
        super::physics::update_player_movement(self, move_input, jump, sprint, aim_dir, dt, world);
        super::stats_update::tick_player_stats(self, dt);
    }

    pub fn eat(&mut self, item: crate::engine::items::ItemType) -> bool {
        super::stats_update::eat_player_food(self, item)
    }

    pub fn add_xp(&mut self, amount: u32) {
        super::stats_update::add_player_xp(self, amount);
    }

    pub fn xp_progress(&self) -> f32 {
        let needed = crate::engine::stats::Experience::xp_needed_for_level(self.xp_level);
        if needed == 0 { 0.0 } else { (self.xp_points as f32 / needed as f32).clamp(0.0, 1.0) }
    }

    pub fn mesh(&self) -> (Vec<Vertex>, Vec<u32>) {
        super::mesh::build_player_mesh(self)
    }
}