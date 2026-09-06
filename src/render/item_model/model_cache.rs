use std::sync::OnceLock;
use crate::engine::items::ItemType;
use super::extruder::{generate_extruded_item_model, ModelQuad};
use super::registry::get_item_texture_bytes;

pub fn get_item_model(item: ItemType) -> &'static [ModelQuad] {
    static APPLE: OnceLock<Vec<ModelQuad>> = OnceLock::new();
    static BREAD: OnceLock<Vec<ModelQuad>> = OnceLock::new();
    static BUCKET: OnceLock<Vec<ModelQuad>> = OnceLock::new();
    static WATER_BUCKET: OnceLock<Vec<ModelQuad>> = OnceLock::new();
    static SWORD: OnceLock<Vec<ModelQuad>> = OnceLock::new();
    static PICKAXE: OnceLock<Vec<ModelQuad>> = OnceLock::new();
    static EMPTY: Vec<ModelQuad> = Vec::new();

    let bake = |it: ItemType| {
        if let Some(bytes) = get_item_texture_bytes(it) {
            generate_extruded_item_model(bytes)
        } else {
            Vec::new()
        }
    };

    match item {
        ItemType::Apple => APPLE.get_or_init(|| bake(ItemType::Apple)),
        ItemType::Bread => BREAD.get_or_init(|| bake(ItemType::Bread)),
        ItemType::Bucket => BUCKET.get_or_init(|| bake(ItemType::Bucket)),
        ItemType::WaterBucket => WATER_BUCKET.get_or_init(|| bake(ItemType::WaterBucket)),
        ItemType::DiamondSword => SWORD.get_or_init(|| bake(ItemType::DiamondSword)),
        ItemType::DiamondPickaxe => PICKAXE.get_or_init(|| bake(ItemType::DiamondPickaxe)),
        _ => &EMPTY,
    }
}
