use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to initialize format context")]
    InitializeFormatContext,
    #[error("Could not find stream in file")]
    FindStreamInfo,
    #[error("Could not find any audio stream")]
    NoAudioStream,
    #[error("Null codec pointer")]
    NullCodec,
    #[error("Null codec context pointer")]
    NullCodecContext,
    #[error("Copying params to codec context")]
    CodecParamsToContext,
    #[error("Failed to initialize decoder")]
    InitializeDecoder,
    #[error("Null frame pointer")]
    NullFrame,
    #[error("Error reading frame: {0}")]
    ReadFrame(i32),
    #[error("Error sending packet: {0}")]
    SendPacket(i32),
    #[error("Error draining decoder: {0}")]
    DrainDecoder(i32),
    #[error("Error receiving frame: {0}")]
    ReceiveFrame(i32),
    #[error("Failed to initialize swr context")]
    InitializeSwr,
}
