use std::cmp::max;

use crossterm::event::KeyCode;

use crate::app::App;

pub fn handle_help(app: &mut App, code: KeyCode) -> bool {
    match code {
        KeyCode::Enter => {
            open::that(app.config.home_page);
            return true;
        }
        _ => {return false;}
    }
}