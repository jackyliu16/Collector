use crate::app::{App, AppResult, Mode};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};

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

pub fn handle_mouse_events(mouse_event: MouseEvent, app: &mut App) -> AppResult<()> {
    match mouse_event.kind {
        MouseEventKind::ScrollUp if app.mode == Mode::Convert => {
            app.cmd.state.select_previous();
        }
        MouseEventKind::ScrollDown if app.mode == Mode::Convert => {
            app.cmd.state.select_next();
        }
        _ => {}
    }
    Ok(())
}
