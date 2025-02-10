use crate::app::{App, AppResult, Mode};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit Editing Mode on `ESC`
        KeyCode::Esc => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') if key_event.modifiers == KeyModifiers::CONTROL => {
            app.quit();
        }
        KeyCode::Enter => {
            app.handler_key_enter();
        }
        KeyCode::Backspace => {
            app.delete_char();
        }
        KeyCode::Char(c) => {
            app.enter_char(c);
        }
        KeyCode::Left if app.mode == Mode::Convert => {
            app.switch_mode(Mode::Editing);
        }
        KeyCode::Right if app.mode == Mode::Editing => {
            app.switch_mode(Mode::Convert);
        }
        _ => {}
    }
    Ok(())
}
