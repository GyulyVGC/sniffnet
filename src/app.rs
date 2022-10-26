use crate::{get_app_count_string, InfoTraffic, Sniffer, Status};
use iced::{ Svg, alignment, button, scrollable, executor, Alignment, Application, Button, Column, Command, Container, Element, Length, Row, Settings, Subscription, Text, Color, Radio, Scrollable, PickList, pick_list, Font, widget};
use std::sync::{Arc, Mutex, Condvar};
use std::time::Duration;
use pcap::Device;
use crate::info_address_port_pair::{AppProtocol, TransProtocol};
use crate::style::{Mode, FontSizeBody, FontSizeSubtitle, FontSizeTitle, icon_sun_moon};


#[derive(Debug, Clone)]
pub(crate) enum Message {
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

        let button_start = Button::new(
            &mut self.start,
            Text::new("Run!").vertical_alignment(alignment::Vertical::Center).horizontal_alignment(alignment::Horizontal::Center),
        )
            .padding(10)
            .height(Length::Units(80))
            .width(Length::Units(160))
            .style(self.style)
            .on_press(Message::Start);

        let button_reset = Button::new(
            &mut self.reset,
            Text::new("Reset").horizontal_alignment(alignment::Horizontal::Center),
        )
            .padding(10)
            .height(Length::Units(40))
            .width(Length::Units(80))
            .style(self.style)
            .on_press(Message::Reset);

        let button_style = Button::new(
            &mut self.mode,
            // icon(match self.style {
            //     Mode::Night => {font_awesome::SUN}
            //     Mode::Day => {font_awesome::MOON}
            icon_sun_moon(self.style)
                .horizontal_alignment(alignment::Horizontal::Center),
        )
            .padding(10)
            .height(Length::Units(40))
            .width(Length::Units(100))
            .style(self.style)
            .on_press(Message::Style);

        let button_report = Button::new(
            &mut self.report,
            Text::new("Open network traffic report").horizontal_alignment(alignment::Horizontal::Center),
        )
            .padding(10)
            .height(Length::Units(40))
            .width(Length::Units(270))
            .style(self.style)
            .on_press(Message::OpenReport);

        let logo = Svg::from_path("./resources/sniffnet_logo.svg", );

        let header = Row::new()
            .height(Length::FillPortion(3))
            .align_items(Alignment::Center)
            .push(Column::new().width(Length::FillPortion(1)))
            .push(Column::new().width(Length::FillPortion(6)).push(logo))
            .push(Row::new().width(Length::FillPortion(1)).align_items(Alignment::Center).push(button_style));

        let mut dev_str_list = vec![];
        for dev in Device::list().expect("Error retrieving device list\r\n") {
            let mut dev_str = String::new();
            match dev.desc {
                None => {
                    dev_str.push_str(&format!("{}", dev.name));
                }
                Some(description) => {
                    dev_str.push_str(&format!("{}\n{}", dev.name, description));
                }
            }
            let num_addresses = dev.addresses.len();
            match num_addresses {
                0 => {},
                1 => {dev_str.push_str("\nAddress:");},
                _ => {dev_str.push_str("\nAddresses:");}
            }

            let mut x = 0;
            for addr in dev.addresses {
                x += 1;
                let address_string = addr.addr.to_string();
                // if x == num_addresses {
                //     dev_str.push_str(&format!("{}", address_string));
                // }
                //else {
                dev_str.push_str(&format!("\n    {}", address_string));
                //}
            }
            dev_str_list.push((dev.name, dev_str));
        }

        let col_adapter = Column::new()
            .padding(20)
            .spacing(10)
            .height(Length::Fill)
            .width(Length::FillPortion(4))
            .push(Text::new("Select network adapter to inspect").size(FontSizeTitle))
            .push(dev_str_list.iter().fold(
                Scrollable::new(&mut self.scroll).style(self.style).padding(10).spacing(20).height(Length::FillPortion(8)),
                |scroll, adapter| {
                    scroll.push(Radio::new(
                        &adapter.0,
                        &adapter.1,
                        Some(&self.device.clone().lock().unwrap().name),
                        |name| Message::AdapterSelection(name.to_string()),
                    ).size(15).style(self.style))
                },
            ));

