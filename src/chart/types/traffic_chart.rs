//! This module defines the behavior of the `TrafficChart` struct, used to display chart in GUI run page

use std::collections::VecDeque;

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Column, Container};
use iced::{Element, Font};
use plotters::style::RGBColor;
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};

use crate::gui::styles::style_constants::{
    get_color_mix_chart, CHARTS_LINE_BORDER, SARASA_MONO_SC_BOLD,
};
use crate::gui::styles::types::palette::to_rgb_color;
use crate::gui::types::message::Message;
use crate::translations::translations::{incoming_translation, outgoing_translation};
use crate::utils::formatted_strings::get_formatted_bytes_string;
use crate::{get_colors, ChartType, Language, StyleType};

/// Struct defining the chart to be displayed in gui run page
pub struct TrafficChart {
    /// Current time interval number
    pub ticks: u32,
    /// Sent bytes filtered and their time occurrence
    pub sent_bytes: VecDeque<(u32, i64)>,
    /// Received bytes filtered and their time occurrence
    pub received_bytes: VecDeque<(u32, i64)>,
    /// Sent packets filtered and their time occurrence
    pub sent_packets: VecDeque<(u32, i64)>,
    /// Received packets filtered and their time occurrence
    pub received_packets: VecDeque<(u32, i64)>,
    /// Minimum number of sent bytes per time interval (computed on last 30 intervals)
    pub min_sent_bytes: i64,
    /// Minimum number of received bytes per time interval (computed on last 30 intervals)
    pub max_received_bytes: i64,
    /// Minimum number of sent packets per time interval (computed on last 30 intervals)
    pub min_sent_packets: i64,
    /// Minimum number of received packets per time interval (computed on last 30 intervals)
    pub max_received_packets: i64,
    pub color_mix: f64,
    pub color_incoming: RGBColor,
    pub color_outgoing: RGBColor,
    pub color_font: RGBColor,
    pub chart_type: ChartType,
    pub language: Language,
}

impl TrafficChart {
    pub fn new(style: StyleType, language: Language) -> Self {
        TrafficChart {
            ticks: 0,
            sent_bytes: VecDeque::default(),
            received_bytes: VecDeque::default(),
            sent_packets: VecDeque::default(),
            received_packets: VecDeque::default(),
            min_sent_bytes: 0,
            max_received_bytes: 0,
            min_sent_packets: 0,
            max_received_packets: 0,
            color_mix: get_color_mix_chart(style),
            color_incoming: to_rgb_color(get_colors(style).incoming),
            color_outgoing: to_rgb_color(get_colors(style).outgoing),
            color_font: to_rgb_color(get_colors(style).text_body),
            chart_type: ChartType::Bytes,
            language,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let color_font = self.color_font;
        Container::new(
            Column::new().push(
                ChartWidget::new(self).resolve_font(move |_, _| match color_font {
                    RGBColor(255, 255, 255) => Font::Default, // if white non-bold
                    _ => SARASA_MONO_SC_BOLD,
                }),
            ),
        )
        .align_x(Horizontal::Left)
        .align_y(Vertical::Bottom)
        .into()
    }

    pub fn change_kind(&mut self, kind: ChartType) {
        self.chart_type = kind;
    }

    pub fn change_language(&mut self, language: Language) {
        self.language = language;
    }

    pub fn change_colors(&mut self, style: StyleType) {
        self.color_font = to_rgb_color(get_colors(style).text_body);
        self.color_incoming = to_rgb_color(get_colors(style).incoming);
        self.color_outgoing = to_rgb_color(get_colors(style).outgoing);
        self.color_mix = get_color_mix_chart(style);
    }
}

impl Chart<Message> for TrafficChart {
    type State = ();

