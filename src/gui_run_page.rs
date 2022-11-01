use std::cmp::min;
use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use iced::{alignment, Alignment, Button, Column, Container, Element, Length, Row, Svg, Text};
use iced::alignment::{Horizontal, Vertical};
use iced::Length::FillPortion;
use plotters::prelude::full_palette::GREY;
use plotters::style::full_palette::GREEN_800;
use thousands::Separable;
use crate::app::Message;
use crate::{FONT_SIZE_TITLE, get_app_count_string, icon_sun_moon, InfoTraffic, Mode, Sniffer};
use crate::address_port_pair::AddressPortPair;
use crate::info_address_port_pair::InfoAddressPortPair;
use crate::style::{COURIER_PRIME_BOLD_ITALIC, FONT_SIZE_FOOTER, HEIGHT_BODY, HEIGHT_FOOTER, HEIGHT_HEADER, icon};
use plotters_iced::{Chart, ChartWidget, DrawingBackend, ChartBuilder};

pub fn run_page(sniffer: &mut Sniffer) -> Column<Message> {
    let logo = Svg::from_path("./resources/sniffnet_logo.svg");

    let button_style = Button::new(
        &mut sniffer.mode,
        icon_sun_moon(sniffer.style)
            .horizontal_alignment(alignment::Horizontal::Center),
    )
        .padding(10)
        .height(Length::Units(40))
        .width(Length::Units(100))
        .style(sniffer.style)
        .on_press(Message::Style);

    let button_reset = Button::new(
        &mut sniffer.reset,
        icon('\u{f177}')
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center),
    )
        .padding(10)
        .height(Length::Units(40))
        .width(Length::Units(80))
        .style(sniffer.style)
        .on_press(Message::Reset);

    let header = Row::new()
        .height(Length::FillPortion(HEIGHT_HEADER))
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .push(Container::new(button_reset).width(Length::FillPortion(1)).align_x(Horizontal::Center))
        .push(Container::new(logo).width(Length::FillPortion(6)).align_x(Horizontal::Center))
        .push(Container::new(button_style).width(Length::FillPortion(1)).align_x(Horizontal::Center));

    let button_report = Button::new(
        &mut sniffer.report,
        Text::new("Open full report")
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center),
    )
        .padding(10)
        .height(Length::Units(85))
        .width(Length::Units(75))
        .style(sniffer.style)
        .on_press(Message::OpenReport);

    let mut sniffer_lock = sniffer.info_traffic.lock().unwrap();
    let observed = sniffer_lock.all_packets;
    let filtered = sniffer_lock.tot_sent_packets + sniffer_lock.tot_received_packets;
    drop(sniffer_lock);

    let mut body = Column::new()
        .height(Length::FillPortion(HEIGHT_BODY))
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .spacing(10);

    match (observed, filtered) {
        (0, 0) => { //no packets observed at all
            if sniffer.waiting.len() > 5 {
                sniffer.waiting = "".to_string();
            }
            sniffer.waiting = ".".repeat(sniffer.waiting.len() + 1);
            let adapter_name = &*sniffer.device.clone().lock().unwrap().name.clone();
            let nothing_to_see_text = Text::new(format!("No traffic has been observed yet. Waiting for network packets...\n\n\
                                                              Network adapter: {}\n\n\
                                                              Are you sure you are connected to the internet and you have selected the right adapter?", adapter_name));
            body = body
                .push(Row::new().height(Length::FillPortion(1)))
                .push(Text::new(sniffer.waiting.clone()).size(50))
                .push(nothing_to_see_text)
                .push(Text::new(sniffer.waiting.clone()).size(50))
                .push(Row::new().height(Length::FillPortion(2)));
        }

        (_observed, 0) => { //no packets have been filtered but some have been observed
            if sniffer.waiting.len() > 5 {
                sniffer.waiting = "".to_string();
            }
            sniffer.waiting = ".".repeat(sniffer.waiting.len() + 1);

            let tot_packets_text = Text::new(format!("Total intercepted packets: {}\n\n\
                                                    Filtered packets: 0\n\n\
                                                    Some packets have been intercepted, but still none has been selected according to the filters you specified...",
                                                     observed.separate_with_spaces()));

            body = body
                .push(Row::new().height(Length::FillPortion(1)))
                .push(Text::new(sniffer.waiting.clone()).size(50))
                .push(tot_packets_text)
                .push(Text::new(sniffer.waiting.clone()).size(50))
                .push(Row::new().height(Length::FillPortion(2)));
        }

        (observed, filtered) => { //observed > filtered > 0 || observed = filtered > 0
            let chart = sniffer.traffic_chart.view(99);
            sniffer_lock = sniffer.info_traffic.lock().unwrap();

            let mut col_packets = Column::new()
                .width(Length::FillPortion(1))
                .align_items(Alignment::Center)
                .spacing(20)
                .push(iced::Text::new(std::env::current_dir().unwrap().to_str().unwrap()))
                .push(chart)
                .push(Text::new(format!("Total intercepted packets: {}",
                                        observed.separate_with_spaces())))
                .push(Text::new(format!("Filtered packets: {}",
                                        filtered.separate_with_spaces())));
            col_packets = col_packets
                .push(iced::Text::new("Packets count per application protocol"))
                .push(iced::Text::new(get_app_count_string(sniffer_lock.app_protocols.clone(), filtered)));

            let mut row_report = Row::new()
                .align_items(Alignment::Center);
            let mut sorted_vec: Vec<(&AddressPortPair, &InfoAddressPortPair)> = sniffer_lock.map.iter().collect();
            sorted_vec.sort_by(|&(_, a), &(_, b)|
                b.final_timestamp.cmp(&a.final_timestamp));
            let n_entry = min(sorted_vec.len(), 10);
            let mut col_report = Column::new()
                .padding(10);
            col_report = col_report
                .push(iced::Text::new("Latest connections\n").size(FONT_SIZE_TITLE))
                .push(iced::Text::new("-------------------------------------------------------------------------------------------------------------------------"))
                .push(iced::Text::new("|     Src IP address      | Src port |     Dst IP address      | Dst port | Layer 4 | Layer 7 |   Packets  |   Bytes    |"))
                .push(iced::Text::new("-------------------------------------------------------------------------------------------------------------------------"));
            for i in 0..n_entry {
                let key_val = sorted_vec.get(i).unwrap();
                col_report = col_report.push(iced::Text::new(format!("{}{}", key_val.0, key_val.1.print_without_timestamps())));
            }
            drop(sniffer_lock);
            col_report = col_report
                .push(iced::Text::new("-------------------------------------------------------------------------------------------------------------------------"));
            let col_open_report = Column::new()
                .push(button_report);
            row_report = row_report
                .push(col_report)
                .push(col_open_report);

            body = body
                .push(col_packets)
                .push(row_report);
        }
    }

    let button_github = Button::new(
        &mut sniffer.git,
        icon('\u{f09b}').size(30)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center),
    )
        .height(Length::Units(35))
        .width(Length::Units(35))
        .style(sniffer.style)
        .on_press(Message::OpenGithub);
    let footer_row = Row::new()
        .align_items(Alignment::Center)
        .push(Text::new("Sniffnet v1.0.0 - by Giuliano Bellini ").size(FONT_SIZE_FOOTER).font(COURIER_PRIME_BOLD_ITALIC))
        .push(button_github)
        .push(Text::new("  "));
    let footer = Container::new(footer_row)
        .width(Length::Fill)
        .height(FillPortion(HEIGHT_FOOTER))
        .align_y(Vertical::Center)
        .align_x(Horizontal::Center)
        .style(Mode::Bordered);

    Column::new()
        .push(header)
        .push(body)
        .push(footer)
}


