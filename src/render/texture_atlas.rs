pub struct TextureAtlas;

impl TextureAtlas {
    pub const RESOLUTION: u32 = super::texture_loader::RESOLUTION;
    pub const LAYER_COUNT: u32 = 69;

    pub fn generate_pixel_atlas() -> Vec<u8> {
        let mut pixels = Vec::with_capacity((64 * 64 * 4 * Self::LAYER_COUNT) as usize);
        let cm = super::super::world::colormap::ColorMap::new();
        let grass_tint = cm.get_grass_color(0.8, 0.4);
        let foliage_tint = cm.get_foliage_color(0.8, 0.4);

        // 0..=7: Initial blocks
        super::atlas_layers_blocks::append_initial_blocks(&mut pixels, grass_tint);
        // 8..=13: Legacy items
        super::atlas_layers_items::append_legacy_items(&mut pixels);
        // 14..=17: Wood, Log, Leaves
        super::atlas_layers_blocks::append_wood_leaves_and_functional_blocks(&mut pixels, foliage_tint);
        // 18..=27: Destroy stages
        super::atlas_destroy::append_destroy_stages(&mut pixels);
        // 28..=35: Survival blocks and entities
        super::atlas_layers_blocks::append_survival_blocks_and_entities(&mut pixels);
        // 36..=51: Survival items
        super::atlas_layers_items::append_survival_items(&mut pixels);
        // 52..=68: Tools and storage blocks
        super::atlas_layers_tools_blocks::append_tools_and_blocks(&mut pixels);

        pixels
    }
}