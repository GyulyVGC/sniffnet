mod thread_parse_packets_functions;
mod address_port_pair;
mod info_address_port_pair;
mod args;
mod thread_write_report_functions;
mod info_traffic;

use std::time::Duration;
use iced::{alignment, button, executor, Alignment, Application, Button, Column, Command, Container, Element, Length, Row, Settings, Subscription, Text, Color};
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


struct Sniffer {
    info_traffic: Arc<Mutex<InfoTraffic>>,
    device: Arc<Mutex<Device>>,
    status_pair: Arc<(Mutex<Status>, Condvar)>,
    reset: button::State,
}


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
    let mutex_map2= mutex_map1.clone();

    //shared tuple containing the application status and the relative condition variable
    let status_pair1 = Arc::new((Mutex::new(Status::Pause), Condvar::new()));
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
        reset: button::State::new(),
    }))

}

#[derive(Debug, Clone)]
enum Message {
    Tick,
    Reset
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
            Message::Reset => {
                *self.device.lock().unwrap() = Device::from("en2");
                *self.status_pair.0.lock().unwrap() = Status::Running;
                &self.status_pair.1.notify_all();
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
                .width(Length::Units(80))
                .style(style)
        };

        let reset_button =
            button(&mut self.reset, "en2", style::Button::Secondary)
                .on_press(Message::Reset);

        // for dev in Device::list().expect("Error retrieving device list\r\n") {
        //     match dev.desc {
        //         None => {
        //             print!("\r\tDevice: {}\r\n\t\tAddresses: ", dev.name.cyan());
        //         }
        //         Some(description) => {
        //             print!("\r\tDevice: {} ({})\r\n\t\tAddresses: ", dev.name.cyan(), description.cyan());
        //         }
        //     }
        //     if dev.addresses.is_empty() {
        //         println!("\r");
        //     }
        //     for addr in dev.addresses {
        //         let address_string = addr.addr.to_string();
        //         print!("{}\r\n\t\t\t   ", address_string.cyan());
        //     }
        // }

        let sniffer = self.info_traffic.lock().unwrap();

        let mut row = Row::new();

        let column1 = Column::new()
            .width(Length::FillPortion(2))
            .align_items(Alignment::Center)
            .spacing(20)
            .push(iced::Text::new("Choose adapter"))
            .push(reset_button);

        row = row.push(column1);

        if sniffer.all_packets > 0 {
            let column2 = Column::new()
                .width(Length::FillPortion(2))
                .align_items(Alignment::Center)
                .spacing(20)
                .push(iced::Text::new("Packets count"))
                .push(iced::Text::new(sniffer.all_packets.to_string()))
                .push(iced::Text::new("Packets count per application protocol"))
                .push(iced::Text::new(get_app_count_string(sniffer.app_protocols.clone(), sniffer.all_packets)));
            row = row.push(column2);
        }

        Container::new(row)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(style::Button::Primary)
            .into()
    }

    // fn background_color(&self) -> Color {
    //     Color::BLACK
    // }
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

mod style {
    use iced::{container, Background, Color, Vector, Container, Element, Row, Application, button};
    use iced::container::{Style, StyleSheet};
    use crate::Message;

    pub enum Button {
        Primary,
        Secondary,
        Destructive,
    }

    impl StyleSheet for Button {
        fn style(&self) -> Style {
            Style {
                text_color: Some(Color{r: 0.0, g: 0.0, b: 0.0, a: 1.0,}),
                background: Some(Background::Color(Color{r: 0.0, g: 0.0, b: 0.0, a: 0.7,})),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Default::default()
            }
        }
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                    Button::Secondary => Color::from_rgb(0.5, 0.5, 0.5),
                    Button::Destructive => Color::from_rgb(0.8, 0.2, 0.2),
                })),
                border_radius: 12.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::WHITE,
                ..button::Style::default()
            }
        }
    }
}