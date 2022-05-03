// Copyright (C) 2022 Kingtous
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

use crate::app::App;

pub fn handle_help(app: &mut App, code: KeyCode) -> bool {
    match code {
        KeyCode::Enter => {
            open::that(app.config.home_page);
            return true;
        }
        KeyCode::Char('r') => {
            let mut config_dir = dirs::config_dir().unwrap();
            config_dir.push("RustPlayer");
            open::that(config_dir);
            return true;
        }
        _ => {return false;}
    }
}