//! Module defining the application structure: messages, updates, subscriptions.

use async_channel::Receiver;
use iced::Event::{Keyboard, Window};
use iced::keyboard::key::Named;
use iced::keyboard::{Event, Key, Modifiers};
use iced::mouse::Event::ButtonPressed;
use iced::widget::Column;
use iced::window::{Id, Level};
use iced::{Element, Point, Size, Subscription, Task, window};
use pcap::Device;
use rfd::FileHandle;
use std::collections::{HashMap, HashSet, VecDeque};
use std::net::IpAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::gui::components::footer::footer;
use crate::gui::components::header::header;
use crate::gui::components::modal::{get_clear_all_overlay, get_exit_overlay, modal};
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
use crate::gui::styles::types::custom_palette::{CustomPalette, ExtraStyles};
use crate::gui::styles::types::palette::Palette;
use crate::gui::types::conf::Conf;
use crate::gui::types::config_window::{
    ConfigWindow, PositionTuple, ScaleAndCheck, SizeTuple, ToPoint, ToSize,
};
use crate::gui::types::message::Message;
use crate::gui::types::settings::Settings;
use crate::gui::types::timing_events::TimingEvents;
use crate::mmdb::asn::ASN_MMDB;
use crate::mmdb::country::COUNTRY_MMDB;
use crate::mmdb::types::mmdb_reader::{MmdbReader, MmdbReaders};
use crate::networking::parse_packets::BackendTrafficMessage;
use crate::networking::parse_packets::parse_packets;
use crate::networking::types::capture_context::{
    CaptureContext, CaptureSource, CaptureSourcePicklist, MyPcapImport,
};
use crate::networking::types::data_representation::DataRepr;
use crate::networking::types::host::{Host, HostMessage};
use crate::networking::types::host_data_states::HostDataStates;
use crate::networking::types::info_traffic::InfoTraffic;
use crate::networking::types::my_device::MyDevice;
use crate::notifications::notify_and_log::notify_and_log;
use crate::notifications::types::logged_notification::LoggedNotification;
use crate::notifications::types::notifications::{DataNotification, Notification};
use crate::notifications::types::sound::{Sound, play};
use crate::report::get_report_entries::get_searched_entries;
use crate::report::types::search_parameters::SearchParameters;
use crate::translations::types::language::Language;
use crate::utils::check_updates::set_newer_release_status;
use crate::utils::error_logger::{ErrorLogger, Location};
use crate::utils::types::file_info::FileInfo;
use crate::utils::types::web_page::WebPage;
use crate::{StyleType, TrafficChart, location};

pub const FONT_FAMILY_NAME: &str = "Sarasa Mono SC for Sniffnet";
pub const ICON_FONT_FAMILY_NAME: &str = "Icons for Sniffnet";

/// Struct on which the GUI is based
///
/// It carries statuses, network traffic statistics, and more
pub struct Sniffer {
    /// Parameters that are persistent across runs
    pub conf: Conf,
    /// Capture receiver clone (to close the channel after every run), with the current capture id (to ignore pending messages from previous captures)
    pub current_capture_rx: (usize, Option<Receiver<BackendTrafficMessage>>),
    /// Capture data
    pub info_traffic: InfoTraffic,
    /// Map of the resolved addresses with their full rDNS value and the corresponding host
    pub addresses_resolved: HashMap<IpAddr, (String, Host)>,
    /// Collection of the favorite hosts
    pub favorite_hosts: HashSet<Host>,
    /// Log of the displayed notifications, with the total number of notifications for this capture
    pub logged_notifications: (VecDeque<LoggedNotification>, usize),
    /// Reports if a newer release of the software is available on GitHub
    pub newer_release_available: Option<bool>,
    /// Network device to be analyzed, or PCAP file to be imported
    pub capture_source: CaptureSource,
    /// List of network devices
    pub my_devices: Vec<MyDevice>,
    /// Signals if a pcap error occurred
    pub pcap_error: Option<String>,
    /// Messages status
    pub dots_pulse: (String, u8),
    /// Chart displayed
    pub traffic_chart: TrafficChart,
    /// Currently displayed modal; None if no modal is displayed
    pub modal: Option<MyModal>,
    /// Currently displayed settings page; None if settings is closed
    pub settings_page: Option<SettingsPage>,
    /// Defines the current running page
    pub running_page: RunningPage,
    /// Number of unread notifications
    pub unread_notifications: usize,
    /// Search parameters of inspect page
    pub search: SearchParameters,
    /// Current page number of inspect search results
    pub page_number: usize,
    /// MMDB readers for country and ASN
    pub mmdb_readers: MmdbReaders,
    /// Time-related events
    pub timing_events: TimingEvents,
    /// Whether thumbnail mode is currently active
    pub thumbnail: bool,
    /// Window id
    pub id: Option<Id>,
    /// Host data for filter dropdowns (comboboxes)
    pub host_data_states: HostDataStates,
}

impl Sniffer {
    pub fn new(conf: Conf) -> Self {
        let Settings {
            style,
            language,
            mmdb_country,
            mmdb_asn,
            ..
        } = conf.settings.clone();
        let capture_source = CaptureSource::from_conf(&conf);
        Self {
            conf,
            current_capture_rx: (0, None),
            info_traffic: InfoTraffic::default(),
            addresses_resolved: HashMap::new(),
            favorite_hosts: HashSet::new(),
            logged_notifications: (VecDeque::new(), 0),
            newer_release_available: None,
            capture_source,
            my_devices: Vec::new(),
            pcap_error: None,
            dots_pulse: (".".to_string(), 0),
            traffic_chart: TrafficChart::new(style, language),
            modal: None,
            settings_page: None,
            running_page: RunningPage::Init,
            unread_notifications: 0,
            search: SearchParameters::default(),
            page_number: 1,
            mmdb_readers: MmdbReaders {
                country: Arc::new(MmdbReader::from(&mmdb_country, COUNTRY_MMDB)),
                asn: Arc::new(MmdbReader::from(&mmdb_asn, ASN_MMDB)),
            },
            timing_events: TimingEvents::default(),
            thumbnail: false,
            id: None,
            host_data_states: HostDataStates::default(),
        }
    }

