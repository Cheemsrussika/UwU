#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, bevy::prelude::Resource)]
pub enum GameMode {
    #[default]
    Survival,
    Creative,
    Adventure,
    Spectator,
}

impl GameMode {
    pub fn is_creative(&self) -> bool { matches!(self, GameMode::Creative) }
    pub fn can_fly(&self) -> bool { matches!(self, GameMode::Creative | GameMode::Spectator) }
    pub fn is_invulnerable(&self) -> bool { matches!(self, GameMode::Creative | GameMode::Spectator) }
    pub fn instant_break(&self) -> bool { matches!(self, GameMode::Creative) }
    pub fn show_hud_bars(&self) -> bool { matches!(self, GameMode::Survival | GameMode::Adventure) }
}
