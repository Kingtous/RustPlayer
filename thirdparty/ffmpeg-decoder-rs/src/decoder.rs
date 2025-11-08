use crate::error::Error;

use ffmpeg_sys_next::{
    self, av_frame_alloc, av_frame_free, av_frame_unref, av_freep, av_get_alt_sample_fmt,
    av_get_bytes_per_sample, av_get_sample_fmt_name,
    av_init_packet, av_packet_unref, av_read_frame, av_sample_fmt_is_planar,
    av_samples_alloc, av_samples_get_buffer_size, avcodec_alloc_context3, avcodec_close,
    avcodec_find_decoder, avcodec_free_context, avcodec_open2, avcodec_parameters_to_context,
    avcodec_receive_frame, avcodec_send_packet, avformat_close_input, avformat_find_stream_info,
    avformat_open_input, swr_alloc_set_opts2, swr_convert, swr_get_out_samples, swr_init, AVCodec,
    AVCodecContext, AVFormatContext, AVFrame, AVMediaType, AVPacket, AVSampleFormat, AVStream, AVChannelLayout
};
use std::ffi::{CStr, CString};
use std::path::Path;
use std::ptr;
use std::slice;
use std::time::Duration;

use log::{error, info};

const AVERROR_EOF: i32 = -0x20_464_F45;
const AVERROR_EAGAIN: i32 = -11;
const AVERROR_EDEADLK: i32 = -35;
const DEFAULT_CONVERSION_FORMAT: AVSampleFormat = AVSampleFormat::AV_SAMPLE_FMT_S16;

pub struct Decoder {
    format_ctx: FormatContext,
    stream: Stream,
    codec_ctx: CodecContext,
    frame: Frame,
    packet: Packet,
    swr_ctx: Option<SwrContext>,
    current_frame: Vec<u8>,
    first_frame_stored: bool,
}

impl Decoder {
    pub fn open(path: impl AsRef<Path>) -> Result<Decoder, Error> {
        // Note: No need to register av for newer ffmpeg (>4).
        // unsafe { av_register_all() };

        // Open the file and get the format context
        let format_ctx = FormatContext::open(&path.as_ref().display().to_string())?;

        // Find first audio stream in file
        format_ctx.find_stream_info()?;
        let stream = format_ctx.get_audio_stream()?;

        // Get the streams codec
        let codec = stream.get_codec()?;

        // Setup codec context and intialize
        let codec_ctx = codec.get_context()?;
        codec_ctx.copy_parameters_from_stream(&stream)?;
        codec_ctx.request_non_planar_format();
        codec_ctx.initialize()?;

        print_codec_info(&codec_ctx);

        // Allocate frame
        let frame = Frame::new()?;

        // Initialize packet
        let packet = Packet::new();

        // Initialize swr context, if conversion is needed
        let swr_ctx = if codec_ctx.sample_format() != DEFAULT_CONVERSION_FORMAT {
            Some(SwrContext::new(&codec_ctx)?)
        } else {
            None
        };

        Ok(Decoder {
            format_ctx,
            stream,
            codec_ctx,
            frame,
            packet,
            swr_ctx,
            current_frame: vec![],
            first_frame_stored: false,
        })
    }

    fn read_next_frame(&mut self) -> ReadFrameStatus {
        let status =
            unsafe { av_read_frame(self.format_ctx.inner, self.packet.inner.as_mut_ptr()) };

        match status {
            AVERROR_EOF => ReadFrameStatus::Eof,
            _ if status != 0 => ReadFrameStatus::Other(status),
            _ => ReadFrameStatus::Ok,
        }
    }

    fn send_packet_for_decoding(&mut self) -> SendPacketStatus {
        let status =
            unsafe { avcodec_send_packet(self.codec_ctx.inner, self.packet.inner.as_mut_ptr()) };

        match status {
            0 => SendPacketStatus::Ok,
            _ => SendPacketStatus::Other(status),
        }
    }

    fn receive_decoded_frame(&self) -> ReceiveFrameStatus {
        let status = unsafe { avcodec_receive_frame(self.codec_ctx.inner, self.frame.inner) };

        match status {
            0 => ReceiveFrameStatus::Ok,
            AVERROR_EAGAIN => ReceiveFrameStatus::Again,
            AVERROR_EDEADLK => ReceiveFrameStatus::Deadlk,
            _ => ReceiveFrameStatus::Other(status),
        }
    }

