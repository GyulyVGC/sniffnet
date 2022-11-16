//! Module defining the application structure: messages, updates, subscriptions.
//!
//! It also is a wrapper of gui's main two pages: initial and run page.

use std::thread;
use std::time::Duration;

use iced::{Application, Command, Container, Element, executor, Length, Subscription};
use pcap::Device;

use crate::{InfoTraffic, RunTimeData};
use crate::enums::message::Message;
use crate::enums::status::Status;
use crate::gui::{gui_initial_page::initial_page, gui_run_page::run_page};
use crate::gui::style::StyleType;
use crate::structs::sniffer::Sniffer;
use crate::structs::traffic_chart::TrafficChart;
use crate::thread_parse_packets::parse_packets_loop;
use crate::utility::manage_charts_data::update_charts_data;

/// Update period when app is running
pub const PERIOD_RUNNING: u64 = 1000;
//milliseconds
/// Update period when app is in its initial state
pub const PERIOD_INIT: u64 = 5000; //milliseconds


impl Application for Sniffer {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = Sniffer;

    fn new(flags: Sniffer) -> (Sniffer, Command<Message>) {
        (
            flags,
            Command::none(),
        )
    }


    fn title(&self) -> String {
        String::from("Sniffnet")
    }


    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TickInit => {}
            Message::TickRun => {
                let mut runtime_data_lock = self.runtime_data.lock().unwrap();
                let info_traffic_lock = self.info_traffic.lock().unwrap();
                runtime_data_lock.all_packets = info_traffic_lock.all_packets;
                runtime_data_lock.tot_sent_packets = info_traffic_lock.tot_sent_packets as i128;
                runtime_data_lock.tot_received_packets = info_traffic_lock.tot_received_packets as i128;
                runtime_data_lock.all_bytes = info_traffic_lock.all_bytes;
                runtime_data_lock.tot_received_bytes = info_traffic_lock.tot_received_bytes as i128;
                runtime_data_lock.tot_sent_bytes = info_traffic_lock.tot_sent_bytes as i128;
                runtime_data_lock.app_protocols = info_traffic_lock.app_protocols.clone();
                drop(info_traffic_lock);
                drop(runtime_data_lock);
                update_charts_data(self.runtime_data.clone());
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
                std::process::Command::new("explorer")
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
                std::process::Command::new("explorer")
                    .arg("https://github.com/GyulyVGC/sniffnet")
                    .spawn()
                    .unwrap();
            }
            Message::Start => {
                let current_capture_id = self.current_capture_id.clone();
                let device = self.device.clone();
                let filters = self.filters.clone();
                let info_traffic_mutex = self.info_traffic.clone();
                *info_traffic_mutex.lock().unwrap() = InfoTraffic::new();
                let runtime_data_mutex = self.runtime_data.clone();
                *runtime_data_mutex.lock().unwrap() = RunTimeData::new();
                *self.status_pair.0.lock().unwrap() = Status::Running;
                self.traffic_chart = TrafficChart::new(runtime_data_mutex);
                self.status_pair.1.notify_all();
                thread::Builder::new().name(format!("thread_parse_packets_{}", current_capture_id.lock().unwrap())).spawn(move || {
                    parse_packets_loop(current_capture_id, device, filters,
                                       info_traffic_mutex);
                }).unwrap();
            }
            Message::Reset => {
                *self.current_capture_id.lock().unwrap() += 1; //change capture id to kill previous capture and to rewrite output file
                *self.status_pair.0.lock().unwrap() = Status::Init;
            }
            Message::Style => {
                self.style = if self.style == StyleType::Day {
                    StyleType::Night
                } else {
                    StyleType::Day
                };
            }
        }
        Command::none()
    }


    fn subscription(&self) -> Subscription<Message> {
        match *self.status_pair.0.lock().unwrap() {
            Status::Running => {
                iced::time::every(Duration::from_millis(PERIOD_RUNNING)).map(|_| Message::TickRun)
            }
            _ => {
                iced::time::every(Duration::from_millis(PERIOD_INIT)).map(|_| Message::TickInit)
            }
        }
    }


    fn view(&mut self) -> Element<Message> {
        let status = *self.status_pair.0.lock().unwrap();
        let mode = self.style;

        let body = match status {
            Status::Init => {
                initial_page(self)
            }
            Status::Running => {
                run_page(self)
            }
        };

        Container::new(
            body
        )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(mode)
            .into()
    }
}