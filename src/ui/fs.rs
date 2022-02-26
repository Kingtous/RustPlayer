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

use std::{fs, os};
use std::env::current_dir;
use std::fmt::{Debug, Display, Formatter};
use std::fs::{DirEntry, ReadDir};
use std::path::Path;

use crossterm::style::Colors;
use failure::{Error, Fail};
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::Text;
use tui::widgets::{Block, Borders, BorderType, List, ListItem, ListState, Paragraph, Widget, Wrap};

use crate::App;
use crate::app::ActiveModules;

pub struct FsExplorer {
    pub current_path: String,
    pub files: Vec<DirEntry>,
    pub dirs: Vec<DirEntry>,
    pub index: ListState,
    on_error_msg_callback: Option<fn(Error)>,
    accept_suffix: Vec<&'static str>,
}

#[derive(Fail, Debug)]
#[fail(display = "FsError: {}", msg)]
pub struct FsError {
    msg: &'static str,
}

impl FsExplorer {
    pub fn default(callback: Option<fn(Error)>) -> Result<Self, Error> {
        let path = current_dir()?;
        let path_str = path.to_str().ok_or(FsError {
            msg: "path to_str error.",
        })?;
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        let mut exp = Self {
            current_path: path_str.to_string(),
            files: vec![],
            dirs: vec![],
            index: list_state,
            on_error_msg_callback: callback,
            accept_suffix: vec!["mp3", "wav"],
        };
        let (dirs, files) = exp.visit_dir(path_str)?;
        exp.files = files;
        exp.dirs = dirs;
        Ok(exp)
    }

    pub fn refresh(&mut self) {
        let str = String::from(self.current_path.as_str());
        match self.visit_dir(str.as_str()) {
            Ok(entries) => {
                self.dirs = entries.0;
                self.files = entries.1;
            }
            Err(_) => {}
        }
    }

    /// dirs,mp3s
    fn visit_dir(&mut self, path: &str) -> Result<(Vec<DirEntry>, Vec<DirEntry>), Error> {
        let path = Path::new(path);
        let mut dir_entries = vec![];
        let mut file_entries = vec![];
        match path.is_dir() {
            true => {
                for entry in fs::read_dir(path)? {
                    match entry {
                        Ok(entry) => {
                            for accept_suffix in self.accept_suffix.iter() {
                                let path = entry.path();
                                // println!("{:?}", path.display());
                                if let Some(ext) = path.extension() {
                                    if ext.to_string_lossy().ends_with(accept_suffix) {
                                        file_entries.push(entry);
                                        break;
                                    }
                                } else if path.is_dir() {
                                    dir_entries.push(entry);
                                    break;
                                }
                            }
                        }
                        Err(_) => {
                            continue;
                        }
                    }
                }
            }
            false => {
                return Err(Error::from(FsError {
                    msg: "is not a valid path",
                }));
            }
        }
        Ok((dir_entries, file_entries))
    }
}

fn draw_dir_item(entry: &DirEntry, vec: &mut Vec<ListItem>) {
    let file_name = String::from(entry.file_name().to_str().unwrap()) + "/";
    vec.push(ListItem::new(file_name));
}

fn draw_file_item(entry: &DirEntry, vec: &mut Vec<ListItem>) {
    let file_name = String::from(entry.file_name().to_str().unwrap());
    vec.push(ListItem::new(file_name));
}

pub fn draw_fs_tree<B>(app: &mut App, frame: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let fse = &mut app.fs;
    let fs_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Percentage(100)])
        .split(area);

    let folder = Paragraph::new(Text::from(fse.current_path.as_str()))
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title("Current Folder")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL),
        );
    frame.render_widget(folder, fs_chunks[0]);
    // list
    let mut items = vec![ListItem::new("Go Back")];
    for entry in &fse.dirs {
        draw_dir_item(entry, &mut items);
    }
    for entry in &fse.files {
        draw_file_item(entry, &mut items);
    }
    let mut blk = Block::default()
                .title("Explorer")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL).border_type(BorderType::Rounded);
    if app.active_modules == ActiveModules::Fs {
        blk = blk.border_style(Style::default().fg(Color::Cyan));
    }
    let file_list = List::new(items)
        .block(
            blk
        )
        .highlight_style(Style::default().bg(Color::Cyan))
        .highlight_symbol("> ");
    frame.render_stateful_widget(file_list, fs_chunks[1], &mut fse.index);
}
