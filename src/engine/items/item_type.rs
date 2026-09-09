use crate::world::block::BlockType;

#[derive(bevy::prelude::Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ItemType {
    Block(BlockType),
    Apple,
    Bread,
    Porkchop,
    CookedPorkchop,
    Beef,
    CookedBeef,
    Mutton,
    CookedMutton,
    WhiteWool,
    Leather,
    Stick,
    Coal,
    IronIngot,
    Diamond,
    WoodenPickaxe,
    StonePickaxe,
    IronPickaxe,
    DiamondPickaxe,
    WoodenSword,
    StoneSword,
    IronSword,
    DiamondSword,
    WoodenAxe,
    StoneAxe,
    IronAxe,
    DiamondAxe,
    WoodenShovel,
    StoneShovel,
    IronShovel,
    DiamondShovel,
    WoodenHoe,
    StoneHoe,
    IronHoe,
    DiamondHoe,
    Bucket,
    WaterBucket,
}

impl ItemType {
    pub fn max_stack_size(&self) -> u32 {
        if self.is_tool() { 1 }
        else if matches!(self, ItemType::Bucket | ItemType::WaterBucket) { 16 }
        else { 64 }
    }

    pub fn name(&self) -> &'static str {
        super::item_props::item_name(self)
    }

    pub fn tex_layer(&self) -> f32 {
        super::item_props::item_tex_layer(self)
    }

    pub fn is_block(&self) -> bool {
        matches!(self, ItemType::Block(_))
    }

    pub fn is_tool(&self) -> bool {
        matches!(
            self,
            ItemType::WoodenPickaxe | ItemType::StonePickaxe | ItemType::IronPickaxe | ItemType::DiamondPickaxe
            | ItemType::WoodenSword | ItemType::StoneSword | ItemType::IronSword | ItemType::DiamondSword
            | ItemType::WoodenAxe | ItemType::StoneAxe | ItemType::IronAxe | ItemType::DiamondAxe
            | ItemType::WoodenShovel | ItemType::StoneShovel | ItemType::IronShovel | ItemType::DiamondShovel
            | ItemType::WoodenHoe | ItemType::StoneHoe | ItemType::IronHoe | ItemType::DiamondHoe
        )
    }
}

