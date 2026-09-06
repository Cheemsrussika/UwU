use crate::engine::items::{ItemStack, ItemType};

#[derive(Clone, Debug)]
pub struct Recipe {
    pub width: usize,
    pub height: usize,
    pub pattern: Vec<Option<ItemType>>,
    pub result: ItemStack,
    pub is_shapeless: bool,
}

impl Recipe {
    pub fn shaped(w: usize, h: usize, pattern: Vec<Option<ItemType>>, result: ItemStack) -> Self {
        Self { width: w, height: h, pattern, result, is_shapeless: false }
    }

    pub fn shapeless(ingredients: Vec<ItemType>, result: ItemStack) -> Self {
        let pattern = ingredients.into_iter().map(Some).collect();
        Self { width: 0, height: 0, pattern, result, is_shapeless: true }
    }

    pub fn matches(&self, grid: &[Option<ItemStack>], grid_w: usize, grid_h: usize) -> bool {
        if self.is_shapeless {
            self.matches_shapeless(grid)
        } else {
            self.matches_shaped(grid, grid_w, grid_h)
        }
    }

    fn matches_shapeless(&self, grid: &[Option<ItemStack>]) -> bool {
        let mut grid_items: Vec<ItemType> = grid.iter().filter_map(|s| s.as_ref().map(|x| x.item)).collect();
        let mut recipe_items: Vec<ItemType> = self.pattern.iter().filter_map(|x| *x).collect();
        if grid_items.len() != recipe_items.len() { return false; }
        grid_items.sort_by_key(|i| format!("{:?}", i));
        recipe_items.sort_by_key(|i| format!("{:?}", i));
        grid_items == recipe_items
    }

    fn matches_shaped(&self, grid: &[Option<ItemStack>], grid_w: usize, grid_h: usize) -> bool {
        if self.width > grid_w || self.height > grid_h { return false; }
        for off_y in 0..=(grid_h - self.height) {
            for off_x in 0..=(grid_w - self.width) {
                if self.check_shaped_at(grid, grid_w, grid_h, off_x, off_y) {
                    return true;
                }
            }
        }
        false
    }

    fn check_shaped_at(&self, grid: &[Option<ItemStack>], gw: usize, gh: usize, ox: usize, oy: usize) -> bool {
        for gy in 0..gh {
            for gx in 0..gw {
                let actual = grid[gy * gw + gx].as_ref().map(|s| s.item);
                let expected = if gx >= ox && gx < ox + self.width && gy >= oy && gy < oy + self.height {
                    self.pattern[(gy - oy) * self.width + (gx - ox)]
                } else {
                    None
                };
                if actual != expected { return false; }
            }
        }
        true
    }
}
