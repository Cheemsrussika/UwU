use glam::Vec3;
use crate::render::types::Vertex;
use crate::world::VoxelWorld;

pub struct Player {
    pub position: Vec3,
    pub velocity: Vec3,
    pub yaw: f32,
    pub on_ground: bool,
    pub in_water: bool,
    pub width: f32,
    pub height: f32,
    pub health: f32,
    pub stamina: f32,
    pub hunger: f32,
    pub walk_time: f32,
    pub walk_speed: f32,
    pub is_sneaking: bool,
    pub is_sprinting: bool,
    pub mining_swing: f32,
    pub held_item: Option<crate::engine::items::ItemType>,
}

impl Player {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: Vec3::new(x, y, z), velocity: Vec3::ZERO, yaw: 0.0,
            on_ground: false, in_water: false, width: 0.35, height: 1.8,
            health: 100.0, stamina: 100.0, hunger: 90.0,
            walk_time: 0.0, walk_speed: 0.0, is_sneaking: false,
            is_sprinting: false, mining_swing: 0.0, held_item: None,
        }
    }

    pub fn update_physics(
        &mut self, move_input: Vec3, jump: bool, sneak: bool, sprint: bool,
        is_mining: bool, aim_yaw: Option<f32>, dt: f32, world: &VoxelWorld,
    ) {
        self.is_sneaking = sneak;
        self.height = if sneak { 1.5 } else { 1.8 };
        if is_mining {
            self.mining_swing = (self.mining_swing + dt * 3.33).rem_euclid(1.0);
        } else {
            self.mining_swing = 0.0;
        }
        super::physics::update_player_movement(self, move_input, jump, sprint, aim_yaw, dt, world);
    }

    pub fn mesh(&self) -> (Vec<Vertex>, Vec<u32>) {
        super::mesh::build_player_mesh(self)
    }
}

pub fn check_player_collision(pos: Vec3, width: f32, height: f32, world: &VoxelWorld) -> bool {
    let (min_x, max_x) = ((pos.x - width).floor() as i32, (pos.x + width - 0.0001).floor() as i32);
    let (min_y, max_y) = (pos.y.floor() as i32, (pos.y + height - 0.0001).floor() as i32);
    let (min_z, max_z) = ((pos.z - width).floor() as i32, (pos.z + width - 0.0001).floor() as i32);

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            for z in min_z..=max_z {
                if world.get_block(x, y, z).is_solid() { return true; }
            }
        }
    }
    false
}