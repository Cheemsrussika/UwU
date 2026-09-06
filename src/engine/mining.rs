use crate::engine::items::ItemType;
use crate::world::block::BlockType;

#[derive(Default, Clone, Debug)]
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
        match block {
            BlockType::Stone => matches!(held_item, Some(ItemType::DiamondPickaxe)),
            _ => true,
        }
    }

    pub fn update(&mut self, dt: f32, block: BlockType, held_item: Option<ItemType>) -> bool {
        if !self.is_active || !block.is_solid() {
            self.progress = 0.0;
            return false;
        }

        let hardness = match block {
            BlockType::Grass => 0.6,
            BlockType::Dirt => 0.5,
            BlockType::Stone => 1.5,
            BlockType::Wood | BlockType::OakLog | BlockType::OakPlanks => 2.0,
            BlockType::OakLeaves => 0.2,
            _ => 1.0,
        };

        let can_harvest = Self::can_harvest(block, held_item);
        let tool_speed = match (block, held_item) {
            (BlockType::Stone, Some(ItemType::DiamondPickaxe)) => 8.0,
            (BlockType::OakLeaves, Some(ItemType::DiamondSword)) => 1.5,
            _ => 1.0,
        };

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
