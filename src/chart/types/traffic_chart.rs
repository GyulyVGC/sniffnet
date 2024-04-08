//! This module defines the behavior of the `TrafficChart` struct, used to display chart in GUI run page

use std::cmp::min;
use std::ops::Range;

use iced::widget::Container;
use iced::Element;
use plotters::prelude::*;
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};
use splines::Spline;

use crate::gui::app::FONT_FAMILY_NAME;
use crate::gui::styles::style_constants::CHARTS_LINE_BORDER;
use crate::gui::styles::types::palette::to_rgb_color;
use crate::gui::types::message::Message;
use crate::networking::types::traffic_direction::TrafficDirection;
use crate::translations::translations::{incoming_translation, outgoing_translation};
use crate::{ByteMultiple, ChartType, Language, StyleType};

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
    /// Whether the chart is for the thumbnail page
    pub thumbnail: bool,
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
            thumbnail: false,
        }
    }

    pub fn view(&self) -> Element<Message, StyleType> {
        Container::new(ChartWidget::new(self)).into()
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

    fn set_margins_and_label_areas<DB: DrawingBackend>(
        &self,
        chart_builder: &mut ChartBuilder<DB>,
    ) {
        if self.thumbnail {
            chart_builder.margin_right(0);
            chart_builder.margin_left(0);
            chart_builder.margin_bottom(0);
            chart_builder.margin_top(5);
        } else {
            chart_builder
                .margin_right(25)
                .margin_top(6)
                .set_label_area_size(LabelAreaPosition::Left, 55)
                .set_label_area_size(LabelAreaPosition::Bottom, 40);
        }
    }

    fn x_axis_range(&self) -> Range<f32> {
        let first_time_displayed = if self.ticks > 30 { self.ticks - 30 } else { 0 };
        let tot_seconds = self.ticks - 1;
        #[allow(clippy::cast_precision_loss)]
        let range = first_time_displayed as f32..tot_seconds as f32;
        range
    }

    fn y_axis_range(&self) -> Range<f32> {
        let (min, max) = match self.chart_type {
            ChartType::Packets => (self.min_packets, self.max_packets),
            ChartType::Bytes => (self.min_bytes, self.max_bytes),
        };
        let fs = max - min;
        let gap = fs * 0.05;
        min - gap..max + gap
    }

    fn font(&self, size: f64) -> TextStyle<'static> {
        (FONT_FAMILY_NAME, size)
            .into_font()
            .style(self.style.get_font_weight())
            .color(&to_rgb_color(self.style.get_palette().text_body))
    }

    fn spline_to_plot(&self, direction: TrafficDirection) -> &Spline<f32, f32> {
        match self.chart_type {
            ChartType::Packets => match direction {
                TrafficDirection::Incoming => &self.in_packets,
                TrafficDirection::Outgoing => &self.out_packets,
            },
            ChartType::Bytes => match direction {
                TrafficDirection::Incoming => &self.in_bytes,
                TrafficDirection::Outgoing => &self.out_bytes,
            },
        }
    }

    fn series_label(&self, direction: TrafficDirection) -> &str {
        match direction {
            TrafficDirection::Incoming => incoming_translation(self.language),
            TrafficDirection::Outgoing => outgoing_translation(self.language),
        }
    }

    fn series_color(&self, direction: TrafficDirection) -> RGBColor {
        match direction {
            TrafficDirection::Incoming => to_rgb_color(self.style.get_palette().secondary),
            TrafficDirection::Outgoing => to_rgb_color(self.style.get_palette().outgoing),
        }
    }

    fn area_series<DB: DrawingBackend>(
        &self,
        direction: TrafficDirection,
    ) -> AreaSeries<DB, f32, f32> {
        let color = self.series_color(direction);
        AreaSeries::new(
            sample_spline(self.spline_to_plot(direction)),
            0.0,
            color.mix(self.style.get_extension().alpha_chart_badge.into()),
        )
        .border_style(ShapeStyle::from(&color).stroke_width(CHARTS_LINE_BORDER))
    }
}

impl Chart<Message> for TrafficChart {
    type State = ();

