use std::path::Path;
use std::sync::Arc;

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Button, Column, Container, Row, Scrollable, Space, Text, TextInput};
use iced::{Alignment, Element, Length};
use iced_native::widget::{horizontal_space, vertical_space, Rule};

use crate::gui::components::tab::get_settings_tabs;
use crate::gui::pages::settings_notifications_page::settings_header;
use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::styles::style_constants::{get_font, BORDER_WIDTH, FONT_SIZE_SUBTITLE};
use crate::gui::styles::types::custom_style::CustomStyle;
use crate::gui::styles::types::element_type::ElementType;
use crate::gui::styles::types::style_tuple::StyleTuple;
use crate::gui::types::message::Message;
use crate::translations::translations::{
    appearance_title_translation, custom_theme_path, deep_sea_translation, mon_amour_translation,
    yeti_day_translation, yeti_night_translation,
};
use crate::translations::types::language::Language;
use crate::StyleType::{Day, DeepSea, MonAmour, Night};
use crate::{Sniffer, StyleType};

pub fn settings_style_page(sniffer: &Sniffer) -> Container<Message> {
    let font = get_font(&sniffer.style);
    let mut content = Column::new()
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .push(settings_header(&sniffer.style, sniffer.language))
        .push(get_settings_tabs(
            [
                SettingsPage::Notifications,
                SettingsPage::Appearance,
                SettingsPage::Language,
            ],
            &["7 ", "K ", "c "],
            &[
                Message::OpenSettings(SettingsPage::Notifications),
                Message::TickInit,
                Message::OpenSettings(SettingsPage::Language),
            ],
            SettingsPage::Appearance,
            &sniffer.style,
            sniffer.language,
        ))
        .push(vertical_space(Length::Fixed(15.0)))
        .push(
            appearance_title_translation(sniffer.language)
                .font(font)
                .size(FONT_SIZE_SUBTITLE),
        )
        .push(vertical_space(Length::Fixed(10.0)))
        .push(
            Row::new()
                .push(get_palette_container(
                    &sniffer.style,
                    "Yeti Night".to_string(),
                    yeti_night_translation(sniffer.language).to_string(),
                    &Arc::new(Night),
                ))
                .push(horizontal_space(Length::Fixed(15.0)))
                .push(get_palette_container(
                    &sniffer.style,
                    "Yeti Day".to_string(),
                    yeti_day_translation(sniffer.language).to_string(),
                    &Arc::new(Day),
                )),
        )
        .push(vertical_space(Length::Fixed(10.0)))
        .push(
            Row::new()
                .push(get_palette_container(
                    &sniffer.style,
                    "Deep Sea".to_string(),
                    deep_sea_translation(sniffer.language).to_string(),
                    &Arc::new(DeepSea),
                ))
                .push(horizontal_space(Length::Fixed(15.0)))
                .push(get_palette_container(
                    &sniffer.style,
                    "Mon Amour".to_string(),
                    mon_amour_translation(sniffer.language).to_string(),
                    &Arc::new(MonAmour),
                )),
        );

    // Append custom style buttons if any exist
    if let Some(custom_styles) = custom_styles_conf_dir(sniffer.language) {
        content = content.push(vertical_space(Length::Fixed(10.0)));
        for child in custom_styles {
            content = content.push(child);
        }
    }

    // Append text box to manually load custom styles from a TOML file
    content = content
        .push(vertical_space(Length::Fixed(10.0)))
        // Custom theme text box
        .push(
            Row::new().push(
                TextInput::new(
                    custom_theme_path(sniffer.language),
                    sniffer.style_path_update.as_deref().unwrap_or_default(),
                )
                .font(font)
                // Iced's TextInput has an on_submit function but there's no callback
                .on_input(Message::UpdateStylePath)
                .on_paste(Message::PasteCustomStyle)
                .on_submit(Message::LoadCustomStyle)
                .padding(10)
                .style(StyleTuple(
                    Arc::clone(&sniffer.style),
                    ElementType::Standard,
                )),
            ),
        );

    // Wrap content into a scrollable widget
    let content = Scrollable::new(content);

    Container::new(content)
        .height(Length::Fixed(440.0))
        .width(Length::Fixed(800.0))
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(Arc::clone(&sniffer.style), ElementType::Standard),
        ))
}

