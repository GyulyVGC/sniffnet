use crate::enums::sound::Sound::{Bubble, Pop, Tic};
use crate::utility::translations::none_translation;
use crate::Language;
use rodio::{Decoder, OutputStream, Sink};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::thread;

/// Enum representing the possible notification sounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Sound {
    Bubble,
    Pop,
    Tic,
    None,
}

pub const BUBBLE: &[u8] = include_bytes!("../../resources/sounds/clearly.mp3");
pub const POP: &[u8] = include_bytes!("../../resources/sounds/pop.mp3");
pub const TIC: &[u8] = include_bytes!("../../resources/sounds/click.mp3");

impl fmt::Display for Sound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Defines a constant to be used in the picklist in gui notifications page
impl Sound {
    pub(crate) const ALL: [Sound; 4] = [Bubble, Pop, Tic, Sound::None];

    fn mp3_sound(self) -> &'static [u8] {
        match self {
            Bubble => BUBBLE,
            Pop => POP,
            Tic => TIC,
            Sound::None => &[],
        }
    }

    pub fn get_radio_label(&self, language: Language) -> &str {
        match self {
            Sound::Bubble => "Bubble",
            Sound::Pop => "Pop",
            Sound::Tic => "Tic",
            Sound::None => none_translation(language),
        }
    }
}

pub fn play_sound(sound: Sound) {
    let mp3_sound = sound.mp3_sound();
    thread::Builder::new()
        .name("thread_play_sound".to_string())
        .spawn(move || {
            // Get a output stream handle to the default physical sound device
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
            //load data
            let data = std::io::Cursor::new(mp3_sound);
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
