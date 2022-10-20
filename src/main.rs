mod thread_parse_packets_functions;
mod address_port_pair;
mod info_address_port_pair;
mod args;
mod thread_write_report_functions;
mod info_traffic;

use std::time::Duration;
use iced::{alignment, button, executor, Alignment, Application, Button, Column, Command, Container, Element, Length, Row, Settings, Subscription, Text};
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
use std::io::Write;
use std::sync::{Arc, Mutex, Condvar};
use crossterm::{screen::RawScreen,  input::{input, InputEvent, KeyEvent}};
use colored::Colorize;
use crate::info_traffic::InfoTraffic;

/// This enum represents the sniffing process status.
#[derive(PartialEq, Eq)]
pub enum Status {
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
    let mutex_map2 = mutex_map1.clone();

    //shared tuple containing the application status and the relative condition variable
    let status_pair1 = Arc::new((Mutex::new(Status::Running), Condvar::new()));

    let found_device = Device::lookup().unwrap().unwrap();
    thread::spawn(move || {
        parse_packets_loop(found_device, 0, 65535, "no filter".to_string(),
                           TransProtocol::Other, AppProtocol::Other,
                           mutex_map1, status_pair1);
    });

    Stopwatch::run(Settings::with_flags(mutex_map2))

}

struct Stopwatch {
    sniffer: Arc<Mutex<InfoTraffic>>,
    state: State,
}

enum State {
    Ticking,
}

#[derive(Debug, Clone)]
enum Message {
    Tick,
}

impl Application for Stopwatch {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = Arc<Mutex<InfoTraffic>>;

    fn new(flags: Arc<Mutex<InfoTraffic>>) -> (Stopwatch, Command<Message>) {
        (
            Stopwatch {
                sniffer: flags,
                state: State::Ticking,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Sniffnet")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick => match &mut self.state {
                State::Ticking => {

                }
            },
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        match self.state {
            State::Ticking => {
                const FPS: u64 = 1;
                iced::time::every(Duration::from_millis(1000 / FPS)).map(|_| Message::Tick)
            }
        }
    }

    fn view(&mut self) -> Element<Message> {

        let sniffer = self.sniffer.lock().unwrap();



        let column1 = Column::new()
            .width(Length::FillPortion(2))
            .align_items(Alignment::Center)
            .spacing(20)
            .push(iced::Text::new("Packets count"))
            .push(iced::Text::new(format!("{}", sniffer.all_packets)));

        let mut row = Row::new().push(column1);

        if sniffer.all_packets > 0 {
            let column2 = Column::new()
                .width(Length::FillPortion(2))
                .align_items(Alignment::Center)
                .spacing(20)
                .push(iced::Text::new("Packets count per application protocol"))
                .push(iced::Text::new(get_app_count_string(sniffer.app_protocols.clone(), sniffer.all_packets)));
            row = row.push(column2);
        }

        Container::new(row)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

// struct Sniffer {
//     packets: u64
// }

// impl Sniffer {
//
//     fn view(&mut self) -> Element<Message> {
//         Row::new().push(iced::Text::new(format!("{}", self.packets))).into()
//     }
//
// }

// mod style {
//     use iced::{container, Background, Color, Vector, Container, Element};
//     use iced::container::Style;
//     use crate::Message;
//
//     pub enum Button {
//         Primary,
//         Secondary,
//         Destructive,
//     }
//
//     impl container::StyleSheet for Element<Message> {
//         fn style(&self) -> Style {
//             Style {
//                 text_color: Some(Color{r: 0.0, g: 0.0, b: 0.0, a: 1.0,}),
//                 background: Some(Background::Color(Color{ r: 64.0, g: 60.0, b: 62.0, a: 0.48})),
//                 border_radius: 0.0,
//                 border_width: 0.0,
//                 border_color: Default::default()
//             }
//         }
//     }
// }