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

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Row, Table},
    Frame,
};

use crate::app::App;

pub fn draw_help<B>(_app: &mut App, frame: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Percentage(100)])
        .split(area);
    let homepage_text = Paragraph::new("Press <Enter> key to open author(Kingtous)'s home page.")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );
    frame.render_widget(homepage_text, chunks[0]);

    let help_table = Table::new([
        Row::new(["h", "open or close this help."]),
        Row::new([
            "Tab",
            "switch highlight block. (Audio Explorer/Radio Explorer)",
        ]),
        Row::new(["r", "open radio config list."]),
        Row::new(["->", "add audio to play list."]),
        Row::new([
            "Enter",
            "play audio immediately and clean play list or enter selected folder.",
        ]),
        Row::new(["-/+", "decrease/increase volume."]),
        Row::new(["s", "pause/resume audio playback."]),
        Row::new(["n", "play the next audio."]),
        Row::new(["q", "quit RustPlayer."]),
        Row::new(["↑/↓", "change selected index."]),
    ])
    .header(
        Row::new(vec!["Key", "Function"])
            .style(Style::default().fg(Color::White))
            .bottom_margin(1),
    )
    .block(
        Block::default()
            .title("Help Table")
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL),
    )
    .column_spacing(2)
    .widths(&[Constraint::Min(6), Constraint::Percentage(100)]);
    frame.render_widget(help_table, chunks[1]);
}
