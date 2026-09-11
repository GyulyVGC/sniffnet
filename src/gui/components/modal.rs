use iced::alignment::Alignment;
use iced::widget::{
    Checkbox, Column, Container, Row, Space, Text, button, center, mouse_area, opaque, stack,
};
use iced::{Element, Length};

use crate::gui::components::button::button_hide;
use crate::gui::styles::button::ButtonType;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::rule::RuleType;
use crate::gui::styles::style_constants::FONT_SIZE_TITLE;
use crate::gui::styles::types::gradient_type::GradientType;
use crate::gui::types::config_updates::ConfigUpdates;
use crate::gui::types::message::Message;
use crate::gui::types::update_status::UpdateStatus;
use crate::translations::translations::{
    ask_clear_all_translation, ask_quit_translation, clear_all_translation,
    quit_analysis_translation, yes_translation,
};
use crate::translations::translations_6::{
    disable_update_checks_translation, notify_on_updates_translation, update_status_translation,
};
use crate::{Language, StyleType};

pub fn get_exit_overlay<'a>(
    message: Message,
    color_gradient: GradientType,
    language: Language,
) -> Container<'a, Message, StyleType> {
    let row_buttons = confirm_button_row(language, message);

    let content = Column::new()
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .push(get_modal_header(
            color_gradient,
            language,
            quit_analysis_translation(language),
        ))
        .push(Space::new().height(20))
        .push(ask_quit_translation(language).align_x(Alignment::Center))
        .push(row_buttons);

    Container::new(content)
        .height(160)
        .width(450)
        .class(ContainerType::Modal)
}

pub fn get_clear_all_overlay<'a>(
    color_gradient: GradientType,
    language: Language,
) -> Container<'a, Message, StyleType> {
    let row_buttons = confirm_button_row(language, Message::ClearAllNotifications);

    let content = Column::new()
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .push(get_modal_header(
            color_gradient,
            language,
            clear_all_translation(language),
        ))
        .push(Space::new().height(20))
        .push(ask_clear_all_translation(language).align_x(Alignment::Center))
        .push(row_buttons);

    Container::new(content)
        .height(160)
        .width(450)
        .class(ContainerType::Modal)
}

pub fn get_update_status_overlay<'a>(
    color_gradient: GradientType,
    language: Language,
    config_updates: ConfigUpdates,
    update_status: &UpdateStatus,
    dots_pulse: &(String, u8),
) -> Container<'a, Message, StyleType> {
    let notify_updates_checkbox = if config_updates.disable_checks() {
        Checkbox::new(false)
            .label(notify_on_updates_translation(language))
            .size(18)
    } else {
        Checkbox::new(config_updates.notify_updates())
            .label(notify_on_updates_translation(language))
            .size(18)
            .on_toggle(move |_| Message::ToggleNotifyUpdates)
    };

    let disable_checks_checkbox = Checkbox::new(config_updates.disable_checks())
        .label(disable_update_checks_translation(language))
        .on_toggle(move |_| Message::ToggleDisableUpdateChecks)
        .size(18);

    let checkboxes = center(
        Column::new()
            .spacing(10)
            .align_x(Alignment::Start)
            .push(notify_updates_checkbox)
            .push(disable_checks_checkbox),
    );

    let content = Column::new()
        .padding(10)
        .width(Length::Fill)
        .push(update_status.desc(language, dots_pulse.0.len()))
        .push(RuleType::Standard.horizontal(10))
        .push(checkboxes);

    Container::new(
        Column::new()
            .push(get_modal_header(
                color_gradient,
                language,
                update_status_translation(language),
            ))
            .push(content),
    )
    .width(700)
    .height(300)
    .class(ContainerType::Modal)
}

fn get_modal_header<'a>(
    color_gradient: GradientType,
    language: Language,
    title: &'static str,
) -> Container<'a, Message, StyleType> {
    Container::new(
        Row::new()
            .push(Space::new().width(Length::Fill))
            .push(
                Text::new(title)
                    .size(FONT_SIZE_TITLE)
                    .width(Length::FillPortion(6))
                    .align_x(Alignment::Center),
            )
            .push(
                Container::new(button_hide(Message::HideModal, language))
                    .width(Length::Fill)
                    .align_x(Alignment::Center),
            ),
    )
    .align_x(Alignment::Center)
    .align_y(Alignment::Center)
    .height(40)
    .width(Length::Fill)
    .class(ContainerType::Gradient(color_gradient))
}

fn confirm_button_row<'a>(language: Language, message: Message) -> Row<'a, Message, StyleType> {
    Row::new()
        .height(Length::Fill)
        .align_y(Alignment::Center)
        .push(
            button(
                yes_translation(language)
                    .align_y(Alignment::Center)
                    .align_x(Alignment::Center),
            )
            .padding(5)
            .height(40)
            .width(80)
            .class(ButtonType::Alert)
            .on_press(message),
        )
}

pub fn modal<'a>(
    base: Element<'a, Message, StyleType>,
    content: Element<'a, Message, StyleType>,
    on_blur: impl Into<Option<Message>>,
) -> Element<'a, Message, StyleType> {
    let mut mouse_area = mouse_area(center(opaque(content)).class(ContainerType::ModalBackground));
    if let Some(message) = on_blur.into() {
        mouse_area = mouse_area.on_press(message);
    }
    stack![base, opaque(mouse_area)].into()
}
