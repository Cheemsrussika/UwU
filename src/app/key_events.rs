use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode;
use super::app_state::App;
use crate::input::LogicCommand;

pub fn handle_keyboard_input(app: &mut App, key: KeyCode, pressed: bool, event_loop: &ActiveEventLoop) {
    app.keys_pressed.insert(key, pressed);
    if !pressed { return; }
    let shift_down = app.keys_pressed.get(&KeyCode::ShiftLeft).copied().unwrap_or(false)
        || app.keys_pressed.get(&KeyCode::ShiftRight).copied().unwrap_or(false);

    if app.game_state == super::game_state::GameState::DirectConnect {
        if key == KeyCode::Enter || key == KeyCode::NumpadEnter {
            let addr = app.direct_ip_input.clone();
            super::menu_action_handler::apply_menu_action(app, crate::input::MenuAction::ConnectLan(addr), event_loop);
            return;
        }
        if key == KeyCode::Escape {
            app.game_state = super::game_state::GameState::LanLobby;
            return;
        }
        if super::text_input::handle_text_input(app, key, shift_down) {
            return;
        }
    }

    if let Some(tx) = &app.command_tx {
        match key {
            KeyCode::F3 => { let _ = tx.send(LogicCommand::TogglePieChart); }
            KeyCode::KeyG => { let _ = tx.send(LogicCommand::ToggleChunkBorders); }
            KeyCode::Digit0 => { let _ = tx.send(LogicCommand::ProfilerNavigate(0)); }
            KeyCode::Digit1 => {
                if shift_down { let _ = tx.send(LogicCommand::ProfilerNavigate(1)); }
                else { let _ = tx.send(LogicCommand::SelectSlot(0)); }
            }
            KeyCode::Digit2 => {
                if shift_down { let _ = tx.send(LogicCommand::ProfilerNavigate(2)); }
                else { let _ = tx.send(LogicCommand::SelectSlot(1)); }
            }
            KeyCode::Digit3 => {
                if shift_down { let _ = tx.send(LogicCommand::ProfilerNavigate(3)); }
                else { let _ = tx.send(LogicCommand::SelectSlot(2)); }
            }
            KeyCode::Digit4 => {
                if shift_down { let _ = tx.send(LogicCommand::ProfilerNavigate(4)); }
                else { let _ = tx.send(LogicCommand::SelectSlot(3)); }
            }
            KeyCode::Digit5 => {
                if shift_down { let _ = tx.send(LogicCommand::ProfilerNavigate(5)); }
                else { let _ = tx.send(LogicCommand::SelectSlot(4)); }
            }
            KeyCode::Digit6 => {
                if shift_down { let _ = tx.send(LogicCommand::ProfilerNavigate(6)); }
                else { let _ = tx.send(LogicCommand::SelectSlot(5)); }
            }
            KeyCode::Digit7 => {
                if shift_down { let _ = tx.send(LogicCommand::ProfilerNavigate(7)); }
                else { let _ = tx.send(LogicCommand::SelectSlot(6)); }
            }
            KeyCode::Digit8 => {
                if shift_down { let _ = tx.send(LogicCommand::ProfilerNavigate(8)); }
                else { let _ = tx.send(LogicCommand::SelectSlot(7)); }
            }
            KeyCode::Digit9 => {
                if shift_down { let _ = tx.send(LogicCommand::ProfilerNavigate(9)); }
                else { let _ = tx.send(LogicCommand::SelectSlot(8)); }
            }
            KeyCode::KeyE => { let _ = tx.send(LogicCommand::ToggleInventory); }
            KeyCode::KeyZ => { let _ = tx.send(LogicCommand::PrevSlot); }
            KeyCode::KeyX => { let _ = tx.send(LogicCommand::NextSlot); }
            KeyCode::Escape => {
                match app.game_state {
                    super::game_state::GameState::TitleScreen => {
                        app.running.store(false, std::sync::atomic::Ordering::Relaxed);
                        event_loop.exit();
                    }
                    super::game_state::GameState::LanLobby => {
                        app.game_state = super::game_state::GameState::TitleScreen;
                    }
                    super::game_state::GameState::DirectConnect => {
                        app.game_state = super::game_state::GameState::LanLobby;
                    }
                    super::game_state::GameState::Playing => {
                        app.game_state = super::game_state::GameState::Paused;
                    }
                    super::game_state::GameState::Paused => {
                        app.game_state = super::game_state::GameState::Playing;
                    }
                }
            }
            _ => {}
        }
    }
}