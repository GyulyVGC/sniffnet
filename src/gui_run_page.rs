use std::cmp::{max, min};
use std::sync::{Arc, Mutex};
use iced::{alignment, Alignment, Button, Column, Container, Element, Length, Radio, Row, Scrollable, Text};
use iced::alignment::{Horizontal, Vertical};
use iced::Length::FillPortion;
use plotters::prelude::IntoFont;
use plotters::style::RGBColor;
use thousands::Separable;
use crate::app::Message;
use crate::{ChartsData, FONT_SIZE_TITLE, get_app_count_string, icon_sun_moon, InfoTraffic, Mode, Sniffer};
use crate::address_port_pair::AddressPortPair;
use crate::info_address_port_pair::InfoAddressPortPair;
use crate::style::{CHARTS_LINE_BORDER, COLOR_CHART_MIX_DAY, COLOR_CHART_MIX_NIGHT, COURIER_PRIME, COURIER_PRIME_BOLD, COURIER_PRIME_BOLD_ITALIC, COURIER_PRIME_ITALIC, FONT_SIZE_FOOTER, HEIGHT_BODY, HEIGHT_FOOTER, HEIGHT_HEADER, icon, logo_glyph, SPECIAL_DAY_RGB, SPECIAL_NIGHT_RGB};
use plotters_iced::{Chart, ChartWidget, DrawingBackend, ChartBuilder};

