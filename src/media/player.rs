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

use bytes::Bytes;
use std::cmp::max;
use std::fs;
use std::future::Future;
use std::io::Cursor;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::{
    cell::{Ref, RefCell},
    fmt::Debug,
    fs::File,
    io::{BufReader, Error, Write},
    ops::Add,
    path::Path,
    ptr::null,
    sync::{
        mpsc::{self, channel},
        Arc, Mutex,
    },
    thread,
    time::{Duration, Instant, SystemTime},
};

use m3u8_rs::{MediaPlaylist, Playlist};
use rodio::decoder::DecoderError;
use rodio::{cpal, source::Delay};
use rodio::{Decoder, Devices, OutputStream, OutputStreamHandle, Sink, Source};
use tui::widgets::ListState;

use crate::net::download;
use crate::util::m3u8::empty_cache;
use crate::{
    app,
    util::lyrics::{Lyric, Lyrics},
};
use crate::{m3u8::download_m3u8_playlist, util::net::download_as_bytes};

use super::media::Media;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum PlayStatus {
    Waiting,
    Playing(Instant, Duration),
    // elapsed, times already played
    Stopped(Duration), // times already played
}

pub struct PlayListItem {
    pub name: String,
    pub duration: Duration,
    pub current_pos: Duration,
    pub status: PlayStatus,
    pub path: String,
    pub lyrics: Lyrics,
    pub lyrics_index: ListState,
}

pub struct PlayList {
    pub lists: Vec<PlayListItem>,
}

pub trait Player {
    // 初始化
    fn new() -> Self;

    // 添加歌曲
    fn add_to_list(&mut self, media: Media, once: bool) -> bool;

    // 播放
    fn play(&mut self) -> bool;

    // 下一首
    fn next(&mut self) -> bool;

    // 停止
    fn stop(&mut self) -> bool;

    // 暂停
    fn pause(&mut self) -> bool;

    // 继续
    fn resume(&mut self) -> bool;

    // 播放进度
    fn get_progress(&self) -> (f32, f32);

    // 是否正在播放
    fn is_playing(&self) -> bool;

    // 提供一个接口，用于更新player状态
    fn tick(&mut self);

    // 当前歌词
    fn current_lyric(&self) -> &str;

    // 有歌词
    fn has_lyrics(&self) -> bool;

    // 音量
    fn volume(&self) -> f32;

    // 设置音量
    fn set_volume(&mut self, new_volume: f32) -> bool;
}

pub struct MusicPlayer {
    // params
    pub current_time: Duration,
    pub total_time: Duration,
    pub play_list: PlayList,
    // media: Media,
    // stream
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sink: Sink,
    current_lyric: Option<String>,
    initialized: bool,
}

impl Player for MusicPlayer {
    fn new() -> Self {
        for dev in cpal::available_hosts() {
            println!("{:?}", dev);
        }
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        Self {
            current_time: Duration::from_secs(0),
            total_time: Duration::from_secs(0),
            play_list: PlayList { lists: vec![] },
            // media: f,
            stream,
            stream_handle,
            sink,
            current_lyric: None,
            initialized: false,
        }
    }

    fn add_to_list(&mut self, media: Media, once: bool) -> bool {
        match media.src {
            super::media::Source::Http(_) => false,
            super::media::Source::Local(path) => {
                return self.play_with_file(path, once);
            }
            super::media::Source::M3u8(path) => false,
        }
    }

    // fn next(&mut self) -> bool {
    //     // self._sink.
    //     true
    // }

    fn play(&mut self) -> bool {
        self.sink.play();
        if let Some(item) = self.play_list.lists.first_mut() {
            let status = &mut item.status;
            match status {
                PlayStatus::Waiting => {
                    *status = PlayStatus::Playing(Instant::now(), Duration::from_nanos(0));
                }
                PlayStatus::Playing(_, _) => {}
                PlayStatus::Stopped(duration) => {
                    *status = PlayStatus::Playing(Instant::now(), *duration);
                }
            }
        }
        true
    }

