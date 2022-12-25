//! This module defines the behavior of the `TrafficChart` struct, used to display charts in GUI run page

use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Column, Container};
use iced::Element;
use plotters::style::RGBColor;
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};

use crate::enums::message::Message;
use crate::structs::colors::to_rgb_color;
use crate::utility::style_constants::{
    CHARTS_LINE_BORDER, COLOR_CHART_MIX, NOTOSANS, NOTOSANS_BOLD,
};
use crate::{get_colors, ChartType, RunTimeData, StyleType};

/// Struct defining the chart to be displayed in gui run page
pub struct TrafficChart {
    charts_data: Rc<RefCell<RunTimeData>>,
    color_mix: f64,
    color_incoming: RGBColor,
    color_outgoing: RGBColor,
    color_font: RGBColor,
    pub chart_type: ChartType,
}

impl TrafficChart {
    pub fn new(charts_data: Rc<RefCell<RunTimeData>>, style: StyleType) -> Self {
        TrafficChart {
            charts_data,
            color_mix: COLOR_CHART_MIX,
            color_incoming: to_rgb_color(get_colors(style).incoming),
            color_outgoing: to_rgb_color(get_colors(style).outgoing),
            color_font: to_rgb_color(get_colors(style).text_body),
            chart_type: ChartType::Packets,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let color_font = self.color_font;
        Container::new(
            Column::new().push(
                ChartWidget::new(self).resolve_font(move |_, _| match color_font {
                    RGBColor(255, 255, 255) => NOTOSANS, // if white non-bold
                    _ => NOTOSANS_BOLD,
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

    pub fn change_colors(&mut self, style: StyleType) {
        self.color_font = to_rgb_color(get_colors(style).text_body);
        self.color_incoming = to_rgb_color(get_colors(style).incoming);
        self.color_outgoing = to_rgb_color(get_colors(style).outgoing);
    }
}

impl Chart<Message> for TrafficChart {
    type State = ();

    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, mut chart: ChartBuilder<DB>) {
        use plotters::prelude::*;

        if self.charts_data.borrow().ticks == 0 {
            return;
        }
        let tot_seconds = self.charts_data.borrow().ticks - 1;
        let first_time_displayed = max(0, self.charts_data.borrow().ticks as i128 - 30) as u128;

        let color_incoming = self.color_incoming;
        let color_outgoing = self.color_outgoing;

        match self.chart_type {
            ChartType::Bytes => {
                //display bytes chart
                let mut chart = chart
                    .margin_right(30)
                    .set_label_area_size(LabelAreaPosition::Left, 60)
                    .set_label_area_size(LabelAreaPosition::Bottom, 50)
                    .build_cartesian_2d(
                        first_time_displayed..tot_seconds,
                        self.charts_data.borrow().min_sent_bytes
                            ..self.charts_data.borrow().max_received_bytes,
                    )
                    .expect("Error drawing graph");

                chart
                    .configure_mesh()
                    .label_style(("notosans", 15).into_font().color(&self.color_font))
                    .y_label_formatter(&|bytes| match bytes {
                        0..=999 | -999..=-1 => {
                            format!("{bytes}")
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
                            self.charts_data.borrow().received_bytes.iter().copied(),
                            0,
                            color_incoming.mix(self.color_mix),
                        )
                        .border_style(
                            ShapeStyle::from(&color_incoming).stroke_width(CHARTS_LINE_BORDER),
                        ),
                    )
                    .expect("Error drawing graph")
                    .label("Incoming")
                    .legend(move |(x, y)| {
                        Rectangle::new([(x, y - 5), (x + 25, y + 5)], color_incoming.filled())
                    });
                chart
                    .draw_series(
                        AreaSeries::new(
                            self.charts_data.borrow().sent_bytes.iter().copied(),
                            0,
                            color_outgoing.mix(self.color_mix),
                        )
                        .border_style(
                            ShapeStyle::from(&color_outgoing).stroke_width(CHARTS_LINE_BORDER),
                        ),
                    )
                    .expect("Error drawing graph")
                    .label("Outgoing")
                    .legend(move |(x, y)| {
                        Rectangle::new([(x, y - 5), (x + 25, y + 5)], color_outgoing.filled())
                    });
                chart
                    .configure_series_labels()
                    .position(SeriesLabelPosition::UpperRight)
                    .border_style(BLACK)
                    .label_font(("notosans", 15).into_font().color(&self.color_font))
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
                        self.charts_data.borrow().min_sent_packets
                            ..self.charts_data.borrow().max_received_packets,
                    )
                    .expect("Error drawing graph");

                chart
                    .configure_mesh()
                    .label_style(("notosans", 15).into_font().color(&self.color_font))
                    .draw()
                    .unwrap();
                chart
                    .draw_series(
                        AreaSeries::new(
                            self.charts_data.borrow().received_packets.iter().copied(),
                            0,
                            color_incoming.mix(self.color_mix),
                        )
                        .border_style(
                            ShapeStyle::from(&color_incoming).stroke_width(CHARTS_LINE_BORDER),
                        ),
                    )
                    .expect("Error drawing graph")
                    .label("Incoming")
                    .legend(move |(x, y)| {
                        Rectangle::new([(x, y - 5), (x + 25, y + 5)], color_incoming.filled())
                    });
                chart
                    .draw_series(
                        AreaSeries::new(
                            self.charts_data.borrow().sent_packets.iter().copied(),
                            0,
                            color_outgoing.mix(self.color_mix),
                        )
                        .border_style(
                            ShapeStyle::from(&color_outgoing).stroke_width(CHARTS_LINE_BORDER),
                        ),
                    )
                    .expect("Error drawing graph")
                    .label("Outgoing")
                    .legend(move |(x, y)| {
                        Rectangle::new([(x, y - 5), (x + 25, y + 5)], color_outgoing.filled())
                    });
                chart
                    .configure_series_labels()
                    .position(SeriesLabelPosition::UpperRight)
                    .border_style(BLACK)
                    .label_font(("notosans", 15).into_font().color(&self.color_font))
                    .draw()
                    .expect("Error drawing graph");
            }
        }
    }
}
