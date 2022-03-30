include!("../src/util/ffmpeg.rs");

#[test]
fn test_ffmpeg(){
    convert_aac_to_wav();
}