//! Module defining the `Sniffer` struct, which trace gui's component statuses and permits
//! to share data among the different threads.

use iced::window;
use iced_native::Command;
use pcap::Device;
use std::collections::{HashSet, VecDeque};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

use crate::chart::manage_chart_data::update_charts_data;
use crate::gui::components::types::my_modal::MyModal;
use crate::gui::pages::types::running_page::RunningPage;
use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::types::message::Message;
use crate::gui::types::status::Status;
use crate::networking::manage_packets::get_capture_result;
use crate::networking::types::filters::Filters;
use crate::notifications::notify_and_log::notify_and_log;
use crate::notifications::types::notifications::{Notification, Notifications};
use crate::notifications::types::sound::{play, Sound};
use crate::report::types::report_type::ReportType;
use crate::secondary_threads::parse_packets::parse_packets;
use crate::translations::types::language::Language;
use crate::utils::formatted_strings::get_report_path;
use crate::{ConfigDevice, ConfigSettings, InfoTraffic, RunTimeData, StyleType, TrafficChart};

/// Struct on which the gui is based
///
/// It contains gui statuses and network traffic statistics to be shared among the different threads
pub struct Sniffer {
    /// Capture number, incremented at every new run
    pub current_capture_id: Arc<Mutex<u16>>,
    /// Capture data updated by thread parsing packets
    pub info_traffic: Arc<Mutex<InfoTraffic>>,
    /// Status of the application (init or running) and the associated condition variable
    pub status_pair: Arc<(Mutex<Status>, Condvar)>,
    /// Reports if a newer release of the software is available on GitHub
    pub newer_release_available: Arc<Mutex<Result<bool, String>>>,
    /// Traffic data displayed in GUI
    pub runtime_data: RunTimeData,
    /// Network adapter to be analyzed
    pub device: Device,
    /// Last network adapter name for which packets were observed; saved into config file
    pub last_device_name_sniffed: String,
    /// Active filters on the observed traffic
    pub filters: Filters,
    /// Signals if a pcap error occurred
    pub pcap_error: Option<String>,
    /// Application style (only values Day and Night are possible for this field)
    pub style: StyleType,
    /// Waiting string
    pub waiting: String,
    /// Chart displayed
    pub traffic_chart: TrafficChart,
    /// Report type to be displayed
    pub report_type: ReportType,
    /// Currently displayed modal; None if no modal is displayed
    pub modal: Option<MyModal>,
    /// Currently displayed settings page; None if settings is closed
    pub settings_page: Option<SettingsPage>,
    /// Remembers the last opened setting page
    pub last_opened_setting: SettingsPage,
    /// Contains the notifications configuration set by the user
    pub notifications: Notifications,
    /// Defines the current running page
    pub running_page: RunningPage,
    /// Language used in the GUI
    pub language: Language,
    /// Number of unread notifications
    pub unread_notifications: usize,
}

