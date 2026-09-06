#[cfg(test)]
mod tests {
    use crate::engine::blocks::*;
    use crate::engine::items::{ItemStack, ItemType};

    #[test]
    fn test_chest_storage() {
        let mut chest = Chest::new();
        let stack = ItemStack::new(ItemType::Apple, 10);
        let rem = chest.insert_auto(stack);
        assert!(rem.is_none());
        assert_eq!(chest.get(0).unwrap().count, 10);

        let stack2 = ItemStack::new(ItemType::Apple, 20);
        let rem2 = chest.insert_auto(stack2);
        assert!(rem2.is_none());
        assert_eq!(chest.get(0).unwrap().count, 30);
    }

    #[test]
    fn test_furnace_smelting() {
        let mut furnace = Furnace::new();
        furnace.input = Some(ItemStack::new(ItemType::Porkchop, 1));
        furnace.fuel = Some(ItemStack::new(ItemType::Coal, 1));

        // Start smelting
        furnace.update(0.1);
        assert!(furnace.is_burning());
        assert!(furnace.fuel.is_none());

        // Fast forward 10 seconds of cooking
        furnace.update(10.0);
        assert_eq!(furnace.output.as_ref().unwrap().item, ItemType::CookedPorkchop);
        assert_eq!(furnace.output.as_ref().unwrap().count, 1);
        assert!(furnace.input.is_none());
    }

    #[test]
    fn test_crafting_table_3x3() {
        let mut table = CraftingTable::new();
        let iron = ItemType::IronIngot;
        let stick = ItemType::Stick;

        // Place pickaxe recipe in 3x3
        table.set_slot(0, Some(ItemStack::new(iron, 1)));
        table.set_slot(1, Some(ItemStack::new(iron, 1)));
        table.set_slot(2, Some(ItemStack::new(iron, 1)));
        table.set_slot(4, Some(ItemStack::new(stick, 1)));
        table.set_slot(7, Some(ItemStack::new(stick, 1)));

        assert_eq!(table.result.as_ref().unwrap().item, ItemType::IronPickaxe);
        let result = table.take_result().unwrap();
        assert_eq!(result.item, ItemType::IronPickaxe);
        assert!(table.grid[0].is_none());
        assert!(table.grid[4].is_none());
    }
}
