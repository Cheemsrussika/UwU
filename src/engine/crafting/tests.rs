#[cfg(test)]
mod tests {
    use crate::engine::crafting::*;
    use crate::engine::items::{ItemStack, ItemType};
    use crate::world::block::BlockType;

    #[test]
    fn test_planks_and_sticks_crafting() {
        let log = ItemType::Block(BlockType::OakLog);
        let planks = ItemType::Block(BlockType::OakPlanks);

        // 2x2 grid with 1 log in slot 0
        let grid2x2 = [Some(ItemStack::new(log, 1)), None, None, None];
        let res = match_crafting_grid(&grid2x2, 2, 2);
        assert_eq!(res, Some(ItemStack::new(planks, 4)));

        // 2x2 grid with 2 planks vertical (slot 0 and slot 2)
        let stick_grid = [
            Some(ItemStack::new(planks, 1)), None,
            Some(ItemStack::new(planks, 1)), None,
        ];
        let res_stick = match_crafting_grid(&stick_grid, 2, 2);
        assert_eq!(res_stick, Some(ItemStack::new(ItemType::Stick, 4)));
    }

    #[test]
    fn test_crafting_table_and_furnace() {
        let planks = ItemType::Block(BlockType::OakPlanks);
        let cobble = ItemType::Block(BlockType::Cobblestone);

        // 4 planks -> crafting table
        let table_grid = [
            Some(ItemStack::new(planks, 1)), Some(ItemStack::new(planks, 1)),
            Some(ItemStack::new(planks, 1)), Some(ItemStack::new(planks, 1)),
        ];
        let res = match_crafting_grid(&table_grid, 2, 2);
        assert_eq!(res, Some(ItemStack::new(ItemType::Block(BlockType::CraftingTable), 1)));

        // 8 cobble -> furnace (3x3)
        let furnace_grid = [
            Some(ItemStack::new(cobble, 1)), Some(ItemStack::new(cobble, 1)), Some(ItemStack::new(cobble, 1)),
            Some(ItemStack::new(cobble, 1)), None,                            Some(ItemStack::new(cobble, 1)),
            Some(ItemStack::new(cobble, 1)), Some(ItemStack::new(cobble, 1)), Some(ItemStack::new(cobble, 1)),
        ];
        let res_f = match_crafting_grid(&furnace_grid, 3, 3);
        assert_eq!(res_f, Some(ItemStack::new(ItemType::Block(BlockType::Furnace), 1)));
    }

    #[test]
    fn test_consume_grid() {
        let mut grid = [
            Some(ItemStack::new(ItemType::Stick, 2)),
            Some(ItemStack::new(ItemType::Stick, 1)),
        ];
        consume_crafting_grid(&mut grid);
        assert_eq!(grid[0].unwrap().count, 1);
        assert!(grid[1].is_none());
    }
}
