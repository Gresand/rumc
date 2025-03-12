use crossterm::event::{self, KeyCode, KeyEvent};

pub async fn handle_input() -> Option<KeyCode> {
    if let Ok(event::Event::Key(KeyEvent {code, .. })) = event::read() {
        return Some(code);
    }
    None
}
