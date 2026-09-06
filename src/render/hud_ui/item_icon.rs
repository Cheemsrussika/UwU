use super::atlas::{HudAtlas, SpriteRect};
use crate::engine::items::ItemType;
use crate::world::block::BlockType;

pub fn get_item_sprite(item: &ItemType) -> SpriteRect {
    match item {
        ItemType::Block(b) => match b {
            BlockType::Grass => HudAtlas::rect(0, 320, 64, 64),
            BlockType::Dirt => HudAtlas::rect(64, 320, 64, 64),
            BlockType::Wood => HudAtlas::rect(128, 320, 64, 64),
            BlockType::Stone => HudAtlas::rect(192, 320, 64, 64),
            BlockType::WaterSource | BlockType::FlowingWater { .. } => HudAtlas::rect(256, 320, 64, 64),
            BlockType::OakLog => HudAtlas::rect(320, 320, 64, 64),
            BlockType::OakPlanks => HudAtlas::rect(384, 320, 64, 64),
            BlockType::OakLeaves => HudAtlas::rect(448, 320, 64, 64),
            _ => HudAtlas::rect(0, 320, 64, 64),
        },
        ItemType::Apple => HudAtlas::rect(512, 320, 64, 64),
        ItemType::Bread => HudAtlas::rect(576, 320, 64, 64),
        ItemType::Bucket => HudAtlas::rect(640, 320, 64, 64),
        ItemType::WaterBucket => HudAtlas::rect(704, 320, 64, 64),
        ItemType::DiamondSword => HudAtlas::rect(768, 320, 64, 64),
        ItemType::DiamondPickaxe => HudAtlas::rect(832, 320, 64, 64),
    }
}
