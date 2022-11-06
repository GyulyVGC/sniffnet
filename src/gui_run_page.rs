use std::cmp::{max, min};
use std::sync::{Arc, Mutex};
use iced::{alignment, Alignment, Button, Column, Container, Element, Length, Radio, Row, Scrollable, Text};
use iced::alignment::{Horizontal, Vertical};
use iced::Length::FillPortion;
use plotters::style::RGBColor;
use thousands::Separable;
use crate::app::Message;
use crate::{AppProtocol, ChartsData, Filters, FONT_SIZE_SUBTITLE, get_app_count_string, icon_sun_moon, Mode, Sniffer, TransProtocol};
use crate::address_port_pair::AddressPortPair;
use crate::info_address_port_pair::{get_formatted_bytes_string, InfoAddressPortPair};
use crate::style::{CHARTS_LINE_BORDER, COLOR_CHART_MIX_DAY, COLOR_CHART_MIX_NIGHT, COURIER_PRIME, COURIER_PRIME_BOLD, COURIER_PRIME_BOLD_ITALIC, COURIER_PRIME_ITALIC, FONT_SIZE_FOOTER, HEIGHT_BODY, HEIGHT_FOOTER, HEIGHT_HEADER, icon, logo_glyph, NOTOSANS, NOTOSANS_BOLD, SPECIAL_DAY_RGB, SPECIAL_NIGHT_RGB};
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
    let observed_bytes = sniffer_lock.all_bytes;
    let filtered_bytes = sniffer_lock.tot_sent_bytes + sniffer_lock.tot_received_bytes;
    drop(sniffer_lock);
    let filtered_bytes_string = get_formatted_bytes_string(filtered_bytes);

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
            let filters_string = get_active_filters_string(sniffer.filters.clone());

            let percentage_string_packets =
                if format!("{:.1}", 100.0*(filtered) as f32/observed as f32).eq("0.0") {
                    "<0.1%".to_string()
                }
                else {
                    format!("{:.1}%", 100.0*(filtered) as f32/observed as f32)
                };

            let percentage_string_bytes =
                if format!("{:.1}", 100.0*(filtered_bytes) as f32/observed_bytes as f32).eq("0.0") {
                    "<0.1%".to_string()
                }
                else {
                    format!("{:.1}%", 100.0*(filtered_bytes) as f32/observed_bytes as f32)
                };

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
            let col_packets = Column::new()
                .width(Length::FillPortion(1))
                .padding(10)
                .spacing(15)
                //.push(iced::Text::new(std::env::current_dir().unwrap().to_str().unwrap()).font(font))
                .push(Text::new(filters_string).font(font))
                .push(Text::new(format!("Filtered packets:\n   {} ({} of the total)",
                                            filtered.separate_with_spaces(), percentage_string_packets)).font(font))
                .push(Text::new(format!("Filtered bytes:\n   {} ({} of the total)",
                                        filtered_bytes_string, percentage_string_bytes)).font(font))
                .push(Scrollable::new(&mut sniffer.scroll_packets).style(sniffer.style)
                          .push(iced::Text::new(format!("Filtered packets per application protocol:\n{}",get_app_count_string(sniffer_lock.app_protocols.clone(), filtered))).font(font)));

            let active_radio_report = &*sniffer.report_type;
            let row_radio_report = Row::new().padding(10)
                .push(Text::new("Relevant connections:    ").size(FONT_SIZE_SUBTITLE).font(font))
                .push(Radio::new(
                    "latest",
                    "most recent",
                    Some(active_radio_report),
                    |what_to_display| Message::ReportSelection(what_to_display.to_string()),
                )
                    .width(Length::Units(200))
                    .font(font).size(15).style(sniffer.style))
                .push(Radio::new(
                    "packets",
                    "most packets",
                    Some(active_radio_report),
                    |what_to_display| Message::ReportSelection(what_to_display.to_string()),
                )
                    .width(Length::Units(200))
                    .font(font).size(15).style(sniffer.style))
                .push(Radio::new(
                    "bytes",
                    "most bytes",
                    Some(active_radio_report),
                    |what_to_display| Message::ReportSelection(what_to_display.to_string()),
                )
                    .width(Length::Units(200))
                    .font(font).size(15).style(sniffer.style))
                ;

            let mut sorted_vec: Vec<(&AddressPortPair, &InfoAddressPortPair)> = sniffer_lock.map.iter().collect();
            match active_radio_report {
                "latest" => {
                    sorted_vec.sort_by(|&(_, a), &(_, b)|
                        b.final_timestamp.cmp(&a.final_timestamp));
                }
                "packets" => {
                    sorted_vec.sort_by(|&(_, a), &(_, b)|
                        b.transmitted_packets.cmp(&a.transmitted_packets));
                }
                "bytes" => {
                    sorted_vec.sort_by(|&(_, a), &(_, b)|
                        b.transmitted_bytes.cmp(&a.transmitted_bytes));
                }
                _ => {}
            }
            let n_entry = min(sorted_vec.len(), 10);
            let mut col_report = Column::new()
                .height(Length::Fill)
                .push(row_radio_report)
                //.push(iced::Text::new("---------------------------------------------------------------------------------------------------------------").font(font))
                .push(Text::new(" "))
                .push(iced::Text::new("     Src IP address       Src port      Dst IP address       Dst port  Layer 4  Layer 7    Packets      Bytes  ").font(font))
                .push(iced::Text::new("---------------------------------------------------------------------------------------------------------------").font(font))
                ;
            let mut scroll_report = Scrollable::new(&mut sniffer.scroll_report).style(sniffer.style);
            for i in 0..n_entry {
                let key_val = sorted_vec.get(i).unwrap();
                scroll_report = scroll_report.push(iced::Text::new(format!("{}{}", key_val.0.print_gui(), key_val.1.print_gui())).font(font));
            }
            col_report = col_report.push(scroll_report);
            drop(sniffer_lock);
            let col_open_report = Container::new(button_report)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center);
            let row_report = Row::new()
                .spacing(10)
                .height(Length::FillPortion(2))
                .width(Length::Fill)
                .align_items(Alignment::Start)
                .push(Container::new(col_report)
                    .padding(10)
                    .height(Length::Fill)
                    .style(Mode::BorderedRound))
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