impl Sniffer {
    pub fn new(
        current_capture_id: Arc<Mutex<u16>>,
        info_traffic: Arc<Mutex<InfoTraffic>>,
        status_pair: Arc<(Mutex<Status>, Condvar)>,
        config_settings: &ConfigSettings,
        config_device: &ConfigDevice,
        newer_release_available: Arc<Mutex<Result<bool, String>>>,
    ) -> Self {
        Self {
            current_capture_id,
            info_traffic,
            status_pair,
            newer_release_available,
            runtime_data: RunTimeData::new(),
            device: config_device.to_pcap_device(),
            last_device_name_sniffed: config_device.device_name.clone(),
            filters: Filters::default(),
            pcap_error: None,
            style: config_settings.style,
            waiting: ".".to_string(),
            traffic_chart: TrafficChart::new(config_settings.style, config_settings.language),
            report_type: ReportType::MostRecent,
            modal: None,
            settings_page: None,
            last_opened_setting: SettingsPage::Notifications,
            notifications: config_settings.notifications,
            running_page: RunningPage::Overview,
            language: config_settings.language,
            unread_notifications: 0,
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TickInit => {}
            Message::TickRun => return self.refresh_data(),
            Message::AdapterSelection(name) => self.set_adapter(&name),
            Message::IpVersionSelection(version) => self.filters.ip = version,
            Message::TransportProtocolSelection(protocol) => self.filters.transport = protocol,
            Message::AppProtocolSelection(protocol) => self.filters.application = protocol,
            Message::ChartSelection(what_to_display) => {
                self.traffic_chart.change_kind(what_to_display);
            }
            Message::ReportSelection(what_to_display) => self.report_type = what_to_display,
            Message::OpenReport => self.open_report_file(),
            Message::OpenGithub(main_page) => Self::open_github(main_page),
            Message::Start => self.start(),
            Message::Reset => return self.reset(),
            Message::Style(style) => {
                self.style = style;
                self.traffic_chart.change_colors(self.style);
            }
            Message::Waiting => self.update_waiting_dots(),
            Message::AddOrRemoveFavorite(index, add) => self.add_or_remove_favorite(index, add),
            Message::ShowModal(modal) => {
                if self.settings_page.is_none() && self.modal.is_none() {
                    self.modal = Some(modal);
                }
            }
            Message::HideModal => self.modal = None,
            Message::OpenSettings(settings_page) => {
                if self.modal.is_none() {
                    self.settings_page = Some(settings_page);
                }
            }
            Message::OpenLastSettings => {
                if self.modal.is_none() && self.settings_page.is_none() {
                    self.settings_page = Some(self.last_opened_setting);
                }
            }
            Message::CloseSettings => self.close_and_save_settings(),
            Message::ChangeRunningPage(running_page) => {
                self.running_page = running_page;
                if running_page.eq(&RunningPage::Notifications) {
                    self.unread_notifications = 0;
                }
            }
            Message::LanguageSelection(language) => {
                self.language = language;
                self.traffic_chart.change_language(language);
            }
            Message::UpdateNotificationSettings(value, emit_sound) => {
                self.update_notification_settings(value, emit_sound);
            }
            Message::ChangeVolume(volume) => {
                play(Sound::Pop, volume);
                self.notifications.volume = volume;
            }
            Message::ClearAllNotifications => {
                self.runtime_data.logged_notifications = VecDeque::new();
                return self.update(Message::HideModal);
            }
            Message::Quit => return window::close(),
            Message::SwitchPage(next) => self.switch_page(next),
            Message::ReturnKeyPressed => return self.shortcut_return(),
            Message::EscKeyPressed => return self.shortcut_esc(),
            Message::ResetButtonPressed => return self.reset_button_pressed(),
            Message::CtrlDPressed => return self.shortcut_ctrl_d(),
        }
        Command::none()
    }

    fn refresh_data(&mut self) -> Command<Message> {
        let info_traffic_lock = self.info_traffic.lock().unwrap();
        self.runtime_data.all_packets = info_traffic_lock.all_packets;
        if info_traffic_lock.tot_received_packets + info_traffic_lock.tot_sent_packets == 0 {
            drop(info_traffic_lock);
            return self.update(Message::Waiting);
        }
        self.runtime_data.tot_sent_packets = info_traffic_lock.tot_sent_packets;
        self.runtime_data.tot_received_packets = info_traffic_lock.tot_received_packets;
        self.runtime_data.all_bytes = info_traffic_lock.all_bytes;
        self.runtime_data.tot_received_bytes = info_traffic_lock.tot_received_bytes;
        self.runtime_data.tot_sent_bytes = info_traffic_lock.tot_sent_bytes;
        drop(info_traffic_lock);
        let emitted_notifications = notify_and_log(
            &mut self.runtime_data,
            self.notifications,
            &self.info_traffic.clone(),
        );
        self.info_traffic.lock().unwrap().favorites_last_interval = HashSet::new();
        self.runtime_data.tot_emitted_notifications += emitted_notifications;
        if self.running_page.ne(&RunningPage::Notifications) {
            self.unread_notifications += emitted_notifications;
        }
        update_charts_data(&mut self.runtime_data, &mut self.traffic_chart);
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
            && self.runtime_data.logged_notifications.is_empty()
        {
            return self.update(Message::Waiting);
        }
        Command::none()
    }

