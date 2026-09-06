#[cfg(test)]
mod tests {
    use crate::engine::crafting::*;
    use crate::engine::items::{ItemStack, ItemType};
    use crate::world::block::BlockType;

    #[test]
    fn test_torch_and_storage_blocks() {
        // 1 Coal + 1 Stick -> 4 Torches
        let torch_grid = [
            Some(ItemStack::new(ItemType::Coal, 1)), None,
            Some(ItemStack::new(ItemType::Stick, 1)), None,
        ];
        let res = match_crafting_grid(&torch_grid, 2, 2);
        assert_eq!(res, Some(ItemStack::new(ItemType::Block(BlockType::Torch), 4)));

        // 9 Iron Ingots -> 1 Iron Block
        let iron_grid = [Some(ItemStack::new(ItemType::IronIngot, 1)); 9];
        let res_block = match_crafting_grid(&iron_grid, 3, 3);
        assert_eq!(res_block, Some(ItemStack::new(ItemType::Block(BlockType::IronBlock), 1)));

        // 1 Iron Block -> 9 Iron Ingots
        let uncraft_grid = [Some(ItemStack::new(ItemType::Block(BlockType::IronBlock), 1))];
        let res_ingots = match_crafting_grid(&uncraft_grid, 1, 1);
        assert_eq!(res_ingots, Some(ItemStack::new(ItemType::IronIngot, 9)));
    }

    #[test]
    fn test_advanced_tools_crafting() {
        let iron = ItemType::IronIngot;
        let stick = ItemType::Stick;

        // Iron Shovel: 1 iron above 2 sticks
        let shovel_grid = [
            Some(ItemStack::new(iron, 1)),
            Some(ItemStack::new(stick, 1)),
            Some(ItemStack::new(stick, 1)),
        ];
        let res_shovel = match_crafting_grid(&shovel_grid, 1, 3);
        assert_eq!(res_shovel, Some(ItemStack::new(ItemType::IronShovel, 1)));

        // Iron Axe (Right orientation)
        let axe_grid = [
            Some(ItemStack::new(iron, 1)), Some(ItemStack::new(iron, 1)),
            Some(ItemStack::new(iron, 1)), Some(ItemStack::new(stick, 1)),
            None,                          Some(ItemStack::new(stick, 1)),
        ];
        let res_axe = match_crafting_grid(&axe_grid, 2, 3);
        assert_eq!(res_axe, Some(ItemStack::new(ItemType::IronAxe, 1)));
    }
}
