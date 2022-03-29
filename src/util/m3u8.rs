use std::fmt::format;
use std::sync::mpsc;
use std::sync::mpsc::RecvTimeoutError;
use std::thread;
use std::time::Duration;

use failure::{Error, format_err};
use m3u8_rs::{MediaPlaylist, Playlist};

use crate::net::{download, DownloadTimeoutError};

pub fn download_m3u8_playlist(url: String) -> Result<Playlist, failure::Error> {
    let (tx, rx) = mpsc::channel();
    thread::spawn( move || {
        download(url.as_str(), &tx)
    });
    let resp = rx.recv_timeout(Duration::from_secs(5));
    return if let Ok(data) = resp {
        let playlist = m3u8_rs::parse_playlist(data.as_bytes());
        match playlist {
            Ok(list) => {
                Ok(list.1)
            }
            Err(err) => {
                Err(format_err!("Parse Playlist Failed"))
            }
        }
    } else {
        println!("{:?}",resp);
        Err(format_err!("Download Timeout in 5 seconds."))
    };
}