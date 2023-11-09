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

use app::*;

use ui::*;
use util::*;

mod app;
mod config;
mod handler;
mod media;
mod ui;
mod util;

fn main() {
    let mut app = App::new().unwrap();
    match app.run() {
        Ok(_) => {}
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
