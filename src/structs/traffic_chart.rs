//! This module defines the behavior of the `TrafficChart` struct, used to display charts in GUI run page

use std::cmp::max;
use std::sync::{Arc, Mutex};

use iced::alignment::{Horizontal, Vertical};
use iced::{Column, Container, Element};
use plotters::style::RGBColor;
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};

use crate::enums::message::Message;
use crate::gui::style::{
    CHARTS_LINE_BORDER, COLOR_CHART_MIX_DAY, COLOR_CHART_MIX_NIGHT, NOTOSANS, NOTOSANS_BOLD,
    SPECIAL_DAY_RGB, SPECIAL_NIGHT_RGB,
};
use crate::{ChartType, RunTimeData, StyleType};

/// Struct defining the chart to be displayed in gui run page
pub struct TrafficChart {
    charts_data: Arc<Mutex<RunTimeData>>,
    color_mix: f64,
    font_color: RGBColor,
    chart_type: ChartType,
}

impl TrafficChart {
    pub fn new(charts_data: Arc<Mutex<RunTimeData>>) -> Self {
        TrafficChart {
            charts_data,
            color_mix: 0.0,
            font_color: Default::default(),
            chart_type: ChartType::Packets,
        }
    }

    pub fn view(&mut self, mode: StyleType, chart_type: ChartType) -> Element<Message> {
        self.color_mix = if mode == StyleType::Day {
            COLOR_CHART_MIX_DAY
        } else {
            COLOR_CHART_MIX_NIGHT
        };
        self.chart_type = chart_type;
        self.font_color = if mode == StyleType::Day {
            plotters::style::colors::BLACK
        } else {
            plotters::style::colors::WHITE
        };

        Container::new(Column::new().push(ChartWidget::new(self).resolve_font(
            move |_, _| match mode {
                StyleType::Night => NOTOSANS,
                StyleType::Day => NOTOSANS_BOLD,
                _ => NOTOSANS,
            },
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
            return;
        }
        let tot_seconds = charts_data_lock.ticks - 1;
        let first_time_displayed = max(0, charts_data_lock.ticks as i128 - 30) as u128;

        match self.chart_type {
            ChartType::Bytes => {
                //display bytes chart
                let mut chart = chart
                    .margin_right(30)
                    .set_label_area_size(LabelAreaPosition::Left, 60)
                    .set_label_area_size(LabelAreaPosition::Bottom, 50)
                    .build_cartesian_2d(
                        first_time_displayed..tot_seconds,
                        charts_data_lock.min_sent_bytes..charts_data_lock.max_received_bytes,
                    )
                    .expect("Error drawing graph");

                chart
                    .configure_mesh()
                    .label_style(("notosans", 15).into_font().color(&self.font_color))
                    .y_label_formatter(&|bytes| match bytes {
                        0..=999 | -999..=-1 => {
                            format!("{}", bytes)
                        }
                        1000..=999_999 | -999_999..=-1000 => {
                            format!("{:.1} {}", *bytes as f64 / 1_000_f64, "k")
                        }
                        1_000_000..=999_999_999 | -999_999_999..=-1_000_000 => {
                            format!("{:.1} {}", *bytes as f64 / 1_000_000_f64, "M")
                        }
                        _ => {
                            format!("{:.1} {}", *bytes as f64 / 1_000_000_000_f64, "G")
                        }
                    })
                    .draw()
                    .unwrap();
                chart
                    .draw_series(
                        AreaSeries::new(
                            charts_data_lock.received_bytes.iter().copied(),
                            0,
                            SPECIAL_NIGHT_RGB.mix(self.color_mix),
                        )
                        .border_style(
                            ShapeStyle::from(&SPECIAL_NIGHT_RGB).stroke_width(CHARTS_LINE_BORDER),
                        ),
                    )
                    .expect("Error drawing graph")
                    .label("Incoming")
                    .legend(|(x, y)| {
                        Rectangle::new([(x, y - 5), (x + 25, y + 5)], SPECIAL_NIGHT_RGB.filled())
                    });
                chart
                    .draw_series(
                        AreaSeries::new(
                            charts_data_lock.sent_bytes.iter().copied(),
                            0,
                            SPECIAL_DAY_RGB.mix(self.color_mix),
                        )
                        .border_style(
                            ShapeStyle::from(&SPECIAL_DAY_RGB).stroke_width(CHARTS_LINE_BORDER),
                        ),
                    )
                    .expect("Error drawing graph")
                    .label("Outgoing")
                    .legend(|(x, y)| {
                        Rectangle::new([(x, y - 5), (x + 25, y + 5)], SPECIAL_DAY_RGB.filled())
                    });
                chart
                    .configure_series_labels()
                    .position(SeriesLabelPosition::UpperRight)
                    .border_style(BLACK)
                    .label_font(("notosans", 15).into_font().color(&self.font_color))
                    .draw()
                    .expect("Error drawing graph");
            }

            ChartType::Packets => {
                //display packets chart
                let mut chart = chart
                    .margin_right(30)
                    .set_label_area_size(LabelAreaPosition::Left, 60)
                    .set_label_area_size(LabelAreaPosition::Bottom, 50)
                    .build_cartesian_2d(
                        first_time_displayed..tot_seconds,
                        charts_data_lock.min_sent_packets..charts_data_lock.max_received_packets,
                    )
                    .expect("Error drawing graph");

                chart
                    .configure_mesh()
                    .label_style(("notosans", 15).into_font().color(&self.font_color))
                    .draw()
                    .unwrap();
                chart
                    .draw_series(
                        AreaSeries::new(
                            charts_data_lock.received_packets.iter().copied(),
                            0,
                            SPECIAL_NIGHT_RGB.mix(self.color_mix),
                        )
                        .border_style(
                            ShapeStyle::from(&SPECIAL_NIGHT_RGB).stroke_width(CHARTS_LINE_BORDER),
                        ),
                    )
                    .expect("Error drawing graph")
                    .label("Incoming")
                    .legend(|(x, y)| {
                        Rectangle::new([(x, y - 5), (x + 25, y + 5)], SPECIAL_NIGHT_RGB.filled())
                    });
                chart
                    .draw_series(
                        AreaSeries::new(
                            charts_data_lock.sent_packets.iter().copied(),
                            0,
                            SPECIAL_DAY_RGB.mix(self.color_mix),
                        )
                        .border_style(
                            ShapeStyle::from(&SPECIAL_DAY_RGB).stroke_width(CHARTS_LINE_BORDER),
                        ),
                    )
                    .expect("Error drawing graph")
                    .label("Outgoing")
                    .legend(|(x, y)| {
                        Rectangle::new([(x, y - 5), (x + 25, y + 5)], SPECIAL_DAY_RGB.filled())
                    });
                chart
                    .configure_series_labels()
                    .position(SeriesLabelPosition::UpperRight)
                    .border_style(BLACK)
                    .label_font(("notosans", 15).into_font().color(&self.font_color))
                    .draw()
                    .expect("Error drawing graph");
            }
        }
    }
}
