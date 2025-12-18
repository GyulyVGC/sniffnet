use std::ops::Range;

use iced::Element;
use iced::widget::Column;
use plotters::prelude::*;
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};

use crate::chart::types::chart_series::{ChartSeries, sample_spline};
use crate::gui::styles::style_constants::CHARTS_LINE_BORDER;
use crate::gui::styles::types::palette::to_rgb_color;
use crate::gui::types::message::Message;
use crate::utils::error_logger::{ErrorLogger, Location};
use crate::{StyleType, location};

/// Struct defining the traffic preview charts to be displayed in GUI initial page
pub struct PreviewChart {
    /// Current time interval number
    pub ticks: u32,
    /// Packets (sent & received)
    pub packets: ChartSeries,
    /// Maximum number of packets per time interval (computed on last 30 intervals)
    pub max_packets: f32,
    /// Style of the chart
    pub style: StyleType,
}

impl PreviewChart {
    pub fn new(style: StyleType) -> Self {
        Self {
            ticks: 0,
            packets: ChartSeries::default(),
            max_packets: 0.0,
            style,
        }
    }

    pub fn update_charts_data(&mut self, packets: u128) {
        #[allow(clippy::cast_precision_loss)]
        let tot_seconds = self.ticks as f32;
        self.ticks += 1;

        #[allow(clippy::cast_precision_loss)]
        let packets_entry = packets as f32;
        let packets_point = (tot_seconds, packets_entry);

        // update traffic data
        self.packets.update_series(packets_point, true, false);
        self.max_packets = self.packets.get_max();
    }

    pub fn view(&self) -> Element<'_, Message, StyleType> {
        Column::new().height(40).push(ChartWidget::new(self)).into()
    }

    pub fn change_style(&mut self, style: StyleType) {
        self.style = style;
    }

    fn x_axis_range(&self) -> Range<f32> {
        // if we have only one tick, we need to add a second point to draw the area
        if self.ticks == 1 {
            return 0.0..0.1;
        }

        let first_time_displayed = self.ticks.saturating_sub(30);
        let last_time_displayed = self.ticks - 1;
        #[allow(clippy::cast_precision_loss)]
        let range = first_time_displayed as f32..last_time_displayed as f32;
        range
    }

    fn y_axis_range(&self) -> Range<f32> {
        let max = self.max_packets;
        0.0..max
    }

    fn area_series<DB: DrawingBackend>(&self) -> AreaSeries<DB, f32, f32> {
        let color = to_rgb_color(self.style.get_palette().secondary);
        let alpha = self.style.get_extension().alpha_chart_badge;
        let spline = &self.packets.spline;

        let data = match spline.keys() {
            // if we have only one tick, we need to add a second point to draw the area
            [k] => vec![(0.0, k.value), (0.1, k.value)],
            _ => sample_spline(spline, 1.0),
        };

        AreaSeries::new(data, 0.0, color.mix(alpha.into()))
            .border_style(ShapeStyle::from(&color).stroke_width(CHARTS_LINE_BORDER))
    }
}

impl Chart<Message> for PreviewChart {
    type State = ();

    fn build_chart<DB: DrawingBackend>(
        &self,
        _state: &Self::State,
        mut chart_builder: ChartBuilder<DB>,
    ) {
        if self.ticks < 1 {
            return;
        }

        let x_axis_range = self.x_axis_range();
        let y_axis_range = self.y_axis_range();

        let Ok(mut chart) = chart_builder
            .build_cartesian_2d(x_axis_range, y_axis_range)
            .log_err(location!())
        else {
            return;
        };

        let buttons_color = to_rgb_color(self.style.get_extension().buttons_color);

        // chart mesh
        let _ = chart
            .configure_mesh()
            .axis_style(buttons_color)
            .bold_line_style(buttons_color.mix(0.3))
            .light_line_style(buttons_color.mix(0.0))
            .max_light_lines(0)
            .y_labels(0)
            .x_labels(0)
            .draw()
            .log_err(location!());

        // draw packets series
        let area_series = self.area_series();
        let _ = chart.draw_series(area_series).log_err(location!());
    }
}

#[cfg(test)]
mod tests {
    use splines::{Interpolation, Key, Spline};

    use crate::chart::types::chart_series::sample_spline;

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

        let pts = spline.len() * 10;
        let samples = sample_spline(&spline, 1.0);
        assert_eq!(samples.len(), pts);

        let delta = samples[1].0 - samples[0].0;

        assert_eq!(samples[0].0, 0.0);
        assert_eq!(samples[0].1, -500.0);
        for i in 0..pts - 1 {
            assert_eq!(
                (samples[i + 1].0 * 10_000.0 - samples[i].0 * 10_000.0).round() / 10_000.0,
                (delta * 10_000.0).round() / 10_000.0
            );
            assert!(samples[i].1 <= -500.0);
            assert!(samples[i].1 >= -1000.0 - eps);
            assert!(samples[i + 1].1 < samples[i].1 + eps);
        }
        assert_eq!(samples[pts - 1].0, 28.0);
        assert_eq!(samples[pts - 1].1, -1000.0);
    }
}
