use rodio::{source::Source, Decoder, OutputStream};
use std::io::Cursor;

static ALERT_SOUND: &[u8] = include_bytes!("../data/alert-sound.ogg");

pub fn play_alert() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let file = Cursor::new(ALERT_SOUND);

    let source = Decoder::new_vorbis(file).unwrap();

    stream_handle.play_raw(source.convert_samples()).unwrap();
}
