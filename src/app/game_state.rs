#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameState {
    TitleScreen,
    LanLobby,
    DirectConnect,
    Playing,
    Paused,
}
