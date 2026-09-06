use super::block::BlockType;

pub fn block_to_u8(b: &BlockType) -> u8 {
    match b {
        BlockType::Air => 0,
        BlockType::Dirt => 1,
        BlockType::Grass => 2,
        BlockType::Wood => 3,
        BlockType::Stone => 4,
        BlockType::WaterSource => 5,
        BlockType::FlowingWater { level } => 6 + level.min(&7).saturating_sub(1),
        BlockType::OakLog => 14,
        BlockType::OakPlanks => 15,
        BlockType::OakLeaves => 16,
    }
}

pub fn block_from_u8(val: u8) -> BlockType {
    match val {
        0 => BlockType::Air,
        1 => BlockType::Dirt,
        2 => BlockType::Grass,
        3 => BlockType::Wood,
        4 => BlockType::Stone,
        5 => BlockType::WaterSource,
        6..=13 => BlockType::FlowingWater { level: (val - 5).min(7) },
        14 => BlockType::OakLog,
        15 => BlockType::OakPlanks,
        16 => BlockType::OakLeaves,
        _ => BlockType::Dirt,
    }
}
