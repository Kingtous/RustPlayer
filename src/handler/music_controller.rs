use std::cmp::max;

use crossterm::event::KeyCode;

use crate::{app::{App, ActiveModules}, media::player::Player};

pub fn handle_music_controller(app: &mut App, code: KeyCode) -> bool {
    if app.active_modules != ActiveModules::MusicController {
        return false;
    }
    // let mc = &app.music_controller;
    let player = &mut app.player;
    match code {
        KeyCode::Char('s') => {
            if player.is_playing() {
                player.pause();
            } else {
                player.resume();
            }
            return true;
        }
        _ => {return false;}
    }
}