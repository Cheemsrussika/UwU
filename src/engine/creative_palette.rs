use crate::engine::items::ItemType;
use crate::world::block::BlockType;

pub fn get_creative_palette() -> Vec<ItemType> {
    vec![
        // Row 1: Building blocks
        ItemType::Block(BlockType::Grass),
        ItemType::Block(BlockType::Dirt),
        ItemType::Block(BlockType::Stone),
        ItemType::Block(BlockType::Cobblestone),
        ItemType::Block(BlockType::OakLog),
        ItemType::Block(BlockType::OakPlanks),
        ItemType::Block(BlockType::OakLeaves),
        ItemType::Block(BlockType::Wood),
        ItemType::WhiteWool,

        // Row 2: Functional & storage blocks
        ItemType::Block(BlockType::CraftingTable),
        ItemType::Block(BlockType::Furnace),
        ItemType::Block(BlockType::Chest),
        ItemType::Block(BlockType::Torch),
        ItemType::Block(BlockType::IronBlock),
        ItemType::Block(BlockType::CoalBlock),
        ItemType::Block(BlockType::DiamondBlock),
        ItemType::Bucket,
        ItemType::WaterBucket,

        // Row 3: Swords & Pickaxes
        ItemType::WoodenSword,
        ItemType::StoneSword,
        ItemType::IronSword,
        ItemType::DiamondSword,
        ItemType::WoodenPickaxe,
        ItemType::StonePickaxe,
        ItemType::IronPickaxe,
        ItemType::DiamondPickaxe,
        ItemType::Coal,

        // Row 4: Axes & Shovels
        ItemType::WoodenAxe,
        ItemType::StoneAxe,
        ItemType::IronAxe,
        ItemType::DiamondAxe,
        ItemType::WoodenShovel,
        ItemType::StoneShovel,
        ItemType::IronShovel,
        ItemType::DiamondShovel,
        ItemType::Stick,

        // Row 5: Hoes, minerals & food
        ItemType::WoodenHoe,
        ItemType::StoneHoe,
        ItemType::IronHoe,
        ItemType::DiamondHoe,
        ItemType::IronIngot,
        ItemType::Diamond,
        ItemType::Bread,
        ItemType::CookedBeef,
        ItemType::CookedPorkchop,
    ]
}