    fn build_chart<DB: DrawingBackend>(
        &self,
        _state: &Self::State,
        mut chart_builder: ChartBuilder<DB>,
    ) {
        use plotters::prelude::*;

        if self.ticks == 0 {
            return;
        }
        let tot_seconds = self.ticks - 1;
        let first_time_displayed = if self.ticks > 30 { self.ticks - 30 } else { 0 };

        let color_incoming = self.color_incoming;
        let color_outgoing = self.color_outgoing;

        chart_builder
            .margin_right(30)
            .set_label_area_size(LabelAreaPosition::Left, 60)
            .set_label_area_size(LabelAreaPosition::Bottom, 50);

        match self.chart_type {
            ChartType::Bytes => {
                //display bytes chart
                let mut chart = chart_builder
                    .build_cartesian_2d(
                        first_time_displayed..tot_seconds,
                        self.min_sent_bytes..self.max_received_bytes,
                    )
                    .expect("Error drawing bytes chart");

                chart
                    .configure_mesh()
                    .label_style(("notosans", 15).into_font().color(&self.color_font))
                    .y_labels(7)
                    .y_label_formatter(&|bytes| {
                        get_formatted_bytes_string(u128::from(bytes.unsigned_abs()))
                            .trim()
                            .to_string()
                    })
                    .draw()
                    .unwrap();
                chart
                    .draw_series(
                        AreaSeries::new(
                            self.received_bytes.iter().copied(),
                            0,
                            color_incoming.mix(self.color_mix),
                        )
                        .border_style(
                            ShapeStyle::from(&color_incoming).stroke_width(CHARTS_LINE_BORDER),
                        ),
                    )
                    .expect("Error drawing graph")
                    .label(incoming_translation(self.language))
                    .legend(move |(x, y)| {
                        Rectangle::new([(x, y - 5), (x + 25, y + 5)], color_incoming.filled())
                    });
                chart
                    .draw_series(
                        AreaSeries::new(
                            self.sent_bytes.iter().copied(),
                            0,
                            color_outgoing.mix(self.color_mix),
                        )
                        .border_style(
                            ShapeStyle::from(&color_outgoing).stroke_width(CHARTS_LINE_BORDER),
                        ),
                    )
                    .expect("Error drawing graph")
                    .label(outgoing_translation(self.language))
                    .legend(move |(x, y)| {
                        Rectangle::new([(x, y - 5), (x + 25, y + 5)], color_outgoing.filled())
                    });
                chart
                    .configure_series_labels()
                    .position(SeriesLabelPosition::UpperRight)
                    .border_style(BLACK)
                    .label_font(("notosans", 17).into_font().color(&self.color_font))
                    .draw()
                    .expect("Error drawing graph");
            }

            ChartType::Packets => {
                //display packets chart
                let mut chart = chart_builder
                    .build_cartesian_2d(
                        first_time_displayed..tot_seconds,
                        self.min_sent_packets..self.max_received_packets,
                    )
                    .expect("Error drawing packets chart");

                chart
                    .configure_mesh()
                    .label_style(("notosans", 15).into_font().color(&self.color_font))
                    .y_labels(7)
                    .y_label_formatter(&|packets| packets.abs().to_string())
                    .draw()
                    .unwrap();
                chart
                    .draw_series(
                        AreaSeries::new(
                            self.received_packets.iter().copied(),
                            0,
                            color_incoming.mix(self.color_mix),
                        )
                        .border_style(
                            ShapeStyle::from(&color_incoming).stroke_width(CHARTS_LINE_BORDER),
                        ),
                    )
                    .expect("Error drawing graph")
                    .label(incoming_translation(self.language))
                    .legend(move |(x, y)| {
                        Rectangle::new([(x, y - 5), (x + 25, y + 5)], color_incoming.filled())
                    });
                chart
                    .draw_series(
                        AreaSeries::new(
                            self.sent_packets.iter().copied(),
                            0,
                            color_outgoing.mix(self.color_mix),
                        )
                        .border_style(
                            ShapeStyle::from(&color_outgoing).stroke_width(CHARTS_LINE_BORDER),
                        ),
                    )
                    .expect("Error drawing graph")
                    .label(outgoing_translation(self.language))
                    .legend(move |(x, y)| {
                        Rectangle::new([(x, y - 5), (x + 25, y + 5)], color_outgoing.filled())
                    });
                chart
                    .configure_series_labels()
                    .position(SeriesLabelPosition::UpperRight)
                    .border_style(BLACK)
                    .label_font(("notosans", 17).into_font().color(&self.color_font))
                    .draw()
                    .expect("Error drawing graph");
            }
        }
    }
}
