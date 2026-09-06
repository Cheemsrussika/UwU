use crate::engine::items::ItemType;

pub fn get_item_texture_bytes(item: ItemType) -> Option<&'static [u8]> {
    match item {
        ItemType::Apple => Some(include_bytes!("../../../assets/textures/item/apple.png")),
        ItemType::Bread => Some(include_bytes!("../../../assets/textures/item/bread.png")),
        ItemType::Bucket => Some(include_bytes!("../../../assets/textures/item/bucket.png")),
        ItemType::WaterBucket => Some(include_bytes!("../../../assets/textures/item/water_bucket.png")),
        ItemType::DiamondSword => Some(include_bytes!("../../../assets/textures/item/diamond_sword.png")),
        ItemType::DiamondPickaxe => Some(include_bytes!("../../../assets/textures/item/diamond_pickaxe.png")),
        _ => None,
    }
}
