use bevy::prelude::Component;
use super::health::Health;

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Hunger {
    pub food_level: f32,
    pub saturation: f32,
    pub exhaustion: f32,
    pub tick_timer: f32,
}

impl Default for Hunger {
    fn default() -> Self {
        Self {
            food_level: 20.0,
            saturation: 5.0,
            exhaustion: 0.0,
            tick_timer: 0.0,
        }
    }
}

impl Hunger {
    pub fn add_exhaustion(&mut self, amount: f32) {
        self.exhaustion += amount;
        while self.exhaustion >= 4.0 {
            self.exhaustion -= 4.0;
            if self.saturation > 0.0 {
                self.saturation = (self.saturation - 1.0).max(0.0);
            } else {
                self.food_level = (self.food_level - 1.0).max(0.0);
            }
        }
    }

    pub fn eat(&mut self, food: f32, sat: f32) -> bool {
        if self.food_level >= 20.0 {
            return false;
        }
        self.food_level = (self.food_level + food).min(20.0);
        self.saturation = (self.saturation + sat).min(self.food_level);
        true
    }

    pub fn update_tick(&mut self, dt: f32, health: &mut Health, is_invuln: bool) {
        if is_invuln { return; }
        self.tick_timer += dt;
        if self.tick_timer >= 4.0 {
            self.tick_timer = 0.0;
            if self.food_level >= 18.0 && health.current < health.max {
                health.heal(1.0);
                self.add_exhaustion(6.0);
            } else if self.food_level <= 0.0 {
                health.damage(1.0);
            }
        }
    }
}
