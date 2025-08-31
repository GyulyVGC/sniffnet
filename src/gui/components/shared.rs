use iced::{
    Alignment, Font,
    widget::{Row, Space, Text, Tooltip, button, text::LineHeight, tooltip::Position},
};

use crate::{
    SNIFFNET_TITLECASE,
    gui::{
        components::button::row_open_link_tooltip,
        styles::{
            button::ButtonType,
            container::ContainerType,
            style_constants::{FONT_SIZE_FOOTER, FONT_SIZE_SUBTITLE},
            text::TextType,
            types::style_type::StyleType,
        },
        types::message::Message,
    },
    translations::{translations_2::new_version_available_translation, types::language::Language},
    utils::{
        formatted_strings::APP_VERSION,
        types::{icon::Icon, web_page::WebPage},
    },
};

pub fn get_release_details<'a>(
    language: Language,
    font: Font,
    font_footer: Font,
    newer_release_available: Option<bool>,
) -> Row<'a, Message, StyleType> {
    let mut ret_val = Row::new().align_y(Alignment::Center).push(
        Text::new(format!("{SNIFFNET_TITLECASE} {APP_VERSION}"))
            .size(FONT_SIZE_FOOTER)
            .font(font_footer),
    );
    if let Some(boolean_response) = newer_release_available {
        if boolean_response {
            // a newer release is available on GitHub
            let button = button(
                Icon::Update
                    .to_text()
                    .class(TextType::Danger)
                    .size(18)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .line_height(LineHeight::Relative(0.8)),
            )
            .padding(0)
            .height(35)
            .width(35)
            .class(ButtonType::Alert)
            .on_press(Message::OpenWebPage(WebPage::WebsiteDownload));
            let tooltip = Tooltip::new(
                button,
                row_open_link_tooltip(new_version_available_translation(language), font),
                Position::Top,
            )
            .gap(7.5)
            .class(ContainerType::Tooltip);
            ret_val = ret_val.push(Space::with_width(10)).push(tooltip);
        } else {
            // this is the latest release
            ret_val = ret_val.push(Text::new(" ✔").size(FONT_SIZE_SUBTITLE).font(font_footer));
        }
    }
    ret_val
}
