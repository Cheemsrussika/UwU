use bevy::prelude::Component;

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Default for Health {
    fn default() -> Self {
        Self { current: 20.0, max: 20.0 }
    }
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    pub fn damage(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }

    pub fn heal(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }
}

pub fn calculate_fall_damage(fall_dist: f32) -> f32 {
    if fall_dist > 3.0 {
        (fall_dist - 3.0).floor()
    } else {
        0.0
    }
}