    fn keyboard_subscription(&self) -> Subscription<Message> {
        const NO_MODIFIER: Modifiers = Modifiers::empty();

        if self.thumbnail {
            iced::event::listen_with(|event, _, _| match event {
                Keyboard(Event::KeyPressed {
                    key,
                    modifiers: Modifiers::COMMAND,
                    ..
                }) => match key.as_ref() {
                    Key::Character("q") => Some(Message::QuitWrapper),
                    Key::Character("t") => Some(Message::CtrlTPressed),
                    _ => None,
                },
                _ => None,
            })
        } else {
            iced::event::listen_with(|event, _, _| match event {
                Keyboard(Event::KeyPressed { key, modifiers, .. }) => match modifiers {
                    Modifiers::COMMAND => match key.as_ref() {
                        Key::Character("q") => Some(Message::QuitWrapper),
                        Key::Character("t") => Some(Message::CtrlTPressed),
                        Key::Character(",") => Some(Message::OpenLastSettings),
                        Key::Named(Named::Backspace) => Some(Message::ResetButtonPressed),
                        Key::Character("d") => Some(Message::CtrlDPressed),
                        Key::Named(Named::ArrowLeft) => Some(Message::ArrowPressed(false)),
                        Key::Named(Named::ArrowRight) => Some(Message::ArrowPressed(true)),
                        Key::Character("-") => Some(Message::ScaleFactorShortcut(false)),
                        Key::Character("+") => Some(Message::ScaleFactorShortcut(true)),
                        _ => None,
                    },
                    Modifiers::SHIFT => match key {
                        Key::Named(Named::Tab) => Some(Message::SwitchPage(false)),
                        _ => None,
                    },
                    NO_MODIFIER => match key {
                        Key::Named(Named::Enter) => Some(Message::ReturnKeyPressed),
                        Key::Named(Named::Escape) => Some(Message::EscKeyPressed),
                        Key::Named(Named::Tab) => Some(Message::SwitchPage(true)),
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            })
        }
    }

    fn mouse_subscription(&self) -> Subscription<Message> {
        if self.thumbnail {
            iced::event::listen_with(|event, _, _| match event {
                iced::event::Event::Mouse(ButtonPressed(_)) => Some(Message::Drag),
                _ => None,
            })
        } else {
            Subscription::none()
        }
    }

    fn time_subscription() -> Subscription<Message> {
        iced::time::every(Duration::from_millis(1000)).map(|_| Message::Periodic)
    }

    fn window_subscription() -> Subscription<Message> {
        iced::event::listen_with(|event, _, _| match event {
            Window(window::Event::Focused) => Some(Message::WindowFocused),
            Window(window::Event::Moved(Point { x, y })) => Some(Message::WindowMoved(x, y)),
            Window(window::Event::Resized(Size { width, height })) => {
                Some(Message::WindowResized(width, height))
            }
            Window(window::Event::CloseRequested) => Some(Message::QuitWrapper),
            _ => None,
        })
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        self.dots_pulse.1 = (self.dots_pulse.1 + 1) % 3;
        match message {
            Message::StartApp(id) => {
                self.id = id;
                return Task::batch([
                    Sniffer::register_sigint_handler(),
                    Task::perform(set_newer_release_status(), Message::SetNewerReleaseStatus),
                ]);
            }
            Message::TickRun(cap_id, msg, host_msgs, no_more_packets) => {
                if cap_id == self.current_capture_rx.0 {
                    for host_msg in host_msgs {
                        self.handle_new_host(host_msg);
                    }
                    self.refresh_data(msg, no_more_packets);
                }
            }
            Message::DeviceSelection(name) => self.set_device(&name),
            Message::SetCaptureSource(cs_pick) => {
                self.conf.capture_source_picklist = cs_pick;
                return if cs_pick == CaptureSourcePicklist::File {
                    Task::done(Message::SetPcapImport(self.conf.import_pcap_path.clone()))
                } else {
                    Task::done(Message::DeviceSelection(
                        self.conf.device.device_name.clone(),
                    ))
                };
            }
            Message::ToggleFilters => {
                self.conf.filters.toggle();
            }
            Message::BpfFilter(value) => {
                self.conf.filters.set_bpf(value);
            }
            Message::DataReprSelection(unit) => self.traffic_chart.change_kind(unit),
            Message::ReportSortSelection(sort) => {
                self.page_number = 1;
                self.conf.report_sort_type = sort;
            }
            Message::OpenWebPage(web_page) => Self::open_web(&web_page),
            Message::Start => {
                if self.is_capture_source_consistent() {
                    return self.start();
                }
            }
            Message::Reset => self.reset(),
            Message::Style(style) => {
                self.conf.settings.style = style;
                self.traffic_chart.change_style(style);
            }
            Message::LoadStyle(path) => {
                self.conf.settings.style_path.clone_from(&path);
                if let Ok(palette) = Palette::from_file(path) {
                    let style = StyleType::Custom(ExtraStyles::CustomToml(
                        CustomPalette::from_palette(palette),
                    ));
                    self.conf.settings.style = style;
                    self.traffic_chart.change_style(style);
                }
            }
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
                    self.settings_page = Some(self.conf.last_opened_setting);
                }
            }
            Message::CloseSettings => self.close_settings(),
            Message::ChangeRunningPage(running_page) => {
                self.running_page = running_page;
                if running_page.eq(&RunningPage::Notifications) {
                    self.unread_notifications = 0;
                }
            }
            Message::LanguageSelection(language) => {
                self.conf.settings.language = language;
                self.traffic_chart.change_language(language);
            }
            Message::UpdateNotificationSettings(notification, emit_sound) => {
                self.update_notifications_settings(notification, emit_sound);
            }
            Message::ChangeVolume(volume) => {
                play(Sound::Pop, volume);
                self.conf.settings.notifications.volume = volume;
            }
            Message::ClearAllNotifications => {
                self.logged_notifications.0 = VecDeque::new();
                self.modal = None;
            }
            Message::SwitchPage(next) => {
                // To prevent SwitchPage be triggered when using `Alt` + `Tab` to switch back,
                // first check if user switch back just now, and ignore the request for a short time.
                if !self.timing_events.was_just_focus() {
                    self.switch_page(next);
                }
            }
            Message::ReturnKeyPressed => return self.shortcut_return(),
            Message::EscKeyPressed => return self.shortcut_esc(),
            Message::ResetButtonPressed => return self.reset_button_pressed(),
            Message::CtrlDPressed => return self.shortcut_ctrl_d(),
            Message::Search(parameters) => {
                // update comboboxes
                let host_data = &mut self.host_data_states.data;
                host_data.countries.1 = self.search.country != parameters.country;
                host_data.asns.1 = self.search.as_name != parameters.as_name;
                host_data.domains.1 = self.search.domain != parameters.domain;
                self.host_data_states.update_states(&parameters);

                self.page_number = 1;
                self.running_page = RunningPage::Inspect;
                self.search = parameters;
            }
            Message::UpdatePageNumber(increment) => {
                if increment {
                    if self.page_number < get_searched_entries(self).1.div_ceil(20) {
                        self.page_number = self.page_number.checked_add(1).unwrap_or(1);
                    }
                } else if self.page_number > 1 {
                    self.page_number = self.page_number.checked_sub(1).unwrap_or(1);
                }
            }
            Message::ArrowPressed(increment) => {
                if self.running_page.eq(&RunningPage::Inspect)
                    && self.settings_page.is_none()
                    && self.modal.is_none()
                {
                    return Task::done(Message::UpdatePageNumber(increment));
                }
            }
            Message::WindowFocused => self.timing_events.focus_now(),
            Message::GradientsSelection(gradient_type) => {
                self.conf.settings.color_gradient = gradient_type;
            }
            Message::ChangeScaleFactor(slider_val) => {
                let scale_factor_str = format!("{:.1}", 3.0_f64.powf(slider_val));
                self.conf.settings.scale_factor = scale_factor_str.parse().unwrap_or(1.0);
            }
            Message::WindowMoved(x, y) => {
                let scale_factor = self.conf.settings.scale_factor;
                let scaled = PositionTuple(x, y).scale_and_check(scale_factor);
                if self.thumbnail {
                    self.conf.window.thumbnail_position = scaled;
                } else {
                    self.conf.window.position = scaled;
                }
            }
            Message::WindowResized(width, height) => {
                if !self.thumbnail {
                    let scale_factor = self.conf.settings.scale_factor;
                    self.conf.window.size = SizeTuple(width, height).scale_and_check(scale_factor);
                } else if !self.timing_events.was_just_thumbnail_enter() {
                    return Task::done(Message::ToggleThumbnail(true));
                }
            }
            Message::CustomCountryDb(db) => {
                self.conf.settings.mmdb_country.clone_from(&db);
                self.mmdb_readers.country = Arc::new(MmdbReader::from(&db, COUNTRY_MMDB));
            }
            Message::CustomAsnDb(db) => {
                self.conf.settings.mmdb_asn.clone_from(&db);
                self.mmdb_readers.asn = Arc::new(MmdbReader::from(&db, ASN_MMDB));
            }
            Message::QuitWrapper => return self.quit_wrapper(),
            Message::Quit => {
                let _ = self.conf.clone().store();
                return window::close(self.id.unwrap_or_else(Id::unique));
            }
            Message::CopyIp(ip) => {
                self.timing_events.copy_ip_now(ip);
                return iced::clipboard::write(ip.to_string());
            }
            Message::OpenFile(old_file, file_info, consumer_message) => {
                return Task::perform(
                    Self::open_file(old_file, file_info, self.conf.settings.language),
                    consumer_message,
                );
            }
            Message::HostSortSelection(sort_type) => {
                self.conf.host_sort_type = sort_type;
            }
            Message::ServiceSortSelection(sort_type) => {
                self.conf.service_sort_type = sort_type;
            }
            Message::ToggleExportPcap => {
                self.conf.export_pcap.toggle();
            }
            Message::OutputPcapDir(path) => {
                self.conf.export_pcap.set_directory(path);
            }
            Message::OutputPcapFile(name) => {
                self.conf.export_pcap.set_file_name(&name);
            }
            Message::ToggleThumbnail(triggered_by_resize) => {
                let window_id = self.id.unwrap_or_else(Id::unique);

                self.thumbnail = !self.thumbnail;
                self.traffic_chart.thumbnail = self.thumbnail;

                return if self.thumbnail {
                    let scale_factor = self.conf.settings.scale_factor;
                    let size = ConfigWindow::thumbnail_size(scale_factor).to_size();
                    let position = self.conf.window.thumbnail_position;
                    self.timing_events.thumbnail_enter_now();
                    Task::batch([
                        window::maximize(window_id, false),
                        window::toggle_decorations(window_id),
                        window::resize(window_id, size),
                        window::move_to(window_id, position.to_point()),
                        window::change_level(window_id, Level::AlwaysOnTop),
                    ])
                } else {
                    if self.running_page.eq(&RunningPage::Notifications) {
                        self.unread_notifications = 0;
                    }
                    let mut commands = vec![
                        window::toggle_decorations(window_id),
                        window::change_level(window_id, Level::Normal),
                    ];
                    if !triggered_by_resize {
                        let size = self.conf.window.size.to_size();
                        let position = self.conf.window.position.to_point();
                        commands.push(window::move_to(window_id, position));
                        commands.push(window::resize(window_id, size));
                    }
                    Task::batch(commands)
                };
            }
            Message::Drag => {
                let was_just_thumbnail_click = self.timing_events.was_just_thumbnail_click();
                self.timing_events.thumbnail_click_now();
                if was_just_thumbnail_click {
                    return window::drag(self.id.unwrap_or_else(Id::unique));
                }
            }
            Message::CtrlTPressed => {
                if self.running_page.ne(&RunningPage::Init)
                    && self.settings_page.is_none()
                    && self.modal.is_none()
                    && !self.timing_events.was_just_thumbnail_enter()
                {
                    return Task::done(Message::ToggleThumbnail(false));
                }
            }
            Message::ScaleFactorShortcut(increase) => {
                let scale_factor = self.conf.settings.scale_factor;
                if !(scale_factor > 2.99 && increase || scale_factor < 0.31 && !increase) {
                    let delta = if increase { 0.1 } else { -0.1 };
                    self.conf.settings.scale_factor += delta;
                }
            }
            Message::SetNewerReleaseStatus(status) => self.newer_release_available = status,
            Message::SetPcapImport(path) => {
                if !path.is_empty() {
                    self.conf.import_pcap_path.clone_from(&path);
                    self.capture_source = CaptureSource::File(MyPcapImport::new(path));
                }
            }
            Message::PendingHosts(cap_id, host_msgs) => {
                if cap_id == self.current_capture_rx.0 {
                    for host_msg in host_msgs {
                        self.handle_new_host(host_msg);
                    }
                }
            }
            Message::OfflineGap(cap_id, gap) => {
                if cap_id == self.current_capture_rx.0 {
                    self.traffic_chart.push_offline_gap_to_splines(gap);
                }
            }
            Message::Periodic => {
                self.update_waiting_dots();
                self.fetch_devices();
                self.update_threshold();
            }
            Message::ExpandNotification(id, expand) => {
                if let Some(n) = self
                    .logged_notifications
                    .0
                    .iter_mut()
                    .find(|n| n.id() == id)
                {
                    n.expand(expand);
                }
            }
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message, StyleType> {
        let Settings {
            style,
            language,
            color_gradient,
            ..
        } = self.conf.settings;
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
            self.newer_release_available,
            self.dots_pulse.1,
        );

        let content: Element<Message, StyleType> =
            Column::new().push(header).push(body).push(footer).into();

        match self.modal.clone() {
            None => {
                if let Some(settings_page) = self.settings_page {
                    let overlay: Element<Message, StyleType> = match settings_page {
                        SettingsPage::Notifications => settings_notifications_page(self),
                        SettingsPage::Appearance => settings_style_page(self),
                        SettingsPage::General => settings_general_page(self),
                    }
                    .into();

                    modal(content, overlay, Message::CloseSettings)
                } else {
                    content
                }
            }
            Some(m) => {
                let overlay: Element<Message, StyleType> = match m {
                    MyModal::Reset => get_exit_overlay(
                        Message::Reset,
                        color_gradient,
                        font,
                        font_headers,
                        language,
                    ),
                    MyModal::Quit => get_exit_overlay(
                        Message::Quit,
                        color_gradient,
                        font,
                        font_headers,
                        language,
                    ),
                    MyModal::ClearAll => {
                        get_clear_all_overlay(color_gradient, font, font_headers, language)
                    }
                    MyModal::ConnectionDetails(key) => connection_details_page(self, key),
                }
                .into();

                modal(content, overlay, Message::HideModal)
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            self.keyboard_subscription(),
            self.mouse_subscription(),
            Sniffer::time_subscription(),
            Sniffer::window_subscription(),
        ])
    }

    pub fn theme(&self) -> StyleType {
        self.conf.settings.style
    }

    pub fn scale_factor(&self) -> f64 {
        self.conf.settings.scale_factor
    }

    /// Updates threshold if it hasn't been edited for a while
    fn update_threshold(&mut self) {
        // Ignore if just edited
        if let Some(temp_threshold) = self.timing_events.threshold_adjust_expired_take() {
            // Apply the temporary threshold to the actual config
            self.conf.settings.notifications.data_notification.threshold = temp_threshold.threshold;
            self.conf
                .settings
                .notifications
                .data_notification
                .byte_multiple = temp_threshold.byte_multiple;
            self.conf
                .settings
                .notifications
                .data_notification
                .previous_threshold = temp_threshold.previous_threshold;
        }
    }

    fn refresh_data(&mut self, mut msg: InfoTraffic, no_more_packets: bool) {
        self.info_traffic.refresh(&mut msg);
        if self.info_traffic.tot_data_info.tot_data(DataRepr::Packets) == 0 {
            return;
        }
        let emitted_notifications = notify_and_log(
            &mut self.logged_notifications,
            self.conf.settings.notifications,
            &msg,
            &self.favorite_hosts,
            &self.capture_source,
        );
        if self.thumbnail || self.running_page.ne(&RunningPage::Notifications) {
            self.unread_notifications += emitted_notifications;
        }
        self.traffic_chart.update_charts_data(&msg, no_more_packets);

        // update host dropdowns
        self.host_data_states.update_states(&self.search);
    }

    fn open_web(web_page: &WebPage) {
        let url = web_page.get_url();

        #[cfg(target_os = "windows")]
        let cmd = "explorer";
        #[cfg(target_os = "macos")]
        let cmd = "open";
        #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
        let cmd = "xdg-open";

        let Ok(mut child) = std::process::Command::new(cmd)
            .arg(url)
            .spawn()
            .log_err(location!())
        else {
            return;
        };

        child.wait().unwrap_or_default();
    }

    fn start(&mut self) -> Task<Message> {
        if matches!(&self.capture_source, CaptureSource::Device(_)) {
            let current_device_name = &self.capture_source.get_name();
            self.set_device(current_device_name);
        }
        let pcap_path = self.conf.export_pcap.full_path();
        let capture_context =
            CaptureContext::new(&self.capture_source, pcap_path.as_ref(), &self.conf.filters);
        self.pcap_error = capture_context.error().map(ToString::to_string);
        self.running_page = RunningPage::Overview;

        if capture_context.error().is_none() {
            // no pcap error
            let curr_cap_id = self.current_capture_rx.0;
            let mmdb_readers = self.mmdb_readers.clone();
            self.capture_source
                .set_link_type(capture_context.my_link_type());
            let capture_source = self.capture_source.clone();
            self.traffic_chart
                .change_capture_source(matches!(capture_source, CaptureSource::Device(_)));
            let (tx, rx) = async_channel::unbounded();
            let _ = thread::Builder::new()
                .name("thread_parse_packets".to_string())
                .spawn(move || {
                    parse_packets(
                        curr_cap_id,
                        capture_source,
                        &mmdb_readers,
                        capture_context,
                        &tx,
                    );
                })
                .log_err(location!());
            self.current_capture_rx.1 = Some(rx.clone());
            return Task::run(rx, |backend_msg| match backend_msg {
                BackendTrafficMessage::TickRun(cap_id, msg, host_msg, no_more_packets) => {
                    Message::TickRun(cap_id, msg, host_msg, no_more_packets)
                }
                BackendTrafficMessage::PendingHosts(cap_id, host_msg) => {
                    Message::PendingHosts(cap_id, host_msg)
                }
                BackendTrafficMessage::OfflineGap(cap_id, gap) => Message::OfflineGap(cap_id, gap),
            });
        }
        Task::none()
    }

    fn reset(&mut self) {
        // close capture channel to kill previous captures
        if let Some(rx) = &self.current_capture_rx.1 {
            rx.close();
        }
        let Settings {
            style, language, ..
        } = self.conf.settings;
        // increment capture id to ignore pending messages from previous captures
        self.current_capture_rx = (self.current_capture_rx.0 + 1, None);
        self.info_traffic = InfoTraffic::default();
        self.addresses_resolved = HashMap::new();
        self.favorite_hosts = HashSet::new();
        self.logged_notifications = (VecDeque::new(), 0);
        self.pcap_error = None;
        self.traffic_chart = TrafficChart::new(style, language);
        self.modal = None;
        self.settings_page = None;
        self.running_page = RunningPage::Init;
        self.unread_notifications = 0;
        self.search = SearchParameters::default();
        self.page_number = 1;
        self.thumbnail = false;
        self.host_data_states = HostDataStates::default();
    }

    fn set_device(&mut self, name: &str) {
        for my_dev in &self.my_devices {
            if my_dev.get_name().eq(&name) {
                self.conf.device.device_name = name.to_string();
                self.capture_source = CaptureSource::Device(my_dev.clone());
                break;
            }
        }
    }

    fn fetch_devices(&mut self) {
        self.my_devices.clear();
        for dev in Device::list().log_err(location!()).unwrap_or_default() {
            if matches!(&self.capture_source, CaptureSource::Device(_))
                && dev.name.eq(&self.capture_source.get_name())
            {
                // refresh active addresses
                self.capture_source.set_addresses(dev.addresses.clone());
            }
            let my_dev = MyDevice::from_pcap_device(dev);
            self.my_devices.push(my_dev);
        }
    }

    fn update_waiting_dots(&mut self) {
        if self.dots_pulse.0.len() > 2 {
            self.dots_pulse.0 = String::new();
        }
        self.dots_pulse.0 = ".".repeat(self.dots_pulse.0.len() + 1);
    }

    fn add_or_remove_favorite(&mut self, host: &Host, add: bool) {
        let info_traffic = &mut self.info_traffic;
        if add {
            self.favorite_hosts.insert(host.clone());
        } else {
            self.favorite_hosts.remove(host);
        }
        if let Some(host_info) = info_traffic.hosts.get_mut(host) {
            host_info.is_favorite = add;
        }
    }

    fn close_settings(&mut self) {
        if let Some(page) = self.settings_page {
            self.conf.last_opened_setting = page;
            self.settings_page = None;
        }
    }

    /// Don't update adjustments to threshold immediately:
    /// that is, sound and toggling threshold on/off should be applied immediately
    /// Threshold adjustments are saved in `self.timing_events.threshold_adjust` and then applied
    /// after timeout
    fn update_notifications_settings(&mut self, notification: Notification, emit_sound: bool) {
        let notifications = self.conf.settings.notifications;
        let sound = match notification {
            Notification::Data(DataNotification {
                data_repr,
                threshold,
                byte_multiple,
                sound,
                previous_threshold,
            }) => {
                let mut temp_threshold = self.get_temp_threshold();
                if temp_threshold.threshold != threshold
                    || temp_threshold.byte_multiple != byte_multiple
                    || temp_threshold.previous_threshold != previous_threshold
                {
                    temp_threshold = DataNotification {
                        data_repr,
                        threshold,
                        byte_multiple,
                        sound,
                        previous_threshold,
                    };
                    self.timing_events.threshold_adjust_now(temp_threshold);
                }
                if threshold.is_some() != notifications.data_notification.threshold.is_some() {
                    self.conf.settings.notifications.data_notification.threshold = threshold;
                    self.conf
                        .settings
                        .notifications
                        .data_notification
                        .byte_multiple = byte_multiple;
                    self.conf
                        .settings
                        .notifications
                        .data_notification
                        .previous_threshold = previous_threshold;
                }
                self.conf.settings.notifications.data_notification.sound = sound;
                self.conf.settings.notifications.data_notification.data_repr = data_repr;
                sound
            }
            Notification::Favorite(favorite_notification) => {
                self.conf.settings.notifications.favorite_notification = favorite_notification;
                favorite_notification.sound
            }
        };
        if emit_sound {
            play(sound, self.conf.settings.notifications.volume);
        }
    }

    /// Returns threshold in `timing_events.threshold_adjust` or copy of current threshold
    fn get_temp_threshold(&self) -> DataNotification {
        if let Some(temp_threshold) = self.timing_events.temp_threshold() {
            temp_threshold
        } else {
            let notifications = self.conf.settings.notifications;
            notifications.data_notification
        }
    }

    fn switch_page(&mut self, next: bool) {
        match (self.running_page, self.settings_page, self.modal.is_none()) {
            (_, Some(current_setting), true) => {
                // Settings opened
                if next {
                    self.settings_page = Some(current_setting.next());
                } else {
                    self.settings_page = Some(current_setting.previous());
                }
            }
            (
                RunningPage::Inspect | RunningPage::Notifications | RunningPage::Overview,
                None,
                true,
            ) => {
                // Running with no overlays
                if self.info_traffic.tot_data_info.tot_data(DataRepr::Packets) > 0 {
                    // Running with no overlays and some packets
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

    fn shortcut_return(&mut self) -> Task<Message> {
        if self.running_page.eq(&RunningPage::Init)
            && self.settings_page.is_none()
            && self.modal.is_none()
        {
            return Task::done(Message::Start);
        } else if self.modal.eq(&Some(MyModal::Reset)) {
            return Task::done(Message::Reset);
        } else if self.modal.eq(&Some(MyModal::Quit)) {
            return Task::done(Message::Quit);
        } else if self.modal.eq(&Some(MyModal::ClearAll)) {
            return Task::done(Message::ClearAllNotifications);
        }
        Task::none()
    }

    fn shortcut_esc(&mut self) -> Task<Message> {
        if self.modal.is_some() {
            return Task::done(Message::HideModal);
        } else if self.settings_page.is_some() {
            return Task::done(Message::CloseSettings);
        }
        Task::none()
    }

    // also called when the backspace shortcut is pressed
    fn reset_button_pressed(&mut self) -> Task<Message> {
        if self.running_page.ne(&RunningPage::Init) {
            let tot_packets = self.info_traffic.tot_data_info.tot_data(DataRepr::Packets);
            return if tot_packets == 0 && self.settings_page.is_none() {
                Task::done(Message::Reset)
            } else {
                Task::done(Message::ShowModal(MyModal::Reset))
            };
        }
        Task::none()
    }

    fn quit_wrapper(&mut self) -> Task<Message> {
        let tot_packets = self.info_traffic.tot_data_info.tot_data(DataRepr::Packets);
        if self.running_page.eq(&RunningPage::Init) || tot_packets == 0 {
            Task::done(Message::Quit)
        } else if self.thumbnail {
            // TODO: uncomment once issue #653 is fixed
            // Task::done(Message::ToggleThumbnail(false))
            //     .chain(Task::done(Message::ShowModal(MyModal::Quit)))
            Task::done(Message::Quit)
        } else {
            Task::done(Message::HideModal)
                .chain(Task::done(Message::CloseSettings))
                .chain(Task::done(Message::ShowModal(MyModal::Quit)))
        }
    }

    fn shortcut_ctrl_d(&mut self) -> Task<Message> {
        if self.running_page.eq(&RunningPage::Notifications)
            && !self.logged_notifications.0.is_empty()
        {
            return Task::done(Message::ShowModal(MyModal::ClearAll));
        }
        Task::none()
    }

    async fn open_file(old_file: String, file_info: FileInfo, language: Language) -> String {
        let starting_directory = if old_file.is_empty() {
            std::env::var("HOME").unwrap_or_default()
        } else if file_info == FileInfo::Directory {
            old_file.clone()
        } else {
            let mut folder_path = PathBuf::from(&old_file);
            folder_path.pop();
            folder_path.to_string_lossy().to_string()
        };

        let dialog = rfd::AsyncFileDialog::new()
            .set_title(file_info.action_info(language))
            .set_directory(starting_directory);

        let picked = if file_info == FileInfo::Directory {
            dialog.pick_folder().await
        } else {
            let extensions = file_info.get_extensions();
            dialog
                .add_filter(format!("{extensions:?}"), &extensions)
                .pick_file()
                .await
        }
        .unwrap_or_else(|| FileHandle::from(PathBuf::from(&old_file)));

        picked.path().to_string_lossy().to_string()
    }

    fn handle_new_host(&mut self, host_msg: HostMessage) {
        let HostMessage {
            host,
            data_info_host,
            address_to_lookup,
            rdns,
        } = host_msg;

        self.info_traffic
            .hosts
            .entry(host.clone())
            .and_modify(|d| {
                d.refresh(&data_info_host);
            })
            .or_insert(data_info_host);

        self.addresses_resolved
            .insert(address_to_lookup, (rdns, host.clone()));

        // update host data states including the new host
        self.host_data_states.data.update(&host);
    }

    fn register_sigint_handler() -> Task<Message> {
        let (tx, rx) = async_channel::bounded(1);

        // gracefully close the app when receiving SIGINT, SIGTERM, or SIGHUP
        let _ = ctrlc::set_handler(move || {
            let _ = tx.send_blocking(());
        })
        .log_err(location!());

        Task::run(rx, |()| Message::Quit)
    }

    pub fn is_capture_source_consistent(&self) -> bool {
        self.conf.capture_source_picklist == CaptureSourcePicklist::Device
            && matches!(self.capture_source, CaptureSource::Device(_))
            || self.conf.capture_source_picklist == CaptureSourcePicklist::File
                && matches!(self.capture_source, CaptureSource::File(_))
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_must_use)]

    use std::collections::{HashSet, VecDeque};
    use std::fs::remove_file;
    use std::path::Path;
    use std::time::Duration;

    use serial_test::{parallel, serial};

    use crate::configs::types::config_window::{PositionTuple, SizeTuple};
    use crate::countries::types::country::Country;
    use crate::gui::components::types::my_modal::MyModal;
    use crate::gui::pages::types::settings_page::SettingsPage;
    use crate::gui::styles::types::custom_palette::ExtraStyles;
    use crate::gui::styles::types::gradient_type::GradientType;
    use crate::gui::types::message::Message;
    use crate::gui::types::timing_events::TimingEvents;
    use crate::networking::types::data_info::DataInfo;
    use crate::networking::types::data_representation::DataRepr;
    use crate::networking::types::host::Host;
    use crate::networking::types::traffic_direction::TrafficDirection;
    use crate::notifications::types::logged_notification::{
        DataThresholdExceeded, LoggedNotification,
    };
    use crate::notifications::types::notifications::{
        DataNotification, FavoriteNotification, Notification, Notifications,
    };
    use crate::notifications::types::sound::Sound;
    use crate::report::types::sort_type::SortType;
    use crate::{
        ByteMultiple, ConfigDevice, ConfigWindow, Configs, Language, ReportSortType, RunningPage,
        Settings, Sniffer, StyleType,
    };

    // helpful to clean up files generated from tests
    impl Drop for Sniffer {
        fn drop(&mut self) {
            let settings_path_str = Settings::test_path();
            let settings_path = Path::new(&settings_path_str);
            if settings_path.exists() {
                remove_file(Settings::test_path()).unwrap();
            }

            let device_path_str = ConfigDevice::test_path();
            let device_path = Path::new(&device_path_str);
            if device_path.exists() {
                remove_file(ConfigDevice::test_path()).unwrap();
            }

            let window_path_str = ConfigWindow::test_path();
            let window_path = Path::new(&window_path_str);
            if window_path.exists() {
                remove_file(ConfigWindow::test_path()).unwrap();
            }
        }
    }

    #[test]
    #[parallel] // needed to not collide with other tests generating configs files
    fn test_correctly_update_chart_kind() {
        let mut sniffer = Sniffer::new(Configs::default());

        assert_eq!(sniffer.traffic_chart.data_repr, DataRepr::Bytes);
        sniffer.update(Message::DataReprSelection(DataRepr::Packets));
        assert_eq!(sniffer.traffic_chart.data_repr, DataRepr::Packets);
        sniffer.update(Message::DataReprSelection(DataRepr::Packets));
        assert_eq!(sniffer.traffic_chart.data_repr, DataRepr::Packets);
        sniffer.update(Message::DataReprSelection(DataRepr::Bytes));
        assert_eq!(sniffer.traffic_chart.data_repr, DataRepr::Bytes);
        sniffer.update(Message::DataReprSelection(DataRepr::Bits));
        assert_eq!(sniffer.traffic_chart.data_repr, DataRepr::Bits);
    }

    #[test]
    #[parallel] // needed to not collide with other tests generating configs files
    fn test_correctly_update_report_sort_kind() {
        let mut sniffer = Sniffer::new(Configs::default());

        let sort = ReportSortType {
            data_sort: SortType::Neutral,
        };

        assert_eq!(sniffer.report_sort_type, sort);
        sniffer.update(Message::ReportSortSelection(sort.next_sort()));
        assert_eq!(
            sniffer.report_sort_type,
            ReportSortType {
                data_sort: SortType::Descending,
            }
        );
        sniffer.update(Message::ReportSortSelection(sort.next_sort().next_sort()));
        assert_eq!(
            sniffer.report_sort_type,
            ReportSortType {
                data_sort: SortType::Ascending,
            }
        );
        sniffer.update(Message::ReportSortSelection(
            sort.next_sort().next_sort().next_sort(),
        ));
        assert_eq!(
            sniffer.report_sort_type,
            ReportSortType {
                data_sort: SortType::Neutral,
            }
        );
    }

    #[test]
    #[parallel] // needed to not collide with other tests generating configs files
    fn test_correctly_update_host_sort_kind() {
        let mut sniffer = Sniffer::new(Configs::default());

        let mut sort = SortType::Neutral;

        assert_eq!(sniffer.host_sort_type, sort);

        sort = sort.next_sort();
        sniffer.update(Message::HostSortSelection(sort));
        assert_eq!(sniffer.host_sort_type, SortType::Descending);

        sort = sort.next_sort();
        sniffer.update(Message::HostSortSelection(sort));
        assert_eq!(sniffer.host_sort_type, SortType::Ascending);

        sort = sort.next_sort();
        sniffer.update(Message::HostSortSelection(sort));
        assert_eq!(sniffer.host_sort_type, SortType::Neutral);
    }

    #[test]
    #[parallel] // needed to not collide with other tests generating configs files
    fn test_correctly_update_service_sort_kind() {
        let mut sniffer = Sniffer::new(Configs::default());

        let mut sort = SortType::Neutral;

        assert_eq!(sniffer.service_sort_type, sort);

        sort = sort.next_sort();
        sniffer.update(Message::ServiceSortSelection(sort));
        assert_eq!(sniffer.service_sort_type, SortType::Descending);

        sort = sort.next_sort();
        sniffer.update(Message::ServiceSortSelection(sort));
        assert_eq!(sniffer.service_sort_type, SortType::Ascending);

        sort = sort.next_sort();
        sniffer.update(Message::ServiceSortSelection(sort));
        assert_eq!(sniffer.service_sort_type, SortType::Neutral);
    }

    #[test]
    #[parallel] // needed to not collide with other tests generating configs files
    fn test_correctly_update_style() {
        let mut sniffer = Sniffer::new(Configs::default());

        sniffer.update(Message::Style(StyleType::MonAmour));
        assert_eq!(sniffer.configs.settings.style, StyleType::MonAmour);
        sniffer.update(Message::Style(StyleType::Day));
        assert_eq!(sniffer.configs.settings.style, StyleType::Day);
        sniffer.update(Message::Style(StyleType::Night));
        assert_eq!(sniffer.configs.settings.style, StyleType::Night);
        sniffer.update(Message::Style(StyleType::DeepSea));
        assert_eq!(sniffer.configs.settings.style, StyleType::DeepSea);
        sniffer.update(Message::Style(StyleType::DeepSea));
        assert_eq!(sniffer.configs.settings.style, StyleType::DeepSea);
    }

    #[test]
    #[parallel] // needed to not collide with other tests generating configs files
    fn test_dots_pulse_update() {
        // every kind of message will the integer, but only Periodic will update the string
        let mut sniffer = Sniffer::new(Configs::default());

        assert_eq!(sniffer.dots_pulse, (".".to_string(), 0));

        sniffer.update(Message::Periodic);
        assert_eq!(sniffer.dots_pulse, ("..".to_string(), 1));

        sniffer.update(Message::HideModal);
        assert_eq!(sniffer.dots_pulse, ("..".to_string(), 2));

        sniffer.update(Message::CtrlDPressed);
        assert_eq!(sniffer.dots_pulse, ("..".to_string(), 0));

        sniffer.update(Message::Periodic);
        assert_eq!(sniffer.dots_pulse, ("...".to_string(), 1));

        sniffer.update(Message::OpenLastSettings);
        assert_eq!(sniffer.dots_pulse, ("...".to_string(), 2));

        sniffer.update(Message::Periodic);
        assert_eq!(sniffer.dots_pulse, (".".to_string(), 0));
    }

    #[test]
    #[parallel] // needed to not collide with other tests generating configs files
    fn test_modify_favorite_connections() {
        let mut sniffer = Sniffer::new(Configs::default());
        // remove 1
        sniffer.update(Message::AddOrRemoveFavorite(
            Host {
                domain: "1.1".to_string(),
                asn: Default::default(),
                country: Country::US,
            },
            false,
        ));
        assert_eq!(sniffer.favorite_hosts, HashSet::new());
        // remove 2
        sniffer.update(Message::AddOrRemoveFavorite(
            Host {
                domain: "2.2".to_string(),
                asn: Default::default(),
                country: Country::US,
            },
            false,
        ));
        assert_eq!(sniffer.favorite_hosts, HashSet::new());
        // add 2
        sniffer.update(Message::AddOrRemoveFavorite(
            Host {
                domain: "2.2".to_string(),
                asn: Default::default(),
                country: Country::US,
            },
            true,
        ));
        assert_eq!(
            sniffer.favorite_hosts,
            HashSet::from([Host {
                domain: "2.2".to_string(),
                asn: Default::default(),
                country: Country::US,
            }])
        );
        // remove 1
        sniffer.update(Message::AddOrRemoveFavorite(
            Host {
                domain: "1.1".to_string(),
                asn: Default::default(),
                country: Country::US,
            },
            false,
        ));
        assert_eq!(
            sniffer.favorite_hosts,
            HashSet::from([Host {
                domain: "2.2".to_string(),
                asn: Default::default(),
                country: Country::US,
            }])
        );
        // add 2
        sniffer.update(Message::AddOrRemoveFavorite(
            Host {
                domain: "2.2".to_string(),
                asn: Default::default(),
                country: Country::US,
            },
            true,
        ));
        assert_eq!(
            sniffer.favorite_hosts,
            HashSet::from([Host {
                domain: "2.2".to_string(),
                asn: Default::default(),
                country: Country::US,
            }])
        );
        // add 1
        sniffer.update(Message::AddOrRemoveFavorite(
            Host {
                domain: "1.1".to_string(),
                asn: Default::default(),
                country: Country::US,
            },
            true,
        ));
        assert_eq!(
            sniffer.favorite_hosts,
            HashSet::from([
                Host {
                    domain: "1.1".to_string(),
                    asn: Default::default(),
                    country: Country::US,
                },
                Host {
                    domain: "2.2".to_string(),
                    asn: Default::default(),
                    country: Country::US,
                }
            ])
        );
        // add 3
        sniffer.update(Message::AddOrRemoveFavorite(
            Host {
                domain: "3.3".to_string(),
                asn: Default::default(),
                country: Country::US,
            },
            true,
        ));
        assert_eq!(
            sniffer.favorite_hosts,
            HashSet::from([
                Host {
                    domain: "1.1".to_string(),
                    asn: Default::default(),
                    country: Country::US,
                },
                Host {
                    domain: "2.2".to_string(),
                    asn: Default::default(),
                    country: Country::US,
                },
                Host {
                    domain: "3.3".to_string(),
                    asn: Default::default(),
                    country: Country::US,
                }
            ])
        );
        // remove 2
        sniffer.update(Message::AddOrRemoveFavorite(
            Host {
                domain: "2.2".to_string(),
                asn: Default::default(),
                country: Country::US,
            },
            false,
        ));
        assert_eq!(
            sniffer.favorite_hosts,
            HashSet::from([
                Host {
                    domain: "1.1".to_string(),
                    asn: Default::default(),
                    country: Country::US,
                },
                Host {
                    domain: "3.3".to_string(),
                    asn: Default::default(),
                    country: Country::US,
                }
            ])
        );
        // remove 3
        sniffer.update(Message::AddOrRemoveFavorite(
            Host {
                domain: "3.3".to_string(),
                asn: Default::default(),
                country: Country::US,
            },
            false,
        ));
        assert_eq!(
            sniffer.favorite_hosts,
            HashSet::from([Host {
                domain: "1.1".to_string(),
                asn: Default::default(),
                country: Country::US,
            }])
        );
        // remove 1
        sniffer.update(Message::AddOrRemoveFavorite(
            Host {
                domain: "1.1".to_string(),
                asn: Default::default(),
                country: Country::US,
            },
            false,
        ));
        assert_eq!(sniffer.favorite_hosts, HashSet::new());
    }

    #[test]
    #[parallel] // needed to not collide with other tests generating configs files
    fn test_show_and_hide_modal_and_settings() {
        let mut sniffer = Sniffer::new(Configs::default());

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
        sniffer.update(Message::OpenSettings(SettingsPage::General));
        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.settings_page, Some(SettingsPage::General));
        // try opening modal with settings opened
        sniffer.update(Message::ShowModal(MyModal::Quit));
        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.settings_page, Some(SettingsPage::General));
        assert_eq!(sniffer.last_opened_setting, SettingsPage::Notifications);
        // close settings
        sniffer.update(Message::CloseSettings);
        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.settings_page, None);
        assert_eq!(sniffer.last_opened_setting, SettingsPage::General);
        // reopen settings
        sniffer.update(Message::OpenLastSettings);
        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.settings_page, Some(SettingsPage::General));
        assert_eq!(sniffer.last_opened_setting, SettingsPage::General);
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
    #[parallel] // needed to not collide with other tests generating configs files
    fn test_correctly_update_language() {
        let mut sniffer = Sniffer::new(Configs::default());

        assert_eq!(sniffer.configs.settings.language, Language::EN);
        assert_eq!(sniffer.traffic_chart.language, Language::EN);
        sniffer.update(Message::LanguageSelection(Language::IT));
        assert_eq!(sniffer.configs.settings.language, Language::IT);
        assert_eq!(sniffer.traffic_chart.language, Language::IT);
        sniffer.update(Message::LanguageSelection(Language::IT));
        assert_eq!(sniffer.configs.settings.language, Language::IT);
        assert_eq!(sniffer.traffic_chart.language, Language::IT);
        sniffer.update(Message::LanguageSelection(Language::ZH));
        assert_eq!(sniffer.configs.settings.language, Language::ZH);
        assert_eq!(sniffer.traffic_chart.language, Language::ZH);
    }

    #[test]
    #[parallel] // needed to not collide with other tests generating configs files
    fn test_correctly_update_notification_settings() {
        fn expire_notifications_timeout(sniffer: &mut Sniffer) {
            // Wait for timeout to expire + small buffer
            std::thread::sleep(Duration::from_millis(
                TimingEvents::TIMEOUT_THRESHOLD_ADJUST + 5,
            ));
            // Threshold adjustments won't be updated if `info_traffic.tot_in_packets`
            // and `info_traffic.tot_out_packets` are both `0`.
            sniffer
                .info_traffic
                .tot_data_info
                .add_packet(0, TrafficDirection::Outgoing);

            // Simulate an update to apply the settings
            sniffer.update(Message::Periodic);
        }
        let mut sniffer = Sniffer::new(Configs::default());

        let bytes_notification_init = DataNotification {
            data_repr: DataRepr::Bytes,
            threshold: None,
            byte_multiple: ByteMultiple::KB,
            sound: Sound::Pop,
            previous_threshold: 800000,
        };

        let bytes_notification_toggled_on = DataNotification {
            data_repr: DataRepr::Bytes,
            threshold: Some(800_000),
            byte_multiple: ByteMultiple::GB,
            sound: Sound::Pop,
            previous_threshold: 800_000,
        };

        let bytes_notification_adjusted_threshold_sound_off = DataNotification {
            data_repr: DataRepr::Bytes,
            threshold: Some(3),
            byte_multiple: ByteMultiple::KB,
            sound: Sound::None,
            previous_threshold: 3,
        };

        let bytes_notification_sound_off_only = DataNotification {
            data_repr: DataRepr::Bytes,
            threshold: Some(800_000),
            byte_multiple: ByteMultiple::GB,
            sound: Sound::None,
            previous_threshold: 800_000,
        };

        let fav_notification_init = FavoriteNotification {
            notify_on_favorite: false,
            sound: Sound::Swhoosh,
        };

        let fav_notification_new = FavoriteNotification {
            notify_on_favorite: true,
            sound: Sound::Pop,
        };

        // initial default state
        assert_eq!(sniffer.configs.settings.notifications.volume, 60);
        assert_eq!(sniffer.configs.settings.notifications.volume, 60);
        assert_eq!(
            sniffer.configs.settings.notifications.data_notification,
            bytes_notification_init
        );
        assert_eq!(
            sniffer.configs.settings.notifications.favorite_notification,
            fav_notification_init
        );

        // change volume
        sniffer.update(Message::ChangeVolume(95));

        assert_eq!(sniffer.configs.settings.notifications.volume, 95);
        assert_eq!(
            sniffer.configs.settings.notifications.data_notification,
            bytes_notification_init,
        );
        assert_eq!(
            sniffer.configs.settings.notifications.favorite_notification,
            fav_notification_init,
        );

        assert_eq!(
            sniffer.configs.settings.notifications.data_notification,
            bytes_notification_init
        );
        assert_eq!(
            sniffer.configs.settings.notifications.favorite_notification,
            fav_notification_init
        );

        // Toggle on bytes notifications
        sniffer.update(Message::UpdateNotificationSettings(
            Notification::Data(bytes_notification_toggled_on),
            true,
        ));

        // Verify that toggling threshold is applied immediately
        assert_eq!(
            sniffer.configs.settings.notifications.data_notification,
            bytes_notification_toggled_on,
        );

        sniffer.update(Message::UpdateNotificationSettings(
            Notification::Data(bytes_notification_adjusted_threshold_sound_off),
            true,
        ));

        // Verify adjusted threshold is not applied before timeout expires,
        // and rest is applied immediately
        assert_eq!(
            sniffer.configs.settings.notifications.data_notification,
            bytes_notification_sound_off_only,
        );

        expire_notifications_timeout(&mut sniffer);

        assert_eq!(sniffer.configs.settings.notifications.volume, 95);
        assert_eq!(
            sniffer.configs.settings.notifications.data_notification,
            bytes_notification_adjusted_threshold_sound_off
        );
        assert_eq!(
            sniffer.configs.settings.notifications.favorite_notification,
            fav_notification_init,
        );

        // change favorite notifications
        sniffer.update(Message::UpdateNotificationSettings(
            Notification::Favorite(fav_notification_new),
            true,
        ));

        // Verify threshold is not applied before timeout expires,
        // and rest is applied immediately
        assert_eq!(
            sniffer.configs.settings.notifications.favorite_notification,
            fav_notification_new,
        );

        // And the rest is intact
        assert_eq!(sniffer.configs.settings.notifications.volume, 95);
        assert_eq!(
            sniffer.configs.settings.notifications.data_notification,
            bytes_notification_adjusted_threshold_sound_off
        );
        assert_eq!(
            sniffer.configs.settings.notifications.favorite_notification,
            fav_notification_new
        );
    }

    #[test]
    #[parallel] // needed to not collide with other tests generating configs files
    fn test_clear_all_notifications() {
        let mut sniffer = Sniffer::new(Configs::default());
        sniffer.logged_notifications.0 =
            VecDeque::from([LoggedNotification::DataThresholdExceeded(
                DataThresholdExceeded {
                    id: 1,
                    data_repr: DataRepr::Packets,
                    threshold: 0,
                    data_info: DataInfo::default(),
                    timestamp: "".to_string(),
                    services: Vec::new(),
                    hosts: Vec::new(),
                    is_expanded: false,
                },
            )]);

        assert_eq!(sniffer.modal, None);
        sniffer.update(Message::ShowModal(MyModal::ClearAll));
        assert_eq!(sniffer.modal, Some(MyModal::ClearAll));
        assert_eq!(sniffer.logged_notifications.0.len(), 1);
        sniffer.update(Message::ClearAllNotifications);
        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.logged_notifications.0.len(), 0);
    }

