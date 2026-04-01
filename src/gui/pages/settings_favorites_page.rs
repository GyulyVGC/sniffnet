use crate::gui::components::tab::get_settings_tabs;
use crate::gui::pages::settings_notifications_page::settings_header;
use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::styles::button::ButtonType;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::scrollbar::ScrollbarType;
use crate::gui::styles::style_constants::FONT_SIZE_SUBTITLE;
use crate::gui::styles::text::TextType;
use crate::gui::types::favorite::FavoriteKey;
use crate::gui::types::message::Message;
use crate::gui::types::settings::Settings;
use crate::networking::types::host::Host;
use crate::networking::types::program::Program;
use crate::networking::types::service::Service;
use crate::translations::translations_2::host_translation;
use crate::translations::translations_3::service_translation;
use crate::translations::translations_5::program_translation;
use crate::utils::types::icon::Icon;
use crate::{Sniffer, StyleType};
use iced::widget::scrollable::Direction;
use iced::widget::{Column, Container, Row, Scrollable, Space, Text, button};
use iced::{Alignment, Length, Padding};

pub fn settings_favorites_page(sniffer: &Sniffer) -> Container<'_, Message, StyleType> {
    let Settings {
        language,
        color_gradient,
        favorites,
        ..
    } = &sniffer.conf.settings;

    let mut hosts: Vec<&Host> = favorites.hosts().iter().collect();
    hosts.sort_by(|a, b| a.to_entry_string().cmp(&b.to_entry_string()));

    let mut services: Vec<&Service> = favorites.services().iter().collect();
    services.sort_by(|a, b| a.to_string().cmp(&b.to_string()));

    let mut programs: Vec<&Program> = favorites.programs().iter().collect();
    programs.sort_by(|a, b| a.to_string().cmp(&b.to_string()));

    let is_empty = hosts.is_empty() && services.is_empty() && programs.is_empty();

    let mut favorites_col = Column::new()
        .spacing(10)
        .padding(Padding::ZERO.left(40).right(40).bottom(10))
        .align_x(Alignment::Center)
        .width(Length::Fill);

    if is_empty {
        favorites_col = favorites_col.push(Space::new().height(20)).push(
            Text::new(match *language {
                _ => "No favorites yet — use the star button to add some!",
            })
            .align_x(Alignment::Center)
            .width(Length::Fill),
        );
    } else {
        if !hosts.is_empty() {
            favorites_col = favorites_col.push(favorite_section(
                host_translation(*language),
                hosts.iter().map(|h| {
                    let key = FavoriteKey::Host((*h).clone());
                    let label = h.to_entry_string();
                    (key, label)
                }),
            ));
        }
        if !services.is_empty() {
            favorites_col = favorites_col.push(favorite_section(
                service_translation(*language),
                services.iter().map(|s| {
                    let key = FavoriteKey::Service(**s);
                    let label = s.to_string();
                    (key, label)
                }),
            ));
        }
        if !programs.is_empty() {
            favorites_col = favorites_col.push(favorite_section(
                program_translation(*language),
                programs.iter().map(|p| {
                    let key = FavoriteKey::Program((*p).clone());
                    let label = p.to_string();
                    (key, label)
                }),
            ));
        }
    }

    let content = Column::new()
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .push(settings_header(*color_gradient, *language))
        .push(get_settings_tabs(SettingsPage::Favorites, *language))
        .push(Space::new().height(10))
        .push(Scrollable::with_direction(
            favorites_col,
            Direction::Vertical(ScrollbarType::properties().margin(15)),
        ));

    Container::new(content)
        .height(400)
        .width(800)
        .class(ContainerType::Modal)
}

fn favorite_section<'a>(
    title: &'static str,
    items: impl Iterator<Item = (FavoriteKey, String)>,
) -> Container<'a, Message, StyleType> {
    let mut col = Column::new()
        .spacing(4)
        .width(Length::Fill)
        .push(
            Text::new(title)
                .class(TextType::Subtitle)
                .size(FONT_SIZE_SUBTITLE),
        )
        .push(Space::new().height(2));

    for (key, label) in items {
        col = col.push(favorite_row(key, label));
    }

    Container::new(col).width(Length::Fill)
}

fn favorite_row<'a>(key: FavoriteKey, label: String) -> Row<'a, Message, StyleType> {
    Row::new()
        .align_y(Alignment::Center)
        .spacing(5)
        .push(
            button(
                Icon::StarFull
                    .to_text()
                    .size(16)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center),
            )
            .padding(0)
            .height(25)
            .width(25)
            .class(ButtonType::Starred)
            .on_press(Message::AddOrRemoveFavorite(key, false)),
        )
        .push(
            Text::new(label)
                .width(Length::Fill)
                .align_y(Alignment::Center),
        )
}