pub struct TrafficChart {
    info_traffic: Arc<Mutex<InfoTraffic>>,
    sent_bits: VecDeque<(u128, i128)>,
    received_bits: VecDeque<(u128, i128)>,
    sent_packets: VecDeque<(u128, i128)>,
    received_packets: VecDeque<(u128, i128)>,
    tot_sent_bits_prev: i128,
    tot_received_bits_prev: i128,
    tot_sent_packets_prev: i128,
    tot_received_packets_prev: i128,
    min_sent_bits: i128,
    max_received_bits: i128,
    min_sent_packets: i128,
    max_received_packets: i128,
    tick: u128
}


impl TrafficChart {
    pub fn new(info_traffic: Arc<Mutex<InfoTraffic>>) -> Self {
        TrafficChart {
            info_traffic,
            sent_bits: Default::default(),
            received_bits: Default::default(),
            sent_packets: Default::default(),
            received_packets: Default::default(),
            tot_sent_bits_prev: 0,
            tot_received_bits_prev: 0,
            tot_sent_packets_prev: 0,
            tot_received_packets_prev: 0,
            min_sent_bits: 0,
            max_received_bits: 0,
            min_sent_packets: 0,
            max_received_packets: 0,
            tick: 0
        }
    }
    
    fn view(&mut self, idx: usize) -> Element<Message> {

        let info_traffic_lock = self.info_traffic.lock().unwrap();
        let tot_received_bytes = info_traffic_lock.tot_received_bytes;
        let tot_sent_bytes = info_traffic_lock.tot_sent_bytes;
        drop(info_traffic_lock);

        let tot_seconds = self.tick;
        let interval = 1;
        self.tick += 1;

        // update bits traffic data
        self.sent_bits.push_back((tot_seconds as u128, (-1 * (tot_sent_bytes * 8) as i128 + self.tot_sent_bits_prev) / interval as i128));
        if -1 * (tot_sent_bytes * 8) as i128 + self.tot_sent_bits_prev < self.min_sent_bits {
            self.min_sent_bits = -1 * (tot_sent_bytes * 8) as i128 + self.tot_sent_bits_prev;
        }
        self.tot_sent_bits_prev = (tot_sent_bytes * 8) as i128;
        self.received_bits.push_back((tot_seconds as u128, (tot_received_bytes as i128 * 8 - self.tot_received_bits_prev) / interval as i128));
        if tot_received_bytes as i128 * 8 - self.tot_received_bits_prev > self.max_received_bits {
            self.max_received_bits = tot_received_bytes as i128 * 8 - self.tot_received_bits_prev;
        }
        self.tot_received_bits_prev = (tot_received_bytes * 8) as i128;

        Container::new(
            Column::new()
                .width(Length::Fill)
                .height(Length::Fill)
                .spacing(5)
                .push(iced::Text::new(format!("ciao {}", self.tick)))
                .push(
                    ChartWidget::new(self).height(Length::Fill)
                ),
        )
            /*            .style(style::ChartContainer)*/
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .into()
    }
}


