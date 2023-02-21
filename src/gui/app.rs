//! Module defining the application structure: messages, updates, subscriptions.
//!
//! It also is a wrapper of gui's main two pages: initial and run page.

use iced::widget::Column;
use iced::{executor, window, Application, Command, Element, Subscription, Theme};
use pcap::Device;
use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;
use std::thread;
use std::time::Duration;

use crate::enums::message::Message;
use crate::enums::my_overlay::MyOverlay;
use crate::enums::running_page::RunningPage;
use crate::enums::sound::{play, Sound};
use crate::enums::status::Status;
use crate::gui::components::footer::footer;
use crate::gui::components::header::header;
use crate::gui::components::modal::{get_clear_all_overlay, get_exit_overlay, Modal};
use crate::gui::pages::initial_page::initial_page;
// use crate::gui::pages::inspect_page::inspect_page;
use crate::gui::pages::notifications_page::notifications_page;
use crate::gui::pages::overview_page::overview_page;
use crate::gui::pages::settings_language_page::settings_language_page;
use crate::gui::pages::settings_notifications_page::settings_notifications_page;
use crate::gui::pages::settings_style_page::settings_style_page;
use crate::structs::configs::ConfigSettings;
use crate::structs::sniffer::Sniffer;
use crate::structs::traffic_chart::TrafficChart;
use crate::thread_parse_packets::parse_packets_loop;
use crate::utility::get_formatted_strings::get_report_path;
use crate::utility::manage_charts_data::update_charts_data;
use crate::utility::manage_notifications::notify_and_log;
use crate::utility::manage_packets::get_capture_result;
use crate::utility::manage_report_data::update_report_data;
use crate::utility::style_constants::get_font;
use crate::{ConfigDevice, InfoTraffic, ReportType, RunTimeData};

