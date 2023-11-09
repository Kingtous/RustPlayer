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

pub fn handle_player(app: &mut App, code: KeyCode) -> bool {
    match code {
        KeyCode::Char('-') => {
            let volume = app.player.volume() - 0.05;
            let new_volume = volume.max(0.0);
            app.player.set_volume(new_volume);
            return true;
        }
        KeyCode::Char('=') | KeyCode::Char('+') => {
            let volume = app.player.volume() + 0.05;
            let new_volume = volume.min(1.0);
            app.player.set_volume(new_volume);
            return true;
        }
        _ => {
            return false;
        }
    }
}

pub fn handle_radio(app: &mut App, code: KeyCode) -> bool {
    match code {
        KeyCode::Char('-') => {
            let volume = app.radio.volume() - 0.05;
            let new_volume = volume.max(0.0);
            app.radio.set_volume(new_volume);
            return true;
        }
        KeyCode::Char('=') | KeyCode::Char('+') => {
            let volume = app.radio.volume() + 0.05;
            let new_volume = volume.min(1.0);
            app.radio.set_volume(new_volume);
            return true;
        }
        _ => {
            return false;
        }
    }
}