    fn convert_and_store_frame(&mut self) {
        let num_samples = self.frame.num_samples();
        let channel_layout = self.frame.channel_layout();
        let num_channels = channel_layout.nb_channels;

        let extended_data = self.frame.extended_data();

        let mut out_buf = std::ptr::null_mut::<u8>();

        let out_slice = if self.swr_ctx.is_some() {
            let out_samples =
                unsafe { swr_get_out_samples(self.swr_ctx.as_ref().unwrap().inner, num_samples) };

            unsafe {
                av_samples_alloc(
                    &mut out_buf,
                    ptr::null_mut(),
                    num_channels,
                    out_samples,
                    DEFAULT_CONVERSION_FORMAT,
                    0,
                )
            };

            unsafe {
                swr_convert(
                    self.swr_ctx.as_ref().unwrap().inner,
                    &mut out_buf,
                    out_samples,
                    extended_data,
                    num_samples,
                )
            };

            let out_size = unsafe {
                av_samples_get_buffer_size(
                    ptr::null_mut(),
                    num_channels,
                    out_samples,
                    DEFAULT_CONVERSION_FORMAT,
                    0,
                )
            };

            unsafe { slice::from_raw_parts(out_buf, out_size as usize) }
        } else {
            unsafe {
                slice::from_raw_parts(
                    extended_data.as_ref().unwrap().as_ref().unwrap(),
                    self.frame.inner.as_ref().unwrap().linesize[0] as usize,
                )
            }
        };

        if !self.current_frame.is_empty() {
            self.current_frame.drain(..);
        }

        self.current_frame.extend_from_slice(out_slice);

        if self.swr_ctx.is_some() {
            // Free samples buffer
            unsafe { av_freep(&mut out_buf as *mut _ as _) };
        }

        unsafe { av_frame_unref(self.frame.inner) };
    }

    fn frame_for_stream(&self) -> bool {
        unsafe { self.packet.inner.as_ptr().as_ref().unwrap().stream_index == self.stream.index }
    }

    fn reset_packet(&mut self) {
        unsafe { av_packet_unref(self.packet.inner.as_mut_ptr()) };
    }

    fn next_sample(&mut self) -> f32 {
        let sample_u8: [u8; 2] = [self.current_frame.remove(0), self.current_frame.remove(0)];

        (((sample_u8[1] as i16) << 8) | sample_u8[0] as i16) as _
    }

    fn process_next_frame(&mut self) -> Option<Result<(), Error>> {
        match self.read_next_frame() {
            ReadFrameStatus::Ok => {}
            ReadFrameStatus::Eof => {
                return None;
            }
            ReadFrameStatus::Other(status) => {
                error!("{}", Error::ReadFrame(status));
                return None;
            }
        }

        if !self.frame_for_stream() {
            self.reset_packet();
            return self.process_next_frame();
        }

        match self.send_packet_for_decoding() {
            SendPacketStatus::Ok => self.reset_packet(),
            SendPacketStatus::Other(status) => {
                error!("{}", Error::SendPacket(status));
                return None;
            }
        }

        match self.receive_decoded_frame() {
            ReceiveFrameStatus::Ok => {}
            ReceiveFrameStatus::Again | ReceiveFrameStatus::Deadlk => {
                return self.process_next_frame()
            }
            ReceiveFrameStatus::Other(status) => {
                error!("{}", Error::ReceiveFrame(status));
                return None;
            }
        }

        self.convert_and_store_frame();

        Some(Ok(()))
    }

    fn cleanup(&mut self) {
        // Drain the decoder.
        drain_decoder(self.codec_ctx.inner).unwrap();

        unsafe {
            // Free all data used by the frame.
            av_frame_free(&mut self.frame.inner);

            // Close the context and free all data associated to it, but not the context itself.
            avcodec_close(self.codec_ctx.inner);

            // Free the context itself.
            avcodec_free_context(&mut self.codec_ctx.inner);

            // Close the input.
            avformat_close_input(&mut self.format_ctx.inner);
        }
    }

    pub(crate) fn _current_frame_len(&self) -> Option<usize> {
        Some(self.current_frame.len())
    }

    pub(crate) fn _channels(&self) -> u16 {
        self.codec_ctx.channels() as _
    }

    pub(crate) fn _sample_rate(&self) -> u32 {
        self.codec_ctx.sample_rate() as _
    }

    pub(crate) fn _total_duration(&self) -> Option<Duration> {
        //TODO let duration = self.stream.duration();
        None
    }
}

unsafe impl Send for Decoder {}

impl Iterator for Decoder {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if !self.first_frame_stored {
            if self.process_next_frame().is_none() {
                self.cleanup();
                return None;
            }

            self.first_frame_stored = true;

            return Some(self.next_sample());
        }

        if !self.current_frame.is_empty() {
            return Some(self.next_sample());
        }

