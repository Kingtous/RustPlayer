use std::{thread, io::{Result, Error}, sync::mpsc, time::Duration};
use m3u8_rs::Playlist;

include!("../src/util/net.rs");

#[test]
fn fetch_and_play() {
    let src = "http://ngcdn002.cnr.cn/live/jjzs/index.m3u8";
    let (tx,rx) = mpsc::channel();
    thread::spawn(move ||{
        download(&src.to_owned(),&tx);
    });
    match rx.recv_timeout(Duration::from_secs(5)){
        Ok(s) => {
            println!("{}",s);
            let entity = m3u8_rs::parse_playlist(s.as_bytes()).unwrap();
            let list = entity.1;
            match list {
                Playlist::MasterPlaylist(master_play_list) => {
                }
                Playlist::MediaPlaylist(media_play_list) => {
                    for seg in &media_play_list.segments {
                        println!("{}", seg.uri);
                    }
                   assert_ne!(media_play_list.segments.len(),0);
                }
            }

        },
        Err(e) => {
            assert!(false);
        },
    }
}
