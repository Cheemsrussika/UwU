use crate::engine::items::ItemType;
use crate::world::block::BlockType;

use bevy::prelude::Component;

#[derive(Component, Default, Clone, Debug)]
pub struct MiningState {
    pub target: Option<(i32, i32, i32)>,
    pub progress: f32,
    pub is_active: bool,
    pub is_holding_left: bool,
}

impl MiningState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_mining(&mut self, target: Option<(i32, i32, i32)>, active: bool) {
        if self.target != target {
            self.target = target;
            self.progress = 0.0;
        }
        self.is_active = active && target.is_some();
        if !self.is_active {
            self.progress = 0.0;
        }
    }

    pub fn get_stage(&self) -> Option<usize> {
        if self.is_active && self.progress > 0.0 {
            Some(((self.progress * 10.0).floor() as usize).min(9))
        } else {
            None
        }
    }

    pub fn can_harvest(block: BlockType, held_item: Option<ItemType>) -> bool {
        super::mining_calc::can_harvest_block(block, held_item)
    }

    pub fn update(&mut self, dt: f32, block: BlockType, held_item: Option<ItemType>) -> bool {
        self.update_with_gamemode(dt, block, held_item, crate::engine::GameMode::Survival)
    }

    pub fn update_with_gamemode(
        &mut self, dt: f32, block: BlockType, held_item: Option<ItemType>, mode: crate::engine::GameMode,
    ) -> bool {
        if !self.is_active || !block.is_solid() {
            self.progress = 0.0;
            return false;
        }
        if mode.instant_break() {
            self.progress = 0.0;
            self.is_active = false;
            return true;
        }

        let hardness = super::mining_calc::get_block_hardness(block);
        let can_harvest = Self::can_harvest(block, held_item);
        let tool_speed = super::mining_calc::get_tool_speed(block, held_item);

        let modifier: f32 = if can_harvest { 30.0 } else { 100.0 };
        let progress_per_sec = (tool_speed / hardness / modifier) * 20.0;

        self.progress += progress_per_sec * dt;
        if self.progress >= 1.0 {
            self.progress = 0.0;
            self.is_active = false;
            true
        } else {
            false
        }
    }
}
