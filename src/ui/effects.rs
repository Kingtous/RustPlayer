use std::{vec};

use tui::{
    backend::Backend,
    layout::{Rect, Layout, Direction, Constraint, Alignment},
    style::{Color, Modifier, Style},
    widgets::{BarChart, Block, Borders, BorderType},
    Frame,
};

use crate::{app::App, media::player::Player};
use rand::Rng;

pub fn draw_bar_charts_effect<B>(app: &mut App, frame: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let player = &app.player;
    

    let mut rng = rand::thread_rng();
    let mut cols = vec![];
    for _ in 0..20 {
        let mut i = rng.gen_range(0..10);
        if !player.is_playing() {
            i = 0
        }
        cols.push(
            ("_",i)
        )
    }
    // let layout_chunks = Layout::default().direction(Direction::Horizontal)
    // .constraints([
    //     Constraint::Percentage(10),
    //     Constraint::Percentage(80),
    //     Constraint::Percentage(10)
    // ]).split(area);
    let items = BarChart::default()
        .bar_width(4)
        .bar_gap(1)
        .bar_style(Style::default().fg(Color::LightBlue).bg(Color::Black))
        .data(&cols)
        .value_style(Style::default().add_modifier(Modifier::ITALIC))
        .label_style(Style::default().add_modifier(Modifier::ITALIC))
        .max(10)
        .block(
            Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("Wave")
            .title_alignment(Alignment::Center)
        )
        ;
    frame.render_widget(items, area);
}
