//! Module defining the application structure: messages, updates, subscriptions.

use crate::chart::types::preview_chart::PreviewChart;
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
use crate::gui::pages::waiting_page::waiting_page;
use crate::gui::pages::welcome_page::welcome_page;
use crate::gui::styles::types::custom_palette::CustomPalette;
use crate::gui::styles::types::gradient_type::GradientType;
use crate::gui::styles::types::palette::Palette;
use crate::gui::types::conf::Conf;
use crate::gui::types::message::Message;
use crate::gui::types::settings::Settings;
use crate::gui::types::timing_events::TimingEvents;
use crate::mmdb::asn::ASN_MMDB;
use crate::mmdb::country::COUNTRY_MMDB;
use crate::mmdb::types::mmdb_reader::{MmdbReader, MmdbReaders};
use crate::networking::manage_packets::get_local_port;
use crate::networking::parse_packets::BackendTrafficMessage;
use crate::networking::parse_packets::parse_packets;
use crate::networking::traffic_preview::{TrafficPreview, traffic_preview};
use crate::networking::types::capture_context::{
    CaptureContext, CaptureSource, CaptureSourcePicklist, MyPcapImport,
};
use crate::networking::types::combobox_data_states::ComboboxDataStates;
use crate::networking::types::data_info::DataInfo;
use crate::networking::types::data_representation::DataRepr;
use crate::networking::types::host::{Host, HostMessage};
use crate::networking::types::info_traffic::InfoTraffic;
use crate::networking::types::ip_blacklist::IpBlacklist;
use crate::networking::types::my_device::MyDevice;
use crate::networking::types::program::Program;
use crate::networking::types::program_lookup::{
    ProgramLookup, VALID_PROGRAM_TIMEOUT, lookup_program,
};
use crate::notifications::notify_and_log::notify_and_log;
use crate::notifications::types::logged_notification::LoggedNotifications;
use crate::notifications::types::notifications::{DataNotification, Notification};
use crate::notifications::types::sound::{Sound, play};
use crate::report::get_report_entries::get_searched_entries;
use crate::report::types::search_parameters::SearchParameters;
use crate::report::types::sort_type::SortType;
use crate::translations::types::language::Language;
use crate::utils::check_updates::set_newer_release_status;
use crate::utils::error_logger::{ErrorLogger, Location};
use crate::utils::types::file_info::FileInfo;
use crate::utils::types::icon::Icon;
use crate::utils::types::web_page::WebPage;
use crate::{StyleType, TrafficChart, location};
use async_channel::Receiver;
use iced::Event::{Keyboard, Window};
use iced::keyboard::key::Named;
use iced::keyboard::{Event, Key, Modifiers};
use iced::mouse::Event::ButtonPressed;
use iced::widget::{Column, center};
use iced::window::{Id, Level};
use iced::{Element, Point, Size, Subscription, Task, window};
use listeners::Process;
use rfd::FileHandle;
use std::collections::{HashMap, HashSet};
use std::net::IpAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub const FONT_FAMILY_NAME: &str = "Sarasa Mono SC for Sniffnet";
pub const ICON_FONT_FAMILY_NAME: &str = "Icons for Sniffnet";

const THUMBNAIL_SIZE: Size = Size {
    width: 360.0,
    height: 222.0,
};