fn get_active_filters_string(filters: Arc<Mutex<Filters>>) -> String {
    let filters_lock = filters.lock().unwrap();
    if filters_lock.ip == "no filter"
        && filters_lock.application.eq(&AppProtocol::Other)
        && filters_lock.transport.eq(&TransProtocol::Other) {
        "Active filters:\n   none".to_string()
    }
    else {
        let mut ret_val = "Active filters:".to_string();
        if filters_lock.ip != "no filter" {
            ret_val.push_str(&*format!("\n   {}", filters_lock.ip.replace('v', "V")));
        }
        if filters_lock.transport.ne(&TransProtocol::Other) {
            ret_val.push_str(&*format!("\n   {}", filters_lock.transport));
        }
        if filters_lock.application.ne(&AppProtocol::Other) {
            ret_val.push_str(&*format!("\n   {}", filters_lock.application));
        }
        ret_val
    }
}


pub struct TrafficChart {
    charts_data: Arc<Mutex<ChartsData>>,
    color_mix: f64,
    font_color: RGBColor,
    chart_packets: bool,
}


impl TrafficChart {
    pub fn new(charts_data: Arc<Mutex<ChartsData>>) -> Self {
        TrafficChart {
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
                .push(ChartWidget::new(self).resolve_font(
                    move |_, _| match mode {
                        Mode::Night => {NOTOSANS}
                        Mode::Day => {NOTOSANS_BOLD}
                        _ => {NOTOSANS}
                    }
                )))
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
                    .label_style(("notosans", 15).into_font().color(&self.font_color))
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
                    .border_style(BLACK).label_font(("notosans", 15).into_font().color(&self.font_color)).draw().expect("Error drawing graph");

            }

            true => { //display packets chart
                let mut chart = chart.margin_right(30)
                    .set_label_area_size(LabelAreaPosition::Left, 60)
                    .set_label_area_size(LabelAreaPosition::Bottom, 50)
                    .build_cartesian_2d(first_time_displayed..tot_seconds as u128, charts_data_lock.min_sent_packets..charts_data_lock.max_received_packets)
                    .expect("Error drawing graph");

                chart.configure_mesh()
                    .label_style(("notosans", 15).into_font().color(&self.font_color))
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
                    .border_style(BLACK).label_font(("notosans", 15).into_font().color(&self.font_color)).draw().expect("Error drawing graph");

            }
        }
    }
}