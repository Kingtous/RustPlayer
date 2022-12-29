use std::{
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

use dirs;
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState},
    Frame,
};

use crate::app::App;

#[derive(Clone)]
pub struct RadioConfig {
    pub name: String,
    pub url: String,
}

pub struct RadioExplorer {
    pub radios: Vec<RadioConfig>,
    pub index: ListState,
}

impl RadioExplorer {
    pub fn new() -> Self {
        let mut config_dir = dirs::config_dir().unwrap();
        let mut configs = vec![];
        config_dir.push("RustPlayer");
        std::fs::create_dir_all(config_dir.clone()).unwrap();
        config_dir.push("radio.ini");
        let f: File;
        if !config_dir.as_path().exists() {
            File::create(config_dir.clone()).unwrap();
        }
        f = File::open(config_dir).unwrap();
        let reader = BufReader::new(f);
        let mut lines = reader.lines().map(|i| i.unwrap());
        while let Some(line) = lines.next() {
            if line.is_empty() {
                continue;
            }
            let radio_bean: Vec<_> = line.split(' ').collect();
            let config = RadioConfig {
                name: radio_bean[0].to_string(),
                url: radio_bean[1].to_string(),
            };
            configs.push(config);
        }
        let mut state = ListState::default();
        state.select(Some(0));
        Self {
            radios: configs,
            index: state,
        }
    }
}

pub fn draw_radio_list<B>(app: &mut App, frame: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let fs = &mut app.radio_fs;
    let mut item_vec = vec![];
    for radio in &fs.radios {
        item_vec.push(ListItem::new(radio.name.as_str()));
    }
    let list = List::new(item_vec.as_ref())
        .block(
            Block::default()
                .borders(Borders::all())
                .title("Radio List")
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Cyan))
                .title_alignment(Alignment::Center),
        )
        .highlight_style(Style::default().bg(Color::Cyan))
        .highlight_symbol("> ");
    frame.render_stateful_widget(list, area, &mut fs.index);
}
