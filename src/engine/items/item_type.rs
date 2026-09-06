use crate::world::block::BlockType;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemType {
    Block(BlockType),
    Apple,
    Bread,
    Bucket,
    WaterBucket,
    DiamondSword,
    DiamondPickaxe,
}

impl ItemType {
    pub fn max_stack_size(&self) -> u32 {
        match self {
            ItemType::DiamondSword | ItemType::DiamondPickaxe => 1,
            ItemType::Bucket | ItemType::WaterBucket => 16,
            _ => 64,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            ItemType::Block(b) => match b {
                BlockType::Grass => "Grass Block",
                BlockType::Dirt => "Dirt",
                BlockType::Wood => "Wood",
                BlockType::Stone => "Stone",
                BlockType::WaterSource => "Water Block",
                BlockType::OakLog => "Oak Log",
                BlockType::OakPlanks => "Oak Planks",
                BlockType::OakLeaves => "Oak Leaves",
                _ => "Block",
            },
            ItemType::Apple => "Apple",
            ItemType::Bread => "Bread",
            ItemType::Bucket => "Bucket",
            ItemType::WaterBucket => "Water Bucket",
            ItemType::DiamondSword => "Diamond Sword",
            ItemType::DiamondPickaxe => "Diamond Pickaxe",
        }
    }

    pub fn tex_layer(&self) -> f32 {
        match self {
            ItemType::Block(b) => b.tex_layer(),
            ItemType::Apple => 8.0,
            ItemType::Bread => 9.0,
            ItemType::Bucket => 10.0,
            ItemType::WaterBucket => 11.0,
            ItemType::DiamondSword => 12.0,
            ItemType::DiamondPickaxe => 13.0,
        }
    }

    pub fn is_block(&self) -> bool {
        matches!(self, ItemType::Block(_))
    }

    pub fn is_tool(&self) -> bool {
        matches!(self, ItemType::DiamondSword | ItemType::DiamondPickaxe)
    }
}
