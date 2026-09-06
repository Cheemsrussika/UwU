use super::def::Player;
use crate::engine::stats::{get_food_values, Experience};
use crate::engine::items::ItemType;

pub fn eat_player_food(player: &mut Player, item: ItemType) -> bool {
    if player.hunger >= 20.0 {
        return false;
    }
    if let Some((food, sat)) = get_food_values(item) {
        player.hunger = (player.hunger + food).min(20.0);
        player.saturation = (player.saturation + sat).min(player.hunger);
        true
    } else {
        false
    }
}

pub fn add_player_xp(player: &mut Player, mut amount: u32) {
    player.xp_total += amount;
    while amount > 0 {
        let needed = Experience::xp_needed_for_level(player.xp_level);
        let remaining = needed.saturating_sub(player.xp_points);
        if amount >= remaining {
            amount -= remaining;
            player.xp_level += 1;
            player.xp_points = 0;
        } else {
            player.xp_points += amount;
            amount = 0;
        }
    }
}

pub fn tick_player_stats(player: &mut Player, dt: f32) {
    if player.game_mode.is_invulnerable() {
        return;
    }
    while player.exhaustion >= 4.0 {
        player.exhaustion -= 4.0;
        if player.saturation > 0.0 {
            player.saturation = (player.saturation - 1.0).max(0.0);
        } else {
            player.hunger = (player.hunger - 1.0).max(0.0);
        }
    }

    player.hunger_timer += dt;
    if player.hunger_timer >= 4.0 {
        player.hunger_timer = 0.0;
        if player.hunger >= 18.0 && player.health < 20.0 {
            player.health = (player.health + 1.0).min(20.0);
            player.exhaustion += 6.0;
        } else if player.hunger <= 0.0 {
            player.health = (player.health - 1.0).max(0.0);
        }
    }
}
