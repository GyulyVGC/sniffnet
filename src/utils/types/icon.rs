use iced::widget::Text;

use crate::StyleType;
use crate::gui::styles::style_constants::ICONS;

pub enum Icon {
    ArrowBack,
    ArrowLeft,
    ArrowRight,
    ArrowsDown,
    AudioHigh,
    AudioMute,
    Bin,
    Book,
    BytesThreshold,
    Clock,
    Copy,
    Generals,
    Error,
    Feedback,
    File,
    Forbidden,
    Funnel,
    GitHub,
    // Globe,
    HalfSun,
    Hourglass1,
    Hourglass2,
    Hourglass3,
    Inspect,
    Lightning,
    Moon,
    News,
    Notification,
    OpenLink,
    Overview,
    PacketsThreshold,
    // Restore,
    Rocket,
    Settings,
    Sniffnet,
    SortAscending,
    SortDescending,
    SortNeutral,
    Star,
    Sun,
    ThumbnailOpen,
    ThumbnailClose,
    Update,
    Warning,
    Waves,
}

impl Icon {
    pub fn codepoint(&self) -> char {
        match self {
            Icon::ArrowBack => 'C',
            Icon::ArrowLeft => 'i',
            Icon::ArrowRight => 'j',
            Icon::ArrowsDown => ':',
            Icon::AudioHigh => 'Z',
            Icon::AudioMute => 'Y',
            Icon::Bin => 'h',
            Icon::BytesThreshold => 'f',
            Icon::Clock => '9',
            Icon::Generals => 'Q',
            Icon::Error => 'U',
            Icon::File => '8',
            Icon::Forbidden => 'x',
            Icon::Funnel => 'V',
            Icon::GitHub => 'H',
            // Icon::Globe => 'c',
            Icon::HalfSun => 'K',
            Icon::Hourglass1 => '1',
            Icon::Hourglass2 => '2',
            Icon::Hourglass3 => '3',
            Icon::Inspect => '5',
            Icon::Lightning => 'z',
            Icon::Moon => 'G',
            Icon::Notification => '7',
            Icon::Overview => 'd',
            Icon::PacketsThreshold => 'e',
            // Icon::Restore => 'k',
            Icon::Rocket => 'S',
            Icon::Settings => 'a',
            Icon::Sniffnet => 'A',
            Icon::Star => 'g',
            Icon::Warning => 'T',
            Icon::Waves => 'y',
            Icon::Copy => 'u',
            Icon::SortAscending => 'm',
            Icon::SortDescending => 'l',
            Icon::SortNeutral => 'n',
            Icon::Sun => 'F',
            Icon::OpenLink => 'o',
            Icon::ThumbnailOpen => 's',
            Icon::ThumbnailClose => 'r',
            Icon::Book => 'B',
            Icon::Feedback => '=',
            Icon::News => '>',
            Icon::Update => '<',
        }
    }

    pub fn to_text<'a>(&self) -> Text<'a, StyleType> {
        Text::new(self.codepoint().to_string()).font(ICONS)
    }

    pub fn get_hourglass<'a>(num: usize) -> Text<'a, StyleType> {
        match num {
            1 => Icon::Hourglass1.to_text(),
            2 => Icon::Hourglass2.to_text(),
            3 => Icon::Hourglass3.to_text(),
            _ => Text::new(""),
        }
    }
}