pub fn run_page(sniffer: &mut Sniffer) -> Column<Message> {
    let font = if sniffer.style == Mode::Day { COURIER_PRIME_BOLD } else { COURIER_PRIME };
    let font_footer = if sniffer.style == Mode::Day { COURIER_PRIME_ITALIC } else { COURIER_PRIME_BOLD_ITALIC };
    let headers_style = if sniffer.style == Mode::Day { Mode::HeadersDay } else { Mode::HeadersNight };
    let logo = logo_glyph().size(100);

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

    let header = Container::new(Row::new()
        .height(Length::Fill)
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .push(Container::new(button_reset).width(Length::FillPortion(1)).width(Length::FillPortion(1)).align_x(Horizontal::Center))
        .push(Container::new(Row::new().align_items(Alignment::Center).push(logo)).width(Length::FillPortion(6)).height(Length::Fill).align_x(Horizontal::Center).align_y(Vertical::Center))
        .push(Container::new(button_style).width(Length::FillPortion(1)).align_x(Horizontal::Center)))
        .height(Length::FillPortion(HEIGHT_HEADER))
        .width(Length::Fill)
        .style(headers_style);

    let button_report = Button::new(
        &mut sniffer.report,
        Text::new("Open full report").font(font)
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
        .padding(10)
        .spacing(10)
        .align_items(Alignment::Center);

    match (observed, filtered) {
        (0, 0) => { //no packets observed at all
            if sniffer.waiting.len() > 4 {
                sniffer.waiting = "".to_string();
            }
            sniffer.waiting = ".".repeat(sniffer.waiting.len() + 1);
            let adapter_name = &*sniffer.device.clone().lock().unwrap().name.clone();
            let nothing_to_see_text = if sniffer.device.lock().unwrap().addresses.len() > 0 {
                Text::new(format!("No traffic has been observed yet. Waiting for network packets...\n\n\
                                                              Network adapter: {}\n\n\
                                                              Are you sure you are connected to the internet and you have selected the right adapter?", adapter_name)).font(font)
            }
            else {
                Text::new(format!("No traffic can be observed because the adapter you selected has no active addresses...\n\n\
                                                              Network adapter: {}\n\n\
                                                              If you are sure you are connected to the internet, try choosing a different adapter.", adapter_name)).font(font)
            };
            body = body
                .push(Row::new().height(Length::FillPortion(1)))
                .push(Text::new(sniffer.waiting.clone()).font(font).size(50))
                .push(nothing_to_see_text)
                .push(Text::new(sniffer.waiting.clone()).font(font).size(50))
                .push(Row::new().height(Length::FillPortion(2)));
        }

        (_observed, 0) => { //no packets have been filtered but some have been observed
            if sniffer.waiting.len() > 4 {
                sniffer.waiting = "".to_string();
            }
            sniffer.waiting = ".".repeat(sniffer.waiting.len() + 1);

            let tot_packets_text = Text::new(format!("Total intercepted packets: {}\n\n\
                                                    Filtered packets: 0\n\n\
                                                    Some packets have been intercepted, but still none has been selected according to the filters you specified...",
                                                     observed.separate_with_spaces())).font(font);

            body = body
                .push(Row::new().height(Length::FillPortion(1)))
                .push(Text::new(sniffer.waiting.clone()).font(font).size(50))
                .push(tot_packets_text)
                .push(Text::new(sniffer.waiting.clone()).font(font).size(50))
                .push(Row::new().height(Length::FillPortion(2)));
        }

        (observed, filtered) => { //observed > filtered > 0 || observed = filtered > 0

            let active_radio_chart = if sniffer.chart_packets { "packets" } else { "bits" };
            let row_radio_chart = Row::new().padding(10).spacing(10)
                .push(Column::new().width(Length::FillPortion(1)))
                .push(Radio::new(
                    "packets",
                    "packets per second",
                    Some(active_radio_chart),
                    |what_to_display| Message::ChartSelection(what_to_display.to_string()),
                ).width(Length::FillPortion(2)).font(font).size(15).style(sniffer.style))
                .push(Radio::new(
                    "bits",
                    "bits per second",
                    Some(active_radio_chart),
                    |what_to_display| Message::ChartSelection(what_to_display.to_string()),
                ).width(Length::FillPortion(2)).font(font).size(15).style(sniffer.style))
                .push(Column::new().width(Length::FillPortion(1)));

            let col_chart = Container::new(
                Column::new()
                    .push(row_radio_chart)
                    .push(sniffer.traffic_chart.view(sniffer.style, sniffer.chart_packets)))
                .width(Length::FillPortion(2))
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
                .style(Mode::BorderedRound);

            sniffer_lock = sniffer.info_traffic.lock().unwrap();
            let mut col_packets = Column::new()
                .width(Length::FillPortion(1))
                .padding(10)
                .spacing(15)
                .push(Scrollable::new(&mut sniffer.scroll_packets)
                    .push(iced::Text::new(std::env::current_dir().unwrap().to_str().unwrap()).font(font))
                    .push(Text::new(format!("Total intercepted packets: {}",
                                            observed.separate_with_spaces())).font(font))
                    .push(Text::new(format!("Filtered packets: {}",
                                            filtered.separate_with_spaces())).font(font))
                );
            col_packets = col_packets
                .push(iced::Text::new(format!("Filtered packets per application protocol:\n{}",get_app_count_string(sniffer_lock.app_protocols.clone(), filtered))).font(font));

            let mut row_report = Row::new()
                .height(Length::FillPortion(2))
                .align_items(Alignment::Center);
            let mut sorted_vec: Vec<(&AddressPortPair, &InfoAddressPortPair)> = sniffer_lock.map.iter().collect();
            sorted_vec.sort_by(|&(_, a), &(_, b)|
                b.final_timestamp.cmp(&a.final_timestamp));
            let n_entry = min(sorted_vec.len(), 9);
            let mut col_report = Column::new()
                .height(Length::Fill);
            col_report = col_report
                .push(iced::Text::new("Latest connections\n").font(font).size(FONT_SIZE_TITLE))
                .push(iced::Text::new("-------------------------------------------------------------------------------------------------------------------------").font(font))
                .push(iced::Text::new("|     Src IP address      | Src port |     Dst IP address      | Dst port | Layer 4 | Layer 7 |   Packets  |   Bytes    |").font(font))
                .push(iced::Text::new("-------------------------------------------------------------------------------------------------------------------------").font(font));
            for i in 0..n_entry {
                let key_val = sorted_vec.get(i).unwrap();
                col_report = col_report.push(iced::Text::new(format!("{}{}", key_val.0, key_val.1.print_without_timestamps())).font(font));
            }
            drop(sniffer_lock);
            col_report = col_report
                .push(iced::Text::new("-------------------------------------------------------------------------------------------------------------------------").font(font));
            let col_open_report = Column::new()
                .push(button_report);
            row_report = row_report
                .push(col_report)
                .push(col_open_report);

            body = body
                .push(Row::new().spacing(10).height(Length::FillPortion(3))
                    .push(col_chart)
                    .push(Container::new(col_packets).padding(10).height(Length::Fill).style(Mode::BorderedRound)))
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
        .push(Text::new("Sniffnet v1.0.0 - by Giuliano Bellini ").font(font_footer).size(FONT_SIZE_FOOTER))
        .push(button_github)
        .push(Text::new("  ").font(font));
    let footer = Container::new(footer_row)
        .width(Length::Fill)
        .height(FillPortion(HEIGHT_FOOTER))
        .align_y(Vertical::Center)
        .align_x(Horizontal::Center)
        .style(headers_style);

    Column::new()
        .spacing(10)
        .push(header)
        .push(body)
        .push(footer)
}


pub struct TrafficChart {
    info_traffic: Arc<Mutex<InfoTraffic>>,
    charts_data: Arc<Mutex<ChartsData>>,
    color_mix: f64,
    font_color: RGBColor,
    chart_packets: bool,
}


impl TrafficChart {
    pub fn new(info_traffic: Arc<Mutex<InfoTraffic>>, charts_data: Arc<Mutex<ChartsData>>) -> Self {
        TrafficChart {
            info_traffic,
            charts_data,
            color_mix: 0.0,
            font_color: Default::default(),
            chart_packets: true,
        }
    }
    
    fn view(&mut self, mode: Mode, chart_packets: bool) -> Element<Message> {

        self.color_mix = if mode == Mode::Day {COLOR_CHART_MIX_DAY} else { COLOR_CHART_MIX_NIGHT };
        self.chart_packets = chart_packets;
        self.font_color = if mode == Mode::Day { plotters::style::colors::BLACK } else { plotters::style::colors::WHITE };

        Container::new(
            Column::new()
                .push(ChartWidget::new(self)))
            .align_x(Horizontal::Left)
            .align_y(Vertical::Bottom)
            .into()
    }
}


impl Chart<Message> for TrafficChart {
    fn build_chart<DB: DrawingBackend>(&self, mut chart: ChartBuilder<DB>) {
        use plotters::{prelude::*, style::Color};

        let charts_data_lock = self.charts_data.lock().unwrap();

        if charts_data_lock.ticks == 0 {
            return
        }
        let tot_seconds = charts_data_lock.ticks - 1;
        let first_time_displayed = max(0, charts_data_lock.ticks as i128 - 30) as u128;

        match self.chart_packets {
            false => { //display bits chart
                let mut chart = chart.margin_right(30)
                    .set_label_area_size(LabelAreaPosition::Left, 60)
                    .set_label_area_size(LabelAreaPosition::Bottom, 50)
                    .build_cartesian_2d(first_time_displayed..tot_seconds as u128, charts_data_lock.min_sent_bits..charts_data_lock.max_received_bits)
                    .expect("Error drawing graph");

                chart.configure_mesh()
                    .label_style(("helvetica", 13).into_font().color(&self.font_color))
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
                    AreaSeries::new(charts_data_lock.received_bits.iter().copied(), 0, SPECIAL_NIGHT_RGB.mix(self.color_mix))
                        .border_style(ShapeStyle::from(&SPECIAL_NIGHT_RGB).stroke_width(CHARTS_LINE_BORDER)))
                    .expect("Error drawing graph")
                    .label("Incoming bits")
                    .legend(|(x, y)| Rectangle::new([(x, y - 5), (x + 25, y + 5)], SPECIAL_NIGHT_RGB.filled()));
                chart.draw_series(
                    AreaSeries::new(charts_data_lock.sent_bits.iter().copied(), 0, SPECIAL_DAY_RGB.mix(self.color_mix))
                        .border_style(ShapeStyle::from(&SPECIAL_DAY_RGB).stroke_width(CHARTS_LINE_BORDER)))
                    .expect("Error drawing graph")
                    .label("Outgoing bits")
                    .legend(|(x, y)| Rectangle::new([(x, y - 5), (x + 25, y + 5)], SPECIAL_DAY_RGB.filled()));
                chart.configure_series_labels().position(SeriesLabelPosition::UpperRight)//.margin(5)
                    .border_style(BLACK).label_font(("helvetica", 13).into_font().color(&self.font_color)).draw().expect("Error drawing graph");

            }

            true => { //display packets chart
                let mut chart = chart.margin_right(30)
                    .set_label_area_size(LabelAreaPosition::Left, 60)
                    .set_label_area_size(LabelAreaPosition::Bottom, 50)
                    .build_cartesian_2d(first_time_displayed..tot_seconds as u128, charts_data_lock.min_sent_packets..charts_data_lock.max_received_packets)
                    .expect("Error drawing graph");

                chart.configure_mesh()
                    .label_style(("helvetica", 13).into_font().color(&self.font_color))
                    // .x_label_formatter(&|seconds| {
                    //     (time_origin + chrono::Duration::from_std(Duration::from_secs(*seconds as u64)).unwrap())
                    //         .format("%H:%M:%S").to_string()
                    // })
                    .draw().unwrap();
                chart.draw_series(
                    AreaSeries::new(charts_data_lock.received_packets.iter().copied(), 0, SPECIAL_NIGHT_RGB.mix(self.color_mix))
                        .border_style(ShapeStyle::from(&SPECIAL_NIGHT_RGB).stroke_width(CHARTS_LINE_BORDER)))
                    .expect("Error drawing graph")
                    .label("Incoming packets")
                    .legend(|(x, y)| Rectangle::new([(x, y - 5), (x + 25, y + 5)], SPECIAL_NIGHT_RGB.filled()));
                chart.draw_series(
                    AreaSeries::new(charts_data_lock.sent_packets.iter().copied(), 0, SPECIAL_DAY_RGB.mix(self.color_mix))
                        .border_style(ShapeStyle::from(&SPECIAL_DAY_RGB).stroke_width(CHARTS_LINE_BORDER)))
                    .expect("Error drawing graph")
                    .label("Outgoing packets")
                    .legend(|(x, y)| Rectangle::new([(x, y - 5), (x + 25, y + 5)], SPECIAL_DAY_RGB.filled()));
                chart.configure_series_labels().position(SeriesLabelPosition::UpperRight)//.margin(5)
                    .border_style(BLACK).label_font(("helvetica", 13).into_font().color(&self.font_color)).draw().expect("Error drawing graph");

            }
        }
    }
}