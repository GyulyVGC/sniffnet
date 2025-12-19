use std::ops::Range;

use iced::Element;
use iced::widget::{Column, Space};
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
        Column::new()
            .height(45)
            .push(Space::with_height(5))
            .push(ChartWidget::new(self))
            .into()
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
