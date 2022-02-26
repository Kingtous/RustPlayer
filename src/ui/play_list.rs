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

use tui::{backend::Backend, Frame, layout::{Alignment, Rect}, widgets::{Block, Borders, BorderType, List, ListItem}};

use crate::app::App;

pub fn draw_play_list<B>(app: &mut App, frame: &mut Frame<B>, area: Rect) where B: Backend {
    let mut items = vec![];
    let player = &app.player;
    for item in &player.play_list.lists {
        items.push(
            ListItem::new(item.name.as_str())
        )
    }
    let list = List::new(items).block(
        Block::default().borders(Borders::ALL).title("Playlist").border_type(BorderType::Rounded).title_alignment(Alignment::Center)
    );
    frame.render_widget(list, area);
}