    fn next(&mut self) -> bool {
        let len = self.play_list.lists.len();
        if len >= 1 {
            self.play_list.lists.remove(0);
            self.stop();
            if !self.play_list.lists.is_empty() {
                // next song
                let top_music = self.play_list.lists.first().unwrap();
                let f = File::open(top_music.path.as_str()).unwrap();
                let buf_reader = BufReader::new(f);
                let (stream, stream_handle) = OutputStream::try_default().unwrap();
                self.stream = stream;
                self.stream_handle = stream_handle;
                let volume = self.volume();
                self.sink = Sink::try_new(&self.stream_handle).unwrap();
                self.set_volume(volume);
                self.sink.append(Decoder::new(buf_reader).unwrap());
                self.play();
            }
            // for
        } else {
            // no more sound to play
            return false;
        }
        true
    }

    fn stop(&mut self) -> bool {
        self.sink.stop();
        true
    }

    fn pause(&mut self) -> bool {
        self.sink.pause();
        if let Some(item) = self.play_list.lists.first_mut() {
            let status = &mut item.status;
            match status {
                PlayStatus::Waiting => {}
                PlayStatus::Playing(instant, duration) => {
                    *status = PlayStatus::Stopped(duration.add(instant.elapsed()));
                }
                PlayStatus::Stopped(_) => {}
            }
        }
        true
    }

    fn resume(&mut self) -> bool {
        self.sink.play();
        if let Some(item) = self.play_list.lists.first_mut() {
            let status = &mut item.status;
            match status {
                PlayStatus::Waiting => {}
                PlayStatus::Playing(_, _) => {}
                PlayStatus::Stopped(duration) => {
                    *status = PlayStatus::Playing(Instant::now(), *duration);
                }
            }
        }
        return true;
    }

    fn get_progress(&self) -> (f32, f32) {
        return (0.0, 0.0);
    }

    fn is_playing(&self) -> bool {
        return self.initialized && !self.sink.is_paused() && !self.play_list.lists.is_empty();
    }

    fn tick(&mut self) {
        let is_playing = self.is_playing();
        if let Some(song) = self.play_list.lists.first_mut() {
            let status = &mut song.status;
            match status {
                PlayStatus::Waiting => {
                    if is_playing {
                        *status = PlayStatus::Playing(Instant::now(), Duration::from_nanos(0));
                    }
                }
                PlayStatus::Playing(instant, duration) => {
                    let now = instant.elapsed().add(duration.clone());
                    if now.ge(&song.duration) {
                        // next song, delete 0
                        self.next();
                    } else {
                        // update status
                        self.current_time = now;
                        self.total_time = song.duration.clone();
                        // add lyrics
                        let selected_index = song.lyrics_index.selected().unwrap();
                        if selected_index + 1 < song.lyrics.list.len() {
                            let next_lyric = &song.lyrics.list[selected_index + 1];
                            if self.current_time > next_lyric.time {
                                song.lyrics_index.select(Some(selected_index + 1));
                            }
                        }
                    }
                }
                PlayStatus::Stopped(dur) => {
                    self.current_time = dur.clone();
                    self.total_time = song.duration.clone();
                }
            }
        } else {
            // stop player when no sounds
            if self.play_list.lists.is_empty() {
                self.stop();
            }
        }
    }

    fn current_lyric(&self) -> &str {
        if let Some(lyric) = &self.current_lyric {
            return lyric.as_str();
        } else {
            return "No Lyrics";
        }
    }

    fn has_lyrics(&self) -> bool {
        !self.play_list.lists.is_empty()
            && !self.play_list.lists.first().unwrap().lyrics.list.is_empty()
    }

    fn volume(&self) -> f32 {
        return self.sink.volume();
    }

    fn set_volume(&mut self, new_volume: f32) -> bool {
        self.sink.set_volume(new_volume);
        true
    }
}

impl MusicPlayer {
    pub fn playing_song(&self) -> Option<&PlayListItem> {
        return self.play_list.lists.first();
    }

