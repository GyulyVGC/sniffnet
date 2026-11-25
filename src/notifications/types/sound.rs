use std::fmt;
use std::sync::mpsc;
use std::thread;

use iced::widget::Text;
use rodio::{Decoder, DeviceSinkBuilder, Player, Source, buffer::SamplesBuffer, source::SineWave};
use serde::{Deserialize, Serialize};

use crate::gui::styles::style_constants::FONT_SIZE_FOOTER;
use crate::notifications::types::sound::Sound::{Gulp, Pop, Swhoosh};
use crate::utils::error_logger::{ErrorLogger, Location};
use crate::utils::types::icon::Icon;
use crate::{StyleType, location};

/// Enum representing the possible notification sounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Sound {
    Gulp,
    #[default]
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

    pub fn get_text<'a>(self) -> iced::widget::Text<'a, StyleType> {
        match self {
            Sound::Gulp => Text::new("Gulp"),
            Sound::Pop => Text::new("Pop"),
            Sound::Swhoosh => Text::new("Swhoosh"),
            Sound::None => Icon::Forbidden.to_text(),
        }
        .size(FONT_SIZE_FOOTER)
    }
}

pub fn play(sound: Sound, volume: u8) {
    if sound.eq(&Sound::None) || volume == 0 || cfg!(test) {
        return;
    }
    let mp3_sound = sound.mp3_sound();
    let _ = thread::Builder::new()
        .name("thread_play_sound".to_string())
        .spawn(move || {
            // Get an output stream handle to the default physical sound device
            let Ok(mut stream_handle) = DeviceSinkBuilder::open_default_sink().log_err(location!())
            else {
                return;
            };
            stream_handle.log_on_drop(false);
            let player = Player::connect_new(stream_handle.mixer());
            //load data
            let data = std::io::Cursor::new(mp3_sound);
            // Decode that sound file into a source
            let Ok(source) = Decoder::new(data).log_err(location!()) else {
                return;
            };
            // Play the sound directly on the device
            player.set_volume(f32::from(volume) / 200.0); // set the desired volume
            player.append(source);
            // The sound plays in a separate thread. This call will block the current thread until the sink
            // has finished playing all its queued sounds.
            player.sleep_until_end();
        })
        .log_err(location!());
}

/// Holds the live audio worker for this capture. Dropping the sender releases the device.
#[derive(Default)]
pub struct TrafficSound {
    sender: Option<mpsc::SyncSender<(f32, u8)>>,
}

impl TrafficSound {
    pub fn pulse(&mut self, bytes_per_second: u128, volume: u8) {
        const MIN_PITCH_HZ: f32 = 200.0;
        const MAX_PITCH_HZ: f32 = 600.0;

        if bytes_per_second == 0 || volume == 0 || cfg!(test) {
            return;
        }
        // Interpolate in log pitch across the 20 doublings from 1 KiB/s to 1 GiB/s.
        let position = (((bytes_per_second as f32).log2() - 10.0) / 20.0).clamp(0.0, 1.0);
        let pulse = (
            MIN_PITCH_HZ * (MAX_PITCH_HZ / MIN_PITCH_HZ).powf(position),
            volume,
        );
        if let Some(sender) = &self.sender {
            // Skip a tick if playback is busy. Queued pulses would describe stale traffic.
            let _ = sender.try_send(pulse);
            return;
        }

        let (sender, receiver) = mpsc::sync_channel(0);
        if thread::Builder::new()
            .name("thread_traffic_sound".to_owned())
            .spawn(move || {
                let Ok(mut stream) = DeviceSinkBuilder::open_default_sink() else {
                    return;
                };
                stream.log_on_drop(false);
                let player = Player::connect_new(stream.mixer());
                for (frequency, volume) in std::iter::once(pulse).chain(receiver) {
                    player.set_volume(f32::from(volume) / 500.0);
                    player.append(traffic_tone(frequency));
                    player.sleep_until_end();
                }
            })
            .log_err(location!())
            .is_ok()
        {
            self.sender = Some(sender);
        }
    }
}

fn traffic_tone(frequency: f32) -> SamplesBuffer {
    let sine = SineWave::new(frequency);
    let channels = sine.channels();
    let rate = sine.sample_rate();
    let samples_per_ms = rate.get() as usize / 1000; // SineWave is mono at 48 kHz.
    let attack = 5 * samples_per_ms;
    let last = 750 * samples_per_ms - 1;
    let samples = (0..=last)
        .zip(sine)
        .map(|(i, sample)| {
            let gain = if i < attack {
                let phase = std::f32::consts::PI * i as f32 / attack as f32;
                (1.0 - phase.cos()) * 0.5
            } else {
                (last - i) as f32 / (last - attack) as f32
            };
            sample * gain
        })
        .collect::<Vec<_>>();
    SamplesBuffer::new(channels, rate, samples)
}
