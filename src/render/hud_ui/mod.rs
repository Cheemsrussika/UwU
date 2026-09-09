pub mod atlas;
pub mod atlas_items;
pub mod atlas_menu;
pub mod atlas_more_items;
pub mod atlas_tools;
pub mod bars;
pub mod builder;
pub mod container_window;
pub mod creative_inventory;
pub mod font;
pub mod icon_blocks;
pub mod icon_tools;
pub mod inventory;
pub mod iso_block;
pub mod item_icon;
pub mod menu_bg;
pub mod menu_button;
pub mod menu_render;
pub mod title_screen;
pub mod direct_connect_screen;
pub mod lan_screen;
pub mod pause_screen;
pub mod piechart;
pub mod player_preview;
pub mod preview_boxes;
pub mod renderer;
pub mod resources;
pub mod slot_render;
pub mod slot_coords;

pub use atlas::{HudAtlas, SpriteRect};
pub use builder::update_hud_mesh;
pub use font::draw_text;
pub use menu_render::update_menu_hud_mesh;
pub use piechart::draw_profiler_piechart;
pub use renderer::{HudRenderer, HudVertex};

#[cfg(test)]
mod tests;