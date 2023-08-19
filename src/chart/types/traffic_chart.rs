//! This module defines the behavior of the `TrafficChart` struct, used to display chart in GUI run page

use std::collections::VecDeque;

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Column, Container};
use iced::{Element, Renderer};
use plotters::prelude::*;
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};

use crate::gui::styles::style_constants::{
    get_alpha_chart_badge, get_font_weight, CHARTS_LINE_BORDER,
};
use crate::gui::styles::types::palette::to_rgb_color;
use crate::gui::types::message::Message;
use crate::translations::translations::{incoming_translation, outgoing_translation};
use crate::utils::formatted_strings::get_formatted_bytes_string_with_b;
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
    /// Language used for the chart legend
    pub language: Language,
    /// Packets or bytes
    pub chart_type: ChartType,
    /// Style of the chart
    pub style: StyleType,
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
            language,
            chart_type: ChartType::Bytes,
            style,
        }
    }

    pub fn view(&self) -> Element<Message, Renderer<StyleType>> {
        Container::new(Column::new().push(ChartWidget::new(self)))
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

    pub fn change_style(&mut self, style: StyleType) {
        self.style = style;
    }
}

impl Chart<Message> for TrafficChart {
    type State = ();

    fn build_chart<DB: DrawingBackend>(
        &self,
        _state: &Self::State,
        mut chart_builder: ChartBuilder<DB>,
    ) {
        let font_weight = get_font_weight(self.style);

        if self.ticks == 0 {
            return;
        }
        let tot_seconds = self.ticks - 1;
        let first_time_displayed = if self.ticks > 30 { self.ticks - 30 } else { 0 };

        let colors = get_colors(self.style);
        let color_incoming = to_rgb_color(colors.secondary);
        let color_outgoing = to_rgb_color(colors.outgoing);
        let color_font = to_rgb_color(colors.text_body);
        let color_mix = get_alpha_chart_badge(self.style);

        chart_builder
            .margin_right(30)
            .margin_bottom(0)
            .set_label_area_size(LabelAreaPosition::Left, 60)
            .set_label_area_size(LabelAreaPosition::Bottom, 50);

        let mut chart = chart_builder
            .build_cartesian_2d(
                first_time_displayed..tot_seconds,
                if self.chart_type.eq(&ChartType::Packets) {
                    self.min_sent_packets..self.max_received_packets
                } else {
                    self.min_sent_bytes..self.max_received_bytes
                },
            )
            .expect("Error drawing packets chart");

        // Mesh
        chart
            .configure_mesh()
            .label_style(
                ("Sarasa Mono SC", 12)
                    .into_font()
                    .style(font_weight)
                    .color(&color_font),
            )
            .y_labels(7)
            .y_label_formatter(if self.chart_type.eq(&ChartType::Packets) {
                &|packets| packets.abs().to_string()
            } else {
                &|bytes| get_formatted_bytes_string_with_b(u128::from(bytes.unsigned_abs()))
            })
            .draw()
            .unwrap();

        // Incoming series
        chart
            .draw_series(
                AreaSeries::new(
                    if self.chart_type.eq(&ChartType::Packets) {
                        self.received_packets.iter().copied()
                    } else {
                        self.received_bytes.iter().copied()
                    },
                    0,
                    color_incoming.mix(color_mix.into()),
                )
                .border_style(ShapeStyle::from(&color_incoming).stroke_width(CHARTS_LINE_BORDER)),
            )
            .expect("Error drawing graph")
            .label(incoming_translation(self.language))
            .legend(move |(x, y)| {
                Rectangle::new([(x, y - 5), (x + 25, y + 5)], color_incoming.filled())
            });

        // Outgoing series
        chart
            .draw_series(
                AreaSeries::new(
                    if self.chart_type.eq(&ChartType::Packets) {
                        self.sent_packets.iter().copied()
                    } else {
                        self.sent_bytes.iter().copied()
                    },
                    0,
                    color_outgoing.mix(color_mix.into()),
                )
                .border_style(ShapeStyle::from(&color_outgoing).stroke_width(CHARTS_LINE_BORDER)),
            )
            .expect("Error drawing graph")
            .label(outgoing_translation(self.language))
            .legend(move |(x, y)| {
                Rectangle::new([(x, y - 5), (x + 25, y + 5)], color_outgoing.filled())
            });

        // Legend
        chart
            .configure_series_labels()
            .position(SeriesLabelPosition::UpperRight)
            .background_style(BLACK.mix(0.3))
            .border_style(BLACK.mix(0.6))
            .label_font(
                ("Sarasa Mono SC", 13.5)
                    .into_font()
                    .style(font_weight)
                    .color(&color_font),
            )
            .draw()
            .expect("Error drawing graph");
    }
}
