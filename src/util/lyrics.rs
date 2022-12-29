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

use std::{fmt::Display, fs::File, io::Read, path::PathBuf, time::Duration, vec};

use regex::Regex;

pub struct Lyrics {
    pub list: Vec<Lyric>,
}

impl Display for Lyrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // for item in self.list {
        //     writeln!(f,"{:?}: {}",item.time,item.content)
        // }
        writeln!(f, "Lyrics Count: {}", self.list.len())
    }
}

pub struct Lyric {
    pub time: Duration,
    pub content: String,
}

impl Lyrics {
    pub fn from_music_path(s: &str) -> Self {
        // change to *.lrc
        let mut p = PathBuf::from(s);
        p.set_extension("lrc");
        let f = File::open(p);
        match f {
            Ok(mut f) => {
                return Lyrics::from_read(&mut f);
            }
            Err(_) => return Self { list: vec![] },
        }
    }

    pub fn from_read(f: &mut File) -> Self {
        let mut buffer = vec![];
        let regex =
            Regex::new(r"\[(?P<min>\d+):(?P<sec>\d+).(?P<ms>\d+)](?P<content>[^\[\]]*)").unwrap();
        f.read_to_end(&mut buffer).unwrap();
        let m = String::from_utf8(buffer).unwrap();
        let mut lyrics_vec = vec![];
        for cap in regex.captures_iter(m.as_str()) {
            let min = cap["min"].parse::<u64>().unwrap();
            let sec = cap["sec"].parse::<u64>().unwrap();
            let ms = cap["ms"].parse::<u64>().unwrap();
            let dur = Duration::from_millis(ms + sec * 1000 + min * 1000 * 60);
            lyrics_vec.push(Lyric {
                time: dur,
                content: String::from(&cap["content"]),
            });
        }
        Self { list: lyrics_vec }
    }

    #[allow(dead_code)]
    pub fn count(&self) -> usize {
        self.list.len()
    }
}
