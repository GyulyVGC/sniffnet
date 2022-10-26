use std::os::macos::raw::stat;
use crate::{get_app_count_string, InfoTraffic, Sniffer, Status};
use iced::{ Svg, alignment, button, scrollable, executor, Alignment, Application, Button, Column, Command, Container, Element, Length, Row, Settings, Subscription, Text, Color, Radio, Scrollable, PickList, pick_list, Font, widget};
use std::sync::{Arc, Mutex, Condvar};
use std::time::Duration;
use pcap::Device;
use crate::info_address_port_pair::{AppProtocol, TransProtocol};
use crate::gui_initial_page::initial_page;
use crate::gui_run_page::run_page;
use crate::style::{Mode, FONT_SIZE_BODY, FONT_SIZE_SUBTITLE, FONT_SIZE_TITLE, icon_sun_moon};


#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    AdapterSelection(String),
    IpVersionSelection(String),
    TransportProtocolSelection(TransProtocol),
    AppProtocolSelection(AppProtocol),
    OpenReport,
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
            Message::IpVersionSelection(version) => {
                self.filters.lock().unwrap().ip = version;
            }
            Message::TransportProtocolSelection(protocol) => {
                self.filters.lock().unwrap().transport = protocol;
            }
            Message::AppProtocolSelection(protocol) => {
                self.filters.lock().unwrap().application = protocol;
            }
            Message::OpenReport => {
                #[cfg(target_os = "windows")]
                    let command = "explorer";
                #[cfg(target_os = "macos")]
                    let command = "open";
                #[cfg(target_os = "linux")]
                    let command = "explorer";
                std::process::Command::new( command )
                    .arg( "./sniffnet_report/report.txt" )
                    .spawn( )
                    .unwrap( );
            }
            Message::Start => {
                *self.status_pair.0.lock().unwrap() = Status::Running;
                &self.status_pair.1.notify_all();
            }
            Message::Reset => {
                let mut info_traffic = self.info_traffic.lock().unwrap();
                *info_traffic = InfoTraffic::new();
                info_traffic.reset();
                *self.status_pair.0.lock().unwrap() = Status::Init;
            }
            Message::Style => {
                self.style = if self.style == Mode::Day {
                    Mode::Night
                }
                else {
                    Mode::Day
                };
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

        let status = *self.status_pair.0.lock().unwrap();
        let mode = self.style;

        let body = match status {
            Status::Init => {
                initial_page(self)
            }
            Status::Running => {
                run_page(self)
            }
            Status::Pause => {Column::new()}
            Status::Stop => {Column::new()}
        };

        Container::new(
            Column::new()
                .push(body)
        )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(mode)
            .into()
    }

}