    fn open_report_file(&mut self) {
        if self.status_pair.0.lock().unwrap().eq(&Status::Running) {
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
    }

    fn open_github(main_page: bool) {
        let url = if main_page {
            "https://github.com/GyulyVGC/sniffnet"
        } else {
            "https://github.com/GyulyVGC/sniffnet/releases/latest"
        };
        #[cfg(target_os = "windows")]
        std::process::Command::new("explorer")
            .arg(url)
            .spawn()
            .unwrap();
        #[cfg(target_os = "macos")]
        std::process::Command::new("open").arg(url).spawn().unwrap();
        #[cfg(target_os = "linux")]
        std::process::Command::new("xdg-open")
            .arg(url)
            .spawn()
            .unwrap();
    }

    fn start(&mut self) {
        let device = self.device.clone();
        let (pcap_error, cap) = get_capture_result(&device);
        self.pcap_error = pcap_error.clone();
        *self.status_pair.0.lock().unwrap() = Status::Running;
        let info_traffic_mutex = self.info_traffic.clone();
        *info_traffic_mutex.lock().unwrap() = InfoTraffic::new();
        self.runtime_data = RunTimeData::new();
        self.traffic_chart = TrafficChart::new(self.style, self.language);

        if pcap_error.is_none() {
            // no pcap error
            let current_capture_id = self.current_capture_id.clone();
            let filters = self.filters.clone();
            self.status_pair.1.notify_all();
            thread::Builder::new()
                .name("thread_parse_packets".to_string())
                .spawn(move || {
                    parse_packets(
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

    fn reset(&mut self) -> Command<Message> {
        *self.status_pair.0.lock().unwrap() = Status::Init;
        self.running_page = RunningPage::Overview;
        *self.current_capture_id.lock().unwrap() += 1; //change capture id to kill previous capture and to rewrite output file
        self.pcap_error = None;
        self.report_type = ReportType::MostRecent;
        self.unread_notifications = 0;
        self.update(Message::HideModal)
    }

    fn set_adapter(&mut self, name: &str) {
        for dev in Device::list().expect("Error retrieving device list\r\n") {
            if dev.name.eq(&name) {
                self.device = dev;
                break;
            }
        }
    }

    fn update_waiting_dots(&mut self) {
        if self.waiting.len() > 2 {
            self.waiting = String::new();
        }
        self.waiting = ".".repeat(self.waiting.len() + 1);
    }

    fn add_or_remove_favorite(&mut self, index: usize, add: bool) {
        let mut info_traffic = self.info_traffic.lock().unwrap();
        if add {
            info_traffic.favorite_connections.insert(index);
        } else {
            info_traffic.favorite_connections.remove(&index);
        }
        let key_val = info_traffic.map.get_index_mut(index).unwrap();
        key_val.1.is_favorite = add;
        drop(info_traffic);
    }

    fn close_and_save_settings(&mut self) {
        if self.settings_page.is_some() {
            self.last_opened_setting = self.settings_page.unwrap();
            self.settings_page = None;
            let store = ConfigSettings {
                style: self.style,
                notifications: self.notifications,
                language: self.language,
            };
            confy::store("sniffnet", "settings", store).unwrap_or(());
        }
    }

    fn update_notification_settings(&mut self, value: Notification, emit_sound: bool) {
        let sound = match value {
            Notification::Packets(packets_notification) => {
                self.notifications.packets_notification = packets_notification;
                packets_notification.sound
            }
            Notification::Bytes(bytes_notification) => {
                self.notifications.bytes_notification = bytes_notification;
                bytes_notification.sound
            }
            Notification::Favorite(favorite_notification) => {
                self.notifications.favorite_notification = favorite_notification;
                favorite_notification.sound
            }
        };
        if emit_sound {
            play(sound, self.notifications.volume);
        }
    }

    fn switch_page(&mut self, next: bool) {
        match (
            *self.status_pair.0.lock().unwrap(),
            self.settings_page,
            self.modal,
        ) {
            (_, Some(current_setting), None) => {
                // Settings opened
                if next {
                    self.settings_page = Some(current_setting.next());
                } else {
                    self.settings_page = Some(current_setting.previous());
                }
            }
            (Status::Running, None, None) => {
                // Running with no overlays
                self.running_page = if next {
                    self.running_page.next()
                } else {
                    self.running_page.previous()
                };
                if self.running_page.eq(&RunningPage::Notifications) {
                    self.unread_notifications = 0;
                }
            }
            (_, _, _) => {}
        }
    }

    fn shortcut_return(&mut self) -> Command<Message> {
        if self.status_pair.0.lock().unwrap().eq(&Status::Init)
            && self.settings_page.is_none()
            && self.modal.is_none()
        {
            return self.update(Message::Start);
        } else if self.modal.eq(&Some(MyModal::Quit)) {
            return self.update(Message::Reset);
        } else if self.modal.eq(&Some(MyModal::ClearAll)) {
            return self.update(Message::ClearAllNotifications);
        }
        Command::none()
    }

    fn shortcut_esc(&mut self) -> Command<Message> {
        if self.modal.is_some() {
            return self.update(Message::HideModal);
        } else if self.settings_page.is_some() {
            return self.update(Message::CloseSettings);
        }
        Command::none()
    }

    // also called when backspace key is pressed on a running state
    fn reset_button_pressed(&mut self) -> Command<Message> {
        if self.status_pair.0.lock().unwrap().eq(&Status::Running) {
            return if self.info_traffic.lock().unwrap().all_packets == 0
                && self.settings_page.is_none()
            {
                self.update(Message::Reset)
            } else {
                self.update(Message::ShowModal(MyModal::Quit))
            };
        }
        Command::none()
    }

    fn shortcut_ctrl_d(&mut self) -> Command<Message> {
        if self.status_pair.0.lock().unwrap().eq(&Status::Running)
            && self.running_page.eq(&RunningPage::Notifications)
            && !self.runtime_data.logged_notifications.is_empty()
        {
            return self.update(Message::ShowModal(MyModal::ClearAll));
        }
        Command::none()
    }
}
