//! Module defining the application structure: messages, updates, subscriptions.
//!
//! It also is a wrapper of gui's main two pages: initial and run page.

use iced::widget::{Column, Container, Row};
use iced::{executor, Application, Command, Element, Subscription, Theme};
use pcap::Device;
use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

use crate::enums::message::Message;
use crate::enums::status::Status;
use crate::gui::components::footer::get_footer;
use crate::gui::components::header::get_header;
use crate::gui::components::modals::{get_exit_overlay, Modal};
use crate::gui::pages::initial_page::initial_page;
use crate::gui::pages::run_page::run_page;
use crate::gui::pages::settings::{
    settings_appearance_page, settings_language_page, settings_notifications_page,
};
use crate::structs::config::Config;
use crate::structs::sniffer::Sniffer;
use crate::structs::traffic_chart::TrafficChart;
use crate::thread_parse_packets::parse_packets_loop;
use crate::utility::manage_charts_data::update_charts_data;
use crate::utility::manage_notifications::notify;
use crate::utility::manage_packets::get_capture_result;
use crate::utility::manage_report_data::update_report_data;
use crate::utility::style_constants::get_font;
use crate::{InfoTraffic, Notifications, RunTimeData};

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
                let info_traffic_lock = self.info_traffic.lock().unwrap();
                self.runtime_data.borrow_mut().all_packets = info_traffic_lock.all_packets;
                if info_traffic_lock.tot_received_packets + info_traffic_lock.tot_sent_packets == 0
                {
                    drop(info_traffic_lock);
                    self.update(Message::Waiting);
                } else {
                    self.runtime_data.borrow_mut().tot_sent_packets =
                        info_traffic_lock.tot_sent_packets as i128;
                    self.runtime_data.borrow_mut().tot_received_packets =
                        info_traffic_lock.tot_received_packets as i128;
                    self.runtime_data.borrow_mut().all_packets = info_traffic_lock.all_packets;
                    self.runtime_data.borrow_mut().all_bytes = info_traffic_lock.all_bytes;
                    self.runtime_data.borrow_mut().tot_received_bytes =
                        info_traffic_lock.tot_received_bytes as i128;
                    self.runtime_data.borrow_mut().tot_sent_bytes =
                        info_traffic_lock.tot_sent_bytes as i128;
                    self.runtime_data.borrow_mut().app_protocols =
                        info_traffic_lock.app_protocols.clone();
                    drop(info_traffic_lock);
                    notify(self.runtime_data.borrow(), self.notifications);
                    update_charts_data(self.runtime_data.borrow_mut());
                    update_report_data(
                        self.runtime_data.borrow_mut(),
                        &self.info_traffic,
                        self.report_type,
                    );
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
            /*
            Message::OpenReport => {
                #[cfg(target_os = "windows")]
                std::process::Command::new("explorer")
                    .arg(r".\sniffnet_report\report.txt")
                    .spawn()
                    .unwrap();
                #[cfg(target_os = "macos")]
                std::process::Command::new("open")
                    .arg("-t")
                    .arg("./sniffnet_report/report.txt")
                    .spawn()
                    .unwrap();
                #[cfg(target_os = "linux")]
                std::process::Command::new("xdg-open")
                    .arg("./sniffnet_report/report.txt")
                    .spawn()
                    .unwrap();
            }*/
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
                self.traffic_chart = TrafficChart::new(self.runtime_data.clone(), self.style);

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
                *self.current_capture_id.lock().unwrap() += 1; //change capture id to kill previous capture and to rewrite output file
                self.pcap_error = None;
                self.update(Message::HideModal(false));
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
                self.overlay = None;
                if save_config {
                    let store = Config {
                        style: self.style,
                        notifications: Notifications {
                            packets_threshold: Some(1000),
                        },
                    };
                    confy::store("sniffnet", None, store).unwrap();
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let status = *self.status_pair.0.lock().unwrap();
        let style = self.style;

        let header = match status {
            Status::Init => get_header(style, false, 0),
            Status::Running => {
                get_header(style, true, self.info_traffic.lock().unwrap().all_packets)
            }
        };

        let body = match status {
            Status::Init => initial_page(self),
            Status::Running => run_page(self),
        };

        let content = Column::new()
            .push(header)
            .push(body)
            .push(get_footer(style));

        if self.overlay.is_none() {
            content.into()
        } else {
            let (overlay, save_config) = match self.overlay.unwrap() {
                "exit" => (get_exit_overlay(style, get_font(style)), false),
                "settings_notifications" => (settings_notifications_page(self), true),
                "settings_appearance" => (settings_appearance_page(self), true),
                "settings_language" => (settings_language_page(self), true),
                _ => (Container::new(Row::new()), false),
            };

            Modal::new(content, overlay)
                .on_blur(Message::HideModal(save_config))
                .into()
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        match *self.status_pair.0.lock().unwrap() {
            Status::Running => {
                iced::time::every(Duration::from_millis(PERIOD_RUNNING)).map(|_| Message::TickRun)
            }
            Status::Init => {
                iced::time::every(Duration::from_millis(PERIOD_INIT)).map(|_| Message::TickInit)
            }
        }
    }
}
