use std::cmp::max;

use crossterm::event::KeyCode;

use crate::app::App;

pub fn handle_player(app: &mut App, code: KeyCode) -> bool {
    match code {
        KeyCode::Char('-') => {
            let volume = app.player.volume() - 0.05;
            let new_volume = volume.max(0.0);
            app.player.set_volume(new_volume);
            return true;
        }
        KeyCode::Char('=') => {
            let volume = app.player.volume() + 0.05;
            let new_volume = volume.min(1.0);
            app.player.set_volume(new_volume);
            return true;
        }
        _ => {return false;}
    }
}