impl Chart<Message> for TrafficChart {
    fn build_chart<DB: DrawingBackend>(&self, mut chart: ChartBuilder<DB>) {
        use plotters::{prelude::*, style::Color};

        let interval = 1;
        let tot_seconds = self.tick;

        // // update packets traffic data
        // self.sent_packets.push((tot_seconds as u128, (-(tot_sent_packets as i128) + tot_sent_packets_prev) / interval as i128));
        // if -(tot_sent_packets as i128) + tot_sent_packets_prev < min_sent_packets_second {
        //     min_sent_packets_second = -(tot_sent_packets as i128) + tot_sent_packets_prev;
        // }
        // tot_sent_packets_prev = tot_sent_packets as i128;
        // self.received_packets.push((tot_seconds as u128, (tot_received_packets as i128 - tot_received_packets_prev) / interval as i128));
        // if tot_received_packets as i128 - tot_received_packets_prev > max_received_packets_second {
        //     max_received_packets_second = tot_received_packets as i128 - tot_received_packets_prev;
        // }
        // tot_received_packets_prev = tot_received_packets as i128;


        // // declare drawing area
        // let (graphs_area, _) = root_area.split_horizontally(1255);
        // let (bits_area, packets_area) = graphs_area.split_vertically(360);
        // let (_, footer) = root_area.split_vertically(700);
        // footer.titled(
        //     &*format!("Charts are updated every {} seconds", interval),
        //     ("helvetica", 16).into_font().color(&BLACK.mix(0.5)),
        // ).expect("Error drawing graph");


        // bits graph

        let mut chart = chart
            .set_label_area_size(LabelAreaPosition::Left, 60)
            .set_label_area_size(LabelAreaPosition::Bottom, 50)
            .caption("Bit traffic per second", ("helvetica", 30))
            .build_cartesian_2d(0..tot_seconds as u128, self.min_sent_bits / interval as i128..self.max_received_bits / interval as i128)
            .expect("Error drawing graph");

        chart.configure_mesh()
            .y_desc("bit/s")
            .label_style(("helvetica", 16))
            .axis_desc_style(("helvetica", 16))
            // .x_label_formatter(&|seconds| {
            //     (time_origin + chrono::Duration::from_std(Duration::from_secs(*seconds as u64)).unwrap())
            //         .format("%H:%M:%S").to_string()
            // })
            .y_label_formatter(&|bits| {
                match bits {
                    0..=999 | -999..=-1 => { format!("{}", bits) }
                    1000..=999_999 | -999_999..=-1000 => { format!("{:.1} {}", *bits as f64 / 1_000_f64, "k") }
                    1_000_000..=999_999_999 | -999_999_999..=-1_000_000 => { format!("{:.1} {}", *bits as f64 / 1_000_000_f64, "M") }
                    _ => { format!("{:.1} {}", *bits as f64 / 1_000_000_000_f64, "G") }
                }
            })
            .draw().unwrap();
        chart.draw_series(
            AreaSeries::new(self.received_bits.iter().copied(), 0, GREEN_800.mix(0.2))
                .border_style(&GREEN_800))
            .expect("Error drawing graph")
            .label("Incoming bits")
            .legend(|(x, y)| Rectangle::new([(x, y - 5), (x + 25, y + 5)], GREEN_800.filled()));
        chart.draw_series(
            AreaSeries::new(self.sent_bits.iter().copied(), 0, BLUE.mix(0.2))
                .border_style(&BLUE))
            .expect("Error drawing graph")
            .label("Outgoing bits")
            .legend(|(x, y)| Rectangle::new([(x, y - 5), (x + 25, y + 5)], BLUE.filled()));
        chart.configure_series_labels().position(SeriesLabelPosition::UpperRight).margin(5)
            .border_style(BLACK).label_font(("helvetica", 16)).draw().expect("Error drawing graph");


        // // packets graph
        //
        // let mut chart_packets = ChartBuilder::on(&packets_area)
        //     .set_label_area_size(LabelAreaPosition::Left, 60)
        //     .set_label_area_size(LabelAreaPosition::Bottom, 50)
        //     .caption("Packet traffic per second", ("helvetica", 30))
        //     .build_cartesian_2d(0..tot_seconds as u128, min_sent_packets_second / interval as i128..max_received_packets_second / interval as i128)
        //     .expect("Error drawing graph");
        // chart_packets.configure_mesh()
        //     .y_desc("packet/s")
        //     .label_style(("helvetica", 16))
        //     .axis_desc_style(("helvetica", 16))
        //     .x_label_formatter(&|seconds| {
        //         (time_origin + chrono::Duration::from_std(Duration::from_secs(*seconds as u64)).unwrap())
        //             .format("%H:%M:%S").to_string()
        //     })
        //     .draw().unwrap();
        // chart_packets.draw_series(
        //     AreaSeries::new(received_packets_graph.iter().copied(), 0, GREEN_800.mix(0.2))
        //         .border_style(&GREEN_800))
        //     .expect("Error drawing graph")
        //     .label("Incoming packets")
        //     .legend(|(x, y)| Rectangle::new([(x, y - 5), (x + 25, y + 5)], GREEN_800.filled()));
        // chart_packets.draw_series(
        //     AreaSeries::new(sent_packets_graph.iter().copied(), 0, BLUE.mix(0.2))
        //         .border_style(&BLUE))
        //     .expect("Error drawing graph")
        //     .label("Outgoing packets")
        //     .legend(|(x, y)| Rectangle::new([(x, y - 5), (x + 25, y + 5)], BLUE.filled()));
        // chart_packets.configure_series_labels().position(SeriesLabelPosition::UpperRight).margin(5)
        //     .border_style(BLACK).label_font(("helvetica", 16)).draw().expect("Error drawing graph");
    }
}