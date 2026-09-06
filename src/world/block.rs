#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BlockType {
    Air,
    Dirt,
    Grass,
    Wood,
    Stone,
    WaterSource, // Level 8
    FlowingWater { level: u8 }, // Level 1..=7
    OakLog,
    OakPlanks,
    OakLeaves,
}

impl BlockType {
    pub fn is_solid(&self) -> bool {
        !matches!(self, BlockType::Air | BlockType::WaterSource | BlockType::FlowingWater { .. })
    }

    pub fn is_fluid(&self) -> bool {
        matches!(self, BlockType::WaterSource | BlockType::FlowingWater { .. })
    }

    pub fn is_transparent(&self) -> bool {
        matches!(self, BlockType::Air | BlockType::WaterSource | BlockType::FlowingWater { .. } | BlockType::OakLeaves)
    }

    pub fn tex_layer(&self) -> f32 {
        match self {
            BlockType::Grass => 0.0,
            BlockType::Dirt => 1.0,
            BlockType::Wood | BlockType::OakPlanks => 2.0,
            BlockType::Stone => 3.0,
            BlockType::WaterSource => 4.0,
            BlockType::FlowingWater { .. } => 5.0,
            BlockType::OakLog => 14.0,
            BlockType::OakLeaves => 17.0,
            BlockType::Air => -1.0,
        }
    }

    pub fn color(&self) -> [f32; 3] {
        match self {
            BlockType::Air => [0.0, 0.0, 0.0],
            BlockType::Grass => [0.28, 0.65, 0.22],
            BlockType::Dirt => [0.45, 0.28, 0.15],
            BlockType::Wood | BlockType::OakPlanks => [0.55, 0.38, 0.22],
            BlockType::Stone => [0.52, 0.52, 0.54],
            BlockType::WaterSource => [0.12, 0.35, 0.88],
            BlockType::FlowingWater { .. } => [0.20, 0.48, 0.90],
            BlockType::OakLog => [0.40, 0.28, 0.16],
            BlockType::OakLeaves => [0.22, 0.48, 0.16],
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            BlockType::Air => "Air",
            BlockType::Grass => "Grass",
            BlockType::Dirt => "Dirt",
            BlockType::Wood => "Wood",
            BlockType::Stone => "Stone",
            BlockType::WaterSource => "Water Source (L8)",
            BlockType::FlowingWater { level } => match level {
                7 => "Flowing Water L7", 6 => "Flowing Water L6",
                5 => "Flowing Water L5", 4 => "Flowing Water L4",
                3 => "Flowing Water L3", 2 => "Flowing Water L2",
                _ => "Flowing Water L1",
            },
            BlockType::OakLog => "Oak Log",
            BlockType::OakPlanks => "Oak Planks",
            BlockType::OakLeaves => "Oak Leaves",
        }
    }
}