//! Module defining the application structure: messages, updates, subscriptions.
//!
//! It also is a wrapper of gui's main two pages: initial and run page.

use std::time::Duration;

use iced::keyboard::{Event, KeyCode, Modifiers};
use iced::widget::Column;
use iced::Event::{Keyboard, Window};
use iced::{
    executor, font, subscription, window, Application, Command, Element, Renderer, Subscription,
};

use crate::gui::components::footer::footer;
use crate::gui::components::header::header;
use crate::gui::components::modal::{get_clear_all_overlay, get_exit_overlay, Modal};
use crate::gui::components::types::my_modal::MyModal;
use crate::gui::pages::connection_details_page::connection_details_page;
use crate::gui::pages::initial_page::initial_page;
use crate::gui::pages::inspect_page::inspect_page;
use crate::gui::pages::notifications_page::notifications_page;
use crate::gui::pages::overview_page::overview_page;
use crate::gui::pages::settings_general_page::settings_general_page;
use crate::gui::pages::settings_notifications_page::settings_notifications_page;
use crate::gui::pages::settings_style_page::settings_style_page;
use crate::gui::pages::types::running_page::RunningPage;
use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::styles::style_constants::{
    get_font, get_font_headers, ICONS_BYTES, SARASA_MONO_BOLD_BYTES, SARASA_MONO_BYTES,
};
use crate::gui::types::message::Message;
use crate::gui::types::sniffer::Sniffer;
use crate::StyleType;

/// Update period (milliseconds)
pub const PERIOD_TICK: u64 = 1000;

impl Application for Sniffer {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = StyleType;
    type Flags = Sniffer;

    fn new(flags: Sniffer) -> (Sniffer, Command<Message>) {
        (
            flags,
            Command::batch(vec![
                font::load(SARASA_MONO_BOLD_BYTES).map(Message::FontLoaded),
                font::load(SARASA_MONO_BYTES).map(Message::FontLoaded),
                font::load(ICONS_BYTES).map(Message::FontLoaded),
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("Sniffnet")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        self.update(message)
    }

    fn view(&self) -> Element<Message, Renderer<StyleType>> {
        let style = self.settings.style;
        let language = self.settings.language;
        let color_gradient = self.settings.color_gradient;
        let font = get_font(style);
        let font_headers = get_font_headers(style);

        let header = header(
            font,
            color_gradient,
            self.running_page.ne(&RunningPage::Init),
            language,
            self.last_opened_setting,
        );

        let body = match self.running_page {
            RunningPage::Init => initial_page(self),
            RunningPage::Overview => overview_page(self),
            RunningPage::Inspect => inspect_page(self),
            RunningPage::Notifications => notifications_page(self),
        };

        let footer = footer(
            language,
            color_gradient,
            font,
            font_headers,
            &self.newer_release_available.clone(),
        );

        let content = Column::new().push(header).push(body).push(footer);

        match self.modal.clone() {
            None => {
                if let Some(settings_page) = self.settings_page {
                    let overlay = match settings_page {
                        SettingsPage::Notifications => settings_notifications_page(self),
                        SettingsPage::Appearance => settings_style_page(self),
                        SettingsPage::General => settings_general_page(self),
                    };

                    Modal::new(content, overlay)
                        .on_blur(Message::CloseSettings)
                        .into()
                } else {
                    content.into()
                }
            }
            Some(modal) => {
                let overlay = match modal {
                    MyModal::Quit => get_exit_overlay(color_gradient, font, font_headers, language),
                    MyModal::ClearAll => {
                        get_clear_all_overlay(color_gradient, font, font_headers, language)
                    }
                    MyModal::ConnectionDetails(key) => connection_details_page(self, key),
                };

                Modal::new(content, overlay)
                    .on_blur(Message::HideModal)
                    .into()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        const NO_MODIFIER: Modifiers = Modifiers::empty();
        let window_events_subscription = subscription::events_with(|event, _| match event {
            Window(window::Event::Focused) => Some(Message::WindowFocused),
            Window(window::Event::Moved { x, y }) => Some(Message::WindowMoved(x, y)),
            Window(window::Event::Resized { width, height }) => {
                Some(Message::WindowResized(width, height))
            }
            Window(window::Event::CloseRequested) => Some(Message::CloseRequested),
            _ => None,
        });
        let hot_keys_subscription = subscription::events_with(|event, _| match event {
            Keyboard(Event::KeyPressed {
                key_code,
                modifiers,
            }) => match modifiers {
                Modifiers::COMMAND => match key_code {
                    KeyCode::Q => Some(Message::Quit),
                    KeyCode::Comma => Some(Message::OpenLastSettings),
                    KeyCode::Backspace => Some(Message::ResetButtonPressed),
                    KeyCode::D => Some(Message::CtrlDPressed),
                    KeyCode::Left => Some(Message::ArrowPressed(false)),
                    KeyCode::Right => Some(Message::ArrowPressed(true)),
                    _ => None,
                },
                Modifiers::SHIFT => match key_code {
                    KeyCode::Tab => Some(Message::SwitchPage(false)),
                    _ => None,
                },
                NO_MODIFIER => match key_code {
                    KeyCode::Enter => Some(Message::ReturnKeyPressed),
                    KeyCode::Escape => Some(Message::EscKeyPressed),
                    KeyCode::Tab => Some(Message::SwitchPage(true)),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        });
        let time_subscription = if self.running_page.eq(&RunningPage::Init) {
            iced::time::every(Duration::from_millis(PERIOD_TICK)).map(|_| Message::TickInit)
        } else {
            iced::time::every(Duration::from_millis(PERIOD_TICK)).map(|_| Message::TickRun)
        };

        Subscription::batch([
            window_events_subscription,
            hot_keys_subscription,
            time_subscription,
        ])
    }

    fn theme(&self) -> Self::Theme {
        self.settings.style
    }

    fn scale_factor(&self) -> f64 {
        self.settings.scale_factor
    }
}
