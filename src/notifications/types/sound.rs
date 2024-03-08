use std::fmt;
use std::thread;

use iced::alignment::{Horizontal, Vertical};
use iced::widget::Text;
use iced::{Font, Length};
use rodio::{Decoder, OutputStream, Sink};
use serde::{Deserialize, Serialize};

use crate::gui::styles::style_constants::FONT_SIZE_FOOTER;
use crate::notifications::types::sound::Sound::{Gulp, Pop, Swhoosh};
use crate::utils::types::icon::Icon;
use crate::StyleType;

/// Enum representing the possible notification sounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Sound {
    Gulp,
    Pop,
    Swhoosh,
    None,
}

pub const GULP: &[u8] = include_bytes!("../../../resources/sounds/gulp.mp3");
pub const POP: &[u8] = include_bytes!("../../../resources/sounds/pop.mp3");
pub const SWHOOSH: &[u8] = include_bytes!("../../../resources/sounds/swhoosh.mp3");

impl fmt::Display for Sound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Sound {
    pub(crate) const ALL: [Sound; 4] = [Gulp, Pop, Swhoosh, Sound::None];

    fn mp3_sound(self) -> &'static [u8] {
        match self {
            Gulp => GULP,
            Pop => POP,
            Swhoosh => SWHOOSH,
            Sound::None => &[],
        }
    }

    pub fn get_text(self, font: Font) -> iced::widget::Text<'static, StyleType> {
        match self {
            Sound::Gulp => Text::new("Gulp").font(font),
            Sound::Pop => Text::new("Pop").font(font),
            Sound::Swhoosh => Text::new("Swhoosh").font(font),
            Sound::None => Icon::Forbidden.to_text(),
        }
        .size(FONT_SIZE_FOOTER)
        .width(Length::Fill)
        .horizontal_alignment(Horizontal::Center)
        .vertical_alignment(Vertical::Center)
    }
}

pub fn play(sound: Sound, volume: u8) {
    if sound.eq(&Sound::None) || volume == 0 {
        return;
    }
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
            sink.set_volume(f32::from(volume) / 200.0); // set the desired volume
            sink.append(source);
            // The sound plays in a separate thread. This call will block the current thread until the sink
            // has finished playing all its queued sounds.
            sink.sleep_until_end();
        })
        .unwrap();
}
