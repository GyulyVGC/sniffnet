use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use crate::{RunTimeData, InfoTraffic, parse_packets_loop, Sniffer, Status, TrafficChart};
use iced::{executor, Application, Column, Command, Container, Element, Length, Subscription};
use std::time::Duration;
use pcap::Device;
use crate::info_address_port_pair::{AppProtocol, TransProtocol};
use crate::gui_initial_page::initial_page;
use crate::gui_run_page::run_page;
use crate::style::{Mode};


pub const PERIOD_RUNNING: u64 = 1000; //milliseconds
pub const PERIOD_INIT: u64 = 5000; //milliseconds


#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    TickRun,
    AdapterSelection(String),
    IpVersionSelection(String),
    TransportProtocolSelection(TransProtocol),
    AppProtocolSelection(AppProtocol),
    ChartSelection(String),
    ReportSelection(String),
    OpenReport,
    OpenGithub,
    Start,
    Reset,
    Style,
}


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
            Message::Tick => {}
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
                        //println!("{}",dev.addresses.len());
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
                if what_to_display.eq("packets") {
                    self.chart_packets = true;
                }
                else {
                    self.chart_packets = false;
                }
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
                let charts_data_mutex = self.runtime_data.clone();
                *charts_data_mutex.lock().unwrap() = RunTimeData::new();
                *self.status_pair.0.lock().unwrap() = Status::Running;
                self.traffic_chart = TrafficChart::new(charts_data_mutex);
                self.status_pair.1.notify_all();
                thread::Builder::new().name(format!("thread_parse_packets_{}",current_capture_id.lock().unwrap())).spawn(move || {
                    parse_packets_loop(current_capture_id, device, filters,
                                       info_traffic_mutex);
                }).unwrap();
            }
            Message::Reset => {
                *self.current_capture_id.lock().unwrap() += 1; //change capture id to kill previous capture and to rewrite output file
                *self.status_pair.0.lock().unwrap() = Status::Init;
            }
            Message::Style => {
                self.style = if self.style == Mode::Day {
                    Mode::Night
                } else {
                    Mode::Day
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
                iced::time::every(Duration::from_millis(PERIOD_INIT)).map(|_| Message::Tick)
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
            Status::Stop => { Column::new() }
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


pub fn update_charts_data(charts_data_mutex: Arc<Mutex<RunTimeData>>) {

    let mut charts_data = charts_data_mutex.lock().unwrap();
    let tot_seconds = charts_data.ticks;
    charts_data.ticks += 1;

    let sent_bytes_entry =  charts_data.tot_sent_bytes_prev - charts_data.tot_sent_bytes;
    let received_bytes_entry = charts_data.tot_received_bytes - charts_data.tot_received_bytes_prev;
    let sent_packets_entry = charts_data.tot_sent_packets_prev - charts_data.tot_sent_packets;
    let received_packets_entry = charts_data.tot_received_packets - charts_data.tot_received_packets_prev;

    // update sent bytes traffic data
    if charts_data.sent_bytes.len() >= 30 {
        charts_data.sent_bytes.pop_front();
    }
    charts_data.sent_bytes.push_back((tot_seconds as u128, sent_bytes_entry));
    charts_data.min_sent_bytes = get_min(charts_data.sent_bytes.clone());
    charts_data.tot_sent_bytes_prev = charts_data.tot_sent_bytes;
    // update received bytes traffic data
    if charts_data.received_bytes.len() >= 30 {
        charts_data.received_bytes.pop_front();
    }
    charts_data.received_bytes.push_back((tot_seconds as u128, received_bytes_entry));
    charts_data.max_received_bytes = get_max(charts_data.received_bytes.clone());
    charts_data.tot_received_bytes_prev = charts_data.tot_received_bytes;

    // update sent packets traffic data
    if charts_data.sent_packets.len() >= 30 {
        charts_data.sent_packets.pop_front();
    }
    charts_data.sent_packets.push_back((tot_seconds as u128, sent_packets_entry));
    charts_data.min_sent_packets = get_min(charts_data.sent_packets.clone());
    charts_data.tot_sent_packets_prev = charts_data.tot_sent_packets;
    // update received packets traffic data
    if charts_data.received_packets.len() >= 30 {
        charts_data.received_packets.pop_front();
    }
    charts_data.received_packets.push_back((tot_seconds as u128, received_packets_entry));
    charts_data.max_received_packets = get_max(charts_data.received_packets.clone());
    charts_data.tot_received_packets_prev = charts_data.tot_received_packets;

}


fn get_min(deque: VecDeque<(u128, i128)>) -> i128 {
    let mut min = 0;
    for (_, x) in deque.iter() {
        if *x < min {
            min = *x;
        }
    }
    min
}


fn get_max(deque: VecDeque<(u128, i128)>) -> i128 {
    let mut max = 0;
    for (_, x) in deque.iter() {
        if *x > max {
            max = *x;
        }
    }
    max
}