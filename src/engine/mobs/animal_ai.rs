use bevy::prelude::Component;
use bevy::math::Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AiState {
    Idle { timer: f32 },
    Wander { timer: f32, target_yaw: f32 },
    LookAtPlayer { timer: f32 },
    Panic { timer: f32, target_yaw: f32 },
}

#[derive(Component, Clone, Debug)]
pub struct AnimalAi {
    pub state: AiState,
}

impl Default for AnimalAi {
    fn default() -> Self {
        Self { state: AiState::Idle { timer: 2.0 } }
    }
}

impl AnimalAi {
    pub fn update(
        &mut self,
        current_pos: Vec3,
        player_pos: Vec3,
        yaw: &mut f32,
        dt: f32,
    ) -> (Vec3, bool) {
        let to_player = player_pos - current_pos;
        let dist_sq = to_player.x * to_player.x + to_player.z * to_player.z;

        match self.state {
            AiState::Idle { mut timer } => {
                timer -= dt;
                if dist_sq < 36.0 {
                    self.state = AiState::LookAtPlayer { timer: 3.0 };
                } else if timer <= 0.0 {
                    let rand_yaw = ((current_pos.x * 12.9898 + current_pos.z * 78.233).sin() * 43758.5453).fract() * std::f32::consts::TAU;
                    self.state = AiState::Wander { timer: 3.0, target_yaw: rand_yaw };
                } else {
                    self.state = AiState::Idle { timer };
                }
                (Vec3::ZERO, false)
            }
            AiState::LookAtPlayer { mut timer } => {
                timer -= dt;
                let look_yaw = to_player.x.atan2(to_player.z);
                *yaw = look_yaw;
                if timer <= 0.0 || dist_sq >= 64.0 {
                    self.state = AiState::Idle { timer: 2.5 };
                } else {
                    self.state = AiState::LookAtPlayer { timer };
                }
                (Vec3::ZERO, false)
            }
            AiState::Wander { mut timer, target_yaw } => {
                timer -= dt;
                *yaw = target_yaw;
                let forward = Vec3::new(target_yaw.sin(), 0.0, target_yaw.cos()) * 1.5;
                if timer <= 0.0 {
                    self.state = AiState::Idle { timer: 3.0 };
                } else {
                    self.state = AiState::Wander { timer, target_yaw };
                }
                (forward, true)
            }
            AiState::Panic { mut timer, target_yaw } => {
                timer -= dt;
                *yaw = target_yaw;
                let forward = Vec3::new(target_yaw.sin(), 0.0, target_yaw.cos()) * 3.5;
                if timer <= 0.0 {
                    self.state = AiState::Idle { timer: 2.0 };
                } else {
                    self.state = AiState::Panic { timer, target_yaw };
                }
                (forward, true)
            }
        }
    }
}