    fn build_chart<DB: DrawingBackend>(
        &self,
        _state: &Self::State,
        mut chart_builder: ChartBuilder<DB>,
    ) {
        if self.ticks < 1 {
            return;
        }

        self.set_margins_and_label_areas(&mut chart_builder);

        let x_axis_range = self.x_axis_range();
        let y_axis_range = self.y_axis_range();

        let x_labels = if self.ticks == 1 || self.thumbnail {
            0
        } else {
            self.ticks as usize
        };
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let y_labels = if self.thumbnail {
            0
        } else {
            1 + (y_axis_range.end - y_axis_range.start) as usize
        };

        let mut chart = chart_builder
            .build_cartesian_2d(x_axis_range, y_axis_range)
            .expect("Error drawing chart");

        let buttons_color = to_rgb_color(self.style.get_extension().buttons_color);

        // chart mesh
        chart
            .configure_mesh()
            .axis_style(buttons_color)
            .bold_line_style(buttons_color.mix(0.3))
            .light_line_style(buttons_color.mix(0.0))
            .max_light_lines(0)
            .label_style(self.font(12.5))
            .y_labels(min(5, y_labels))
            .y_label_formatter(if self.chart_type.eq(&ChartType::Packets) {
                &|packets| packets.abs().to_string()
            } else {
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                &|bytes| ByteMultiple::formatted_string(bytes.abs() as u128)
            })
            .x_labels(min(6, x_labels))
            .x_label_formatter(&std::string::ToString::to_string)
            .draw()
            .unwrap();

        // draw incoming and outgoing series
        for direction in [TrafficDirection::Incoming, TrafficDirection::Outgoing] {
            let area_series = self.area_series(direction);
            let label = self.series_label(direction);
            let legend_style = self.series_color(direction).filled();
            chart
                .draw_series(area_series)
                .expect("Error drawing graph")
                .label(label)
                .legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 25, y + 5)], legend_style));
        }

        // chart legend
        if !self.thumbnail {
            chart
                .configure_series_labels()
                .position(SeriesLabelPosition::UpperRight)
                .background_style(buttons_color.mix(0.6))
                .border_style(buttons_color.stroke_width(CHARTS_LINE_BORDER * 2))
                .label_font(self.font(13.5))
                .draw()
                .expect("Error drawing graph");
        }
    }
}

const PTS: usize = 300;
fn sample_spline(spline: &Spline<f32, f32>) -> Vec<(f32, f32)> {
    let mut ret_val = Vec::new();
    let len = spline.len();
    let first_x = spline.get(0).unwrap().t;
    let last_x = spline.get(len - 1).unwrap().t;
    #[allow(clippy::cast_precision_loss)]
    let delta = (last_x - first_x) / (PTS as f32 - 1.0);
    for i in 0..PTS {
        #[allow(clippy::cast_precision_loss)]
        let x = first_x + delta * i as f32;
        let p = spline.clamped_sample(x).unwrap_or_default();
        ret_val.push((x, p));
    }
    ret_val
}

#[cfg(test)]
mod tests {
    use splines::{Interpolation, Key, Spline};

    use crate::chart::types::traffic_chart::{sample_spline, PTS};

    #[test]
    fn test_spline_samples() {
        let vec = vec![
            (0, -500),
            (1, -1000),
            (2, -1000),
            (3, -1000),
            (4, -1000),
            (5, -1000),
            (6, -1000),
            (7, -1000),
            (8, -1000),
            (9, -1000),
            (10, -1000),
            (11, -1000),
            (12, -1000),
            (13, -1000),
            (14, -1000),
            (15, -1000),
            (16, -1000),
            (17, -1000),
            (18, -1000),
            (19, -1000),
            (20, -1000),
            (21, -1000),
            (22, -1000),
            (23, -1000),
            (24, -1000),
            (25, -1000),
            (26, -1000),
            (27, -1000),
            (28, -1000),
        ];
        let spline = Spline::from_vec(
            vec.iter()
                .map(|&(x, y)| Key::new(x as f32, y as f32, Interpolation::Cosine))
                .collect::<Vec<Key<f32, f32>>>(),
        );

        let eps = 0.001;

        let samples = sample_spline(&spline);
        assert_eq!(samples.len(), PTS);

        let delta = samples[1].0 - samples[0].0;

        assert_eq!(samples[0].0, 0.0);
        assert_eq!(samples[0].1, -500.0);
        for i in 0..PTS - 1 {
            assert_eq!(
                (samples[i + 1].0 * 10_000.0 - samples[i].0 * 10_000.0).round() / 10_000.0,
                (delta * 10_000.0).round() / 10_000.0
            );
            assert!(samples[i].1 <= -500.0);
            assert!(samples[i].1 >= -1000.0 - eps);
            assert!(samples[i + 1].1 < samples[i].1 + eps);
        }
        assert_eq!(samples[PTS - 1].0, 28.0);
        assert_eq!(samples[PTS - 1].1, -1000.0);
    }
}