    #[test]
    #[parallel] // needed to not collide with other tests generating configs files
    fn test_correctly_switch_running_and_settings_pages() {
        let mut sniffer = Sniffer::new(Configs::default());

        // initial status
        assert_eq!(sniffer.settings_page, None);
        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.running_page, RunningPage::Init);
        // nothing changes
        sniffer.update(Message::SwitchPage(true));
        assert_eq!(sniffer.settings_page, None);
        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.running_page, RunningPage::Init);
        // switch settings
        sniffer.update(Message::OpenLastSettings);
        assert_eq!(sniffer.settings_page, Some(SettingsPage::Notifications));
        assert_eq!(sniffer.running_page, RunningPage::Init);
        sniffer.update(Message::SwitchPage(false));
        assert_eq!(sniffer.settings_page, Some(SettingsPage::General));
        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.running_page, RunningPage::Init);
        sniffer.update(Message::SwitchPage(true));
        assert_eq!(sniffer.settings_page, Some(SettingsPage::Notifications));
        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.running_page, RunningPage::Init);
        sniffer.update(Message::CloseSettings);
        assert_eq!(sniffer.settings_page, None);
        assert_eq!(sniffer.running_page, RunningPage::Init);
        // change state to running
        sniffer.running_page = RunningPage::Overview;
        assert_eq!(sniffer.settings_page, None);
        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.running_page, RunningPage::Overview);
        // switch with closed setting and no packets received => nothing changes
        sniffer.update(Message::SwitchPage(true));
        assert_eq!(sniffer.running_page, RunningPage::Overview);
        assert_eq!(sniffer.settings_page, None);
        // switch with closed setting and some packets received => change running page
        sniffer
            .info_traffic
            .tot_data_info
            .add_packet(0, TrafficDirection::Outgoing);
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

        // focus the window and try to switch => nothing changes
        sniffer.update(Message::WindowFocused);
        sniffer.update(Message::SwitchPage(true));
        assert_eq!(sniffer.running_page, RunningPage::Inspect);
        assert_eq!(sniffer.settings_page, Some(SettingsPage::Appearance));
    }

    #[test]
    #[serial] // needed to not collide with other tests generating configs files
    fn test_config_settings() {
        let path_string = Settings::test_path();
        let path = Path::new(&path_string);

        assert!(!path.exists());

        let mut sniffer = Sniffer::new(Configs::load());

        assert!(path.exists());

        // check that the current settings are the default ones
        let settings_start = sniffer.configs.settings.clone();
        assert_eq!(
            settings_start,
            Settings {
                color_gradient: GradientType::None,
                language: Language::EN,
                scale_factor: 1.0,
                mmdb_country: "".to_string(),
                mmdb_asn: "".to_string(),
                style_path: "".to_string(),
                notifications: Notifications {
                    volume: 60,
                    data_notification: Default::default(),
                    favorite_notification: Default::default()
                },
                style: StyleType::Custom(ExtraStyles::A11yDark)
            }
        );

        // change some configs by sending messages
        sniffer.update(Message::GradientsSelection(GradientType::Wild));
        sniffer.update(Message::LanguageSelection(Language::ZH));
        sniffer.update(Message::ChangeScaleFactor(0.0));
        sniffer.update(Message::CustomCountryDb("countrymmdb".to_string()));
        sniffer.update(Message::CustomAsnDb("asnmmdb".to_string()));
        sniffer.update(Message::LoadStyle(format!(
            "{}/resources/themes/catppuccin.toml",
            env!("CARGO_MANIFEST_DIR")
        )));
        sniffer.update(Message::Style(StyleType::Custom(ExtraStyles::DraculaDark)));
        sniffer.update(Message::ChangeVolume(100));

        // quit the app by sending a CloseRequested message
        sniffer.update(Message::Quit);

        assert!(path.exists());

        // check that updated configs are inherited by a new sniffer instance
        let settings_end = Sniffer::new(Configs::load()).configs.settings.clone();
        assert_eq!(
            settings_end,
            Settings {
                color_gradient: GradientType::Wild,
                language: Language::ZH,
                scale_factor: 1.0,
                mmdb_country: "countrymmdb".to_string(),
                mmdb_asn: "asnmmdb".to_string(),
                style_path: format!(
                    "{}/resources/themes/catppuccin.toml",
                    env!("CARGO_MANIFEST_DIR")
                ),
                notifications: Notifications {
                    volume: 100,
                    data_notification: Default::default(),
                    favorite_notification: Default::default()
                },
                style: StyleType::Custom(ExtraStyles::DraculaDark)
            }
        );
    }

    #[test]
    #[serial] // needed to not collide with other tests generating configs files
    fn test_config_window() {
        let path_string = ConfigWindow::test_path();
        let path = Path::new(&path_string);

        assert!(!path.exists());

        let mut sniffer = Sniffer::new(Configs::load());

        assert!(path.exists());

        // check that the current window properties are the default ones
        let window_start = sniffer.configs.window;
        assert_eq!(
            window_start,
            ConfigWindow {
                position: PositionTuple(0.0, 0.0),
                size: SizeTuple(1190.0, 670.0),
                thumbnail_position: PositionTuple(0.0, 0.0),
            }
        );

        // change window properties by sending messages
        sniffer.update(Message::WindowMoved(-10.0, 555.0));
        sniffer.update(Message::WindowResized(1000.0, 999.0));
        sniffer.thumbnail = true;
        sniffer.update(Message::WindowMoved(40.0, 40.0));

        // quit the app by sending a CloseRequested message
        sniffer.update(Message::Quit);

        assert!(path.exists());

        // check that updated configs are inherited by a new sniffer instance
        let window_end = Sniffer::new(Configs::load()).configs.window.clone();
        assert_eq!(
            window_end,
            ConfigWindow {
                position: PositionTuple(-10.0, 555.0),
                size: SizeTuple(1000.0, 999.0),
                thumbnail_position: PositionTuple(40.0, 40.0),
            }
        );
    }

    #[test]
    #[parallel] // needed to not collide with other tests generating configs files
    fn test_window_resized() {
        let mut sniffer = Sniffer::new(Configs::default());
        assert!(!sniffer.thumbnail);
        let factor = sniffer.configs.settings.scale_factor;
        assert_eq!(factor, 1.0);
        assert_eq!(sniffer.configs.window.size, SizeTuple(1190.0, 670.0));
        assert_eq!(
            ConfigWindow::thumbnail_size(factor),
            SizeTuple(360.0, 222.0)
        );

        sniffer.update(Message::WindowResized(850.0, 600.0));
        assert_eq!(sniffer.configs.window.size, SizeTuple(850.0, 600.0));

        sniffer.update(Message::ChangeScaleFactor(0.369));
        let factor = sniffer.configs.settings.scale_factor;
        assert_eq!(factor, 1.5);
        assert_eq!(
            ConfigWindow::thumbnail_size(factor),
            SizeTuple(540.0, 333.0)
        );
        sniffer.update(Message::WindowResized(1000.0, 800.0));
        assert_eq!(sniffer.configs.window.size, SizeTuple(1500.0, 1200.0));

        sniffer.update(Message::ChangeScaleFactor(-0.631));
        let factor = sniffer.configs.settings.scale_factor;
        assert_eq!(factor, 0.5);
        assert_eq!(
            ConfigWindow::thumbnail_size(factor),
            SizeTuple(180.0, 111.0)
        );
        sniffer.update(Message::WindowResized(1000.0, 800.0));
        assert_eq!(sniffer.configs.window.size, SizeTuple(500.0, 400.0));
    }

    #[test]
    #[parallel] // needed to not collide with other tests generating configs files
    fn test_window_moved() {
        let mut sniffer = Sniffer::new(Configs::default());
        assert!(!sniffer.thumbnail);
        assert_eq!(sniffer.configs.settings.scale_factor, 1.0);
        assert_eq!(sniffer.configs.window.position, PositionTuple(0.0, 0.0));
        assert_eq!(
            sniffer.configs.window.thumbnail_position,
            PositionTuple(0.0, 0.0)
        );

        sniffer.update(Message::WindowMoved(850.0, 600.0));
        assert_eq!(sniffer.configs.window.position, PositionTuple(850.0, 600.0));
        assert_eq!(
            sniffer.configs.window.thumbnail_position,
            PositionTuple(0.0, 0.0)
        );
        sniffer.thumbnail = true;
        sniffer.update(Message::WindowMoved(400.0, 600.0));
        assert_eq!(sniffer.configs.window.position, PositionTuple(850.0, 600.0));
        assert_eq!(
            sniffer.configs.window.thumbnail_position,
            PositionTuple(400.0, 600.0)
        );

        sniffer.update(Message::ChangeScaleFactor(0.369));
        assert_eq!(sniffer.configs.settings.scale_factor, 1.5);
        sniffer.update(Message::WindowMoved(20.0, 40.0));
        assert_eq!(sniffer.configs.window.position, PositionTuple(850.0, 600.0));
        assert_eq!(
            sniffer.configs.window.thumbnail_position,
            PositionTuple(30.0, 60.0)
        );
        sniffer.thumbnail = false;
        sniffer.update(Message::WindowMoved(-20.0, 300.0));
        assert_eq!(sniffer.configs.window.position, PositionTuple(-30.0, 450.0));
        assert_eq!(
            sniffer.configs.window.thumbnail_position,
            PositionTuple(30.0, 60.0)
        );

        sniffer.update(Message::ChangeScaleFactor(-0.631));
        assert_eq!(sniffer.configs.settings.scale_factor, 0.5);
        sniffer.update(Message::WindowMoved(500.0, -100.0));
        assert_eq!(sniffer.configs.window.position, PositionTuple(250.0, -50.0));
        assert_eq!(
            sniffer.configs.window.thumbnail_position,
            PositionTuple(30.0, 60.0)
        );
        sniffer.thumbnail = true;
        sniffer.update(Message::WindowMoved(-2.0, -34.0));
        assert_eq!(sniffer.configs.window.position, PositionTuple(250.0, -50.0));
        assert_eq!(
            sniffer.configs.window.thumbnail_position,
            PositionTuple(-1.0, -17.0)
        );
    }

    #[test]
    #[parallel] // needed to not collide with other tests generating configs files
    fn test_toggle_thumbnail() {
        let mut sniffer = Sniffer::new(Configs::default());
        assert!(!sniffer.thumbnail);
        assert!(!sniffer.traffic_chart.thumbnail);

        sniffer.update(Message::ToggleThumbnail(false));
        assert!(sniffer.thumbnail);
        assert!(sniffer.traffic_chart.thumbnail);

        sniffer.unread_notifications = 8;
        sniffer.update(Message::ToggleThumbnail(false));
        assert!(!sniffer.thumbnail);
        assert!(!sniffer.traffic_chart.thumbnail);
        assert_eq!(sniffer.unread_notifications, 8);

        sniffer.update(Message::ChangeRunningPage(RunningPage::Notifications));
        assert_eq!(sniffer.unread_notifications, 0);

        sniffer.update(Message::ToggleThumbnail(false));
        sniffer.unread_notifications = 8;
        assert_eq!(sniffer.unread_notifications, 8);
        sniffer.update(Message::ToggleThumbnail(false));
        assert_eq!(sniffer.unread_notifications, 0);
    }

    #[test]
    #[parallel] // needed to not collide with other tests generating configs files
    fn test_scale_factor_shortcut() {
        let mut sniffer = Sniffer::new(Configs::default());
        assert_eq!(sniffer.configs.settings.scale_factor, 1.0);

        sniffer.update(Message::ScaleFactorShortcut(true));
        assert_eq!(sniffer.configs.settings.scale_factor, 1.1);
        sniffer.update(Message::ScaleFactorShortcut(false));
        assert_eq!(sniffer.configs.settings.scale_factor, 1.0);
        sniffer.update(Message::ScaleFactorShortcut(false));
        assert_eq!(sniffer.configs.settings.scale_factor, 0.9);

        for _ in 0..100 {
            sniffer.update(Message::ScaleFactorShortcut(true));
        }
        assert_eq!(
            format!("{:.2}", sniffer.configs.settings.scale_factor),
            "3.00".to_string()
        );

        for _ in 0..100 {
            sniffer.update(Message::ScaleFactorShortcut(false));
        }
        assert_eq!(
            format!("{:.2}", sniffer.configs.settings.scale_factor),
            "0.30".to_string()
        );
    }
}
