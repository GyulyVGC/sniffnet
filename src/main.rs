mod thread_parse_packets_functions;
mod address_port_pair;
mod info_address_port_pair;
mod args;
mod thread_write_report_functions;
mod info_traffic;

use std::time::Duration;
use plotters_iced::{Chart, ChartWidget, DrawingBackend, ChartBuilder};
use iced::{Svg, alignment, button, scrollable, executor, Alignment, Application, Button, Column, Command, Container, Element, Length, Row, Settings, Subscription, Text, Color, Radio, Scrollable};
use pcap::{Capture, Device};
use crate::info_address_port_pair::{AppProtocol, TransProtocol};
use crate::thread_parse_packets_functions::parse_packets_loop;
use std::cmp::Ordering::Equal;
use crate::args::Args;
use crate::thread_write_report_functions::sleep_and_write_report_loop;
use crate::thread_write_report_functions::get_app_count_string;
use clap::Parser;
use std::{io, panic, process, thread};
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::io::Write;
use std::sync::{Arc, Mutex, Condvar};
use crossterm::{screen::RawScreen,  input::{input, InputEvent, KeyEvent}};
use colored::Colorize;
use crate::info_traffic::InfoTraffic;
use crate::style::Mode;


struct Sniffer {
    info_traffic: Arc<Mutex<InfoTraffic>>,
    device: Arc<Mutex<Device>>,
    status_pair: Arc<(Mutex<Status>, Condvar)>,
    start: button::State,
    reset: button::State,
    mode: button::State,
    scroll: scrollable::State,
    style: Mode
}


/// This enum represents the sniffing process status.
#[derive(PartialEq, Eq)]
pub enum Status {
    /// Sniffnet has just been launched/restarted
    Init,
    /// The sniffing process is running: the application parses packets and periodically update the output report.
    Running,
    /// The sniffing process is pause by the user and waiting to be later resumed.
    Pause,
    /// The sniffing process is killed.
    Stop
}

pub fn main() -> iced::Result {

    //shared tuple containing:
    // - the map of the address:ports pairs with the relative info
    // - the total number of sniffed packets
    // - the number of filtered packets
    // - the map of the observed app protocols with the relative packet count
    let mutex_map1 = Arc::new(Mutex::new(InfoTraffic::new()));
    let mutex_map2= mutex_map1.clone();

    //shared tuple containing the application status and the relative condition variable
    let status_pair1 = Arc::new((Mutex::new(Status::Init), Condvar::new()));
    let status_pair2 =  status_pair1.clone();

    let found_device1 = Arc::new(Mutex::new(Device::lookup().unwrap().unwrap()));
    let found_device2 = found_device1.clone();

    thread::spawn(move || {
        parse_packets_loop(found_device1, 0, 65535, "no filter".to_string(),
                           TransProtocol::Other, AppProtocol::Other,
                           mutex_map1, status_pair1);
    });

    Sniffer::run(Settings::with_flags(Sniffer {
        info_traffic: mutex_map2,
        device: found_device2,
        status_pair: status_pair2,
        start: button::State::new(),
        reset: button::State::new(),
        mode: button::State::new(),
        scroll: scrollable::State::new(),
        style: Mode::Night
    }))

}

#[derive(Debug, Clone)]
enum Message {
    Tick,
    AdapterSelection(String),
    Start,
    Reset,
    Style
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
            Message::AdapterSelection(name) => {
                *self.device.lock().unwrap() = Device::from(&*name);
            }
            Message::Start => {
                *self.status_pair.0.lock().unwrap() = Status::Running;
                &self.status_pair.1.notify_all();
            }
            Message::Reset => {
                let mut info_traffic = self.info_traffic.lock().unwrap();
                info_traffic.all_packets = 0;
                info_traffic.app_protocols = HashMap::new();
            }
            Message::Style => {
                self.style = if self.style == Mode::Day {
                    Mode::Night
                }
                else {
                    Mode::Day
                }
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        match self.status_pair.0.lock().unwrap() {
            _ => {
                iced::time::every(Duration::from_millis(1000)).map(|_| Message::Tick)
            }
        }
    }

