//! Module defining the application structure: messages, updates, subscriptions.
//!
//! It also is a wrapper of gui's main two pages: initial and run page.

use std::thread;
use std::time::Duration;
use iced::{executor, Application, Command, Element, Length, Subscription, Renderer, Theme};
use iced::widget::Container;
use pcap::Device;

use crate::enums::element_type::ElementType;
use crate::enums::message::Message;
use crate::enums::status::Status;
use crate::gui::style::StyleTuple;
use crate::gui::{gui_initial_page::initial_page, gui_run_page::run_page};
use crate::structs::config::Config;
use crate::structs::sniffer::Sniffer;
use crate::structs::traffic_chart::TrafficChart;
use crate::thread_parse_packets::parse_packets_loop;
use crate::utility::manage_charts_data::update_charts_data;
use crate::utility::manage_report_data::update_report_data;
use crate::utility::sounds::play_sound;
use crate::{InfoTraffic, RunTimeData, StyleType};
use crate::structs::colors::Colors;
use crate::utility::style_constants::{ALMOND_STYLE, DAY_STYLE, NIGHT_STYLE, RED_STYLE, TRY_STYLE};

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
        (flags, Command::none())
    }

    fn title(&self) -> String {
        String::from("Sniffnet")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TickInit => {
                play_sound();
            }
            Message::TickRun => {
                play_sound();
                let info_traffic_lock = self.info_traffic.lock().unwrap();
                if info_traffic_lock.tot_received_packets + info_traffic_lock.tot_sent_packets == 0 {
                    drop(info_traffic_lock);
                    self.update(Message::Waiting);
                }
                else {
                    let mut runtime_data_lock = self.runtime_data.lock().unwrap();
                    runtime_data_lock.tot_sent_packets = info_traffic_lock.tot_sent_packets as i128;
                    runtime_data_lock.tot_received_packets =
                        info_traffic_lock.tot_received_packets as i128;
                    runtime_data_lock.all_packets = info_traffic_lock.all_packets;
                    runtime_data_lock.all_bytes = info_traffic_lock.all_bytes;
                    runtime_data_lock.tot_received_bytes = info_traffic_lock.tot_received_bytes as i128;
                    runtime_data_lock.tot_sent_bytes = info_traffic_lock.tot_sent_bytes as i128;
                    runtime_data_lock.app_protocols = info_traffic_lock.app_protocols.clone();
                    drop(info_traffic_lock);
                    drop(runtime_data_lock);
                    update_charts_data(self.runtime_data.clone());
                    update_report_data(
                        self.runtime_data.clone(),
                        self.info_traffic.clone(),
                        self.report_type,
                    );
                }
            }
            Message::AdapterSelection(name) => {
                for dev in Device::list().expect("Error retrieving device list\r\n") {
                    if dev.name.eq(&name) {
                        *self.device.lock().unwrap() = dev;
                        break;
                    }
                }
            }
            Message::IpVersionSelection(version) => {
                self.filters.lock().unwrap().ip = version;
            }
            Message::TransportProtocolSelection(protocol) => {
                self.filters.lock().unwrap().transport = protocol;
            }
            Message::AppProtocolSelection(protocol) => {
                self.filters.lock().unwrap().application = protocol;
            }
            Message::ChartSelection(what_to_display) => {
                self.chart_type = what_to_display;
            }
            Message::ReportSelection(what_to_display) => {
                self.report_type = what_to_display;
                update_report_data(
                    self.runtime_data.clone(),
                    self.info_traffic.clone(),
                    self.report_type,
                );
            }
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
                let current_capture_id = self.current_capture_id.clone();
                let device = self.device.clone();
                let filters = self.filters.clone();
                let pcap_error = self.pcap_error.clone();
                let info_traffic_mutex = self.info_traffic.clone();
                *info_traffic_mutex.lock().unwrap() = InfoTraffic::new();
                let runtime_data_mutex = self.runtime_data.clone();
                *runtime_data_mutex.lock().unwrap() = RunTimeData::new();
                *self.status_pair.0.lock().unwrap() = Status::Running;
                self.traffic_chart = TrafficChart::new(runtime_data_mutex, self.style);
                self.status_pair.1.notify_all();
                thread::Builder::new()
                    .name(format!(
                        "thread_parse_packets_{}",
                        current_capture_id.lock().unwrap()
                    ))
                    .spawn(move || {
                        parse_packets_loop(
                            current_capture_id,
                            device,
                            filters,
                            info_traffic_mutex,
                            pcap_error,
                        );
                    })
                    .unwrap();
            }
            Message::Reset => {
                *self.current_capture_id.lock().unwrap() += 1; //change capture id to kill previous capture and to rewrite output file
                *self.status_pair.0.lock().unwrap() = Status::Init;
                *self.pcap_error.lock().unwrap() = Option::None;
            }
            Message::Style => {
                let current_style = self.style;
                self.style = match current_style {
                    StyleType::Night => StyleType::Day,
                    StyleType::Day => StyleType::Try,
                    StyleType::Try => StyleType::Almond,
                    StyleType::Almond => StyleType::Red,
                    StyleType::Red => StyleType::Night,
                };
                let cfg = Config { style: self.style };
                confy::store("sniffnet", None, cfg).unwrap();
            }
            Message::Waiting => {
                if self.waiting.len() > 2 {
                    self.waiting = "".to_string();
                }
                self.waiting = ".".repeat(self.waiting.len() + 1);
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let status = *self.status_pair.0.lock().unwrap();
        let style = self.style;

        let body = match status {
            Status::Init => initial_page(self),
            Status::Running => run_page(self),
        };

        Container::new(body)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            //.style(StyleTuple(style, ElementType::Standard))
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        match *self.status_pair.0.lock().unwrap() {
            Status::Running => {
                iced::time::every(Duration::from_millis(PERIOD_RUNNING)).map(|_| Message::TickRun)
            }
            _ => iced::time::every(Duration::from_millis(PERIOD_INIT)).map(|_| Message::TickInit),
        }
    }

    fn theme(&self) -> Theme {
        Theme::Dark
        // match self.style {
        //     StyleType::Night => NIGHT_STYLE,
        //     StyleType::Day => DAY_STYLE,
        //     StyleType::Try => TRY_STYLE,
        //     StyleType::Almond => ALMOND_STYLE,
        //     StyleType::Red => RED_STYLE,
        // }
    }
}
