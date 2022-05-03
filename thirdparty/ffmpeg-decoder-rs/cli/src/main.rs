use anyhow::Error;
use env_logger::Env;
use log::{error, info};
use structopt::StructOpt;

use rodio::Sink;

use std::path::PathBuf;

fn main() {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let opts = Opts::from_args();

    let status = match opts.command {
        Command::Convert { input } => decode_to_file(input),
        Command::Play { input } => play_file(input),
    };

    if let Err(e) = status {
        log_error(e);
        std::process::exit(1);
    }
}

fn decode_to_file(input: PathBuf) -> Result<(), Error> {
    let decoder = ffmpeg_decoder::Decoder::open(&input)?;

    let samples = decoder.collect::<Vec<i16>>();

    let samples_u8 =
        unsafe { std::slice::from_raw_parts(samples.as_ptr() as *const u8, samples.len() * 2) };

    let mut out_path: PathBuf = input;
    out_path.set_extension("raw");

    std::fs::write(&out_path, samples_u8)?;

    info!(
        "File successfully decoded, converted and saved to: {:?}",
        out_path
    );

    Ok(())
}

fn play_file(input: PathBuf) -> Result<(), Error> {
    let decoder = ffmpeg_decoder::Decoder::open(&input)?;

    let device = rodio::default_output_device().unwrap();
    let sink = Sink::new(&device);

    sink.append(decoder);
    sink.play();
    sink.sleep_until_end();

    Ok(())
}

fn log_error(e: Error) {
    error!("{}", e);
}

#[derive(StructOpt)]
#[structopt(
    name = "libav-decoder-cli",
    about = "Convert input audio file sample format to signed 16bit"
)]
struct Opts {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt)]
enum Command {
    /// Convert file and save as `.raw` alongside input file
    Convert {
        /// Input audio file
        #[structopt(parse(from_os_str))]
        input: PathBuf,
    },
    /// Play input file
    Play {
        /// Input audio file
        #[structopt(parse(from_os_str))]
        input: PathBuf,
    },
}
