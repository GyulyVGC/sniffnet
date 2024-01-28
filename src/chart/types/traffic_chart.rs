//! This module defines the behavior of the `TrafficChart` struct, used to display chart in GUI run page

use std::collections::VecDeque;

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Column, Container};
use iced::{Element, Renderer};
use plotters::prelude::*;
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};

use crate::gui::styles::style_constants::CHARTS_LINE_BORDER;
use crate::gui::styles::types::palette::to_rgb_color;
use crate::gui::types::message::Message;
use crate::translations::translations::{incoming_translation, outgoing_translation};
use crate::utils::formatted_strings::get_formatted_bytes_string_with_b;
use crate::{ChartType, Language, StyleType};

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
    /// Minimum number of bytes per time interval (computed on last 30 intervals)
    pub min_bytes: i64,
    /// Maximum number of bytes per time interval (computed on last 30 intervals)
    pub max_bytes: i64,
    /// Minimum number of packets per time interval (computed on last 30 intervals)
    pub min_packets: i64,
    /// Maximum number of packets per time interval (computed on last 30 intervals)
    pub max_packets: i64,
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
            min_bytes: 0,
            max_bytes: 0,
            min_packets: 0,
            max_packets: 0,
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
        let font_weight = self.style.get_font_weight();

        if self.ticks == 0 {
            return;
        }
        let tot_seconds = self.ticks - 1;
        let first_time_displayed = if self.ticks > 30 { self.ticks - 30 } else { 0 };

        let colors = self.style.get_palette();
        let ext = self.style.get_extension();
        let color_incoming = to_rgb_color(colors.secondary);
        let color_outgoing = to_rgb_color(colors.outgoing);
        let color_font = to_rgb_color(colors.text_body);
        let color_mix = ext.alpha_chart_badge;
        let buttons_color = to_rgb_color(ext.buttons_color);

        chart_builder
            .margin_right(30)
            .margin_bottom(0)
            .set_label_area_size(LabelAreaPosition::Left, 60)
            .set_label_area_size(LabelAreaPosition::Bottom, 50);

        let get_y_axis_range = |min: i64, max: i64| {
            let fs = max - min;
            #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
            let gap = (fs as f64 * 0.1) as i64;
            min - gap..max + gap
        };

        let y_axis_range = match self.chart_type {
            ChartType::Packets => get_y_axis_range(self.min_packets, self.max_packets),
            ChartType::Bytes => get_y_axis_range(self.min_bytes, self.max_bytes),
        };

        let mut chart = chart_builder
            .build_cartesian_2d(first_time_displayed..tot_seconds, y_axis_range)
            .expect("Error drawing chart");

        // Mesh
        chart
            .configure_mesh()
            .axis_style(buttons_color)
            .bold_line_style(buttons_color.mix(0.4))
            .light_line_style(buttons_color.mix(0.2))
            .y_max_light_lines(1)
            .y_labels(7)
            .label_style(
                ("Sarasa Mono SC for Sniffnet", 12.5)
                    .into_font()
                    .style(font_weight)
                    .color(&color_font),
            )
            .y_label_formatter(if self.chart_type.eq(&ChartType::Packets) {
                &|packets| packets.abs().to_string()
            } else {
                &|bytes| get_formatted_bytes_string_with_b(u128::from(bytes.unsigned_abs()), 0)
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
            .background_style(buttons_color.mix(0.6))
            .border_style(buttons_color.stroke_width(CHARTS_LINE_BORDER * 2))
            .label_font(
                ("Sarasa Mono SC for Sniffnet", 13.5)
                    .into_font()
                    .style(font_weight)
                    .color(&color_font),
            )
            .draw()
            .expect("Error drawing graph");
    }
}
