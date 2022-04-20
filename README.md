# RustPlayer [![GitHub tag](https://img.shields.io/github/tag/Kingtous/RustPlayer)](https://GitHub.com/Kingtous/RustPlayer/tags/) [![GitHub stars](https://badgen.net/github/stars/Kingtous/RustPlayer)](https://github.com/Kingtous/RustPlayer/stargazers/)

![build status](https://github.com/Kingtous/RustPlayer/actions/workflows/rust.yml/badge.svg)
[![Rust Build, bump tag version and release](https://github.com/Kingtous/RustPlayer/actions/workflows/rust-release.yml/badge.svg)](https://github.com/Kingtous/RustPlayer/actions/workflows/rust-release.yml)
[![fclash](https://snapcraft.io/fclash/badge.svg)](https://snapcraft.io/fclash)
[![fclash](https://snapcraft.io/fclash/trending.svg?name=0)](https://snapcraft.io/fclash)

[![Linux](https://svgshare.com/i/Zhy.svg)](https://svgshare.com/i/Zhy.svg)
[![macOS](https://svgshare.com/i/ZjP.svg)](https://svgshare.com/i/ZjP.svg)
[![Windows](https://svgshare.com/i/ZhY.svg)](https://svgshare.com/i/ZhY.svg)

An local audio player & network m3u8 radio player using completely terminal gui. MacOS, Linux, Windows are all supported.

RustPlayer is under development. If u have encountered any problem, please open issues :)

## Features

- Support mp3, wav, flac format
- Support m3u8 network radio
    - tested: 央广之声、经济之声. check `radio.ini` for details.
    - please copy `radio.ini` to `~/.config/rustplayer`
- Lyrics Supported
- Multi-platform supported
- Low CPU and memory usage
- File explorer
- Playlist playback supported
- Wave animation
- Playback progress
- Next audio
- Adjust volume
- Developed by Kingtous

## Screenshots

### Windows

Play with lyrics. If no lyrics found, the wave animation will be the replacement of the block. See screenshots from Linux and macOS below.

![image.png](https://s2.loli.net/2022/03/04/SbK6RN7tXAym4g3.png)

### Linux 

The screenshot from Deepin

![Deepin RustPlayer](https://s2.loli.net/2022/03/03/YtJWvnDuV4rHs7T.png)

### macOS

![macOS RustPlayer](https://s2.loli.net/2022/03/03/Z9altpG63qk24W8.png)

## Demo Video

- [Bilibili Video](https://www.bilibili.com/video/BV1T34y1k7Xf)


## Install RustPlayer by Snap Store

`snap install rustplayer --devmode`

## Download Binary Release Directly and Run

The binary release of macOS, Linux, Windows can be found in artifacts of [RustPlayer Release Action](https://github.com/Kingtous/RustPlayer/actions/workflows/rust-release.yml). Click the top item of the list to download the latest release.

## Compile RustPlayer and run

If u found this binary release is not working or u like compiling RustPlayer by youselef. Yes, The step to compile RustPlayer is really easy.

- clone this repo.
- install dependencies
    - check `.github/rust.yml` for details
- `cargo run` in root of this project.

if u think this repo is helpful, ⭐ this project and let me know :)
