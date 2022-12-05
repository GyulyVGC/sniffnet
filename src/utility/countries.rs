use iced::{Image, Length};

pub const IT: &[u8] = include_bytes!("../../resources/countries_flags/png-16/italy-16x16-32999.png");
pub const ES: &[u8] = include_bytes!("../../resources/countries_flags/png-16/spain-16x16-33105.png");
pub const DE: &[u8] = include_bytes!("../../resources/countries_flags/png-16/germany-16x16-32989.png");
pub const US: &[u8] = include_bytes!("../../resources/countries_flags/png-16/united-16x16-33137.png");
pub const UNKNOWN: &[u8] = include_bytes!("../../resources/countries_flags/png-16/question.png");

pub fn get_flag(country: &String) -> Image {
    match country.as_str() {
        "IT" => iced::Image::new(iced::image::Handle::from_memory(Vec::from(IT))).width(Length::Units(16)),
        "ES" => iced::Image::new(iced::image::Handle::from_memory(Vec::from(ES))).width(Length::Units(16)),
        "DE" => iced::Image::new(iced::image::Handle::from_memory(Vec::from(DE))).width(Length::Units(16)),
        "US" => iced::Image::new(iced::image::Handle::from_memory(Vec::from(US))).width(Length::Units(16)),
        _ => {iced::Image::new(iced::image::Handle::from_memory(Vec::from(UNKNOWN))).width(Length::Units(16))}
    }
}