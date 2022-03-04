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

use std::vec;

use rand::Rng;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{BarChart, Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::{app::App, media::player::Player};

pub fn draw_bar_charts_effect<B>(app: &mut App, frame: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let player = &mut app.player;

    let mut rng = rand::thread_rng();
    let mut cols = vec![];
    for _ in 0..20 {
        let mut i = rng.gen_range(0..10);
        if !player.is_playing() {
            i = 0
        }
        cols.push(("_", i))
    }
    match player.has_lyrics() {
        true => {
            let mut lyrics = vec![];
            if let Some(item) = player.play_list.lists.first_mut() {
                for ele in &item.lyrics.list {
                    lyrics.push(ListItem::new(ele.content.as_str()));
                }
                let list = List::new(lyrics)
                    .highlight_symbol("*")
                    .highlight_style(
                        Style::default()
                            .add_modifier(Modifier::BOLD)
                            .bg(Color::Cyan),
                    )
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded)
                            .title("Lyrics")
                            .title_alignment(Alignment::Center),
                    );
                frame.render_stateful_widget(list, area, &mut item.lyrics_index);
            }
        }
        false => {
            let items = BarChart::default()
                .bar_width(4)
                .bar_gap(1)
                .bar_style(Style::default().fg(Color::Cyan).bg(Color::Black))
                .data(&cols)
                .value_style(Style::default().add_modifier(Modifier::ITALIC))
                .label_style(Style::default().add_modifier(Modifier::ITALIC))
                .max(10)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .title("Wave")
                        .title_alignment(Alignment::Center),
                );
            frame.render_widget(items, area);
        }
    }
}
