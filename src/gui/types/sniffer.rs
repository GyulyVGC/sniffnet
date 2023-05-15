//! Module defining the `Sniffer` struct, which trace gui's component statuses and permits
//! to share data among the different threads.

use std::collections::{HashSet, VecDeque};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

use iced::window;
use iced_native::Command;
use pcap::Device;

use crate::chart::manage_chart_data::update_charts_data;
use crate::gui::components::types::my_modal::MyModal;
use crate::gui::pages::types::running_page::RunningPage;
use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::types::message::Message;
use crate::gui::types::status::Status;
use crate::networking::manage_packets::get_capture_result;
use crate::networking::types::filters::Filters;
use crate::networking::types::host::Host;
use crate::networking::types::my_device::MyDevice;
use crate::networking::types::search_parameters::SearchParameters;
use crate::notifications::notify_and_log::notify_and_log;
use crate::notifications::types::notifications::{Notification, Notifications};
use crate::notifications::types::sound::{play, Sound};
use crate::report::get_report_entries::get_searched_entries;
use crate::report::types::report_sort_type::ReportSortType;
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
    pub device: MyDevice,
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
    pub report_sort_type: ReportSortType,
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
    /// Search parameters of inspect page
    pub search: SearchParameters,
    /// Current page number of inspect search results
    pub page_number: usize,
    /// Currently selected connection for inspection of its details
    pub selected_connection: usize,
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
            device: config_device.to_my_device(),
            last_device_name_sniffed: config_device.device_name.clone(),
            filters: Filters::default(),
            pcap_error: None,
            style: config_settings.style,
            waiting: ".".to_string(),
            traffic_chart: TrafficChart::new(config_settings.style, config_settings.language),
            report_sort_type: ReportSortType::MostRecent,
            modal: None,
            settings_page: None,
            last_opened_setting: SettingsPage::Notifications,
            notifications: config_settings.notifications,
            running_page: RunningPage::Overview,
            language: config_settings.language,
            unread_notifications: 0,
            search: SearchParameters::default(),
            page_number: 1,
            selected_connection: 0,
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
            Message::ReportSortSelection(what_to_display) => {
                self.report_sort_type = what_to_display;
            }
            Message::OpenReport => self.open_report_file(),
            Message::OpenGithub(main_page) => Self::open_github(main_page),
            Message::Start => self.start(),
            Message::Reset => return self.reset(),
            Message::Style(style) => {
                self.style = style;
                self.traffic_chart.change_colors(self.style);
            }
            Message::Waiting => self.update_waiting_dots(),
            Message::AddOrRemoveFavorite(host, add) => self.add_or_remove_favorite(&host, add),
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
            Message::Search(parameters) => {
                self.page_number = 1;
                self.running_page = RunningPage::Inspect;
                self.search = parameters;
            }
            Message::UpdatePageNumber(increment) => {
                let new_page = if increment {
                    self.page_number.checked_add(1)
                } else {
                    self.page_number.checked_sub(1)
                }
                .unwrap();
                self.page_number = new_page;
            }
            Message::ArrowPressed(increment) => {
                if self.running_page.eq(&RunningPage::Inspect)
                    && self.settings_page.is_none()
                    && self.modal.is_none()
                {
                    if increment {
                        if self.page_number < (get_searched_entries(self).1 + 20 - 1) / 20 {
                            return self.update(Message::UpdatePageNumber(increment));
                        }
                    } else if self.page_number > 1 {
                        return self.update(Message::UpdatePageNumber(increment));
                    }
                }
            }
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
        self.runtime_data.dropped_packets = info_traffic_lock.dropped_packets;
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

        let current_device_name = self.device.name.clone();
        // update ConfigDevice stored if different from last sniffed device
        if current_device_name.ne(&self.last_device_name_sniffed) {
            self.last_device_name_sniffed = current_device_name.clone();
            confy::store(
                "sniffnet",
                "device",
                ConfigDevice {
                    device_name: current_device_name,
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
        let current_device_name = &*self.device.name.clone();
        self.set_adapter(current_device_name);
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
                        &device,
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
        self.report_sort_type = ReportSortType::MostRecent;
        self.unread_notifications = 0;
        self.search = SearchParameters::default();
        self.page_number = 1;
        self.update(Message::HideModal)
    }

    fn set_adapter(&mut self, name: &str) {
        for dev in Device::list().expect("Error retrieving device list\r\n") {
            if dev.name.eq(&name) {
                let mut addresses_mutex = self.device.addresses.lock().unwrap();
                *addresses_mutex = dev.addresses;
                drop(addresses_mutex);
                self.device = MyDevice {
                    name: dev.name,
                    desc: dev.desc,
                    addresses: self.device.addresses.clone(),
                };
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

    fn add_or_remove_favorite(&mut self, host: &Host, add: bool) {
        let mut info_traffic = self.info_traffic.lock().unwrap();
        if add {
            info_traffic.favorite_hosts.insert(host.clone());
        } else {
            info_traffic.favorite_hosts.remove(host);
        }
        if let Some(host_info) = info_traffic.hosts.get_mut(host) {
            host_info.is_favorite = add;
        }
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
                if self.runtime_data.tot_sent_packets + self.runtime_data.tot_received_packets > 0 {
                    // Running with no overlays and some packets filtered
                    self.running_page = if next {
                        self.running_page.next()
                    } else {
                        self.running_page.previous()
                    };
                    if self.running_page.eq(&RunningPage::Notifications) {
                        self.unread_notifications = 0;
                    }
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

#[cfg(test)]
mod tests {
    #![allow(unused_must_use)]

    use std::collections::{HashSet, VecDeque};
    use std::sync::{Arc, Mutex};

    use crate::gui::components::types::my_modal::MyModal;
    use crate::gui::pages::types::settings_page::SettingsPage;
    use crate::gui::styles::style_constants::get_color_mix_chart;
    use crate::gui::styles::types::palette::to_rgb_color;
    use crate::gui::types::message::Message;
    use crate::networking::types::host::Host;
    use crate::notifications::types::logged_notification::{
        LoggedNotification, PacketsThresholdExceeded,
    };
    use crate::notifications::types::notifications::{
        BytesNotification, FavoriteNotification, Notification, PacketsNotification,
    };
    use crate::notifications::types::sound::Sound;
    use crate::{
        get_colors, AppProtocol, ByteMultiple, ChartType, InfoTraffic, IpVersion, Language,
        ReportSortType, RunningPage, Sniffer, Status, StyleType, TransProtocol,
    };

    #[test]
    fn test_correctly_update_ip_version() {
        let mut sniffer = Sniffer::new(
            Arc::new(Mutex::new(0)),
            Arc::new(Mutex::new(InfoTraffic::new())),
            Arc::new((Mutex::new(Status::Init), Default::default())),
            &Default::default(),
            &Default::default(),
            Arc::new(Mutex::new(Err(String::new()))),
        );

        assert_eq!(sniffer.filters.ip, IpVersion::Other);
        sniffer.update(Message::IpVersionSelection(IpVersion::IPv6));
        assert_eq!(sniffer.filters.ip, IpVersion::IPv6);
        sniffer.update(Message::IpVersionSelection(IpVersion::IPv4));
        assert_eq!(sniffer.filters.ip, IpVersion::IPv4);
        sniffer.update(Message::IpVersionSelection(IpVersion::IPv4));
        assert_eq!(sniffer.filters.ip, IpVersion::IPv4);
        sniffer.update(Message::IpVersionSelection(IpVersion::Other));
        assert_eq!(sniffer.filters.ip, IpVersion::Other);
    }

    #[test]
    fn test_correctly_update_transport_protocol() {
        let mut sniffer = Sniffer::new(
            Arc::new(Mutex::new(0)),
            Arc::new(Mutex::new(InfoTraffic::new())),
            Arc::new((Mutex::new(Status::Init), Default::default())),
            &Default::default(),
            &Default::default(),
            Arc::new(Mutex::new(Err(String::new()))),
        );

        assert_eq!(sniffer.filters.transport, TransProtocol::Other);
        sniffer.update(Message::TransportProtocolSelection(TransProtocol::UDP));
        assert_eq!(sniffer.filters.transport, TransProtocol::UDP);
        sniffer.update(Message::TransportProtocolSelection(TransProtocol::UDP));
        assert_eq!(sniffer.filters.transport, TransProtocol::UDP);
        sniffer.update(Message::TransportProtocolSelection(TransProtocol::TCP));
        assert_eq!(sniffer.filters.transport, TransProtocol::TCP);
        sniffer.update(Message::TransportProtocolSelection(TransProtocol::Other));
        assert_eq!(sniffer.filters.transport, TransProtocol::Other);
    }

    #[test]
    fn test_correctly_update_application_protocol() {
        let mut sniffer = Sniffer::new(
            Arc::new(Mutex::new(0)),
            Arc::new(Mutex::new(InfoTraffic::new())),
            Arc::new((Mutex::new(Status::Init), Default::default())),
            &Default::default(),
            &Default::default(),
            Arc::new(Mutex::new(Err(String::new()))),
        );

        assert_eq!(sniffer.filters.application, AppProtocol::Other);
        sniffer.update(Message::AppProtocolSelection(AppProtocol::HTTPS));
        assert_eq!(sniffer.filters.application, AppProtocol::HTTPS);
        sniffer.update(Message::AppProtocolSelection(AppProtocol::HTTP));
        assert_eq!(sniffer.filters.application, AppProtocol::HTTP);
        sniffer.update(Message::AppProtocolSelection(AppProtocol::HTTP));
        assert_eq!(sniffer.filters.application, AppProtocol::HTTP);
        sniffer.update(Message::AppProtocolSelection(AppProtocol::XMPP));
        assert_eq!(sniffer.filters.application, AppProtocol::XMPP);
    }

    #[test]
    fn test_correctly_update_chart_kind() {
        let mut sniffer = Sniffer::new(
            Arc::new(Mutex::new(0)),
            Arc::new(Mutex::new(InfoTraffic::new())),
            Arc::new((Mutex::new(Status::Init), Default::default())),
            &Default::default(),
            &Default::default(),
            Arc::new(Mutex::new(Err(String::new()))),
        );

        assert_eq!(sniffer.traffic_chart.chart_type, ChartType::Bytes);
        sniffer.update(Message::ChartSelection(ChartType::Packets));
        assert_eq!(sniffer.traffic_chart.chart_type, ChartType::Packets);
        sniffer.update(Message::ChartSelection(ChartType::Packets));
        assert_eq!(sniffer.traffic_chart.chart_type, ChartType::Packets);
        sniffer.update(Message::ChartSelection(ChartType::Bytes));
        assert_eq!(sniffer.traffic_chart.chart_type, ChartType::Bytes);
    }

    #[test]
    fn test_correctly_update_report_kind() {
        let mut sniffer = Sniffer::new(
            Arc::new(Mutex::new(0)),
            Arc::new(Mutex::new(InfoTraffic::new())),
            Arc::new((Mutex::new(Status::Init), Default::default())),
            &Default::default(),
            &Default::default(),
            Arc::new(Mutex::new(Err(String::new()))),
        );

        assert_eq!(sniffer.report_sort_type, ReportSortType::MostRecent);
        sniffer.update(Message::ReportSortSelection(ReportSortType::MostBytes));
        assert_eq!(sniffer.report_sort_type, ReportSortType::MostBytes);
        sniffer.update(Message::ReportSortSelection(ReportSortType::MostPackets));
        assert_eq!(sniffer.report_sort_type, ReportSortType::MostPackets);
        sniffer.update(Message::ReportSortSelection(ReportSortType::MostPackets));
        assert_eq!(sniffer.report_sort_type, ReportSortType::MostPackets);
        sniffer.update(Message::ReportSortSelection(ReportSortType::MostRecent));
        assert_eq!(sniffer.report_sort_type, ReportSortType::MostRecent);
    }

    #[test]
    fn test_correctly_update_style() {
        let mut sniffer = Sniffer::new(
            Arc::new(Mutex::new(0)),
            Arc::new(Mutex::new(InfoTraffic::new())),
            Arc::new((Mutex::new(Status::Init), Default::default())),
            &Default::default(),
            &Default::default(),
            Arc::new(Mutex::new(Err(String::new()))),
        );

        sniffer.update(Message::Style(StyleType::MonAmour));
        assert_eq!(sniffer.style, StyleType::MonAmour);
        assert_eq!(
            sniffer.traffic_chart.color_font,
            to_rgb_color(get_colors(StyleType::MonAmour).text_body)
        );
        assert_eq!(
            sniffer.traffic_chart.color_outgoing,
            to_rgb_color(get_colors(StyleType::MonAmour).outgoing)
        );
        assert_eq!(
            sniffer.traffic_chart.color_incoming,
            to_rgb_color(get_colors(StyleType::MonAmour).secondary)
        );
        assert_eq!(
            sniffer.traffic_chart.color_mix,
            get_color_mix_chart(StyleType::MonAmour)
        );
        sniffer.update(Message::Style(StyleType::Day));
        assert_eq!(sniffer.style, StyleType::Day);
        assert_eq!(
            sniffer.traffic_chart.color_font,
            to_rgb_color(get_colors(StyleType::Day).text_body)
        );
        assert_eq!(
            sniffer.traffic_chart.color_outgoing,
            to_rgb_color(get_colors(StyleType::Day).outgoing)
        );
        assert_eq!(
            sniffer.traffic_chart.color_incoming,
            to_rgb_color(get_colors(StyleType::Day).secondary)
        );
        assert_eq!(
            sniffer.traffic_chart.color_mix,
            get_color_mix_chart(StyleType::Day)
        );
        sniffer.update(Message::Style(StyleType::Night));
        assert_eq!(sniffer.style, StyleType::Night);
        assert_eq!(
            sniffer.traffic_chart.color_font,
            to_rgb_color(get_colors(StyleType::Night).text_body)
        );
        assert_eq!(
            sniffer.traffic_chart.color_outgoing,
            to_rgb_color(get_colors(StyleType::Night).outgoing)
        );
        assert_eq!(
            sniffer.traffic_chart.color_incoming,
            to_rgb_color(get_colors(StyleType::Night).secondary)
        );
        assert_eq!(
            sniffer.traffic_chart.color_mix,
            get_color_mix_chart(StyleType::Night)
        );
        sniffer.update(Message::Style(StyleType::DeepSea));
        assert_eq!(sniffer.style, StyleType::DeepSea);
        assert_eq!(
            sniffer.traffic_chart.color_font,
            to_rgb_color(get_colors(StyleType::DeepSea).text_body)
        );
        assert_eq!(
            sniffer.traffic_chart.color_outgoing,
            to_rgb_color(get_colors(StyleType::DeepSea).outgoing)
        );
        assert_eq!(
            sniffer.traffic_chart.color_incoming,
            to_rgb_color(get_colors(StyleType::DeepSea).secondary)
        );
        assert_eq!(
            sniffer.traffic_chart.color_mix,
            get_color_mix_chart(StyleType::DeepSea)
        );
        sniffer.update(Message::Style(StyleType::DeepSea));
        assert_eq!(sniffer.style, StyleType::DeepSea);
        assert_eq!(
            sniffer.traffic_chart.color_font,
            to_rgb_color(get_colors(StyleType::DeepSea).text_body)
        );
        assert_eq!(
            sniffer.traffic_chart.color_outgoing,
            to_rgb_color(get_colors(StyleType::DeepSea).outgoing)
        );
        assert_eq!(
            sniffer.traffic_chart.color_incoming,
            to_rgb_color(get_colors(StyleType::DeepSea).secondary)
        );
        assert_eq!(
            sniffer.traffic_chart.color_mix,
            get_color_mix_chart(StyleType::DeepSea)
        );
    }

    #[test]
    fn test_waiting_dots_update() {
        let mut sniffer = Sniffer::new(
            Arc::new(Mutex::new(0)),
            Arc::new(Mutex::new(InfoTraffic::new())),
            Arc::new((Mutex::new(Status::Init), Default::default())),
            &Default::default(),
            &Default::default(),
            Arc::new(Mutex::new(Err(String::new()))),
        );

        assert_eq!(sniffer.waiting, ".".to_string());
        sniffer.update(Message::Waiting);
        assert_eq!(sniffer.waiting, "..".to_string());

        sniffer.update(Message::Waiting);
        assert_eq!(sniffer.waiting, "...".to_string());

        sniffer.update(Message::Waiting);
        assert_eq!(sniffer.waiting, ".".to_string());
    }

    #[test]
    fn test_modify_favorite_connections() {
        let mut sniffer = Sniffer::new(
            Arc::new(Mutex::new(0)),
            Arc::new(Mutex::new(InfoTraffic::new())),
            Arc::new((Mutex::new(Status::Init), Default::default())),
            &Default::default(),
            &Default::default(),
            Arc::new(Mutex::new(Err(String::new()))),
        );
        // remove 1
        sniffer.update(Message::AddOrRemoveFavorite(
            Host {
                domain: "1.1".to_string(),
                asn: Default::default(),
                country: "US".to_string(),
            },
            false,
        ));
        assert_eq!(
            sniffer.info_traffic.lock().unwrap().favorite_hosts,
            HashSet::new()
        );
        // remove 2
        sniffer.update(Message::AddOrRemoveFavorite(
            Host {
                domain: "2.2".to_string(),
                asn: Default::default(),
                country: "US".to_string(),
            },
            false,
        ));
        assert_eq!(
            sniffer.info_traffic.lock().unwrap().favorite_hosts,
            HashSet::new()
        );
        // add 2
        sniffer.update(Message::AddOrRemoveFavorite(
            Host {
                domain: "2.2".to_string(),
                asn: Default::default(),
                country: "US".to_string(),
            },
            true,
        ));
        assert_eq!(
            sniffer.info_traffic.lock().unwrap().favorite_hosts,
            HashSet::from([Host {
                domain: "2.2".to_string(),
                asn: Default::default(),
                country: "US".to_string()
            }])
        );
        // remove 1
        sniffer.update(Message::AddOrRemoveFavorite(
            Host {
                domain: "1.1".to_string(),
                asn: Default::default(),
                country: "US".to_string(),
            },
            false,
        ));
        assert_eq!(
            sniffer.info_traffic.lock().unwrap().favorite_hosts,
            HashSet::from([Host {
                domain: "2.2".to_string(),
                asn: Default::default(),
                country: "US".to_string()
            }])
        );
        // add 2
        sniffer.update(Message::AddOrRemoveFavorite(
            Host {
                domain: "2.2".to_string(),
                asn: Default::default(),
                country: "US".to_string(),
            },
            true,
        ));
        assert_eq!(
            sniffer.info_traffic.lock().unwrap().favorite_hosts,
            HashSet::from([Host {
                domain: "2.2".to_string(),
                asn: Default::default(),
                country: "US".to_string()
            }])
        );
        // add 1
        sniffer.update(Message::AddOrRemoveFavorite(
            Host {
                domain: "1.1".to_string(),
                asn: Default::default(),
                country: "US".to_string(),
            },
            true,
        ));
        assert_eq!(
            sniffer.info_traffic.lock().unwrap().favorite_hosts,
            HashSet::from([
                Host {
                    domain: "1.1".to_string(),
                    asn: Default::default(),
                    country: "US".to_string()
                },
                Host {
                    domain: "2.2".to_string(),
                    asn: Default::default(),
                    country: "US".to_string()
                }
            ])
        );
        // add 3
        sniffer.update(Message::AddOrRemoveFavorite(
            Host {
                domain: "3.3".to_string(),
                asn: Default::default(),
                country: "US".to_string(),
            },
            true,
        ));
        assert_eq!(
            sniffer.info_traffic.lock().unwrap().favorite_hosts,
            HashSet::from([
                Host {
                    domain: "1.1".to_string(),
                    asn: Default::default(),
                    country: "US".to_string()
                },
                Host {
                    domain: "2.2".to_string(),
                    asn: Default::default(),
                    country: "US".to_string()
                },
                Host {
                    domain: "3.3".to_string(),
                    asn: Default::default(),
                    country: "US".to_string()
                }
            ])
        );
        // remove 2
        sniffer.update(Message::AddOrRemoveFavorite(
            Host {
                domain: "2.2".to_string(),
                asn: Default::default(),
                country: "US".to_string(),
            },
            false,
        ));
        assert_eq!(
            sniffer.info_traffic.lock().unwrap().favorite_hosts,
            HashSet::from([
                Host {
                    domain: "1.1".to_string(),
                    asn: Default::default(),
                    country: "US".to_string()
                },
                Host {
                    domain: "3.3".to_string(),
                    asn: Default::default(),
                    country: "US".to_string()
                }
            ])
        );
        // remove 3
        sniffer.update(Message::AddOrRemoveFavorite(
            Host {
                domain: "3.3".to_string(),
                asn: Default::default(),
                country: "US".to_string(),
            },
            false,
        ));
        assert_eq!(
            sniffer.info_traffic.lock().unwrap().favorite_hosts,
            HashSet::from([Host {
                domain: "1.1".to_string(),
                asn: Default::default(),
                country: "US".to_string()
            }])
        );
        // remove 1
        sniffer.update(Message::AddOrRemoveFavorite(
            Host {
                domain: "1.1".to_string(),
                asn: Default::default(),
                country: "US".to_string(),
            },
            false,
        ));
        assert_eq!(
            sniffer.info_traffic.lock().unwrap().favorite_hosts,
            HashSet::new()
        );
    }

    #[test]
    fn test_show_and_hide_modal_and_settings() {
        let mut sniffer = Sniffer::new(
            Arc::new(Mutex::new(0)),
            Arc::new(Mutex::new(InfoTraffic::new())),
            Arc::new((Mutex::new(Status::Init), Default::default())),
            &Default::default(),
            &Default::default(),
            Arc::new(Mutex::new(Err(String::new()))),
        );

        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.settings_page, None);
        assert_eq!(sniffer.last_opened_setting, SettingsPage::Notifications);
        // open settings
        sniffer.update(Message::OpenLastSettings);
        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.settings_page, Some(SettingsPage::Notifications));
        assert_eq!(sniffer.last_opened_setting, SettingsPage::Notifications);
        // switch settings page
        sniffer.update(Message::OpenSettings(SettingsPage::Appearance));
        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.settings_page, Some(SettingsPage::Appearance));
        sniffer.update(Message::OpenSettings(SettingsPage::Language));
        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.settings_page, Some(SettingsPage::Language));
        // try opening modal with settings opened
        sniffer.update(Message::ShowModal(MyModal::Quit));
        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.settings_page, Some(SettingsPage::Language));
        assert_eq!(sniffer.last_opened_setting, SettingsPage::Notifications);
        // close settings
        sniffer.update(Message::CloseSettings);
        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.settings_page, None);
        assert_eq!(sniffer.last_opened_setting, SettingsPage::Language);
        // reopen settings
        sniffer.update(Message::OpenLastSettings);
        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.settings_page, Some(SettingsPage::Language));
        assert_eq!(sniffer.last_opened_setting, SettingsPage::Language);
        // switch settings page
        sniffer.update(Message::OpenSettings(SettingsPage::Appearance));
        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.settings_page, Some(SettingsPage::Appearance));
        // close settings
        sniffer.update(Message::CloseSettings);
        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.settings_page, None);
        assert_eq!(sniffer.last_opened_setting, SettingsPage::Appearance);

        // open clear all modal
        sniffer.update(Message::ShowModal(MyModal::ClearAll));
        assert_eq!(sniffer.modal, Some(MyModal::ClearAll));
        assert_eq!(sniffer.settings_page, None);
        assert_eq!(sniffer.last_opened_setting, SettingsPage::Appearance);
        // try opening settings with clear all modal opened
        sniffer.update(Message::OpenLastSettings);
        assert_eq!(sniffer.modal, Some(MyModal::ClearAll));
        assert_eq!(sniffer.settings_page, None);
        assert_eq!(sniffer.last_opened_setting, SettingsPage::Appearance);
        // try opening quit modal with clear all modal opened
        sniffer.update(Message::ShowModal(MyModal::Quit));
        assert_eq!(sniffer.modal, Some(MyModal::ClearAll));
        assert_eq!(sniffer.settings_page, None);
        assert_eq!(sniffer.last_opened_setting, SettingsPage::Appearance);
        // close clear all modal
        sniffer.update(Message::HideModal);
        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.settings_page, None);
        assert_eq!(sniffer.last_opened_setting, SettingsPage::Appearance);

        // open quit modal
        sniffer.update(Message::ShowModal(MyModal::Quit));
        assert_eq!(sniffer.modal, Some(MyModal::Quit));
        assert_eq!(sniffer.settings_page, None);
        assert_eq!(sniffer.last_opened_setting, SettingsPage::Appearance);
        // try opening settings with clear all modal opened
        sniffer.update(Message::OpenLastSettings);
        assert_eq!(sniffer.modal, Some(MyModal::Quit));
        assert_eq!(sniffer.settings_page, None);
        assert_eq!(sniffer.last_opened_setting, SettingsPage::Appearance);
        // try opening clear all modal with quit modal opened
        sniffer.update(Message::ShowModal(MyModal::ClearAll));
        assert_eq!(sniffer.modal, Some(MyModal::Quit));
        assert_eq!(sniffer.settings_page, None);
        assert_eq!(sniffer.last_opened_setting, SettingsPage::Appearance);
        // close quit modal
        sniffer.update(Message::HideModal);
        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.settings_page, None);
        assert_eq!(sniffer.last_opened_setting, SettingsPage::Appearance);
    }

    #[test]
    fn test_correctly_update_language() {
        let mut sniffer = Sniffer::new(
            Arc::new(Mutex::new(0)),
            Arc::new(Mutex::new(InfoTraffic::new())),
            Arc::new((Mutex::new(Status::Init), Default::default())),
            &Default::default(),
            &Default::default(),
            Arc::new(Mutex::new(Err(String::new()))),
        );

        assert_eq!(sniffer.language, Language::EN);
        assert_eq!(sniffer.traffic_chart.language, Language::EN);
        sniffer.update(Message::LanguageSelection(Language::IT));
        assert_eq!(sniffer.language, Language::IT);
        assert_eq!(sniffer.traffic_chart.language, Language::IT);
        sniffer.update(Message::LanguageSelection(Language::IT));
        assert_eq!(sniffer.language, Language::IT);
        assert_eq!(sniffer.traffic_chart.language, Language::IT);
        sniffer.update(Message::LanguageSelection(Language::ZH));
        assert_eq!(sniffer.language, Language::ZH);
        assert_eq!(sniffer.traffic_chart.language, Language::ZH);
    }

    #[test]
    fn test_correctly_update_notification_settings() {
        let mut sniffer = Sniffer::new(
            Arc::new(Mutex::new(0)),
            Arc::new(Mutex::new(InfoTraffic::new())),
            Arc::new((Mutex::new(Status::Init), Default::default())),
            &Default::default(),
            &Default::default(),
            Arc::new(Mutex::new(Err(String::new()))),
        );

        // initial default state
        assert_eq!(sniffer.notifications.volume, 60);
        assert_eq!(
            sniffer.notifications.packets_notification,
            PacketsNotification {
                threshold: None,
                sound: Sound::Gulp,
                previous_threshold: 750
            }
        );
        assert_eq!(
            sniffer.notifications.bytes_notification,
            BytesNotification {
                threshold: None,
                byte_multiple: ByteMultiple::KB,
                sound: Sound::Pop,
                previous_threshold: 800000
            }
        );
        assert_eq!(
            sniffer.notifications.favorite_notification,
            FavoriteNotification {
                notify_on_favorite: false,
                sound: Sound::Swhoosh,
            }
        );
        // change volume
        sniffer.update(Message::ChangeVolume(95));
        assert_eq!(sniffer.notifications.volume, 95);
        assert_eq!(
            sniffer.notifications.packets_notification,
            PacketsNotification {
                threshold: None,
                sound: Sound::Gulp,
                previous_threshold: 750
            }
        );
        assert_eq!(
            sniffer.notifications.bytes_notification,
            BytesNotification {
                threshold: None,
                byte_multiple: ByteMultiple::KB,
                sound: Sound::Pop,
                previous_threshold: 800000
            }
        );
        assert_eq!(
            sniffer.notifications.favorite_notification,
            FavoriteNotification {
                notify_on_favorite: false,
                sound: Sound::Swhoosh,
            }
        );
        // change packets notifications
        sniffer.update(Message::UpdateNotificationSettings(
            Notification::Packets(PacketsNotification {
                threshold: Some(1122),
                sound: Sound::None,
                previous_threshold: 1122,
            }),
            false,
        ));
        assert_eq!(sniffer.notifications.volume, 95);
        assert_eq!(
            sniffer.notifications.packets_notification,
            PacketsNotification {
                threshold: Some(1122),
                sound: Sound::None,
                previous_threshold: 1122
            }
        );
        assert_eq!(
            sniffer.notifications.bytes_notification,
            BytesNotification {
                threshold: None,
                byte_multiple: ByteMultiple::KB,
                sound: Sound::Pop,
                previous_threshold: 800000
            }
        );
        assert_eq!(
            sniffer.notifications.favorite_notification,
            FavoriteNotification {
                notify_on_favorite: false,
                sound: Sound::Swhoosh,
            }
        );
        // change bytes notifications
        sniffer.update(Message::UpdateNotificationSettings(
            Notification::Bytes(BytesNotification {
                threshold: Some(3),
                byte_multiple: ByteMultiple::GB,
                sound: Sound::None,
                previous_threshold: 3,
            }),
            true,
        ));
        assert_eq!(sniffer.notifications.volume, 95);
        assert_eq!(
            sniffer.notifications.packets_notification,
            PacketsNotification {
                threshold: Some(1122),
                sound: Sound::None,
                previous_threshold: 1122
            }
        );
        assert_eq!(
            sniffer.notifications.bytes_notification,
            BytesNotification {
                threshold: Some(3),
                byte_multiple: ByteMultiple::GB,
                sound: Sound::None,
                previous_threshold: 3,
            }
        );
        assert_eq!(
            sniffer.notifications.favorite_notification,
            FavoriteNotification {
                notify_on_favorite: false,
                sound: Sound::Swhoosh,
            }
        );
        // change favorite notifications
        sniffer.update(Message::UpdateNotificationSettings(
            Notification::Favorite(FavoriteNotification {
                notify_on_favorite: true,
                sound: Sound::Pop,
            }),
            true,
        ));
        assert_eq!(sniffer.notifications.volume, 95);
        assert_eq!(
            sniffer.notifications.packets_notification,
            PacketsNotification {
                threshold: Some(1122),
                sound: Sound::None,
                previous_threshold: 1122
            }
        );
        assert_eq!(
            sniffer.notifications.bytes_notification,
            BytesNotification {
                threshold: Some(3),
                byte_multiple: ByteMultiple::GB,
                sound: Sound::None,
                previous_threshold: 3,
            }
        );
        assert_eq!(
            sniffer.notifications.favorite_notification,
            FavoriteNotification {
                notify_on_favorite: true,
                sound: Sound::Pop
            }
        );
    }

    #[test]
    fn test_clear_all_notifications() {
        let mut sniffer = Sniffer::new(
            Arc::new(Mutex::new(0)),
            Arc::new(Mutex::new(InfoTraffic::new())),
            Arc::new((Mutex::new(Status::Init), Default::default())),
            &Default::default(),
            &Default::default(),
            Arc::new(Mutex::new(Err(String::new()))),
        );
        sniffer.runtime_data.logged_notifications =
            VecDeque::from([LoggedNotification::PacketsThresholdExceeded(
                PacketsThresholdExceeded {
                    threshold: 0,
                    incoming: 0,
                    outgoing: 0,
                    timestamp: "".to_string(),
                },
            )]);

        assert_eq!(sniffer.modal, None);
        sniffer.update(Message::ShowModal(MyModal::ClearAll));
        assert_eq!(sniffer.modal, Some(MyModal::ClearAll));
        assert_eq!(sniffer.runtime_data.logged_notifications.len(), 1);
        sniffer.update(Message::ClearAllNotifications);
        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.runtime_data.logged_notifications.len(), 0);
    }

    #[test]
    fn test_correctly_switch_running_and_notification_pages() {
        let mut sniffer = Sniffer::new(
            Arc::new(Mutex::new(0)),
            Arc::new(Mutex::new(InfoTraffic::new())),
            Arc::new((Mutex::new(Status::Init), Default::default())),
            &Default::default(),
            &Default::default(),
            Arc::new(Mutex::new(Err(String::new()))),
        );

        // initial status
        assert_eq!(*sniffer.status_pair.0.lock().unwrap(), Status::Init);
        assert_eq!(sniffer.settings_page, None);
        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.running_page, RunningPage::Overview);
        // nothing changes
        sniffer.update(Message::SwitchPage(true));
        assert_eq!(*sniffer.status_pair.0.lock().unwrap(), Status::Init);
        assert_eq!(sniffer.settings_page, None);
        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.running_page, RunningPage::Overview);
        // switch settings
        sniffer.update(Message::OpenLastSettings);
        assert_eq!(sniffer.settings_page, Some(SettingsPage::Notifications));
        assert_eq!(sniffer.running_page, RunningPage::Overview);
        sniffer.update(Message::SwitchPage(false));
        assert_eq!(sniffer.settings_page, Some(SettingsPage::Language));
        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.running_page, RunningPage::Overview);
        sniffer.update(Message::SwitchPage(true));
        assert_eq!(sniffer.settings_page, Some(SettingsPage::Notifications));
        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.running_page, RunningPage::Overview);
        sniffer.update(Message::CloseSettings);
        assert_eq!(sniffer.settings_page, None);
        assert_eq!(sniffer.running_page, RunningPage::Overview);
        // change state to running
        *sniffer.status_pair.0.lock().unwrap() = Status::Running;
        assert_eq!(*sniffer.status_pair.0.lock().unwrap(), Status::Running);
        assert_eq!(sniffer.settings_page, None);
        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.running_page, RunningPage::Overview);
        // switch with closed setting and no packets received => nothing changes
        sniffer.update(Message::SwitchPage(true));
        assert_eq!(sniffer.running_page, RunningPage::Overview);
        assert_eq!(sniffer.settings_page, None);
        // switch with closed setting and some packets received => change running page
        sniffer.runtime_data.tot_received_packets += 1;
        sniffer.update(Message::SwitchPage(true));
        assert_eq!(sniffer.running_page, RunningPage::Inspect);
        assert_eq!(sniffer.settings_page, None);
        // switch with opened settings => change settings
        sniffer.update(Message::OpenLastSettings);
        assert_eq!(sniffer.running_page, RunningPage::Inspect);
        assert_eq!(sniffer.settings_page, Some(SettingsPage::Notifications));
        sniffer.update(Message::SwitchPage(true));
        assert_eq!(sniffer.running_page, RunningPage::Inspect);
        assert_eq!(sniffer.settings_page, Some(SettingsPage::Appearance));
    }
}
