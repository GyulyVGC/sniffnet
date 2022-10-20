use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use iced::{alignment, button, executor, Alignment, Application, Button, Column, Command, Container, Element, Length, Row, Settings, Subscription, Text};
use pcap::{Capture, Device, Error, Packet};

pub fn main() -> iced::Result {

    let sniffer = Arc::new(Mutex::new(Sniffer { packets: 0 }));
    let mut sniffer1 = sniffer.clone();
    thread::spawn(move || {
        let found_device = Device::lookup().unwrap().unwrap();
        let mut cap = Capture::from_device(found_device.clone())
            .expect("Capture initialization error\n\r")
            .promisc(true)
            .snaplen(256)
            .open()
            .expect("Capture initialization error\n\r");
        loop {
            match cap.next_packet() {
                Err(_) => {}
                Ok(_) => {
                    sniffer1.lock().unwrap().packets += 1;
                }
            }
        }
    });

    Stopwatch::run(Settings::with_flags(sniffer))

}

struct Stopwatch {
    sniffer: Arc<Mutex<Sniffer>>,
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
    type Flags = Arc<Mutex<Sniffer>>;

    fn new(flags: Arc<Mutex<Sniffer>>) -> (Stopwatch, Command<Message>) {
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
                   // self.sniffer.update();
                }
            },
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        match self.state {
            State::Ticking => {
                const FPS: u64 = 50;
                iced::time::every(Duration::from_millis(1000 / FPS)).map(|_| Message::Tick)
            }
        }
    }

    fn view(&mut self) -> Element<Message> {

        let mut x = self.sniffer.lock().unwrap();
        let content = Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(iced::Text::new("Packets count"))
            .push(iced::Text::new(format!("{}", x.packets)));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

struct Sniffer {
    packets: u64
}

impl Sniffer {

    fn view(&mut self) -> Element<Message> {
        Row::new().push(iced::Text::new(format!("{}", self.packets))).into()
    }

}

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