use std::fs::read_dir;
use std::sync::mpsc;

use std::thread;
use std::time::Duration;

use failure::format_err;
use m3u8_rs::Playlist;

use crate::net::download;

pub fn download_m3u8_playlist(url: String) -> Result<Playlist, failure::Error> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || download(url.as_str(), &tx));
    let resp = rx.recv_timeout(Duration::from_secs(5));
    return if let Ok(data) = resp {
        let playlist = m3u8_rs::parse_playlist(data.as_bytes());
        match playlist {
            Ok(list) => Ok(list.1),
            Err(_err) => Err(format_err!("Parse Playlist Failed")),
        }
    } else {
        println!("{:?}", resp);
        Err(format_err!("Download Timeout in 5 seconds."))
    };
}

pub fn empty_cache() {
    let mut dir = dirs::cache_dir().unwrap();
    dir.push("RustPlayer");
    if dir.exists() && dir.is_dir() {
        let it = read_dir(dir.clone());
        if let Ok(dirs) = it {
            for dir in dirs {
                if let Ok(d) = dir {
                    std::fs::remove_file(d.path()).unwrap();
                }
            }
        }
    }
}
