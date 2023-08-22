use crate::gui::components::tab::get_settings_tabs;
use crate::gui::pages::settings_notifications_page::settings_header;
use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::style_constants::{get_font, get_font_headers, FONT_SIZE_SUBTITLE};
use crate::gui::styles::text::TextType;
use crate::gui::types::message::Message;
use crate::translations::translations_3::{
    advanced_settings_translation, scale_factor_translation,
};
use crate::{Language, Sniffer, StyleType};
use iced::advanced::widget::Text;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{vertical_space, Column, Container, Slider};
use iced::Length::Fixed;
use iced::{Alignment, Font, Length, Renderer};

pub fn settings_advanced_page(sniffer: &Sniffer) -> Container<Message, Renderer<StyleType>> {
    let font = get_font(sniffer.style);
    let font_headers = get_font_headers(sniffer.style);

    let content = Column::new()
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .push(settings_header(
            font,
            font_headers,
            sniffer.color_gradient,
            sniffer.language,
        ))
        .push(get_settings_tabs(
            SettingsPage::Advanced,
            font,
            sniffer.language,
        ))
        .push(vertical_space(Fixed(15.0)))
        .push(
            Text::new(advanced_settings_translation(sniffer.language))
                .style(TextType::Subtitle)
                .font(font)
                .size(FONT_SIZE_SUBTITLE),
        )
        .push(vertical_space(Fixed(5.0)))
        .push(scale_factor_slider(
            sniffer.language,
            font,
            sniffer.advanced_settings.scale_factor,
        ));

    Container::new(content)
        .height(Fixed(400.0))
        .width(Fixed(800.0))
        .style(ContainerType::Modal)
}

fn scale_factor_slider(
    language: Language,
    font: Font,
    scale_factor: f64,
) -> Container<'static, Message, Renderer<StyleType>> {
    Container::new(
        Column::new()
            .spacing(5)
            .align_items(Alignment::Center)
            .push(
                Text::new(format!(
                    "{}: x{scale_factor:.2}",
                    scale_factor_translation(language)
                ))
                .font(font),
            )
            .push(
                Slider::new(0.5..=1.5, scale_factor, Message::ChangeScaleFactor)
                    .step(0.05)
                    .width(Fixed(150.0)),
            ),
    )
    .padding(5)
    .width(Length::FillPortion(1))
    .height(Length::Fill)
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
}
