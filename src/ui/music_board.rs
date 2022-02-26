use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Modifier},
    text::{Span, Spans, Text},
    widgets::{
        canvas::Line, Block, BorderType, Borders, Gauge, List, ListItem, ListState, Paragraph,
        Table, LineGauge,
    },
    Frame, symbols::{line::Set, self},
};

use crate::{
    app::{ActiveModules, App},
    media::player::Player,
};

use super::{effects::draw_bar_charts_effect, progress::draw_progress, play_list::draw_play_list};

pub struct MusicController {
    pub state: ListState,
}

pub fn draw_music_board<B>(app: &mut App, frame: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let main_layout_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Percentage(80),Constraint::Percentage(20)])
        .split(area);

    draw_header(app, frame, main_layout_chunks[0]);

    let mid_layout_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70),Constraint::Percentage(30)])
        .split(main_layout_chunks[1]);

    draw_bar_charts_effect(app, frame, mid_layout_chunks[0]);
    draw_play_list(app, frame, mid_layout_chunks[1]);
    draw_progress(app, frame, main_layout_chunks[2]);
}

pub fn draw_header<B>(app: &mut App, frame: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let player = &app.player;
    let main_layout_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(area);

    let mut playing_text = "".to_string();
    if let Some(item) = player.playing_song() {
        playing_text = String::from(item.name.as_str());
    } else {
        playing_text = String::from("None");
    }
    let text = Paragraph::new(playing_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title("Now Playing").title_alignment(Alignment::Center),
    ).style(Style::default().add_modifier(Modifier::SLOW_BLINK));

    let sub_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(main_layout_chunks[0]);

    let sound_volume_percent = app.player.volume();
    let bar = LineGauge::default()
        .ratio(sound_volume_percent.into())
        .label("VOL")
        .line_set(symbols::line::THICK)
        .block(
            Block::default().border_type(BorderType::Rounded).borders(Borders::ALL)
        )
        .gauge_style(Style::default().fg(Color::LightCyan).bg(Color::Black).add_modifier(Modifier::BOLD));
    
    frame.render_widget(text, sub_layout[0]);
    frame.render_widget(bar, sub_layout[1]);
    let mut p = Paragraph::new(vec![
        Spans::from("▶(s) EXT(q) SWH(Tab) HLP(h)"),
    ]).style(Style::default())
    .alignment(Alignment::Center);
    if player.is_playing() {
        p = Paragraph::new(vec![
            Spans::from("||(s) EXT(q) SWH(Tab) HELP(h)"),
        ])
        .alignment(Alignment::Center);
    }
    let mut blk = Block::default().borders(Borders::ALL).title("Panel").border_type(BorderType::Rounded).title_alignment(Alignment::Center);
    if app.active_modules == ActiveModules::MusicController {
        blk = blk.border_style(Style::default().fg(Color::Cyan));
    }
    p = p.block(blk);
    frame.render_widget(p, main_layout_chunks[1]);
}
