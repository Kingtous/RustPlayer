#!/usr/bin/python3
import os
import platform
import urllib.request
import tarfile

work_dir = os.path.dirname(os.path.abspath(__file__))


def get_os_arch() -> tuple[str, str]:
    
    os_name = platform.system().lower()
    arch = platform.machine().lower()
    return os_name, arch

def download_ffmpeg_static_libs():
    system, arch = get_os_arch()
    if arch == 'x86_64':
        arch = 'x64'
    link = f'https://github.com/eugeneware/ffmpeg-static/releases/download/b6.0/ffmpeg-{system}-{arch}.gz'
    dest_path = os.path.join(work_dir, f'ffmpeg-{system}-{arch}.gz')
    if os.path.exists(dest_path):
        print("ffmpeg static libs already exist. Skipping download.")
    else:
        print(f"Downloading ffmpeg static libs from {link}...")
        urllib.request.urlretrieve(link, dest_path)
    print("Download complete. Extracting...")
    with tarfile.open(dest_path, 'r:gz') as tar:
        tar.extractall(path=work_dir)
    # os.remove(dest_path)


if __name__ == "__main__":
    download_ffmpeg_static_libs()


