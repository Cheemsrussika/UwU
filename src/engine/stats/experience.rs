use bevy::prelude::Component;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Experience {
    pub level: u32,
    pub points: u32,
    pub total_xp: u32,
}

impl Experience {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn xp_needed_for_level(lvl: u32) -> u32 {
        if lvl < 16 {
            2 * lvl + 7
        } else if lvl < 31 {
            5 * lvl - 38
        } else {
            9 * lvl - 158
        }
    }

    pub fn add_xp(&mut self, mut amount: u32) {
        self.total_xp += amount;
        while amount > 0 {
            let needed = Self::xp_needed_for_level(self.level);
            let remaining = needed.saturating_sub(self.points);
            if amount >= remaining {
                amount -= remaining;
                self.level += 1;
                self.points = 0;
            } else {
                self.points += amount;
                amount = 0;
            }
        }
    }

    pub fn progress(&self) -> f32 {
        let needed = Self::xp_needed_for_level(self.level);
        if needed == 0 { 0.0 } else { (self.points as f32 / needed as f32).clamp(0.0, 1.0) }
    }
}