/// Update period when app is running
pub const PERIOD_RUNNING: u64 = 1000;
//milliseconds
/// Update period when app is in its initial state
pub const PERIOD_INIT: u64 = 5000; //milliseconds

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
        match message {
            Message::TickInit => {}
            Message::TickRun => {
                let mut info_traffic_lock = self.info_traffic.lock().unwrap();
                self.runtime_data.borrow_mut().all_packets = info_traffic_lock.all_packets;
                if info_traffic_lock.tot_received_packets + info_traffic_lock.tot_sent_packets == 0
                {
                    drop(info_traffic_lock);
                    return self.update(Message::Waiting);
                }
                self.runtime_data.borrow_mut().tot_sent_packets =
                    info_traffic_lock.tot_sent_packets;
                self.runtime_data.borrow_mut().tot_received_packets =
                    info_traffic_lock.tot_received_packets;
                self.runtime_data.borrow_mut().all_packets = info_traffic_lock.all_packets;
                self.runtime_data.borrow_mut().all_bytes = info_traffic_lock.all_bytes;
                self.runtime_data.borrow_mut().tot_received_bytes =
                    info_traffic_lock.tot_received_bytes;
                self.runtime_data.borrow_mut().tot_sent_bytes = info_traffic_lock.tot_sent_bytes;
                self.runtime_data.borrow_mut().app_protocols =
                    info_traffic_lock.app_protocols.clone();
                self.runtime_data.borrow_mut().favorites_last_interval =
                    info_traffic_lock.favorites_last_interval.clone();
                info_traffic_lock.favorites_last_interval = HashSet::new();
                drop(info_traffic_lock);
                notify_and_log(
                    self.runtime_data.borrow_mut(),
                    self.notifications,
                    &self.info_traffic.clone(),
                );
                update_charts_data(self.runtime_data.borrow_mut());
                update_report_data(
                    self.runtime_data.borrow_mut(),
                    &self.info_traffic,
                    self.report_type,
                );
                // update ConfigDevice stored if different from last sniffed device
                if self.device.name.ne(&self.last_device_name_sniffed) {
                    self.last_device_name_sniffed = self.device.name.clone();
                    confy::store(
                        "sniffnet",
                        "device",
                        ConfigDevice {
                            device_name: self.device.name.clone(),
                        },
                    )
                    .unwrap_or(());
                }
                // waiting notifications
                if self.running_page.eq(&RunningPage::Notifications)
                    && self.runtime_data.borrow().logged_notifications.is_empty()
                {
                    return self.update(Message::Waiting);
                }
            }
            Message::AdapterSelection(name) => {
                for dev in Device::list().expect("Error retrieving device list\r\n") {
                    if dev.name.eq(&name) {
                        self.device = dev;
                        break;
                    }
                }
            }
            Message::IpVersionSelection(version) => {
                self.filters.ip = version;
            }
            Message::TransportProtocolSelection(protocol) => {
                self.filters.transport = protocol;
            }
            Message::AppProtocolSelection(protocol) => {
                self.filters.application = protocol;
            }
            Message::ChartSelection(what_to_display) => {
                self.traffic_chart.change_kind(what_to_display);
            }
            Message::ReportSelection(what_to_display) => {
                if what_to_display.ne(&self.report_type) {
                    self.report_type = what_to_display;
                    update_report_data(
                        self.runtime_data.borrow_mut(),
                        &self.info_traffic,
                        self.report_type,
                    );
                }
            }
            Message::OpenReport => {
                let report_path = get_report_path();
                #[cfg(target_os = "windows")]
                std::process::Command::new("explorer")
                    .arg(report_path)
                    .spawn()
                    .unwrap();
                #[cfg(target_os = "macos")]
                std::process::Command::new("open")
                    .arg("-t")
                    .arg(report_path)
                    .spawn()
                    .unwrap();
                #[cfg(target_os = "linux")]
                std::process::Command::new("xdg-open")
                    .arg(report_path)
                    .spawn()
                    .unwrap();
            }
            Message::OpenGithub => {
                #[cfg(target_os = "windows")]
                std::process::Command::new("explorer")
                    .arg("https://github.com/GyulyVGC/sniffnet")
                    .spawn()
                    .unwrap();
                #[cfg(target_os = "macos")]
                std::process::Command::new("open")
                    .arg("https://github.com/GyulyVGC/sniffnet")
                    .spawn()
                    .unwrap();
                #[cfg(target_os = "linux")]
                std::process::Command::new("xdg-open")
                    .arg("https://github.com/GyulyVGC/sniffnet")
                    .spawn()
                    .unwrap();
            }
            Message::Start => {
                let device = self.device.clone();
                let (pcap_error, cap) = get_capture_result(&device);
                self.pcap_error = pcap_error.clone();
                *self.status_pair.0.lock().unwrap() = Status::Running;
                let info_traffic_mutex = self.info_traffic.clone();
                *info_traffic_mutex.lock().unwrap() = InfoTraffic::new();
                self.runtime_data = Rc::new(RefCell::new(RunTimeData::new()));
                self.traffic_chart =
                    TrafficChart::new(self.runtime_data.clone(), self.style, self.language);

                if pcap_error.is_none() {
                    // no pcap error
                    let current_capture_id = self.current_capture_id.clone();
                    let filters = self.filters.clone();
                    self.status_pair.1.notify_all();
                    thread::Builder::new()
                        .name("thread_parse_packets".to_string())
                        .spawn(move || {
                            parse_packets_loop(
                                &current_capture_id,
                                device.clone(),
                                cap.unwrap(),
                                &filters,
                                &info_traffic_mutex,
                            );
                        })
                        .unwrap();
                }
            }
            Message::Reset => {
                *self.status_pair.0.lock().unwrap() = Status::Init;
                self.running_page = RunningPage::Overview;
                *self.current_capture_id.lock().unwrap() += 1; //change capture id to kill previous capture and to rewrite output file
                self.pcap_error = None;
                self.report_type = ReportType::MostRecent;
                return self.update(Message::HideModal(false));
            }
            Message::Style(style) => {
                self.style = style;
                self.traffic_chart.change_colors(self.style);
            }
            Message::Waiting => {
                if self.waiting.len() > 2 {
                    self.waiting = String::new();
                }
                self.waiting = ".".repeat(self.waiting.len() + 1);
            }
            Message::SaveConnection(index) => {
                let mut info_traffic = self.info_traffic.lock().unwrap();
                info_traffic.favorite_connections.insert(index);
                let key_val = info_traffic.map.get_index_mut(index).unwrap();
                key_val.1.is_favorite = true;
                drop(info_traffic);
                update_report_data(
                    self.runtime_data.borrow_mut(),
                    &self.info_traffic,
                    self.report_type,
                );
            }
            Message::UnSaveConnection(index) => {
                let mut info_traffic = self.info_traffic.lock().unwrap();
                info_traffic.favorite_connections.remove(&index);
                let key_val = info_traffic.map.get_index_mut(index).unwrap();
                key_val.1.is_favorite = false;
                drop(info_traffic);
                update_report_data(
                    self.runtime_data.borrow_mut(),
                    &self.info_traffic,
                    self.report_type,
                );
            }
            Message::ShowModal(overlay) => {
                self.overlay = Some(overlay);
            }
            Message::HideModal(save_config) => {
                let last_opened = self.overlay;
                self.overlay = None;
                if save_config {
                    // closed a setting page
                    self.last_opened_setting = last_opened.unwrap();
                    let store = ConfigSettings {
                        style: self.style,
                        notifications: self.notifications,
                        language: self.language,
                    };
                    confy::store("sniffnet", "settings", store).unwrap_or(());
                }
            }
            Message::ChangeRunningPage(running_page) => {
                self.running_page = running_page;
            }
            Message::LanguageSelection(language) => {
                self.language = language;
                self.traffic_chart.change_language(language);
            }
            Message::UpdatePacketsNotification(value, emit_sound) => {
                if emit_sound {
                    play(value.sound, self.notifications.volume);
                }
                self.notifications.packets_notification = value;
            }
            Message::UpdateBytesNotification(value, emit_sound) => {
                if emit_sound {
                    play(value.sound, self.notifications.volume);
                }
                self.notifications.bytes_notification = value;
            }
            Message::UpdateFavoriteNotification(value, emit_sound) => {
                if emit_sound {
                    play(value.sound, self.notifications.volume);
                }
                self.notifications.favorite_notification = value;
            }
            Message::ChangeVolume(volume) => {
                play(Sound::Pop, volume);
                self.notifications.volume = volume;
            }
            Message::ClearAllNotifications => {
                self.runtime_data.borrow_mut().logged_notifications = VecDeque::new();
                return self.update(Message::HideModal(false));
            }
            Message::Exit => {
                return window::close();
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let status = *self.status_pair.0.lock().unwrap();
        let style = self.style;
        let font = get_font(style);

        let header = match status {
            Status::Init => header(style, false, 0, self.language, self.last_opened_setting),
            Status::Running => header(
                style,
                true,
                self.info_traffic.lock().unwrap().all_packets,
                self.language,
                self.last_opened_setting,
            ),
        };

        let body = match status {
            Status::Init => initial_page(self),
            Status::Running => match self.running_page {
                RunningPage::Overview => overview_page(self),
                // RunningPage::Inspect => inspect_page(self),
                RunningPage::Notifications => notifications_page(self),
            },
        };

        let content = Column::new().push(header).push(body).push(footer(style));

        if self.overlay.is_none() {
            content.into()
        } else {
            let (overlay, save_config) = match self.overlay.unwrap() {
                MyOverlay::Quit => (get_exit_overlay(style, font, self.language), false),
                MyOverlay::ClearAll => (get_clear_all_overlay(style, font, self.language), false),
                MyOverlay::SettingsNotifications => (settings_notifications_page(self), true),
                MyOverlay::SettingsAppearance => (settings_style_page(self), true),
                MyOverlay::SettingsLanguage => (settings_language_page(self), true),
            };

            Modal::new(content, overlay)
                .on_blur(Message::HideModal(save_config))
                .into()
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        let quit_keys_subscription = iced_native::subscription::events_with(|event, _| {
            if let iced_native::Event::Keyboard(iced_native::keyboard::Event::KeyPressed {
                key_code: iced_native::keyboard::KeyCode::Q,
                modifiers: iced_native::keyboard::Modifiers::CTRL,
            }) = event
            {
                Some(Message::Exit)
            } else {
                None
            }
        });
        let time_subscription = match *self.status_pair.0.lock().unwrap() {
            Status::Running => {
                iced::time::every(Duration::from_millis(PERIOD_RUNNING)).map(|_| Message::TickRun)
            }
            Status::Init => {
                iced::time::every(Duration::from_millis(PERIOD_INIT)).map(|_| Message::TickInit)
            }
        };
        Subscription::batch([quit_keys_subscription, time_subscription])
    }
}
