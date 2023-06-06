//! Module defining the application structure: messages, updates, subscriptions.
//!
//! It also is a wrapper of gui's main two pages: initial and run page.

use std::time::Duration;

use iced::widget::Column;
use iced::{executor, Application, Command, Element, Subscription, Theme};

use crate::gui::components::footer::footer;
use crate::gui::components::header::header;
use crate::gui::components::modal::{get_clear_all_overlay, get_exit_overlay, Modal};
use crate::gui::components::types::my_modal::MyModal;
use crate::gui::pages::connection_details_page::connection_details_page;
use crate::gui::pages::initial_page::initial_page;
use crate::gui::pages::inspect_page::inspect_page;
use crate::gui::pages::notifications_page::notifications_page;
use crate::gui::pages::overview_page::overview_page;
use crate::gui::pages::settings_language_page::settings_language_page;
use crate::gui::pages::settings_notifications_page::settings_notifications_page;
use crate::gui::pages::settings_style_page::settings_style_page;
use crate::gui::pages::types::running_page::RunningPage;
use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::styles::style_constants::get_font;
use crate::gui::types::message::Message;
use crate::gui::types::sniffer::Sniffer;
use crate::gui::types::status::Status;

/// Update period (milliseconds)
pub const PERIOD_TICK: u64 = 1000;

impl Application for Sniffer {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = Sniffer;

    fn new(flags: Sniffer) -> (Sniffer, Command<Message>) {
        (flags, iced::window::maximize(true))
    }

    fn title(&self) -> String {
        String::from("Sniffnet")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        self.update(message)
    }

    fn view(&self) -> Element<Message> {
        let status = *self.status_pair.0.lock().unwrap();
        let style = self.style;
        let font = get_font(style);

        let header = match status {
            Status::Init => header(style, false, self.language, self.last_opened_setting),
            Status::Running => header(style, true, self.language, self.last_opened_setting),
        };

        let body = match status {
            Status::Init => initial_page(self),
            Status::Running => match self.running_page {
                RunningPage::Overview => overview_page(self),
                RunningPage::Inspect => inspect_page(self),
                RunningPage::Notifications => notifications_page(self),
            },
        };

        let footer = footer(self.language, style, &self.newer_release_available.clone());

        let content = Column::new().push(header).push(body).push(footer);

        match self.modal {
            None => {
                if let Some(settings_page) = self.settings_page {
                    let overlay = match settings_page {
                        SettingsPage::Notifications => settings_notifications_page(self),
                        SettingsPage::Appearance => settings_style_page(self),
                        SettingsPage::Language => settings_language_page(self),
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
                    MyModal::Quit => get_exit_overlay(style, font, self.language),
                    MyModal::ClearAll => get_clear_all_overlay(style, font, self.language),
                    MyModal::ConnectionDetails(connection_index) => {
                        connection_details_page(self, connection_index)
                    }
                };

                Modal::new(content, overlay)
                    .on_blur(Message::HideModal)
                    .into()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        use iced_native::keyboard::{Event, KeyCode, Modifiers};
        let hot_keys_subscription =
            iced_native::subscription::events_with(|event, _| match event {
                iced_native::Event::Keyboard(Event::KeyPressed {
                    key_code,
                    modifiers,
                }) => match modifiers {
                    Modifiers::COMMAND => match key_code {
                        KeyCode::Q => Some(Message::Quit),
                        KeyCode::O => Some(Message::OpenReport),
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
                    _ => match key_code {
                        KeyCode::Enter => Some(Message::ReturnKeyPressed),
                        KeyCode::Escape => Some(Message::EscKeyPressed),
                        KeyCode::Tab => Some(Message::SwitchPage(true)),
                        _ => None,
                    },
                },
                _ => None,
            });
        let time_subscription = match *self.status_pair.0.lock().unwrap() {
            Status::Running => {
                iced::time::every(Duration::from_millis(PERIOD_TICK)).map(|_| Message::TickRun)
            }
            Status::Init => {
                iced::time::every(Duration::from_millis(PERIOD_TICK)).map(|_| Message::TickInit)
            }
        };
        Subscription::batch([hot_keys_subscription, time_subscription])
    }
}