fn get_palette_container(
    style: &Arc<StyleType>,
    name: String,
    description: String,
    on_press: &Arc<StyleType>,
) -> Button<'static, Message> {
    let font = get_font(style);
    let content = Column::new()
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .spacing(5)
        .push(Text::new(name).font(font))
        .push(get_palette(on_press))
        .push(Text::new(description).font(font));

    Button::new(content)
        .height(Length::Fixed(120.0))
        .width(Length::Fixed(380.0))
        .padding(5)
        .style(
            StyleTuple(
                Arc::clone(style),
                if on_press.eq(style) {
                    ElementType::BorderedRoundSelected
                } else {
                    ElementType::BorderedRound
                },
            )
            .into(),
        )
        .on_press(Message::Style(Arc::clone(on_press)))
}

fn get_palette(style: &Arc<StyleType>) -> Container<'static, Message> {
    Container::new(
        Row::new()
            .padding(0)
            .push(Row::new().padding(0).width(Length::Fixed(120.0)).push(
                Rule::horizontal(50).style(<StyleTuple as Into<iced::theme::Rule>>::into(
                    StyleTuple(Arc::clone(style), ElementType::PalettePrimary),
                )),
            ))
            .push(Row::new().padding(0).width(Length::Fixed(80.0)).push(
                Rule::horizontal(50).style(<StyleTuple as Into<iced::theme::Rule>>::into(
                    StyleTuple(Arc::clone(style), ElementType::PaletteSecondary),
                )),
            ))
            .push(Row::new().padding(0).width(Length::Fixed(60.0)).push(
                Rule::horizontal(50).style(<StyleTuple as Into<iced::theme::Rule>>::into(
                    StyleTuple(Arc::clone(style), ElementType::PaletteOutgoing),
                )),
            ))
            .push(Row::new().padding(0).width(Length::Fixed(40.0)).push(
                Rule::horizontal(50).style(<StyleTuple as Into<iced::theme::Rule>>::into(
                    StyleTuple(Arc::clone(style), ElementType::PaletteButtons),
                )),
            )),
    )
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .width(300.0 + 2.0 * BORDER_WIDTH)
    .height(50.0 + 1.7 * BORDER_WIDTH)
    .style(<StyleTuple as Into<iced::theme::Container>>::into(
        StyleTuple(Arc::clone(style), ElementType::Palette),
    ))
}

#[inline]
fn custom_styles_conf_dir(lang: Language) -> Option<Vec<Element<'static, Message>>> {
    confy::get_configuration_file_path("sniffnet", None)
        .ok()
        .and_then(|mut path| {
            path.pop();
            path.push("themes");
            load_custom_styles(path, lang)
        })
}

// Load and process `CustomStyles`.
fn load_custom_styles<P>(path: P, lang: Language) -> Option<Vec<Element<'static, Message>>>
where
    P: AsRef<Path>,
{
    // Lazily load `CustomStyles` and process them into GUI elements.
    let mut styles = CustomStyle::from_dir(path.as_ref()).ok()?.map(|style| {
        let name = style.name.clone();
        let description = style.description(lang).to_owned();
        let style = Arc::new(StyleType::Custom(style));
        get_palette_container(&style, name, description, &style)
    });

    // The easiest way to do this is with itertools, but I don't want to introduce another
    // dependency just for this one function. So I'll do it iteratively for now.
    let mut children = if let (_, Some(capacity)) = styles.size_hint() {
        Vec::with_capacity(capacity)
    } else {
        Vec::new()
    };

    // This handles the case where there aren't an even number of styles.
    // [Iterator::zip] drops remainders. Itertools' `zip_longest` and the unstable array chunks API
    // are both better solutions.
    while let (Some(first), second) = (styles.next(), styles.next()) {
        // Add both styles and the vertical space to a row if there are two styles.
        if let Some(second) = second {
            children.extend([
                Row::new()
                    .push(first)
                    .push(horizontal_space(Length::Fixed(15.0)))
                    .push(second)
                    .into(),
                <Space as Into<iced::Element<Message>>>::into(vertical_space(Length::Fixed(10.0))),
            ]);
        }
        // Or else just add the single style and the space
        else {
            children.extend([
                Row::new().push(first).into(),
                <Space as Into<iced::Element<Message>>>::into(vertical_space(Length::Fixed(10.0))),
            ]);
        }
    }

    Some(children)
}

#[cfg(test)]
mod tests {
    use std::{fs::read_dir, io};

    use super::load_custom_styles;
    use crate::Language;

    #[test]
    fn test_deserialize_all_builtin_styles() -> Result<(), io::Error> {
        let path = format!("{}/resources/themes", env!("CARGO_MANIFEST_DIR"));
        // The counts `Result<DirEntry>`, but that doesn't matter since none of the files should fail
        let count = read_dir(&path)?.count();
        let themes = load_custom_styles(path, Language::EN)
            .expect("should be able to read all of the custom themes");

        // Make sure number of themes matches number of dir entries
        assert_eq!(count, themes.len());

        Ok(())
    }
}