    fn view(&mut self) -> Element<Message> {

        let button = |state, label, style| {
            Button::new(
                state,
                Text::new(label)
                    .horizontal_alignment(alignment::Horizontal::Center),
            )
                .padding(10)
                .height(Length::Units(40))
                .width(Length::Units(80))
                .style(style)
        };

        let button_start =
            button(&mut self.start, "Start", self.style)
                .on_press(Message::Start);

        let button_reset =
            button(&mut self.reset, "Reset", self.style)
                .on_press(Message::Reset);

        let button_style =
            button(&mut self.mode, "Style", self.style)
                .on_press(Message::Style);

        let svg = Svg::from_path("./img/sniffnet_logo.svg", )
            .width(Length::Fill)
            .height(Length::FillPortion(3));

        let mut dev_str_list = vec![];
        for dev in Device::list().expect("Error retrieving device list\r\n") {
            let mut dev_str = String::new();
            match dev.desc {
                None => {
                    dev_str.push_str(&format!("Device: {}\nAddresses: ", dev.name));
                }
                Some(description) => {
                    dev_str.push_str(&format!("Device: {} ({})\nAddresses: ", dev.name.cyan(), description));
                }
            }
            // if dev.addresses.is_empty() {
            //     dev_str.push_str("\r");
            // }
            for addr in dev.addresses {
                let address_string = addr.addr.to_string();
                dev_str.push_str(&format!("{}\n                ", address_string));
            }
            dev_str_list.push((dev.name, dev_str));
        }

        let col_adapter = Column::new()
            .padding(20)
            .spacing(10)
            .push(Text::new("Select network adapter to inspect").size(24))
            .push(dev_str_list.iter().fold(
                Scrollable::new(&mut self.scroll).padding(10).spacing(20).height(Length::FillPortion(8)),
                |choices, adapter| {
                    choices.push(Radio::new(
                        &adapter.0,
                        &adapter.1,
                        Some(&self.device.clone().lock().unwrap().name),
                        |name| Message::AdapterSelection(name.to_string()),
                    ))
                },
            ))
            .push(button_start);

        let sniffer = self.info_traffic.lock().unwrap();

        let mut row = Row::new().height(Length::FillPortion(9));

        let mut col_packets = Column::new()
            .width(Length::FillPortion(2))
            .align_items(Alignment::Center)
            .spacing(20)
            .push(iced::Text::new("Packets count"))
            .push(iced::Text::new(sniffer.all_packets.to_string()));
        if sniffer.all_packets > 0 {
            col_packets = col_packets
                .push(iced::Text::new("Packets count per application protocol"))
                .push(iced::Text::new(get_app_count_string(sniffer.app_protocols.clone(), sniffer.all_packets)));
        }
        col_packets = col_packets.push(button_reset);

        match *self.status_pair.0.lock().unwrap() {
            Status::Init => {row = row.push(col_adapter);}
            Status::Running => {row = row.push(col_packets);}
            Status::Pause => {}
            Status::Stop => {}
        }

        Container::new(Column::new().push(svg).push(button_style).push(row))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(self.style)
            .into()
    }

}

mod style {
    use iced::{container, Background, Color, Vector, Container, Element, Row, Application, button};
    use iced::container::{Style, StyleSheet};
    use crate::Message;

    #[derive(Copy, Eq, PartialEq)]
    pub enum Mode {
        Day,
        Night
    }

    impl Clone for Mode {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl StyleSheet for Mode {
        fn style(&self) -> Style {
            Style {
                text_color: Some(Color{r: 0.0, g: 0.0, b: 0.0, a: 1.0,}),
                background: match self {
                    Mode::Day => {Some(Background::Color(Color{r: 0.0, g: 0.0, b: 0.0, a: 0.48,}))}
                    Mode::Night => {Some(Background::Color(Color{r: 0.0, g: 0.0, b: 0.0, a: 0.96,}))}
                },
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Default::default()
            }
        }
    }

    impl button::StyleSheet for Mode {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Mode::Day => Color{r: 0.0, g: 0.0, b: 0.0, a: 0.48,},
                    Mode::Night => Color{r: 0.0, g: 0.0, b: 0.0, a: 0.96,},
                })),
                border_radius: 12.0,
                border_width: 3.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::BLACK,
                border_color: match self {
                    Mode::Day => Color{r: 0.0, g: 0.0, b: 0.0, a: 0.96,},
                    Mode::Night => Color{r: 0.0, g: 0.0, b: 0.0, a: 0.48,},
                }
            }
        }
    }
}