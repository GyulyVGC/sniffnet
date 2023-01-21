use rodio::{Decoder, OutputStream, Sink};
use std::thread;

pub const SOUND: &[u8] = include_bytes!("../../resources/sounds/clearly.mp3");

pub fn play_sound() {
    thread::Builder::new()
        .name("thread_play_sound".to_string())
        .spawn(move || {
            // Get a output stream handle to the default physical sound device
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
            //load data
            let data = std::io::Cursor::new(SOUND);
            // Decode that sound file into a source
            let source = Decoder::new(data).unwrap();
            // Play the sound directly on the device
            sink.set_volume(0.1);
            sink.append(source);
            // The sound plays in a separate thread. This call will block the current thread until the sink
            // has finished playing all its queued sounds.
            sink.sleep_until_end();
        })
        .unwrap();
}
