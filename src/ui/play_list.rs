use tui::{Frame, layout::{Rect, Alignment}, backend::Backend, widgets::{ListItem, List, Block, Borders, BorderType}};

use crate::app::App;

pub fn draw_play_list<B>(app: &mut App,frame:&mut Frame<B>, area: Rect) where B:Backend {
    let mut items = vec![];
    let player = &app.player;
    for item in &player.play_list.lists{
        items.push(
            ListItem::new(item.name.as_str())
        )
    }
    let list = List::new(items).block(
        Block::default().borders(Borders::ALL).title("Playlist").border_type(BorderType::Rounded).title_alignment(Alignment::Center)
    );
    frame.render_widget(list, area);
}