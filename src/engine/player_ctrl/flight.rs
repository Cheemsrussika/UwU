use super::def::Player;

pub fn update_flight_movement(p: &mut Player, jump: bool, sneak: bool, dt: f32) {
    let mut target_vy = 0.0f32;
    if jump { target_vy += 11.0; }
    if sneak { target_vy -= 11.0; }
    p.velocity.y += (target_vy - p.velocity.y) * (15.0 * dt).min(1.0);
    p.on_ground = false;
}