/// Struct on which the GUI is based
///
/// It carries statuses, network traffic statistics, and more
pub struct Sniffer {
    /// Parameters that are persistent across runs
    pub conf: Conf,
    /// Are we during welcome / quit animation? None if not, true if welcome, false if quit
    pub welcome: Option<(bool, u8)>,
    /// Capture receiver clone (to close the channel after every run), with the current capture id (to ignore pending messages from previous captures)
    pub current_capture_rx: (usize, Option<Receiver<BackendTrafficMessage>>),
    /// Preview captures receiver clone (to close the channel when starting the analysis)
    pub preview_captures_rx: Option<Receiver<TrafficPreview>>,
    /// Capture data
    pub info_traffic: InfoTraffic,
    /// Map of the resolved addresses with their full rDNS value and the corresponding host
    pub addresses_resolved: HashMap<IpAddr, (String, Host)>,
    /// Collection of the favorite hosts
    pub favorite_hosts: HashSet<Host>,
    /// Log of the displayed notifications, with the total number of notifications for this capture
    pub logged_notifications: LoggedNotifications,
    /// Reports if a newer release of the software is available on GitHub
    pub newer_release_available: Option<bool>,
    /// Network device to be analyzed, or PCAP file to be imported
    pub capture_source: CaptureSource,
    /// Signals if a pcap error occurred
    pub pcap_error: Option<String>,
    /// Messages status
    pub dots_pulse: (String, u8),
    /// Traffic chart displayed in the Overview page
    pub traffic_chart: TrafficChart,
    /// Traffic preview charts displayed in the initial page
    pub preview_charts: Vec<(MyDevice, PreviewChart)>,
    /// Currently displayed modal; None if no modal is displayed
    pub modal: Option<MyModal>,
    /// Currently displayed settings page; None if settings is closed
    pub settings_page: Option<SettingsPage>,
    /// Defines the current running page; None if initial page
    pub running_page: Option<RunningPage>,
    /// Number of unread notifications
    pub unread_notifications: usize,
    /// Search parameters of inspect page
    pub search: SearchParameters,
    /// Current page number of inspect search results
    pub page_number: usize,
    /// MMDB readers for country and ASN
    pub mmdb_readers: MmdbReaders,
    /// IP blacklist
    pub ip_blacklist: IpBlacklist,
    /// Time-related events
    pub timing_events: TimingEvents,
    /// Whether thumbnail mode is currently active
    pub thumbnail: bool,
    /// Window id
    pub id: Option<Id>,
    /// Combobox data for filter dropdowns
    pub combobox_data_states: ComboboxDataStates,
    /// Flag reporting whether the packet capture is frozen
    pub frozen: bool,
    /// Sender to freeze the packet capture
    pub freeze_tx: Option<tokio::sync::broadcast::Sender<()>>,
    /// State of the port to program lookups
    pub program_lookup: Option<ProgramLookup>,
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
        let data_repr = conf.data_repr;
        let capture_source = CaptureSource::from_conf(&conf);
        let preview_charts = pcap::Device::list()
            .unwrap_or_default()
            .into_iter()
            .map(|dev| (MyDevice::from_pcap_device(dev), PreviewChart::new(style)))
            .collect();
        Self {
            conf,
            welcome: Some((true, 0)),
            current_capture_rx: (0, None),
            preview_captures_rx: None,
            info_traffic: InfoTraffic::default(),
            addresses_resolved: HashMap::new(),
            favorite_hosts: HashSet::new(),
            logged_notifications: LoggedNotifications::default(),
            newer_release_available: None,
            capture_source,
            pcap_error: None,
            dots_pulse: (".".to_string(), 0),
            traffic_chart: TrafficChart::new(style, language, data_repr),
            preview_charts,
            modal: None,
            settings_page: None,
            running_page: None,
            unread_notifications: 0,
            search: SearchParameters::default(),
            page_number: 1,
            mmdb_readers: MmdbReaders {
                country: Arc::new(MmdbReader::from(&mmdb_country, COUNTRY_MMDB)),
                asn: Arc::new(MmdbReader::from(&mmdb_asn, ASN_MMDB)),
            },
            ip_blacklist: IpBlacklist::default(), // load it later
            timing_events: TimingEvents::default(),
            thumbnail: false,
            id: None,
            combobox_data_states: ComboboxDataStates::default(),
            frozen: false,
            freeze_tx: None,
            program_lookup: None,
        }
    }

    fn keyboard_subscription(&self) -> Subscription<Message> {
        if self.welcome.is_some() || self.ip_blacklist.is_loading() {
            return Subscription::none();
        }

        if self.thumbnail {
            iced::event::listen_with(|event, _, _| match event {
                Keyboard(Event::KeyPressed {
                    key,
                    modifiers: Modifiers::COMMAND,
                    ..
                }) => match key.as_ref() {
                    Key::Character("q") => Some(Message::QuitWrapper),
                    Key::Character("t") => Some(Message::CtrlTPressed),
                    Key::Named(Named::Space) => Some(Message::CtrlSpacePressed),
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
                        Key::Named(Named::Space) => Some(Message::CtrlSpacePressed),
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
                    Modifiers::NONE => match key {
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

    fn time_subscription(&self) -> Subscription<Message> {
        if let Some((w, _)) = self.welcome {
            let sub = iced::time::every(Duration::from_millis(100));
            if w {
                sub.map(|_| Message::Welcome)
            } else {
                sub.map(|_| Message::Quit)
            }
        } else {
            iced::time::every(Duration::from_millis(1000)).map(|_| Message::Periodic)
        }
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
            Message::StartApp(id) => return self.start_app(id),
            Message::TickRun(cap_id, msg, host_msgs, no_more_packets) => {
                self.tick_run(cap_id, msg, host_msgs, no_more_packets);
            }
            Message::DeviceSelection(name) => self.device_selection(&name),
            Message::SetCaptureSource(cs_pick) => self.set_capture_source(cs_pick),
            Message::ToggleFilters => self.toggle_filters(),
            Message::BpfFilter(value) => self.bpf_filter(value),
            Message::DataReprSelection(unit) => self.data_repr_selection(unit),
            Message::ReportSortSelection(sort) => self.report_sort_selection(sort),
            Message::OpenWebPage(web_page) => Self::open_web_page(&web_page),
            Message::Start => return self.start(),
            Message::Reset => return self.reset(),
            Message::Style(style) => self.style(style),
            Message::LoadStyle(path) => self.load_style(path),
            Message::AddOrRemoveFavorite(host, add) => self.add_or_remove_favorite(&host, add),
            Message::ShowModal(modal) => self.show_modal(modal),
            Message::HideModal => self.hide_modal(),
            Message::OpenSettings(settings_page) => self.open_settings(settings_page),
            Message::OpenLastSettings => self.open_last_settings(),
            Message::CloseSettings => self.close_settings(),
            Message::ChangeRunningPage(running_page) => self.change_running_page(running_page),
            Message::LanguageSelection(language) => self.language_selection(language),
            Message::UpdateNotificationSettings(notification, emit_sound) => {
                self.update_notifications_settings(notification, emit_sound);
            }
            Message::ChangeVolume(volume) => self.change_volume(volume),
            Message::ClearAllNotifications => self.clear_all_notifications(),
            Message::SwitchPage(next) => self.switch_page(next),
            Message::ReturnKeyPressed => return self.return_key_pressed(),
            Message::EscKeyPressed => self.esc_key_pressed(),
            Message::ResetButtonPressed => return self.reset_button_pressed(),
            Message::CtrlDPressed => self.ctrl_d_pressed(),
            Message::Search(parameters) => self.search(parameters),
            Message::UpdatePageNumber(increment) => self.update_page_number(increment),
            Message::ArrowPressed(increment) => self.arrow_pressed(increment),
            Message::WindowFocused => self.window_focused(),
            Message::GradientsSelection(gradient_type) => self.gradients_selection(gradient_type),
            Message::ChangeScaleFactor(slider_val) => self.change_scale_factor(slider_val),
            Message::WindowMoved(x, y) => self.window_moved(x, y),
            Message::WindowResized(width, height) => return self.window_resized(width, height),
            Message::CustomCountryDb(db) => self.custom_country_db(db),
            Message::CustomAsnDb(db) => self.custom_asn_db(db),
            Message::LoadIpBlacklist(path) => return self.load_ip_blacklist(path),
            Message::SetIpBlacklist(blacklist) => self.set_ip_blacklist(blacklist),
            Message::QuitWrapper => return self.quit_wrapper(),
            Message::Quit => return self.quit(),
            Message::Welcome => self.welcome(),
            Message::CopyIp(ip) => return self.copy_ip(ip),
            Message::OpenFile(old_file, file_info, consumer_message) => {
                return self.open_file(old_file, file_info, consumer_message);
            }
            Message::HostSortSelection(sort_type) => self.host_sort_selection(sort_type),
            Message::ServiceSortSelection(sort_type) => self.service_sort_selection(sort_type),
            Message::ProgramSortSelection(sort_type) => self.program_sort_selection(sort_type),
            Message::ToggleExportPcap => self.toggle_export_pcap(),
            Message::OutputPcapDir(path) => self.output_pcap_dir(path),
            Message::OutputPcapFile(name) => self.output_pcap_file(&name),
            Message::ToggleThumbnail(triggered_by_resize) => {
                return self.toggle_thumbnail(triggered_by_resize);
            }
            Message::Drag => return self.drag(),
            Message::CtrlTPressed => return self.ctrl_t_pressed(),
            Message::CtrlSpacePressed => self.ctrl_space_pressed(),
            Message::ScaleFactorShortcut(increase) => self.scale_factor_shortcut(increase),
            Message::SetNewerReleaseStatus(status) => self.set_newer_release_status(status),
            Message::SetPcapImport(path) => self.set_pcap_import(path),
            Message::PendingHosts(cap_id, host_msgs) => self.pending_hosts(cap_id, host_msgs),
            Message::OfflineGap(cap_id, gap) => self.offline_gap(cap_id, gap),
            Message::Periodic => self.periodic(),
            Message::ExpandNotification(id, expand) => self.expand_notification(id, expand),
            Message::ToggleRemoteNotifications => self.toggle_remote_notifications(),
            Message::RemoteNotificationsUrl(url) => self.remote_notifications_url(&url),
            Message::Freeze => self.freeze(),
            Message::TrafficPreview(msg) => self.traffic_preview(msg),
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message, StyleType> {
        let Settings {
            language,
            color_gradient,
            ..
        } = self.conf.settings;

        if let Some((_, x)) = self.welcome {
            return welcome_page(x, self.thumbnail).into();
        }

        let header = header(self);

        let body = if self.thumbnail {
            thumbnail_page(self)
        } else {
            match self.running_page {
                None => initial_page(self),
                Some(running_page) => {
                    if let Some(waiting_page) = waiting_page(self) {
                        waiting_page
                    } else {
                        match running_page {
                            RunningPage::Overview => overview_page(self),
                            RunningPage::Inspect => inspect_page(self),
                            RunningPage::Notifications => notifications_page(self),
                        }
                    }
                }
            }
        };

        let footer = footer(
            self.thumbnail,
            language,
            color_gradient,
            self.newer_release_available,
            &self.dots_pulse,
        );

        let content: Element<Message, StyleType> =
            Column::new().push(header).push(body).push(footer).into();

        let ret_val: Element<'_, Message, StyleType> = match self.modal.clone() {
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
                    MyModal::Reset => get_exit_overlay(Message::Reset, color_gradient, language),
                    MyModal::Quit => get_exit_overlay(Message::Quit, color_gradient, language),
                    MyModal::ClearAll => get_clear_all_overlay(color_gradient, language),
                    MyModal::ConnectionDetails(key) => connection_details_page(self, key),
                }
                .into();

                modal(content, overlay, Message::HideModal)
            }
        };

        if self.ip_blacklist.is_loading() {
            let overlay = Into::<Element<Message, StyleType>>::into(center(
                Icon::get_hourglass(self.dots_pulse.0.len()).size(60),
            ));

            modal(ret_val, overlay, None)
        } else {
            ret_val
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            self.keyboard_subscription(),
            self.mouse_subscription(),
            self.time_subscription(),
            Sniffer::window_subscription(),
        ])
    }

    pub fn theme(&self) -> StyleType {
        self.conf.settings.style
    }

    pub fn scale_factor(&self) -> f32 {
        self.conf.settings.scale_factor
    }

    fn start_app(&mut self, id: Option<Id>) -> Task<Message> {
        self.id = id;
        let previews_task = self.start_traffic_previews();
        Task::batch([
            Sniffer::register_sigint_handler(),
            Task::perform(set_newer_release_status(), Message::SetNewerReleaseStatus),
            previews_task,
            self.load_ip_blacklist(self.conf.settings.ip_blacklist.clone()),
        ])
    }

    fn tick_run(
        &mut self,
        cap_id: usize,
        msg: InfoTraffic,
        host_msgs: Vec<HostMessage>,
        no_more_packets: bool,
    ) {
        if cap_id == self.current_capture_rx.0 {
            for host_msg in host_msgs {
                self.handle_new_host(host_msg);
            }
            if let Some(program_lookup) = &mut self.program_lookup {
                for program_res in program_lookup.pending_results() {
                    self.handle_program_lookup_result(program_res);
                }
            }
            self.refresh_data(msg, no_more_packets);
        }
    }

    fn set_capture_source(&mut self, cs_pick: CaptureSourcePicklist) {
        self.conf.capture_source_picklist = cs_pick;
        if cs_pick == CaptureSourcePicklist::File {
            self.set_pcap_import(self.conf.import_pcap_path.clone());
        } else {
            self.device_selection(&self.conf.device.device_name.clone());
        }
    }

    fn toggle_filters(&mut self) {
        self.conf.filters.toggle();
    }

    fn bpf_filter(&mut self, value: String) {
        self.conf.filters.set_bpf(value);
    }

    fn data_repr_selection(&mut self, unit: DataRepr) {
        self.conf.data_repr = unit;
        self.traffic_chart.change_kind(unit);
    }

    fn report_sort_selection(&mut self, sort: SortType) {
        self.page_number = 1;
        self.conf.report_sort_type = sort;
    }

    fn style(&mut self, style: StyleType) {
        self.conf.settings.style = style;
        self.change_charts_style();
    }

    fn load_style(&mut self, path: String) {
        self.conf.settings.style_path.clone_from(&path);
        if let Some(palette) = Palette::from_file(path) {
            let style = StyleType::Custom(CustomPalette::from_palette(palette));
            self.conf.settings.style = style;
            self.change_charts_style();
        }
    }

    fn show_modal(&mut self, modal: MyModal) {
        if self.settings_page.is_none() && self.modal.is_none() {
            self.modal = Some(modal);
        }
    }

    fn hide_modal(&mut self) {
        self.modal = None;
    }

    fn open_settings(&mut self, settings_page: SettingsPage) {
        if self.modal.is_none() {
            self.settings_page = Some(settings_page);
            self.conf.last_opened_setting = settings_page;
        }
    }

    fn open_last_settings(&mut self) {
        if self.modal.is_none() && self.settings_page.is_none() {
            self.settings_page = Some(self.conf.last_opened_setting);
        }
    }

    fn change_running_page(&mut self, running_page: RunningPage) {
        self.running_page = Some(running_page);
        self.conf.last_opened_page = running_page;
        if running_page.eq(&RunningPage::Notifications) {
            self.unread_notifications = 0;
        }
    }

    fn language_selection(&mut self, language: Language) {
        self.conf.settings.language = language;
        self.traffic_chart.change_language(language);
    }

    fn change_volume(&mut self, volume: u8) {
        play(Sound::Pop, volume);
        self.conf.settings.notifications.volume = volume;
    }

    fn clear_all_notifications(&mut self) {
        self.logged_notifications.clear_notifications();
        self.modal = None;
    }

    fn search(&mut self, parameters: SearchParameters) {
        // update comboboxes
        let combobox_data = &mut self.combobox_data_states.data;
        combobox_data.countries.1 = self.search.country != parameters.country;
        combobox_data.asns.1 = self.search.as_name != parameters.as_name;
        combobox_data.domains.1 = self.search.domain != parameters.domain;
        combobox_data.programs.1 = self.search.program != parameters.program;
        self.combobox_data_states.update_states(&parameters);

        self.page_number = 1;
        self.running_page = Some(RunningPage::Inspect);
        self.conf.last_opened_page = RunningPage::Inspect;
        self.search = parameters;
    }

    fn update_page_number(&mut self, increment: bool) {
        if increment {
            if self.page_number < get_searched_entries(self).1.div_ceil(30) {
                self.page_number = self.page_number.checked_add(1).unwrap_or(1);
            }
        } else if self.page_number > 1 {
            self.page_number = self.page_number.checked_sub(1).unwrap_or(1);
        }
    }

    fn arrow_pressed(&mut self, increment: bool) {
        if self
            .running_page
            .is_some_and(|p| p.eq(&RunningPage::Inspect))
            && self.settings_page.is_none()
            && self.modal.is_none()
        {
            self.update_page_number(increment);
        }
    }

    fn window_focused(&mut self) {
        self.timing_events.focus_now();
    }

    fn gradients_selection(&mut self, gradient_type: GradientType) {
        self.conf.settings.color_gradient = gradient_type;
    }

    fn change_scale_factor(&mut self, scale_factor: f32) {
        let old = self.conf.settings.scale_factor;
        self.conf.settings.scale_factor = scale_factor;
        self.conf.window.scale_size(old, scale_factor);
    }

    fn window_moved(&mut self, x: f32, y: f32) {
        let scale_factor = self.conf.settings.scale_factor;
        if self.thumbnail {
            self.conf.window.set_thumbnail_position(x, y, scale_factor);
        } else {
            self.conf.window.set_position(x, y, scale_factor);
        }
    }

    fn window_resized(&mut self, width: f32, height: f32) -> Task<Message> {
        if !self.thumbnail {
            let scale_factor = self.conf.settings.scale_factor;
            self.conf.window.set_size(width, height, scale_factor);
        } else if !self.timing_events.was_just_thumbnail_enter() {
            return self.toggle_thumbnail(true);
        }
        Task::none()
    }

    fn custom_country_db(&mut self, db: String) {
        self.mmdb_readers.country = Arc::new(MmdbReader::from(&db, COUNTRY_MMDB));
        self.conf.settings.mmdb_country = db;
    }

    fn custom_asn_db(&mut self, db: String) {
        self.mmdb_readers.asn = Arc::new(MmdbReader::from(&db, ASN_MMDB));
        self.conf.settings.mmdb_asn = db;
    }

    fn load_ip_blacklist(&mut self, path: String) -> Task<Message> {
        self.conf.settings.ip_blacklist.clone_from(&path);
        if path.is_empty() {
            self.ip_blacklist = IpBlacklist::default();
            Task::none()
        } else {
            self.ip_blacklist.start_loading();
            Task::perform(IpBlacklist::from_file(path), Message::SetIpBlacklist)
        }
    }

    fn set_ip_blacklist(&mut self, blacklist: IpBlacklist) {
        self.ip_blacklist = blacklist;
    }

    fn open_file(
        &mut self,
        old_file: String,
        file_info: FileInfo,
        consumer_message: fn(String) -> Message,
    ) -> Task<Message> {
        Task::perform(
            Self::open_file_inner(old_file, file_info, self.conf.settings.language),
            consumer_message,
        )
    }

    fn host_sort_selection(&mut self, sort_type: SortType) {
        self.conf.host_sort_type = sort_type;
    }

    fn service_sort_selection(&mut self, sort_type: SortType) {
        self.conf.service_sort_type = sort_type;
    }

    fn program_sort_selection(&mut self, sort_type: SortType) {
        self.conf.program_sort_type = sort_type;
    }

    fn toggle_export_pcap(&mut self) {
        self.conf.export_pcap.toggle();
    }

    fn output_pcap_dir(&mut self, path: String) {
        self.conf.export_pcap.set_directory(path);
    }

    fn output_pcap_file(&mut self, name: &str) {
        self.conf.export_pcap.set_file_name(name);
    }

    fn toggle_thumbnail(&mut self, triggered_by_resize: bool) -> Task<Message> {
        let window_id = self.id.unwrap_or_else(Id::unique);

        self.thumbnail = !self.thumbnail;
        self.traffic_chart.thumbnail = self.thumbnail;

        if self.thumbnail {
            let size = THUMBNAIL_SIZE;
            let position = self.conf.window.thumbnail_position();
            self.timing_events.thumbnail_enter_now();
            Task::batch([
                window::maximize(window_id, false),
                window::toggle_decorations(window_id),
                window::resize(window_id, size),
                window::move_to(window_id, position),
                window::set_level(window_id, Level::AlwaysOnTop),
            ])
        } else {
            if self
                .running_page
                .is_some_and(|p| p.eq(&RunningPage::Notifications))
            {
                self.unread_notifications = 0;
            }
            let mut commands = vec![
                window::toggle_decorations(window_id),
                window::set_level(window_id, Level::Normal),
            ];
            if !triggered_by_resize {
                let size = self.conf.window.size();
                let position = self.conf.window.position();
                commands.push(window::move_to(window_id, position));
                commands.push(window::resize(window_id, size));
            }
            Task::batch(commands)
        }
    }

    fn drag(&mut self) -> Task<Message> {
        let was_just_thumbnail_click = self.timing_events.was_just_thumbnail_click();
        self.timing_events.thumbnail_click_now();
        if was_just_thumbnail_click {
            return window::drag(self.id.unwrap_or_else(Id::unique));
        }
        Task::none()
    }

    fn ctrl_t_pressed(&mut self) -> Task<Message> {
        if self.running_page.is_some()
            && self.settings_page.is_none()
            && self.modal.is_none()
            && !self.timing_events.was_just_thumbnail_enter()
        {
            return self.toggle_thumbnail(false);
        }
        Task::none()
    }

    fn ctrl_space_pressed(&mut self) {
        if self.running_page.is_some() && self.settings_page.is_none() && self.modal.is_none() {
            self.freeze();
        }
    }

    fn scale_factor_shortcut(&mut self, increase: bool) {
        let scale_factor = self.conf.settings.scale_factor;
        if !(scale_factor > 2.99 && increase || scale_factor < 0.31 && !increase) {
            let delta = if increase { 0.1 } else { -0.1 };
            let new = scale_factor + delta;
            self.change_scale_factor(new);
        }
    }

    fn set_newer_release_status(&mut self, status: Option<bool>) {
        self.newer_release_available = status;
    }

    fn set_pcap_import(&mut self, path: String) {
        if !path.is_empty() {
            self.conf.import_pcap_path.clone_from(&path);
            self.capture_source = CaptureSource::File(MyPcapImport::new(path));
        }
    }

    fn pending_hosts(&mut self, cap_id: usize, host_msgs: Vec<HostMessage>) {
        if cap_id == self.current_capture_rx.0 {
            for host_msg in host_msgs {
                self.handle_new_host(host_msg);
            }
        }
    }

    fn offline_gap(&mut self, cap_id: usize, gap: u32) {
        if cap_id == self.current_capture_rx.0 {
            self.traffic_chart.push_offline_gap_to_splines(gap);
        }
    }

    fn periodic(&mut self) {
        self.update_waiting_dots();
        self.capture_source.set_addresses();
        self.update_threshold();
    }

    fn expand_notification(&mut self, id: usize, expand: bool) {
        if let Some(n) = self
            .logged_notifications
            .notifications_mut()
            .iter_mut()
            .find(|n| n.id() == id)
        {
            n.expand(expand);
        }
    }

    fn toggle_remote_notifications(&mut self) {
        self.conf
            .settings
            .notifications
            .remote_notifications
            .toggle();
    }

    fn remote_notifications_url(&mut self, url: &str) {
        self.conf
            .settings
            .notifications
            .remote_notifications
            .set_url(url);
    }

    fn freeze(&mut self) {
        self.frozen = !self.frozen;
        if let Some(tx) = &self.freeze_tx {
            let _ = tx.send(());
        }
    }

    fn traffic_preview(&mut self, msg: TrafficPreview) {
        self.preview_charts.retain(|(my_dev, _)| {
            msg.data
                .iter()
                .any(|(d, _)| d.get_name().eq(my_dev.get_name()))
        });
        for (dev, packets) in msg.data {
            let Some((my_dev, chart)) = self
                .preview_charts
                .iter_mut()
                .find(|(my_dev, _)| my_dev.get_name().eq(dev.get_name()))
            else {
                let mut chart = PreviewChart::new(self.conf.settings.style);
                chart.update_charts_data(packets);
                self.preview_charts.push((dev, chart));
                continue;
            };
            *my_dev = dev;
            chart.update_charts_data(packets);
        }
        self.preview_charts
            .sort_by(|(_, c1), (_, c2)| c2.tot_packets.total_cmp(&c1.tot_packets));
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
        self.info_traffic
            .refresh(&mut msg, &mut self.program_lookup);
        if self.info_traffic.tot_data_info.tot_data(DataRepr::Packets) == 0 {
            return;
        }
        let emitted_notifications = notify_and_log(
            &mut self.logged_notifications,
            &self.conf.settings.notifications,
            &msg,
            &self.favorite_hosts,
            &self.capture_source,
            &self.addresses_resolved,
        );
        if self.thumbnail
            || self
                .running_page
                .is_some_and(|p| p.ne(&RunningPage::Notifications))
        {
            self.unread_notifications += emitted_notifications;
        }
        self.traffic_chart.update_charts_data(&msg, no_more_packets);

        // update combobox dropdowns
        self.combobox_data_states.update_states(&self.search);
    }

    fn open_web_page(web_page: &WebPage) {
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
        if self.is_capture_source_consistent() {
            // close captures preview channel to kill previous preview captures
            if let Some(rx) = &self.preview_captures_rx {
                rx.close();
            }
            self.preview_captures_rx = None;
            self.preview_charts
                .iter_mut()
                .for_each(|(_, chart)| *chart = PreviewChart::new(self.conf.settings.style));

            if matches!(&self.capture_source, CaptureSource::Device(_)) {
                let current_device_name = &self.capture_source.get_name();
                self.device_selection(current_device_name);
            }
            let pcap_path = self.conf.export_pcap.full_path();
            let capture_context =
                CaptureContext::new(&self.capture_source, pcap_path.as_ref(), &self.conf.filters);
            self.pcap_error = capture_context.error().map(ToString::to_string);
            self.running_page = Some(self.conf.last_opened_page);

            if capture_context.error().is_none() {
                // no pcap error
                let curr_cap_id = self.current_capture_rx.0;
                let mmdb_readers = self.mmdb_readers.clone();
                let ip_blacklist = self.ip_blacklist.clone();
                self.capture_source
                    .set_link_type(capture_context.my_link_type());
                self.capture_source.set_addresses();
                let capture_source = self.capture_source.clone();
                self.traffic_chart
                    .change_capture_source(matches!(capture_source, CaptureSource::Device(_)));
                let (tx, rx) = async_channel::unbounded();
                let (freeze_tx, freeze_rx) = tokio::sync::broadcast::channel(1_048_575);
                let freeze_rx2 = freeze_tx.subscribe();
                let filters = self.conf.filters.clone();
                let _ = thread::Builder::new()
                    .name("thread_parse_packets".to_string())
                    .spawn(move || {
                        parse_packets(
                            curr_cap_id,
                            capture_source,
                            mmdb_readers,
                            &ip_blacklist,
                            capture_context,
                            filters,
                            &tx,
                            (freeze_rx, freeze_rx2),
                        );
                    })
                    .log_err(location!());
                self.current_capture_rx.1 = Some(rx.clone());
                self.freeze_tx = Some(freeze_tx);

                if matches!(self.capture_source, CaptureSource::Device(_)) {
                    let (port_tx, port_rx) = std::sync::mpsc::channel();
                    let (program_tx, program_rx) = std::sync::mpsc::channel();
                    let _ = thread::Builder::new()
                        .name("thread_lookup_program".to_string())
                        .spawn(move || {
                            lookup_program(&port_rx, &program_tx);
                        })
                        .log_err(location!());
                    self.program_lookup = Some(ProgramLookup::new(port_tx, program_rx));
                }

                return Task::run(rx, |backend_msg| match backend_msg {
                    BackendTrafficMessage::TickRun(cap_id, msg, host_msg, no_more_packets) => {
                        Message::TickRun(cap_id, msg, host_msg, no_more_packets)
                    }
                    BackendTrafficMessage::PendingHosts(cap_id, host_msg) => {
                        Message::PendingHosts(cap_id, host_msg)
                    }
                    BackendTrafficMessage::OfflineGap(cap_id, gap) => {
                        Message::OfflineGap(cap_id, gap)
                    }
                });
            }
        }
        Task::none()
    }

    fn reset(&mut self) -> Task<Message> {
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
        self.logged_notifications = LoggedNotifications::default();
        self.pcap_error = None;
        self.traffic_chart = TrafficChart::new(style, language, self.conf.data_repr);
        self.modal = None;
        self.settings_page = None;
        self.running_page = None;
        self.unread_notifications = 0;
        self.search = SearchParameters::default();
        self.page_number = 1;
        self.thumbnail = false;
        self.combobox_data_states = ComboboxDataStates::default();
        self.frozen = false;
        self.freeze_tx = None;
        self.program_lookup = None;
        self.start_traffic_previews()
    }

    fn start_traffic_previews(&mut self) -> Task<Message> {
        let (tx, rx) = async_channel::unbounded();
        let _ = thread::Builder::new()
            .name("thread_traffic_preview".to_string())
            .spawn(move || {
                traffic_preview(&tx);
            })
            .log_err(location!());
        self.preview_captures_rx = Some(rx.clone());
        Task::run(rx, |traffic_preview| {
            Message::TrafficPreview(traffic_preview)
        })
    }

    fn device_selection(&mut self, name: &str) {
        for (my_dev, _) in &self.preview_charts {
            if my_dev.get_name().eq(&name) {
                self.conf.device.device_name = name.to_string();
                self.capture_source = CaptureSource::Device(my_dev.clone());
                break;
            }
        }
    }

    fn update_waiting_dots(&mut self) {
        if !self.frozen {
            if self.dots_pulse.0.len() > 2 {
                self.dots_pulse.0 = String::new();
            }
            self.dots_pulse.0 = ".".repeat(self.dots_pulse.0.len() + 1);
        }
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
        self.settings_page = None;
    }

    /// Don't update adjustments to threshold immediately:
    /// that is, sound and toggling threshold on/off should be applied immediately
    /// Threshold adjustments are saved in `self.timing_events.threshold_adjust` and then applied
    /// after timeout
    fn update_notifications_settings(&mut self, notification: Notification, emit_sound: bool) {
        let data_notification = self.conf.settings.notifications.data_notification;
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
                        sound,
                        data_repr,
                        threshold,
                        byte_multiple,
                        previous_threshold,
                    };
                    self.timing_events.threshold_adjust_now(temp_threshold);
                }
                if threshold.is_some() != data_notification.threshold.is_some() {
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
            Notification::IpBlacklist(ip_blacklist_notification) => {
                self.conf.settings.notifications.ip_blacklist_notification =
                    ip_blacklist_notification;
                ip_blacklist_notification.sound
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
            self.conf.settings.notifications.data_notification
        }
    }

    fn switch_page(&mut self, next: bool) {
        // To prevent SwitchPage be triggered when using `Alt` + `Tab` to switch back,
        // first check if user switch back just now, and ignore the request for a short time.
        if !self.timing_events.was_just_focus() {
            match (self.running_page, self.settings_page, self.modal.is_none()) {
                (_, Some(current_setting), true) => {
                    // Settings opened
                    let new_setting = if next {
                        current_setting.next()
                    } else {
                        current_setting.previous()
                    };
                    self.settings_page = Some(new_setting);
                    self.conf.last_opened_setting = new_setting;
                }
                (Some(current_page), None, true) => {
                    // Running with no overlays
                    if self.info_traffic.tot_data_info.tot_data(DataRepr::Packets) > 0 {
                        // Running with no overlays and some packets
                        let new_page = if next {
                            current_page.next()
                        } else {
                            current_page.previous()
                        };
                        self.running_page = Some(new_page);
                        self.conf.last_opened_page = new_page;
                        if self
                            .running_page
                            .is_some_and(|p| p.eq(&RunningPage::Notifications))
                        {
                            self.unread_notifications = 0;
                        }
                    }
                }
                (_, _, _) => {}
            }
        }
    }

    fn return_key_pressed(&mut self) -> Task<Message> {
        if self.running_page.is_none() && self.settings_page.is_none() && self.modal.is_none() {
            return self.start();
        } else if self.modal.eq(&Some(MyModal::Reset)) {
            return self.reset();
        } else if self.modal.eq(&Some(MyModal::Quit)) {
            return self.quit();
        } else if self.modal.eq(&Some(MyModal::ClearAll)) {
            self.clear_all_notifications();
        }
        Task::none()
    }

    fn esc_key_pressed(&mut self) {
        if self.modal.is_some() {
            self.hide_modal();
        } else if self.settings_page.is_some() {
            self.close_settings();
        }
    }

    // also called when the backspace shortcut is pressed
    fn reset_button_pressed(&mut self) -> Task<Message> {
        if self.running_page.is_some() {
            let tot_packets = self.info_traffic.tot_data_info.tot_data(DataRepr::Packets);
            if tot_packets == 0 && self.settings_page.is_none() {
                return self.reset();
            }
            self.show_modal(MyModal::Reset);
        }
        Task::none()
    }

    fn quit_wrapper(&mut self) -> Task<Message> {
        let tot_packets = self.info_traffic.tot_data_info.tot_data(DataRepr::Packets);
        if self.running_page.is_none() || tot_packets == 0 {
            self.quit()
        } else if self.thumbnail {
            self.toggle_thumbnail(false)
                .chain(Task::done(Message::ShowModal(MyModal::Quit)))
        } else {
            self.hide_modal();
            self.close_settings();
            self.show_modal(MyModal::Quit);
            Task::none()
        }
    }

    fn quit(&mut self) -> Task<Message> {
        if self.welcome.is_none() {
            self.welcome = Some((false, 13));
        } else if let Some((false, x)) = self.welcome {
            if x <= 2 {
                let _ = self.conf.clone().store();
                return window::close(self.id.unwrap_or_else(Id::unique));
            }
            self.welcome = Some((false, x.saturating_sub(1)));
        }
        Task::none()
    }

    fn welcome(&mut self) {
        if let Some((true, x)) = self.welcome {
            if x >= 19 {
                self.welcome = None;
            } else {
                self.welcome = Some((true, x + 1));
            }
        }
    }

    fn copy_ip(&mut self, ip: IpAddr) -> Task<Message> {
        self.timing_events.copy_ip_now(ip);
        iced::clipboard::write(ip.to_string())
    }

    fn ctrl_d_pressed(&mut self) {
        if self
            .running_page
            .is_some_and(|p| p.eq(&RunningPage::Notifications))
            && !self.logged_notifications.is_empty()
        {
            self.show_modal(MyModal::ClearAll);
        }
    }

    async fn open_file_inner(old_file: String, file_info: FileInfo, language: Language) -> String {
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
            if extensions.is_empty() {
                dialog.pick_file().await
            } else {
                dialog
                    .add_filter(format!("{extensions:?}"), &extensions)
                    .pick_file()
                    .await
            }
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

        // update hosts combobox states including the new host
        self.combobox_data_states.data.update_host(&host);
    }

    fn handle_program_lookup_result(
        &mut self,
        lookup_res: (u16, listeners::Protocol, Option<Process>),
    ) {
        if let Some(program_lookup) = &mut self.program_lookup {
            // update programs combobox state including the new program
            self.combobox_data_states
                .data
                .update_program(lookup_res.2.as_ref());

            // associate unassigned recent connections on port with the program
            let mut reassigned_data = DataInfo::default();
            if lookup_res.2.is_some() {
                self.info_traffic
                    .map
                    .iter_mut()
                    .filter(|(k, v)| {
                        v.program.is_unknown()
                            && v.final_instant.elapsed().as_millis() < VALID_PROGRAM_TIMEOUT
                            && get_local_port(k, v.traffic_direction)
                                == Some((lookup_res.0, lookup_res.1))
                    })
                    .for_each(|(_, v)| {
                        v.program = Program::from_proc(lookup_res.2.as_ref());
                        reassigned_data.refresh(v.data_info());
                    });
            }

            // update program lookup state with the new lookup result
            program_lookup.update(lookup_res, reassigned_data);
        }
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

    fn change_charts_style(&mut self) {
        let style = self.conf.settings.style;
        self.traffic_chart.change_style(style);
        for (_, chart) in &mut self.preview_charts {
            chart.change_style(style);
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_must_use)]

    use iced::{Point, Size};
    use serial_test::{parallel, serial};
    use std::collections::{HashSet, VecDeque};
    use std::fs::remove_file;
    use std::path::Path;
    use std::time::Duration;

    use crate::countries::types::country::Country;
    use crate::gui::components::types::my_modal::MyModal;
    use crate::gui::pages::types::settings_page::SettingsPage;
    use crate::gui::styles::types::gradient_type::GradientType;
    use crate::gui::types::conf::Conf;
    use crate::gui::types::config_window::ConfigWindow;
    use crate::gui::types::export_pcap::ExportPcap;
    use crate::gui::types::filters::Filters;
    use crate::gui::types::message::Message;
    use crate::gui::types::settings::Settings;
    use crate::gui::types::timing_events::TimingEvents;
    use crate::networking::types::capture_context::CaptureSourcePicklist;
    use crate::networking::types::config_device::ConfigDevice;
    use crate::networking::types::data_info::DataInfo;
    use crate::networking::types::data_representation::DataRepr;
    use crate::networking::types::host::Host;
    use crate::networking::types::traffic_direction::TrafficDirection;
    use crate::notifications::types::logged_notification::{
        DataThresholdExceeded, LoggedNotification,
    };
    use crate::notifications::types::notifications::{
        DataNotification, Notification, Notifications, SimpleNotification,
    };
    use crate::notifications::types::sound::Sound;
    use crate::report::types::sort_type::SortType;
    use crate::{ByteMultiple, Language, RunningPage, Sniffer, StyleType};

    // helpful to clean up files generated from tests
    impl Drop for Sniffer {
        fn drop(&mut self) {
            let conf_path_str = Conf::test_path();
            let conf_path = Path::new(&conf_path_str);
            if conf_path.exists() {
                remove_file(Conf::test_path()).unwrap();
            }
        }
    }

    #[test]
    #[parallel] // needed to not collide with other tests generating configs files
    fn test_correctly_update_chart_kind() {
        let mut sniffer = Sniffer::new(Conf::default());

        assert_eq!(sniffer.traffic_chart.data_repr, DataRepr::Bytes);
        assert_eq!(sniffer.conf.data_repr, DataRepr::Bytes);
        sniffer.update(Message::DataReprSelection(DataRepr::Packets));
        assert_eq!(sniffer.traffic_chart.data_repr, DataRepr::Packets);
        assert_eq!(sniffer.conf.data_repr, DataRepr::Packets);
        sniffer.update(Message::DataReprSelection(DataRepr::Packets));
        assert_eq!(sniffer.traffic_chart.data_repr, DataRepr::Packets);
        assert_eq!(sniffer.conf.data_repr, DataRepr::Packets);
        sniffer.update(Message::DataReprSelection(DataRepr::Bytes));
        assert_eq!(sniffer.traffic_chart.data_repr, DataRepr::Bytes);
        assert_eq!(sniffer.conf.data_repr, DataRepr::Bytes);
        sniffer.update(Message::DataReprSelection(DataRepr::Bits));
        assert_eq!(sniffer.traffic_chart.data_repr, DataRepr::Bits);
        assert_eq!(sniffer.conf.data_repr, DataRepr::Bits);
    }

    #[test]
    #[parallel] // needed to not collide with other tests generating configs files
    fn test_correctly_update_report_sort_kind() {
        let mut sniffer = Sniffer::new(Conf::default());

        let sort = SortType::Neutral;

        assert_eq!(sniffer.conf.report_sort_type, sort);
        sniffer.update(Message::ReportSortSelection(sort.next_sort()));
        assert_eq!(sniffer.conf.report_sort_type, SortType::Descending);
        sniffer.update(Message::ReportSortSelection(sort.next_sort().next_sort()));
        assert_eq!(sniffer.conf.report_sort_type, SortType::Ascending);
        sniffer.update(Message::ReportSortSelection(
            sort.next_sort().next_sort().next_sort(),
        ));
        assert_eq!(sniffer.conf.report_sort_type, SortType::Neutral);
    }

    #[test]
    #[parallel] // needed to not collide with other tests generating configs files
    fn test_correctly_update_host_sort_kind() {
        let mut sniffer = Sniffer::new(Conf::default());

        let mut sort = SortType::Neutral;

        assert_eq!(sniffer.conf.host_sort_type, sort);

        sort = sort.next_sort();
        sniffer.update(Message::HostSortSelection(sort));
        assert_eq!(sniffer.conf.host_sort_type, SortType::Descending);

        sort = sort.next_sort();
        sniffer.update(Message::HostSortSelection(sort));
        assert_eq!(sniffer.conf.host_sort_type, SortType::Ascending);

        sort = sort.next_sort();
        sniffer.update(Message::HostSortSelection(sort));
        assert_eq!(sniffer.conf.host_sort_type, SortType::Neutral);
    }

    #[test]
    #[parallel] // needed to not collide with other tests generating configs files
    fn test_correctly_update_service_sort_kind() {
        let mut sniffer = Sniffer::new(Conf::default());

        let mut sort = SortType::Neutral;

        assert_eq!(sniffer.conf.service_sort_type, sort);

        sort = sort.next_sort();
        sniffer.update(Message::ServiceSortSelection(sort));
        assert_eq!(sniffer.conf.service_sort_type, SortType::Descending);

        sort = sort.next_sort();
        sniffer.update(Message::ServiceSortSelection(sort));
        assert_eq!(sniffer.conf.service_sort_type, SortType::Ascending);

        sort = sort.next_sort();
        sniffer.update(Message::ServiceSortSelection(sort));
        assert_eq!(sniffer.conf.service_sort_type, SortType::Neutral);
    }

    #[test]
    #[parallel] // needed to not collide with other tests generating configs files
    fn test_correctly_update_style() {
        let mut sniffer = Sniffer::new(Conf::default());

        sniffer.update(Message::Style(StyleType::A11yLight));
        assert_eq!(sniffer.traffic_chart.style, StyleType::A11yLight);
        assert_eq!(sniffer.conf.settings.style, StyleType::A11yLight);
        sniffer.update(Message::Style(StyleType::DraculaLight));
        assert_eq!(sniffer.traffic_chart.style, StyleType::DraculaLight);
        assert_eq!(sniffer.conf.settings.style, StyleType::DraculaLight);
        sniffer.update(Message::Style(StyleType::A11yDark));
        assert_eq!(sniffer.traffic_chart.style, StyleType::A11yDark);
        assert_eq!(sniffer.conf.settings.style, StyleType::A11yDark);
        sniffer.update(Message::Style(StyleType::GruvboxDark));
        assert_eq!(sniffer.traffic_chart.style, StyleType::GruvboxDark);
        assert_eq!(sniffer.conf.settings.style, StyleType::GruvboxDark);
        sniffer.update(Message::Style(StyleType::GruvboxDark));
        assert_eq!(sniffer.traffic_chart.style, StyleType::GruvboxDark);
        assert_eq!(sniffer.conf.settings.style, StyleType::GruvboxDark);
    }

    #[test]
    #[parallel] // needed to not collide with other tests generating configs files
    fn test_dots_pulse_update() {
        // every kind of message will update the integer, but only Periodic will update the string
        let mut sniffer = Sniffer::new(Conf::default());

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

        // if frozen, string won't update
        sniffer.frozen = true;

        sniffer.update(Message::Periodic);
        assert_eq!(sniffer.dots_pulse, (".".to_string(), 1));

        sniffer.update(Message::BpfFilter(String::new()));
        assert_eq!(sniffer.dots_pulse, (".".to_string(), 2));

        sniffer.frozen = false;

        sniffer.update(Message::Periodic);
        assert_eq!(sniffer.dots_pulse, ("..".to_string(), 0));
    }

    #[test]
    #[parallel] // needed to not collide with other tests generating configs files
    fn test_modify_favorite_connections() {
        let mut sniffer = Sniffer::new(Conf::default());
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
        let mut sniffer = Sniffer::new(Conf::default());

        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.settings_page, None);
        assert_eq!(
            sniffer.conf.last_opened_setting,
            SettingsPage::Notifications
        );
        // open settings
        sniffer.update(Message::OpenLastSettings);
        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.settings_page, Some(SettingsPage::Notifications));
        assert_eq!(
            sniffer.conf.last_opened_setting,
            SettingsPage::Notifications
        );
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
        assert_eq!(sniffer.conf.last_opened_setting, SettingsPage::General);
        // close settings
        sniffer.update(Message::CloseSettings);
        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.settings_page, None);
        assert_eq!(sniffer.conf.last_opened_setting, SettingsPage::General);
        // reopen settings
        sniffer.update(Message::OpenLastSettings);
        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.settings_page, Some(SettingsPage::General));
        assert_eq!(sniffer.conf.last_opened_setting, SettingsPage::General);
        // switch settings page
        sniffer.update(Message::OpenSettings(SettingsPage::Appearance));
        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.settings_page, Some(SettingsPage::Appearance));
        // close settings
        sniffer.update(Message::CloseSettings);
        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.settings_page, None);
        assert_eq!(sniffer.conf.last_opened_setting, SettingsPage::Appearance);

        // open clear all modal
        sniffer.update(Message::ShowModal(MyModal::ClearAll));
        assert_eq!(sniffer.modal, Some(MyModal::ClearAll));
        assert_eq!(sniffer.settings_page, None);
        assert_eq!(sniffer.conf.last_opened_setting, SettingsPage::Appearance);
        // try opening settings with clear all modal opened
        sniffer.update(Message::OpenLastSettings);
        assert_eq!(sniffer.modal, Some(MyModal::ClearAll));
        assert_eq!(sniffer.settings_page, None);
        assert_eq!(sniffer.conf.last_opened_setting, SettingsPage::Appearance);
        // try opening quit modal with clear all modal opened
        sniffer.update(Message::ShowModal(MyModal::Quit));
        assert_eq!(sniffer.modal, Some(MyModal::ClearAll));
        assert_eq!(sniffer.settings_page, None);
        assert_eq!(sniffer.conf.last_opened_setting, SettingsPage::Appearance);
        // close clear all modal
        sniffer.update(Message::HideModal);
        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.settings_page, None);
        assert_eq!(sniffer.conf.last_opened_setting, SettingsPage::Appearance);

        // open quit modal
        sniffer.update(Message::ShowModal(MyModal::Quit));
        assert_eq!(sniffer.modal, Some(MyModal::Quit));
        assert_eq!(sniffer.settings_page, None);
        assert_eq!(sniffer.conf.last_opened_setting, SettingsPage::Appearance);
        // try opening settings with clear all modal opened
        sniffer.update(Message::OpenLastSettings);
        assert_eq!(sniffer.modal, Some(MyModal::Quit));
        assert_eq!(sniffer.settings_page, None);
        assert_eq!(sniffer.conf.last_opened_setting, SettingsPage::Appearance);
        // try opening clear all modal with quit modal opened
        sniffer.update(Message::ShowModal(MyModal::ClearAll));
        assert_eq!(sniffer.modal, Some(MyModal::Quit));
        assert_eq!(sniffer.settings_page, None);
        assert_eq!(sniffer.conf.last_opened_setting, SettingsPage::Appearance);
        // close quit modal
        sniffer.update(Message::HideModal);
        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.settings_page, None);
        assert_eq!(sniffer.conf.last_opened_setting, SettingsPage::Appearance);
    }

    #[test]
    #[parallel] // needed to not collide with other tests generating configs files
    fn test_correctly_update_language() {
        let mut sniffer = Sniffer::new(Conf::default());

        assert_eq!(sniffer.conf.settings.language, Language::EN);
        assert_eq!(sniffer.traffic_chart.language, Language::EN);
        sniffer.update(Message::LanguageSelection(Language::IT));
        assert_eq!(sniffer.conf.settings.language, Language::IT);
        assert_eq!(sniffer.traffic_chart.language, Language::IT);
        sniffer.update(Message::LanguageSelection(Language::IT));
        assert_eq!(sniffer.conf.settings.language, Language::IT);
        assert_eq!(sniffer.traffic_chart.language, Language::IT);
        sniffer.update(Message::LanguageSelection(Language::ZH));
        assert_eq!(sniffer.conf.settings.language, Language::ZH);
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
        let mut sniffer = Sniffer::new(Conf::default());

        let bytes_notification_init = DataNotification {
            data_repr: DataRepr::Bytes,
            threshold: None,
            byte_multiple: ByteMultiple::KB,
            sound: Sound::Gulp,
            previous_threshold: 800000,
        };

        let bytes_notification_toggled_on = DataNotification {
            data_repr: DataRepr::Bytes,
            threshold: Some(800_000),
            byte_multiple: ByteMultiple::GB,
            sound: Sound::Gulp,
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

        let fav_notification_init = SimpleNotification {
            is_active: false,
            sound: Sound::Pop,
        };

        let fav_notification_new = SimpleNotification {
            is_active: true,
            sound: Sound::Pop,
        };

        let blacklist_notification_init = SimpleNotification {
            is_active: false,
            sound: Sound::Swhoosh,
        };

        let blacklist_notification_new = SimpleNotification {
            is_active: true,
            sound: Sound::Gulp,
        };

        // initial default state
        assert_eq!(sniffer.conf.settings.notifications.volume, 50);
        assert_eq!(
            sniffer.conf.settings.notifications.data_notification,
            bytes_notification_init
        );
        assert_eq!(
            sniffer.conf.settings.notifications.favorite_notification,
            fav_notification_init
        );
        assert_eq!(
            sniffer
                .conf
                .settings
                .notifications
                .ip_blacklist_notification,
            blacklist_notification_init
        );

        // change volume
        sniffer.update(Message::ChangeVolume(95));

        assert_eq!(sniffer.conf.settings.notifications.volume, 95);
        assert_eq!(
            sniffer.conf.settings.notifications.data_notification,
            bytes_notification_init,
        );
        assert_eq!(
            sniffer.conf.settings.notifications.favorite_notification,
            fav_notification_init,
        );
        assert_eq!(
            sniffer
                .conf
                .settings
                .notifications
                .ip_blacklist_notification,
            blacklist_notification_init
        );

        // Toggle on bytes notifications
        sniffer.update(Message::UpdateNotificationSettings(
            Notification::Data(bytes_notification_toggled_on),
            true,
        ));

        // Verify that toggling threshold is applied immediately
        assert_eq!(
            sniffer.conf.settings.notifications.data_notification,
            bytes_notification_toggled_on,
        );

        sniffer.update(Message::UpdateNotificationSettings(
            Notification::Data(bytes_notification_adjusted_threshold_sound_off),
            true,
        ));

        // Verify adjusted threshold is not applied before timeout expires,
        // and rest is applied immediately
        assert_eq!(
            sniffer.conf.settings.notifications.data_notification,
            bytes_notification_sound_off_only,
        );

        expire_notifications_timeout(&mut sniffer);

        assert_eq!(sniffer.conf.settings.notifications.volume, 95);
        assert_eq!(
            sniffer.conf.settings.notifications.data_notification,
            bytes_notification_adjusted_threshold_sound_off
        );
        assert_eq!(
            sniffer.conf.settings.notifications.favorite_notification,
            fav_notification_init,
        );
        assert_eq!(
            sniffer
                .conf
                .settings
                .notifications
                .ip_blacklist_notification,
            blacklist_notification_init,
        );

        // change favorite notifications
        sniffer.update(Message::UpdateNotificationSettings(
            Notification::Favorite(fav_notification_new),
            true,
        ));

        // Verify threshold is not applied before timeout expires,
        // and rest is applied immediately
        assert_eq!(
            sniffer.conf.settings.notifications.favorite_notification,
            fav_notification_new,
        );

        // And the rest is intact
        assert_eq!(sniffer.conf.settings.notifications.volume, 95);
        assert_eq!(
            sniffer.conf.settings.notifications.data_notification,
            bytes_notification_adjusted_threshold_sound_off
        );
        assert_eq!(
            sniffer.conf.settings.notifications.favorite_notification,
            fav_notification_new
        );

        // change favorite notifications
        sniffer.update(Message::UpdateNotificationSettings(
            Notification::IpBlacklist(blacklist_notification_new),
            true,
        ));

        assert_eq!(
            sniffer
                .conf
                .settings
                .notifications
                .ip_blacklist_notification,
            blacklist_notification_new,
        );
    }

    #[test]
    #[parallel] // needed to not collide with other tests generating configs files
    fn test_clear_all_notifications() {
        let mut sniffer = Sniffer::new(Conf::default());
        sniffer
            .logged_notifications
            .set_notifications(VecDeque::from([LoggedNotification::DataThresholdExceeded(
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
            )]));

        assert_eq!(sniffer.modal, None);
        sniffer.update(Message::ShowModal(MyModal::ClearAll));
        assert_eq!(sniffer.modal, Some(MyModal::ClearAll));
        assert_eq!(sniffer.logged_notifications.len(), 1);
        sniffer.update(Message::ClearAllNotifications);
        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.logged_notifications.len(), 0);
    }

    #[test]
    #[parallel] // needed to not collide with other tests generating configs files
    fn test_correctly_switch_running_and_settings_pages() {
        let mut sniffer = Sniffer::new(Conf::default());

        // initial status
        assert_eq!(sniffer.settings_page, None);
        assert_eq!(sniffer.modal, None);
        assert!(sniffer.running_page.is_none());
        // nothing changes
        sniffer.update(Message::SwitchPage(true));
        assert_eq!(sniffer.settings_page, None);
        assert_eq!(
            sniffer.conf.last_opened_setting,
            SettingsPage::Notifications
        );
        assert_eq!(sniffer.modal, None);
        assert!(sniffer.running_page.is_none());
        // switch settings
        sniffer.update(Message::OpenLastSettings);
        assert_eq!(sniffer.settings_page, Some(SettingsPage::Notifications));
        assert_eq!(
            sniffer.conf.last_opened_setting,
            SettingsPage::Notifications
        );
        assert!(sniffer.running_page.is_none());
        sniffer.update(Message::SwitchPage(false));
        assert_eq!(sniffer.settings_page, Some(SettingsPage::General));
        assert_eq!(sniffer.conf.last_opened_setting, SettingsPage::General);
        assert_eq!(sniffer.modal, None);
        assert!(sniffer.running_page.is_none());
        sniffer.update(Message::SwitchPage(true));
        assert_eq!(sniffer.settings_page, Some(SettingsPage::Notifications));
        assert_eq!(
            sniffer.conf.last_opened_setting,
            SettingsPage::Notifications
        );
        assert_eq!(sniffer.modal, None);
        assert!(sniffer.running_page.is_none());
        sniffer.update(Message::CloseSettings);
        assert_eq!(sniffer.settings_page, None);
        assert!(sniffer.running_page.is_none());
        // change state to running
        sniffer.running_page = Some(RunningPage::Overview);
        assert_eq!(sniffer.settings_page, None);
        assert_eq!(sniffer.modal, None);
        assert_eq!(sniffer.running_page, Some(RunningPage::Overview));
        // switch with closed setting and no packets received => nothing changes
        sniffer.update(Message::SwitchPage(true));
        assert_eq!(sniffer.running_page, Some(RunningPage::Overview));
        assert_eq!(sniffer.settings_page, None);
        // switch with closed setting and some packets received => change running page
        sniffer
            .info_traffic
            .tot_data_info
            .add_packet(0, TrafficDirection::Outgoing);
        sniffer.update(Message::SwitchPage(true));
        assert_eq!(sniffer.running_page, Some(RunningPage::Inspect));
        assert_eq!(sniffer.settings_page, None);
        // switch with opened settings => change settings
        sniffer.update(Message::OpenLastSettings);
        assert_eq!(sniffer.running_page, Some(RunningPage::Inspect));
        assert_eq!(sniffer.settings_page, Some(SettingsPage::Notifications));
        assert_eq!(
            sniffer.conf.last_opened_setting,
            SettingsPage::Notifications
        );
        sniffer.update(Message::SwitchPage(true));
        assert_eq!(sniffer.running_page, Some(RunningPage::Inspect));
        assert_eq!(sniffer.settings_page, Some(SettingsPage::Appearance));
        assert_eq!(sniffer.conf.last_opened_setting, SettingsPage::Appearance);

        // focus the window and try to switch => nothing changes
        sniffer.update(Message::WindowFocused);
        sniffer.update(Message::SwitchPage(true));
        assert_eq!(sniffer.running_page, Some(RunningPage::Inspect));
        assert_eq!(sniffer.settings_page, Some(SettingsPage::Appearance));
    }

    #[test]
    #[serial] // needed to not collide with other tests generating configs files
    fn test_conf() {
        let path_string = Conf::test_path();
        let path = Path::new(&path_string);

        assert!(!path.exists());

        let mut sniffer = Sniffer::new(Conf::load());

        assert!(path.exists());

        // check that the current settings are the default ones
        let conf_start = sniffer.conf.clone();
        assert_eq!(conf_start, Conf::default(),);

        // change some conf by sending messages
        sniffer.update(Message::GradientsSelection(GradientType::Wild));
        sniffer.update(Message::LanguageSelection(Language::ZH));
        sniffer.update(Message::ChangeScaleFactor(0.5));
        sniffer.update(Message::CustomCountryDb("countrymmdb".to_string()));
        sniffer.update(Message::CustomAsnDb("asnmmdb".to_string()));
        sniffer.update(Message::LoadStyle(format!(
            "{}/resources/themes/catppuccin.toml",
            env!("CARGO_MANIFEST_DIR")
        )));
        sniffer.update(Message::Style(StyleType::DraculaDark));
        sniffer.update(Message::ChangeVolume(100));
        sniffer.update(Message::WindowMoved(-10.0, 555.0));
        sniffer.update(Message::WindowResized(1000.0, 999.0));
        sniffer.thumbnail = true;
        sniffer.update(Message::WindowMoved(40.0, 40.0));
        sniffer.update(Message::SetCaptureSource(CaptureSourcePicklist::File));
        sniffer.update(Message::ToggleFilters);
        sniffer.update(Message::BpfFilter("tcp or udp".to_string()));
        sniffer.update(Message::ReportSortSelection(SortType::Ascending));
        sniffer.update(Message::HostSortSelection(SortType::Descending));
        sniffer.update(Message::ServiceSortSelection(SortType::Descending));
        sniffer.update(Message::OpenSettings(SettingsPage::Appearance));
        sniffer.update(Message::ToggleExportPcap);
        sniffer.update(Message::OutputPcapFile("test.cap".to_string()));
        sniffer.update(Message::OutputPcapDir("/".to_string()));
        sniffer.update(Message::SetPcapImport("/test.pcap".to_string()));
        sniffer.update(Message::ChangeRunningPage(RunningPage::Notifications));
        sniffer.update(Message::DataReprSelection(DataRepr::Bits));
        sniffer.update(Message::LoadIpBlacklist("blacklist_file.csv".to_string()));

        // force saving configs by quitting the app
        sniffer.welcome = Some((false, 0));
        sniffer.update(Message::Quit);

        assert!(path.exists());

        // check that updated configs are inherited by a new sniffer instance
        let conf_end = Sniffer::new(Conf::load()).conf.clone();
        assert_eq!(
            conf_end,
            Conf {
                settings: Settings {
                    color_gradient: GradientType::Wild,
                    language: Language::ZH,
                    scale_factor: 0.5,
                    mmdb_country: "countrymmdb".to_string(),
                    mmdb_asn: "asnmmdb".to_string(),
                    style_path: format!(
                        "{}/resources/themes/catppuccin.toml",
                        env!("CARGO_MANIFEST_DIR")
                    ),
                    notifications: Notifications {
                        volume: 100,
                        ..Notifications::default()
                    },
                    style: StyleType::DraculaDark,
                    ip_blacklist: "blacklist_file.csv".to_string(),
                },
                window: ConfigWindow::new((1000.0, 999.0), (-5.0, 277.5), (20.0, 20.0)),
                device: ConfigDevice::default(),
                capture_source_picklist: CaptureSourcePicklist::File,
                filters: Filters {
                    expanded: true,
                    bpf: "tcp or udp".to_string(),
                },
                report_sort_type: SortType::Ascending,
                host_sort_type: SortType::Descending,
                service_sort_type: SortType::Descending,
                program_sort_type: SortType::Neutral,
                last_opened_setting: SettingsPage::Appearance,
                last_opened_page: RunningPage::Notifications,
                export_pcap: ExportPcap {
                    enabled: true,
                    file_name: "test.cap".to_string(),
                    directory: "/".to_string()
                },
                import_pcap_path: "/test.pcap".to_string(),
                data_repr: DataRepr::Bits,
            }
        );
    }

    #[test]
    #[parallel] // needed to not collide with other tests generating configs files
    fn test_window_resized() {
        let mut sniffer = Sniffer::new(Conf::default());
        assert!(!sniffer.thumbnail);
        let factor = sniffer.conf.settings.scale_factor;
        assert_eq!(factor, 1.0);
        assert_eq!(
            sniffer.conf.window.size(),
            Size {
                width: 1190.0,
                height: 670.0
            }
        );

        sniffer.update(Message::WindowResized(850.0, 600.0));
        assert_eq!(
            sniffer.conf.window.size(),
            Size {
                width: 850.0,
                height: 600.0
            }
        );

        sniffer.update(Message::ChangeScaleFactor(1.5));
        let factor = sniffer.conf.settings.scale_factor;
        assert_eq!(factor, 1.5);
        sniffer.update(Message::WindowResized(1000.0, 800.0));
        assert_eq!(
            sniffer.conf.window.size(),
            Size {
                width: 1000.0,
                height: 800.0
            }
        );

        sniffer.update(Message::ChangeScaleFactor(0.5));
        let factor = sniffer.conf.settings.scale_factor;
        assert_eq!(factor, 0.5);
        sniffer.update(Message::WindowResized(1000.0, 800.0));
        assert_eq!(
            sniffer.conf.window.size(),
            Size {
                width: 1000.0,
                height: 800.0
            }
        );
    }

    #[test]
    #[parallel] // needed to not collide with other tests generating configs files
    fn test_window_moved() {
        let mut sniffer = Sniffer::new(Conf::default());
        assert!(!sniffer.thumbnail);
        assert_eq!(sniffer.conf.settings.scale_factor, 1.0);
        assert_eq!(sniffer.conf.window.position(), Point { x: 0.0, y: 0.0 });
        assert_eq!(
            sniffer.conf.window.thumbnail_position(),
            Point { x: 0.0, y: 0.0 }
        );

        sniffer.update(Message::WindowMoved(850.0, 600.0));
        assert_eq!(sniffer.conf.window.position(), Point { x: 850.0, y: 600.0 });
        assert_eq!(
            sniffer.conf.window.thumbnail_position(),
            Point { x: 0.0, y: 0.0 }
        );
        sniffer.thumbnail = true;
        sniffer.update(Message::WindowMoved(400.0, 600.0));
        assert_eq!(sniffer.conf.window.position(), Point { x: 850.0, y: 600.0 });
        assert_eq!(
            sniffer.conf.window.thumbnail_position(),
            Point { x: 400.0, y: 600.0 }
        );

        sniffer.update(Message::ChangeScaleFactor(1.5));
        assert_eq!(sniffer.conf.settings.scale_factor, 1.5);
        sniffer.update(Message::WindowMoved(20.0, 40.0));
        assert_eq!(sniffer.conf.window.position(), Point { x: 850.0, y: 600.0 });
        assert_eq!(
            sniffer.conf.window.thumbnail_position(),
            Point { x: 30.0, y: 60.0 }
        );
        sniffer.thumbnail = false;
        sniffer.update(Message::WindowMoved(-20.0, 300.0));
        assert_eq!(sniffer.conf.window.position(), Point { x: -30.0, y: 450.0 });
        assert_eq!(
            sniffer.conf.window.thumbnail_position(),
            Point { x: 30.0, y: 60.0 }
        );

        sniffer.update(Message::ChangeScaleFactor(0.5));
        assert_eq!(sniffer.conf.settings.scale_factor, 0.5);
        sniffer.update(Message::WindowMoved(500.0, -100.0));
        assert_eq!(sniffer.conf.window.position(), Point { x: 250.0, y: -50.0 });
        assert_eq!(
            sniffer.conf.window.thumbnail_position(),
            Point { x: 30.0, y: 60.0 }
        );
        sniffer.thumbnail = true;
        sniffer.update(Message::WindowMoved(-2.0, -34.0));
        assert_eq!(sniffer.conf.window.position(), Point { x: 250.0, y: -50.0 });
        assert_eq!(
            sniffer.conf.window.thumbnail_position(),
            Point { x: -1.0, y: -17.0 }
        );
    }

    #[test]
    #[parallel] // needed to not collide with other tests generating configs files
    fn test_toggle_thumbnail() {
        let mut sniffer = Sniffer::new(Conf::default());
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
        let mut sniffer = Sniffer::new(Conf::default());
        assert_eq!(sniffer.conf.settings.scale_factor, 1.0);

        sniffer.update(Message::ScaleFactorShortcut(true));
        assert_eq!(sniffer.conf.settings.scale_factor, 1.1);
        sniffer.update(Message::ScaleFactorShortcut(false));
        assert_eq!(sniffer.conf.settings.scale_factor, 1.0);
        sniffer.update(Message::ScaleFactorShortcut(false));
        assert_eq!(sniffer.conf.settings.scale_factor, 0.9);

        for _ in 0..100 {
            sniffer.update(Message::ScaleFactorShortcut(true));
        }
        assert_eq!(
            format!("{:.2}", sniffer.conf.settings.scale_factor),
            "3.00".to_string()
        );

        for _ in 0..100 {
            sniffer.update(Message::ScaleFactorShortcut(false));
        }
        assert_eq!(
            format!("{:.2}", sniffer.conf.settings.scale_factor),
            "0.30".to_string()
        );
    }
}
