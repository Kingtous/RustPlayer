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

use tui::{
    backend::Backend,
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols::{self, line::Set, Marker},
    widgets::{Block, Borders, BorderType, Gauge, LineGauge},
};

use crate::{app::App, media::player::Player};

pub fn draw_progress<B>(app: &mut App, frame: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let player = &app.player;

    let current_time = player.current_time;
    let total_time = player.total_time;

    let minute_mins = current_time.as_secs() / 60;
    let minute_secs = current_time.as_secs() % 60;

    let total_mins = total_time.as_secs() / 60;
    let total_secs = total_time.as_secs() % 60;
    let mut percent = 0.0;
    if total_time.as_secs() != 0 {
        percent = if player.is_playing() {current_time.as_secs_f64() / total_time.as_secs_f64()} else {0.0};
    }
    let s = if player.is_playing() { format!(
        "{:0>2}:{:0>2} / {:0>2}:{:0>2}",
        minute_mins, minute_secs, total_mins, total_secs
    )} else {"No More Sound".to_string()};

    let gauge = LineGauge::default()
        .ratio(percent)
        .line_set(symbols::line::THICK)
        .label(s)
        .style(Style::default().add_modifier(Modifier::ITALIC))
        .gauge_style(
            Style::default()
                .bg(Color::DarkGray)
                .fg(Color::LightBlue)
                .add_modifier(Modifier::BOLD),
        );
    let layout = Layout::default()
        .horizontal_margin(1)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(area);
    frame.render_widget(gauge, layout[0]);
}
