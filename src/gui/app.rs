//! Module defining the application structure: messages, updates, subscriptions.
//!
//! It also is a wrapper of gui's main two pages: initial and run page.

use iced::widget::Column;
use iced::{executor, Application, Command, Element, Subscription};

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
use crate::gui::pages::thumbnail_page::thumbnail_page;
use crate::gui::pages::types::running_page::RunningPage;
use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::types::message::Message;
use crate::gui::types::sniffer::Sniffer;
use crate::{ConfigSettings, StyleType, SNIFFNET_TITLECASE};

/// Update period (milliseconds)
pub const PERIOD_TICK: u64 = 1000;

pub const FONT_FAMILY_NAME: &str = "Sarasa Mono SC for Sniffnet";
pub const ICON_FONT_FAMILY_NAME: &str = "Icons for Sniffnet";

impl Application for Sniffer {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = StyleType;
    type Flags = Sniffer;

    fn new(flags: Sniffer) -> (Sniffer, Command<Message>) {
        (flags, Command::none())
    }

    fn title(&self) -> String {
        String::from(SNIFFNET_TITLECASE)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        self.update(message)
    }

    fn view(&self) -> Element<Message, StyleType> {
        let ConfigSettings {
            style,
            language,
            color_gradient,
            ..
        } = self.configs.lock().unwrap().settings;
        let font = style.get_extension().font;
        let font_headers = style.get_extension().font_headers;

        let header = header(self);

        let body = if self.thumbnail {
            thumbnail_page(self)
        } else {
            match self.running_page {
                RunningPage::Init => initial_page(self),
                RunningPage::Overview => overview_page(self),
                RunningPage::Inspect => inspect_page(self),
                RunningPage::Notifications => notifications_page(self),
            }
        };

        let footer = footer(
            self.thumbnail,
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
        Subscription::batch([
            self.keyboard_subscription(),
            self.mouse_subscription(),
            self.time_subscription(),
            Sniffer::window_subscription(),
        ])
    }

    fn theme(&self) -> Self::Theme {
        self.configs.lock().unwrap().settings.style
    }

    fn scale_factor(&self) -> f64 {
        self.configs.lock().unwrap().settings.scale_factor
    }
}
