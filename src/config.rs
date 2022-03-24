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

use std::time::Duration;

pub struct Config {
    pub refresh_rate: Duration,
    pub tick_gap: Duration,
    pub home_page: &'static str,
}

impl Config {
    pub fn default() -> Self {
        Self {
            refresh_rate: Duration::from_millis(100),
            tick_gap: Duration::from_millis(200),
            home_page: "https://github.com/Kingtous",
        }
    }
}