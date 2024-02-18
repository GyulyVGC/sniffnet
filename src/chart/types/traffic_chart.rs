//! This module defines the behavior of the `TrafficChart` struct, used to display chart in GUI run page

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Column, Container};
use iced::{Element, Renderer};
use plotters::prelude::*;
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};
use splines::Spline;
use std::cmp::min;

use crate::gui::app::FONT_FAMILY_NAME;
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
    pub out_bytes: Spline<f32, f32>,
    /// Received bytes filtered and their time occurrence
    pub in_bytes: Spline<f32, f32>,
    /// Sent packets filtered and their time occurrence
    pub out_packets: Spline<f32, f32>,
    /// Received packets filtered and their time occurrence
    pub in_packets: Spline<f32, f32>,
    /// Minimum number of bytes per time interval (computed on last 30 intervals)
    pub min_bytes: f32,
    /// Maximum number of bytes per time interval (computed on last 30 intervals)
    pub max_bytes: f32,
    /// Minimum number of packets per time interval (computed on last 30 intervals)
    pub min_packets: f32,
    /// Maximum number of packets per time interval (computed on last 30 intervals)
    pub max_packets: f32,
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
            out_bytes: Spline::default(),
            in_bytes: Spline::default(),
            out_packets: Spline::default(),
            in_packets: Spline::default(),
            min_bytes: 0.0,
            max_bytes: 0.0,
            min_packets: 0.0,
            max_packets: 0.0,
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

        if self.ticks <= 1 {
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

        let get_y_axis_range = |min: f32, max: f32| {
            let fs = max - min;
            #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
            let gap = fs * 0.05;
            min - gap..max + gap
        };

        let y_axis_range = match self.chart_type {
            ChartType::Packets => get_y_axis_range(self.min_packets, self.max_packets),
            ChartType::Bytes => get_y_axis_range(self.min_bytes, self.max_bytes),
        };

        let mut chart = chart_builder
            .build_cartesian_2d(
                first_time_displayed as f32..tot_seconds as f32,
                y_axis_range,
            )
            .expect("Error drawing chart");

        // Mesh
        chart
            .configure_mesh()
            .axis_style(buttons_color)
            .bold_line_style(buttons_color.mix(0.3))
            .light_line_style(buttons_color.mix(0.0))
            .label_style(
                (FONT_FAMILY_NAME, 12.5)
                    .into_font()
                    .style(font_weight)
                    .color(&color_font),
            )
            .y_labels(7)
            .y_label_formatter(if self.chart_type.eq(&ChartType::Packets) {
                &|packets| packets.abs().to_string()
            } else {
                &|bytes| get_formatted_bytes_string_with_b(bytes.abs() as u128)
            })
            .x_labels(min(6, self.ticks as usize))
            .x_label_formatter(&|t| t.to_string())
            .draw()
            .unwrap();

        // Incoming series
        chart
            .draw_series(
                AreaSeries::new(
                    if self.chart_type.eq(&ChartType::Packets) {
                        sample_spline(&self.in_packets)
                    } else {
                        sample_spline(&self.in_bytes)
                    },
                    0.0,
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
                        sample_spline(&self.out_packets)
                    } else {
                        sample_spline(&self.out_bytes)
                    },
                    0.0,
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
                (FONT_FAMILY_NAME, 13.5)
                    .into_font()
                    .style(font_weight)
                    .color(&color_font),
            )
            .draw()
            .expect("Error drawing graph");
    }
}

const PTS: f32 = 300.0;
fn sample_spline(spline: &Spline<f32, f32>) -> Vec<(f32, f32)> {
    let mut ret_val = Vec::new();
    let len = spline.len();
    let first_x = spline.get(0).unwrap().t;
    let last_x = spline.get(len - 1).unwrap().t;
    let delta = (last_x - first_x) / PTS;
    for i in 0..=PTS as usize {
        let x = first_x + i as f32 * delta;
        let p = spline.clamped_sample(x).unwrap_or_default();
        ret_val.push((x, p));
    }
    ret_val
}
