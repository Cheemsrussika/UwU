use crate::engine::items::ItemType;

pub fn get_food_values(item: ItemType) -> Option<(f32, f32)> {
    match item {
        ItemType::Apple => Some((4.0, 2.4)),
        ItemType::Bread => Some((5.0, 6.0)),
        ItemType::Porkchop => Some((3.0, 1.8)),
        ItemType::CookedPorkchop => Some((8.0, 12.8)),
        ItemType::Beef => Some((3.0, 1.8)),
        ItemType::CookedBeef => Some((8.0, 12.8)),
        ItemType::Mutton => Some((2.0, 1.2)),
        ItemType::CookedMutton => Some((6.0, 9.6)),
        _ => None,
    }
}
