use crossterm::event::KeyCode;

use crate::{
    app::App,
    media::{
        media::{Media, Source},
        player::Player,
    },
};

pub fn handle_radio_fs(app: &mut App, code: KeyCode) -> bool {
    let rfs = &mut app.radio_fs;
    let list = &rfs.radios;
    if list.is_empty() {
        return false;
    }
    match code {
        KeyCode::Up => {
            let curr_index = rfs.index.selected().unwrap();
            if curr_index == 0 {
                rfs.index.select(Some(list.len() - 1));
            } else {
                rfs.index.select(Some(curr_index - 1));
            }
            true
        }
        KeyCode::Down => {
            let curr_index = rfs.index.selected().unwrap();
            if curr_index == list.len() - 1 {
                rfs.index.select(Some(0));
            } else {
                rfs.index.select(Some(curr_index + 1));
            }
            true
        }
        KeyCode::Enter => {
            let s = rfs.index.selected();
            if let Some(selected_index) = s {
                if app.player.is_playing() {
                    app.player.pause();
                }
                app.radio.add_to_list(
                    Media {
                        src: Source::M3u8(rfs.radios[selected_index].clone()),
                    },
                    true,
                );
            }

            true
        }
        _ => false,
    }
}