        match self.receive_decoded_frame() {
            ReceiveFrameStatus::Ok => {
                self.convert_and_store_frame();
                Some(self.next_sample())
            }
            ReceiveFrameStatus::Again | ReceiveFrameStatus::Deadlk => {
                if self.process_next_frame().is_none() {
                    self.cleanup();
                    return None;
                }

                Some(self.next_sample())
            }
            ReceiveFrameStatus::Other(status) => {
                error!("{}", Error::ReceiveFrame(status));
                self.cleanup();
                None
            }
        }
    }
}

struct FormatContext {
    inner: *mut AVFormatContext,
}

impl FormatContext {
    fn open(path: &str) -> Result<FormatContext, Error> {
        let mut inner = std::ptr::null_mut::<AVFormatContext>();

        let path = CString::new(path).unwrap();

        let status = unsafe {
            avformat_open_input(
                &mut inner,
                path.as_ptr(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
        };
        if status != 0 {
            return Err(Error::InitializeFormatContext);
        }

        Ok(FormatContext { inner })
    }

    /// Look at first few frames to determine stream info
    fn find_stream_info(&self) -> Result<(), Error> {
        let status = unsafe { avformat_find_stream_info(self.inner, ptr::null_mut()) };
        if status < 0 {
            return Err(Error::FindStreamInfo);
        }
        Ok(())
    }

    ///  Get the first audio stream
    fn get_audio_stream(&self) -> Result<Stream, Error> {
        let num_streams = unsafe { self.inner.as_ref().unwrap().nb_streams };
        let streams = unsafe { self.inner.as_ref().unwrap().streams };

        let streams = unsafe { slice::from_raw_parts(streams, num_streams as usize) };

        let stream_idx = find_audio_stream(streams)?;

        Ok(Stream::new(streams[0], stream_idx))
    }
}

struct SwrContext {
    inner: *mut ffmpeg_sys_next::SwrContext,
}

impl SwrContext {
    fn new(codec_ctx: &CodecContext) -> Result<SwrContext, Error> {
        unsafe  {
            let mut swr_ctx: *mut ffmpeg_sys_next::SwrContext = std::ptr::null_mut();

            let channel_layout = codec_ctx.channel_layout();

            let ret = swr_alloc_set_opts2(
                &mut swr_ctx,
                &channel_layout as *const AVChannelLayout,
                DEFAULT_CONVERSION_FORMAT,
                codec_ctx.sample_rate(),
                &channel_layout as *const AVChannelLayout,
                codec_ctx.sample_format(),
                codec_ctx.sample_rate(),
                0,
                ptr::null_mut(),
            );
            
            if ret != 0 || swr_ctx.is_null() {
                return Err(Error::InitializeSwr);
            }

            Ok(SwrContext { inner: swr_ctx })
        }
    }
}

struct Packet {
    inner: std::mem::MaybeUninit<AVPacket>,
}

impl Packet {
    fn new() -> Packet {
        let mut packet = std::mem::MaybeUninit::uninit();

        unsafe { av_init_packet(packet.as_mut_ptr()) };

        Packet { inner: packet }
    }
}

struct Frame {
    inner: *mut AVFrame,
}

impl Frame {
    fn new() -> Result<Frame, Error> {
        let frame: *mut AVFrame = unsafe { av_frame_alloc() };

        if frame.is_null() {
            return Err(Error::NullFrame);
        }

        Ok(Frame { inner: frame })
    }

    fn num_samples(&self) -> i32 {
        unsafe { self.inner.as_ref().unwrap().nb_samples }
    }

    fn channel_layout(&self) -> AVChannelLayout {
        unsafe { self.inner.as_ref().unwrap().ch_layout }
    }

    fn extended_data(&self) -> *mut *const u8 {
        unsafe { self.inner.as_ref().unwrap().extended_data as *mut *const u8 }
    }
}

struct Stream {
    inner: *mut AVStream,
    index: i32,
}

impl Stream {
    fn new(inner: *mut AVStream, index: i32) -> Stream {
        Stream { inner, index }
    }

    fn get_codec(&self) -> Result<Codec, Error> {
        // Get streams codec
        let codec_params = unsafe { self.inner.as_ref().unwrap().codecpar };
        let codec_id = unsafe { codec_params.as_ref().unwrap().codec_id };

        let codec: *mut AVCodec = unsafe { avcodec_find_decoder(codec_id) as _ };
        if codec.is_null() {
            return Err(Error::NullCodec);
        }

        Ok(Codec::new(codec))
    }

    #[allow(dead_code)]
    fn duration(&self) -> i64 {
        unsafe { self.inner.as_ref().unwrap().duration }
    }
}

struct CodecContext {
    inner: *mut AVCodecContext,
    codec: *mut AVCodec,
}

impl CodecContext {
    fn new(inner: *mut AVCodecContext, codec: *mut AVCodec) -> CodecContext {
        CodecContext { inner, codec }
    }

    fn copy_parameters_from_stream(&self, stream: &Stream) -> Result<(), Error> {
        let params = unsafe { stream.inner.as_ref().unwrap().codecpar };

        let status = unsafe { avcodec_parameters_to_context(self.inner, params) };

        if status != 0 {
            return Err(Error::CodecParamsToContext);
        }

        Ok(())
    }

    fn request_non_planar_format(&self) {
        unsafe {
            let sample_fmt = self.inner.as_ref().unwrap().sample_fmt;
            let alt_format = av_get_alt_sample_fmt(sample_fmt, 0);

            self.inner.as_mut().unwrap().request_sample_fmt = alt_format;
        }
    }

    fn initialize(&self) -> Result<(), Error> {
        let status = unsafe { avcodec_open2(self.inner, self.codec, &mut std::ptr::null_mut()) };

        if status != 0 {
            return Err(Error::InitializeDecoder);
        }

        Ok(())
    }

    fn codec_name(&self) -> &str {
        let name = unsafe { CStr::from_ptr(self.codec.as_ref().unwrap().long_name) };

        name.to_str().unwrap()
    }

    fn sample_format(&self) -> AVSampleFormat {
        unsafe { self.inner.as_ref().unwrap().sample_fmt }
    }

    fn sample_format_name(&self) -> &str {
        let sample_fmt = unsafe { CStr::from_ptr(av_get_sample_fmt_name(self.sample_format())) };

        sample_fmt.to_str().unwrap()
    }

    fn sample_rate(&self) -> i32 {
        unsafe { self.inner.as_ref().unwrap().sample_rate }
    }

    fn sample_size(&self) -> i32 {
        unsafe { av_get_bytes_per_sample(self.inner.as_ref().unwrap().sample_fmt) }
    }

    fn channels(&self) -> i32 {
        unsafe { self.inner.as_ref().unwrap().ch_layout.nb_channels }
    }

    fn channel_layout(&self) -> AVChannelLayout {
        unsafe { self.inner.as_ref().unwrap().ch_layout }
    }

    fn is_planar(&self) -> i32 {
        unsafe { av_sample_fmt_is_planar(self.inner.as_ref().unwrap().sample_fmt) }
    }
}

struct Codec {
    inner: *mut AVCodec,
}

impl Codec {
    fn new(inner: *mut AVCodec) -> Codec {
        Codec { inner }
    }

    fn get_context(&self) -> Result<CodecContext, Error> {
        let ctx: *mut AVCodecContext = unsafe { avcodec_alloc_context3(self.inner) };

        if ctx.is_null() {
            return Err(Error::NullCodecContext);
        }

        Ok(CodecContext::new(ctx, self.inner))
    }
}

enum ReadFrameStatus {
    Ok,
    Eof,
    Other(i32),
}

enum SendPacketStatus {
    Ok,
    Other(i32),
}

enum ReceiveFrameStatus {
    Ok,
    Again,
    Deadlk,
    Other(i32),
}

fn find_audio_stream(streams: &[*mut AVStream]) -> Result<i32, Error> {
    for stream in streams {
        let codec_type = unsafe {
            stream
                .as_ref()
                .unwrap()
                .codecpar
                .as_ref()
                .unwrap()
                .codec_type
        };
        let index = unsafe { stream.as_ref().unwrap().index };

        if codec_type == AVMediaType::AVMEDIA_TYPE_AUDIO {
            return Ok(index);
        }
    }

    Err(Error::NoAudioStream)
}

fn print_codec_info(codec_ctx: &CodecContext) {
    info!("Codec:         {}", codec_ctx.codec_name());
    info!("Sample Format: {}", codec_ctx.sample_format_name());
    info!("Sample Rate:   {}", codec_ctx.sample_rate());
    info!("Sample Size:   {}", codec_ctx.sample_size());
    info!("Channels:      {}", codec_ctx.channels());
    info!("Planar:        {}", codec_ctx.is_planar());
}

fn drain_decoder(codec_ctx: *mut AVCodecContext) -> Result<(), Error> {
    let status = unsafe { avcodec_send_packet(codec_ctx, std::ptr::null()) };
    if status == 0 {
    } else {
        return Err(Error::DrainDecoder(status));
    }

    Ok(())
}