    fn play_with_file(&mut self, path: String, once: bool) -> bool {
        let duration: Duration;
        if path.ends_with(".mp3") {
            let dur = mp3_duration::from_path(path.clone());
            match dur {
                Ok(dur) => {
                    duration = dur;
                }
                Err(err) => {
                    // EOF catch
                    duration = err.at_duration;
                    if duration.is_zero() {
                        return false;
                    }
                }
            }
        } else {
            if let Ok(f) = File::open(path.as_str()) {
                let dec = Decoder::new(f);
                if let Ok(dec) = dec {
                    if let Some(dur) = dec.total_duration() {
                        duration = dur;
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        // find lyrics
        let lyrics = Lyrics::from_music_path(path.as_str());
        // open
        match File::open(path.as_str()) {
            Ok(f) => {
                let path = Path::new(path.as_str());
                let file_name = path.file_name().unwrap().to_string_lossy().to_string();
                // Result<(stream,streamHanlde),std::error:Error>
                if once || self.play_list.lists.is_empty() {
                    // rebuild
                    self.stop();
                    let buf_reader = BufReader::new(f);
                    let sink = self.stream_handle.play_once(buf_reader).unwrap();
                    self.sink = sink;
                    self.play_list.lists.clear();
                }
                let mut state = ListState::default();
                state.select(Some(0));
                self.play_list.lists.push(PlayListItem {
                    name: file_name,
                    duration,
                    current_pos: Duration::from_secs(0),
                    status: PlayStatus::Waiting,
                    path: path.to_string_lossy().to_string(),
                    lyrics,
                    lyrics_index: state,
                });
                if !self.initialized {
                    self.initialized = true;
                }
                self.play();
                self.tick();
                return true;
            }
            Err(_) => false,
        }
    }
}

impl Drop for MusicPlayer {
    fn drop(&mut self) {
        // println!()
    }
}

pub struct RadioItem {
    list: MediaPlaylist,
    url: String,
}

pub struct RadioPlayer {
    pub item: Option<RadioItem>,
    pub list: Vec<PlayListItem>,
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sink: Sink,
    is_playing: bool,
    last_playing_id: i32,
    data_tx: Sender<bytes::Bytes>,
    data_rx: Receiver<bytes::Bytes>,
    elasped: SystemTime,
    gap: Duration,
}

impl Player for RadioPlayer {
    fn new() -> Self {
        let (stream, handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).unwrap();
        let (tx, rx) = channel();
        empty_cache();
        RadioPlayer {
            item: None,
            list: vec![],
            stream,
            stream_handle: handle,
            sink,
            is_playing: false,
            last_playing_id: -1,
            data_rx: rx,
            data_tx: tx,
            elasped: SystemTime::now(),
            gap: Duration::from_secs(5),
        }
    }

    fn add_to_list(&mut self, media: Media, _: bool) -> bool {
        self.last_playing_id = -1;
        let src = media.src;
        match src {
            super::media::Source::Http(_) => false,
            super::media::Source::M3u8(url) => {
                let (tx, mut rx) = channel();
                let m3u8_url = url.url.clone();
                thread::spawn(move || {
                    let playlist = download_m3u8_playlist(m3u8_url);
                    tx.send(playlist).unwrap();
                });
                match rx.recv_timeout(Duration::from_secs(5)) {
                    Ok(list) => {
                        if let Ok(playlist) = list {
                            match playlist {
                                Playlist::MasterPlaylist(_) => {}
                                Playlist::MediaPlaylist(pl) => {
                                    let item = RadioItem {
                                        list: pl,
                                        url: url.url.clone(),
                                    };
                                    self.item = Some(item);
                                    self.download_and_push();
                                    self.play();
                                }
                            }
                            return true;
                        }
                    }
                    Err(err) => {
                        // ignore
                    }
                }
                false
            }
            super::media::Source::Local(_) => false,
        }
    }

    fn play(&mut self) -> bool {
        self.sink.play();
        self.is_playing = true;
        true
    }

    fn next(&mut self) -> bool {
        false
    }

    fn stop(&mut self) -> bool {
        self.sink.stop();
        true
    }

    fn pause(&mut self) -> bool {
        self.sink.pause();
        self.is_playing = false;
        true
    }

    fn resume(&mut self) -> bool {
        self.sink.play();
        self.is_playing = true;
        true
    }

    fn get_progress(&self) -> (f32, f32) {
        return (0.0, 0.0);
    }

    fn is_playing(&self) -> bool {
        self.is_playing
    }

    fn tick(&mut self) {
        match self.data_rx.try_recv() {
            Ok(data) => {
                // let f = File::open("D:\\audio.wav").unwrap();
                let mut cache_dir = dirs::cache_dir().unwrap();
                cache_dir.push("RustPlayer");
                std::fs::create_dir_all(cache_dir.clone()).unwrap();
                let timestamp = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap();
                let fname = timestamp.as_nanos().to_string();
                cache_dir.push(fname.as_str());
                let mut f = File::create(cache_dir.clone()).unwrap();
                f.write_all(data.as_ref()).unwrap();
                let decoder = ffmpeg_decoder::Decoder::open(cache_dir);
                // let buffer = BufReader::new(f);
                // let decoder = rodio::Decoder::new_mp4(buffer, rodio::decoder::Mp4Type::M4a);
                // Decoder::
                match decoder {
                    Ok(dec) => {
                        self.sink.append(dec);
                    }
                    Err(err) => {
                        eprintln!("{:?}", err);
                    }
                }
            }
            Err(_) => {}
        }
        // check length
        if let Ok(elapsed) = self.elasped.elapsed() {
            // 过了gap时间重新拉取m3u8
            if self.is_playing() && elapsed.as_secs() > self.gap.as_secs() {
                self.download_and_push();
            }
        }
    }

    fn current_lyric(&self) -> &str {
        return "";
    }

    fn has_lyrics(&self) -> bool {
        false
    }

    fn volume(&self) -> f32 {
        self.sink.volume()
    }

    fn set_volume(&mut self, new_volume: f32) -> bool {
        self.sink.set_volume(new_volume);
        true
    }
}

impl RadioPlayer {
    /// 触发下载
    fn download_and_push(&mut self) {
        self.elasped = SystemTime::now();
        // 第一次下载，直接全部下载
        if self.last_playing_id == -1 {
            // 直接全部下载
            let item = &self.item;
            if let Some(radio) = item {
                let index = radio.url.clone().rfind("/").unwrap();
                let base_url = radio.url.as_str()[0..index + 1].to_string();
                let tx_clone = self.data_tx.clone();
                let urls: Vec<String> = radio
                    .list
                    .segments
                    .iter()
                    .map(|e| e.uri.clone())
                    .map(|uri| base_url.clone() + &uri)
                    .collect();
                self.last_playing_id =
                    radio.list.media_sequence + max((radio.list.segments.len() - 1) as i32, 0);
                thread::spawn(move || {
                    for url in urls {
                        match download_as_bytes(url.as_str(), &tx_clone) {
                            _ => {
                                // println!("{:?} downloaded",url);
                            }
                        }
                    }
                });
            }
        } else {
            // 更新playlist列表
            if let Some(radio) = &self.item {
                let index = radio.url.clone().rfind("/").unwrap();
                let base_url = radio.url.as_str()[0..index + 1].to_string();
                match download_m3u8_playlist(radio.url.clone()) {
                    Ok(playlist) => match playlist {
                        Playlist::MasterPlaylist(_) => todo!(),
                        Playlist::MediaPlaylist(media_playlist) => {
                            let seq = media_playlist.media_sequence;
                            let skip_num = max(self.last_playing_id - seq + 1, 0) as usize;
                            let segs = &media_playlist.segments[skip_num..];
                            // println!("add {:?}",segs);
                            let urls: Vec<String> = segs
                                .iter()
                                .map(|e| e.uri.clone())
                                .map(|uri| base_url.clone() + &uri)
                                .collect();
                            self.last_playing_id =
                                seq + max((media_playlist.segments.len() - 1) as i32, 0);
                            let tx_clone = self.data_tx.clone();
                            thread::spawn(move || {
                                for url in urls {
                                    match download_as_bytes(url.as_str(), &tx_clone) {
                                        _ => {
                                            // println!("update {:?}",url);
                                        }
                                    }
                                }
                            });
                        }
                    },
                    Err(_) => {
                        // ignore
                    }
                }
            }
        }
    }
}
