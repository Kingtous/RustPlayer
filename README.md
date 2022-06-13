# RustPlayer [![GitHub tag](https://img.shields.io/github/tag/Kingtous/RustPlayer)](https://GitHub.com/Kingtous/RustPlayer/tags/) [![GitHub stars](https://badgen.net/github/stars/Kingtous/RustPlayer)](https://github.com/Kingtous/RustPlayer/stargazers/)

![build status](https://github.com/Kingtous/RustPlayer/actions/workflows/rust.yml/badge.svg)
[![rustplayer](https://snapcraft.io/rustplayer/badge.svg)](https://snapcraft.io/rustplayer)
[![rustplayer](https://snapcraft.io/rustplayer/trending.svg?name=0)](https://snapcraft.io/rustplayer)

[![Linux](https://svgshare.com/i/Zhy.svg)](https://svgshare.com/i/Zhy.svg)
[![macOS](https://svgshare.com/i/ZjP.svg)](https://svgshare.com/i/ZjP.svg)
[![Windows](https://svgshare.com/i/ZhY.svg)](https://svgshare.com/i/ZhY.svg)
[![Codacy Badge](https://app.codacy.com/project/badge/Grade/549d1445d4f14a18b89fbb2340fe15fc)](https://www.codacy.com/gh/Kingtous/RustPlayer/dashboard?utm_source=github.com&amp;utm_medium=referral&amp;utm_content=Kingtous/RustPlayer&amp;utm_campaign=Badge_Grade)
![commit](https://img.shields.io/github/commit-activity/y/kingtous/RustPlayer)
[![stars](https://img.shields.io/github/stars/kingtous/RustPlayer?style=social)]()

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

## Demo Video

- [Bilibili Video](https://www.bilibili.com/video/BV1T34y1k7Xf)


## Install RustPlayer by Snap Store

`snap install rustplayer --devmode`

## Download Binary Release Directly and Run

The binary release of macOS, Linux, Windows can be found in artifacts of [RustPlayer Release Action](https://github.com/Kingtous/RustPlayer/actions/workflows/rust.yml). Click the top item of the list to download the latest release.

## Screenshots

### Windows

Play with lyrics. If no lyrics found, the wave animation will be the replacement of the block. See screenshots from Linux and macOS below.

![image.png](https://s2.loli.net/2022/03/04/SbK6RN7tXAym4g3.png)

### Linux 

The screenshot from Deepin

![Deepin RustPlayer](https://s2.loli.net/2022/03/03/YtJWvnDuV4rHs7T.png)

### macOS

![macOS RustPlayer](https://s2.loli.net/2022/03/03/Z9altpG63qk24W8.png)

## Compile RustPlayer and run

If u found this binary release is not working or u like compiling RustPlayer by youselef. Yes, The step to compile RustPlayer is really easy.

- clone this repo.
  - for arch/manjaro, please use [fix/arch](https://github.com/Kingtous/RustPlayer/tree/fix/arch) branch.
- install dependencies
    - check `.github/rust.yml` for details
- `cargo run` in root of this project.

if u think this repo is helpful, ⭐ this project and let me know :)

## TroubleShoot

### Linux

Q: No sound in Linux, console shows "unable to open slave". I'm using `snd_hda_intel` drivers.

A: check your valid sound card. Check by `lspci -knn|grep -iA2 audio`. An example is:
```
04:00.1 Audio device [0403]: Advanced Micro Devices, Inc. [AMD/ATI] Renoir Radeon High Definition Audio Controller [1002:1637]
        Subsystem: Lenovo Device [17aa:3814]
        Kernel driver in use: snd_hda_intel
--
04:00.5 Multimedia controller [0480]: Advanced Micro Devices, Inc. [AMD] ACP/ACP3X/ACP6x Audio Coprocessor [1022:15e2] (rev 01)
        Subsystem: Lenovo Device [17aa:3832]
        Kernel modules: snd_pci_acp3x, snd_rn_pci_acp3x, snd_pci_acp5x
04:00.6 Audio device [0403]: Advanced Micro Devices, Inc. [AMD] Family 17h/19h HD Audio Controller [1022:15e3]
        Subsystem: Lenovo Device [17aa:3833]
        Kernel driver in use: snd_hda_intel
```

In the case above, 2 audio devices found in your Linux. Let's check which device is in use, we will use `index` to identify the default device. Type `modinfo snd_hda_intel | grep index`, if only shows:

```
parm: index:Index value for Intel HD audio interface. (array of int)
```

which means index 0 will be chosen to be the default output device.

In this case, you can try device 1. create files below:
```shell
> cat /etc/modprobe.d/default.conf                

options snd_hda_intel index=1
```

reboot and check if it works.