        let col_space = Column::new()
            .padding(20)
            .spacing(10)
            .width(Length::FillPortion(1));

        let filtri = self.filters.lock().unwrap();
        let ip_active = &*filtri.ip;
        let col_ip_radio = Column::new().spacing(10)
            .push(Text::new("IP version").size(FontSizeSubtitle))
            .push(Radio::new(
                "ipv4",
                "IPv4",
                Some(ip_active),
                |version| Message::IpVersionSelection(version.to_string())
            ).size(15).style(self.style))
            .push(Radio::new(
                "ipv6",
                "IPv6",
                Some(ip_active),
                |version| Message::IpVersionSelection(version.to_string())
            ).size(15).style(self.style))
            .push(Radio::new(
                "no filter",
                "both",
                Some(ip_active),
                |version| Message::IpVersionSelection(version.to_string())
            ).size(15).style(self.style));
        let col_ip = Column::new()
            .spacing(10)
            .width(Length::FillPortion(1))
            .push(col_ip_radio);

        let transport_active = filtri.transport;
        let col_transport_radio = Column::new().spacing(10)
            .push(Text::new("Transport protocol").size(FontSizeSubtitle))
            .push(Radio::new(
                TransProtocol::TCP,
                "TCP",
                Some(transport_active),
                |protocol| Message::TransportProtocolSelection(protocol)
            ).size(15).style(self.style))
            .push(Radio::new(
                TransProtocol::UDP,
                "UDP",
                Some(transport_active),
                |protocol| Message::TransportProtocolSelection(protocol)
            ).size(15).style(self.style))
            .push(Radio::new(
                TransProtocol::Other,
                "both",
                Some(transport_active),
                |protocol| Message::TransportProtocolSelection(protocol)
            ).size(15).style(self.style));
        let col_transport = Column::new()
            .align_items(Alignment::Center)
            .spacing(10)
            .width(Length::FillPortion(2))
            .push(col_transport_radio)
            .push(Row::new().height(Length::FillPortion(2)))
            .push(button_start)
            .push(Row::new().height(Length::FillPortion(1)));

        let app_active = filtri.application;
        let picklist_app = PickList::new(
            &mut self.app,
            &AppProtocol::ALL[..],
            Some(app_active),
            |protocol| Message::AppProtocolSelection(protocol),
        )
            .placeholder("Select application protocol")
            .style(self.style);
        let mut col_app = Column::new()
            .width(Length::FillPortion(2))
            .spacing(10)
            .push(iced::Text::new("Application protocol").size(FontSizeSubtitle))
            .push(picklist_app);

        let filters = Column::new().width(Length::FillPortion(6)).padding(20).spacing(20)
            .push(Row::new().push(Text::new("Select filters to be applied on network traffic").size(FontSizeTitle)))
            .push(Row::new().height(Length::FillPortion(3)).push(col_ip).push(col_transport).push(col_app));


        let sniffer = self.info_traffic.lock().unwrap();

        let mut col_packets = Column::new()
            .width(Length::FillPortion(1))
            .align_items(Alignment::Center)
            .spacing(20)
            .push(iced::Text::new(std::env::current_dir().unwrap().to_str().unwrap()))
            .push(iced::Text::new(sniffer.all_packets.to_string()));
        if sniffer.tot_received_packets + sniffer.tot_sent_packets > 0 {
            col_packets = col_packets
                .push(iced::Text::new("Packets count per application protocol"))
                .push(iced::Text::new(get_app_count_string(sniffer.app_protocols.clone(), sniffer.tot_received_packets + sniffer.tot_sent_packets)));
        }
        col_packets = col_packets.push(button_reset).push(button_report);

        let mut body = Row::new().height(Length::FillPortion(9));

        match *self.status_pair.0.lock().unwrap() {
            Status::Init => {body = body
                .push(col_adapter)
                .push(col_space)
                .push(filters);}
            Status::Running => {body = body
                .push(col_packets);}
            Status::Pause => {}
            Status::Stop => {}
        }

        Container::new(Column::new().push(header).push(body))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(self.style)
            .into()
    }

}