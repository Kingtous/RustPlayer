// Copyright (C) 2022 KetaNetwork
//
// This file is part of RustPlayer.
//
// RustPlayer is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// RustPlayer is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with RustPlayer.  If not, see <http://www.gnu.org/licenses/>.

use crossterm::event::KeyCode;

use crate::{app::App, media::player::Player};

pub fn handle_music_controller(app: &mut App, code: KeyCode) -> bool {
    // if app.active_modules != ActiveModules::MusicController {
    //     return false;
    // }
    let player = &mut app.player;
    match code {
        KeyCode::Char('s') | KeyCode::Char('S') => {
            if player.is_playing() {
                player.pause();
            } else {
                player.resume();
            }
            return true;
        }
        KeyCode::Char('n') | KeyCode::Char('N') => {
            player.next();
            return true;
        }
        _ => {
            return false;
        }
    }
}

pub fn handle_radio_controller(app: &mut App, code: KeyCode) -> bool {
    // if app.active_modules != ActiveModules::MusicController {
    //     return false;
    // }
    let player = &mut app.radio;
    match code {
        KeyCode::Char('s') | KeyCode::Char('S') => {
            if player.is_playing() {
                player.pause();
            } else {
                player.resume();
            }
            return true;
        }
        _ => {
            return false;
        }
    }
}
