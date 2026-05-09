//! Experimental Iced Canvas replacement for the Overview traffic line chart.

use std::cmp::min;
use std::ops::Range;

use iced::alignment::Vertical;
use iced::widget::canvas::{self, Frame, Stroke, Text};
use iced::widget::text::Alignment;
use iced::widget::{Canvas, Column, Space};
use iced::{Color, Element, Length, Point, Rectangle, Renderer, Size, mouse};

use crate::StyleType;
use crate::chart::types::chart_series::sample_spline;
use crate::chart::types::traffic_chart::TrafficChart;
use crate::gui::styles::style_constants::{CHARTS_LINE_BORDER, SARASA_MONO};
use crate::gui::types::message::Message;
use crate::networking::types::data_representation::DataRepr;
use crate::networking::types::traffic_direction::TrafficDirection;
use crate::translations::translations::{incoming_translation, outgoing_translation};
use crate::utils::formatted_strings::get_formatted_num_seconds;

const LEFT_LABEL_AREA: f32 = 55.0;
const RIGHT_MARGIN: f32 = 25.0;
const TOP_MARGIN: f32 = 6.0;
const BOTTOM_LABEL_AREA: f32 = 40.0;

#[derive(Default)]
pub struct CanvasLineChartState {
    hovered_tick: Option<f32>,
}

pub struct CanvasLineChart<'a> {
    chart: &'a TrafficChart,
}

impl<'a> CanvasLineChart<'a> {
    fn new(chart: &'a TrafficChart) -> Self {
        Self { chart }
    }

    fn chart_area(&self, bounds: Rectangle) -> Rectangle {
        if self.chart.thumbnail {
            return Rectangle {
                x: 0.0,
                y: 5.0,
                width: bounds.width,
                height: (bounds.height - 5.0).max(0.0),
            };
        }

        Rectangle {
            x: LEFT_LABEL_AREA,
            y: TOP_MARGIN,
            width: (bounds.width - LEFT_LABEL_AREA - RIGHT_MARGIN).max(0.0),
            height: (bounds.height - TOP_MARGIN - BOTTOM_LABEL_AREA).max(0.0),
        }
    }

    fn x_axis_range(&self) -> Range<f32> {
        if self.chart.ticks == 1 {
            return 0.0..0.1;
        }

        let first_time_displayed = if self.chart.no_more_packets {
            0
        } else {
            self.chart.ticks.saturating_sub(30)
        };
        let last_time_displayed = self.chart.ticks - 1;
        #[allow(clippy::cast_precision_loss)]
        let range = first_time_displayed as f32..last_time_displayed as f32;
        range
    }

    fn y_axis_range(&self) -> Range<f32> {
        let (min, max) = match self.chart.data_repr {
            DataRepr::Packets => (self.chart.min_packets, self.chart.max_packets),
            DataRepr::Bytes => (self.chart.min_bytes, self.chart.max_bytes),
            DataRepr::Bits => (self.chart.min_bytes * 8.0, self.chart.max_bytes * 8.0),
        };
        let fs = max - min;
        let gap = if fs == 0.0 { 1.0 } else { fs * 0.05 };
        min - gap..max + gap
    }

    fn series_points(&self, direction: TrafficDirection) -> Vec<(f32, f32)> {
        let spline = match self.chart.data_repr {
            DataRepr::Packets => match direction {
                TrafficDirection::Incoming => &self.chart.in_packets.spline,
                TrafficDirection::Outgoing => &self.chart.out_packets.spline,
            },
            DataRepr::Bytes | DataRepr::Bits => match direction {
                TrafficDirection::Incoming => &self.chart.in_bytes.spline,
                TrafficDirection::Outgoing => &self.chart.out_bytes.spline,
            },
        };
        let multiplier = if self.chart.data_repr == DataRepr::Bits {
            8.0
        } else {
            1.0
        };

        match spline.keys() {
            [k] => vec![(0.0, k.value * multiplier), (0.1, k.value * multiplier)],
            _ => sample_spline(spline, multiplier),
        }
    }

    fn bucket_points(&self) -> Vec<(f32, f32, f32)> {
        let multiplier = if self.chart.data_repr == DataRepr::Bits {
            8.0
        } else {
            1.0
        };
        let incoming = match self.chart.data_repr {
            DataRepr::Packets => self.chart.in_packets.spline.keys(),
            DataRepr::Bytes | DataRepr::Bits => self.chart.in_bytes.spline.keys(),
        };
        let outgoing = match self.chart.data_repr {
            DataRepr::Packets => self.chart.out_packets.spline.keys(),
            DataRepr::Bytes | DataRepr::Bits => self.chart.out_bytes.spline.keys(),
        };

        incoming
            .iter()
            .zip(outgoing.iter())
            .map(|(in_key, out_key)| {
                (
                    in_key.t,
                    in_key.value * multiplier,
                    out_key.value * multiplier,
                )
            })
            .collect()
    }

    fn point(
        &self,
        area: Rectangle,
        x_range: Range<f32>,
        y_range: Range<f32>,
        p: (f32, f32),
    ) -> Point {
        let x_span = (x_range.end - x_range.start).max(f32::EPSILON);
        let y_span = (y_range.end - y_range.start).max(f32::EPSILON);
        let x = area.x + area.width * (p.0 - x_range.start) / x_span;
        let y = area.y + area.height - area.height * (p.1 - y_range.start) / y_span;
        Point::new(x, y)
    }

    fn tick_at_x(&self, area: Rectangle, x_range: Range<f32>, x: f32) -> Option<f32> {
        if x < area.x || x > area.x + area.width || area.width <= 0.0 {
            return None;
        }
        let ratio = (x - area.x) / area.width;
        let hovered_tick = x_range.start + ratio * (x_range.end - x_range.start);
        self.bucket_points()
            .into_iter()
            .min_by(|a, b| {
                (a.0 - hovered_tick)
                    .abs()
                    .total_cmp(&(b.0 - hovered_tick).abs())
            })
            .map(|p| p.0)
    }

    fn hovered_bucket(&self, hovered_tick: f32) -> Option<(f32, f32, f32)> {
        self.bucket_points()
            .into_iter()
            .find(|bucket| (bucket.0 - hovered_tick).abs() < f32::EPSILON)
    }

    fn draw_grid(
        &self,
        frame: &mut Frame,
        area: Rectangle,
        x_range: Range<f32>,
        y_range: Range<f32>,
    ) {
        let style = self.chart.style;
        let ext = style.get_extension();
        let axis_color = ext.buttons_color;
        let grid_color = Color {
            a: ext.alpha_chart_badge,
            ..axis_color
        };
        let text_color = style.get_palette().text_body;
        let axis_stroke = Stroke {
            style: canvas::stroke::Style::Solid(axis_color),
            width: CHARTS_LINE_BORDER as f32,
            ..Default::default()
        };
        let grid_stroke = Stroke {
            style: canvas::stroke::Style::Solid(grid_color),
            width: 1.0,
            ..Default::default()
        };

        let y_labels = min(5, 1 + (y_range.end - y_range.start).max(0.0) as usize).max(2);
        for i in 0..y_labels {
            #[allow(clippy::cast_precision_loss)]
            let ratio = if y_labels == 1 {
                0.0
            } else {
                i as f32 / (y_labels - 1) as f32
            };
            let y = area.y + area.height - area.height * ratio;
            let path = canvas::Path::new(|builder| {
                builder.move_to(Point::new(area.x, y));
                builder.line_to(Point::new(area.x + area.width, y));
            });
            frame.stroke(&path, if i == 0 { axis_stroke } else { grid_stroke });

            if !self.chart.thumbnail {
                let value = y_range.start + (y_range.end - y_range.start) * ratio;
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let content = self.chart.data_repr.formatted_string(value.abs() as u128);
                frame.fill_text(Text {
                    content,
                    position: Point::new(area.x - 8.0, y),
                    color: text_color,
                    size: 12.5.into(),
                    font: SARASA_MONO,
                    align_x: Alignment::Right,
                    align_y: Vertical::Center,
                    ..Default::default()
                });
            }
        }

        if !self.chart.thumbnail && self.chart.is_live_capture {
            let x_labels = if self.chart.ticks == 1 {
                2
            } else {
                min(6, self.chart.ticks as usize)
            };
            for i in 0..x_labels {
                #[allow(clippy::cast_precision_loss)]
                let ratio = if x_labels == 1 {
                    0.0
                } else {
                    i as f32 / (x_labels - 1) as f32
                };
                let x = area.x + area.width * ratio;
                let seconds = x_range.start + (x_range.end - x_range.start) * ratio;
                let path = canvas::Path::new(|builder| {
                    builder.move_to(Point::new(x, area.y));
                    builder.line_to(Point::new(x, area.y + area.height));
                });
                frame.stroke(&path, grid_stroke);

                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let content = get_formatted_num_seconds(seconds.abs() as u128);
                frame.fill_text(Text {
                    content,
                    position: Point::new(x, area.y + area.height + 18.0),
                    color: text_color,
                    size: 12.5.into(),
                    font: SARASA_MONO,
                    align_x: Alignment::Center,
                    align_y: Vertical::Center,
                    ..Default::default()
                });
            }
        }
    }

    fn draw_series(
        &self,
        frame: &mut Frame,
        area: Rectangle,
        x_range: Range<f32>,
        y_range: Range<f32>,
        direction: TrafficDirection,
    ) {
        let points = self.series_points(direction);
        if points.is_empty() {
            return;
        }

        let color = match direction {
            TrafficDirection::Incoming => self.chart.style.get_palette().secondary,
            TrafficDirection::Outgoing => self.chart.style.get_palette().outgoing,
        };
        let alpha = self.chart.style.get_extension().alpha_chart_badge;
        let mapped: Vec<_> = points
            .iter()
            .map(|point| self.point(area, x_range.clone(), y_range.clone(), *point))
            .collect();
        let zero = self
            .point(area, x_range.clone(), y_range, (x_range.start, 0.0))
            .y;

        let area_path = canvas::Path::new(|builder| {
            builder.move_to(Point::new(mapped[0].x, zero));
            for point in &mapped {
                builder.line_to(*point);
            }
            builder.line_to(Point::new(mapped[mapped.len() - 1].x, zero));
            builder.close();
        });
        frame.fill(&area_path, Color { a: alpha, ..color });

        let line_path = canvas::Path::new(|builder| {
            builder.move_to(mapped[0]);
            for point in mapped.iter().skip(1) {
                builder.line_to(*point);
            }
        });
        frame.stroke(
            &line_path,
            Stroke {
                style: canvas::stroke::Style::Solid(color),
                width: CHARTS_LINE_BORDER as f32,
                ..Default::default()
            },
        );
    }

    fn draw_hover(
        &self,
        frame: &mut Frame,
        area: Rectangle,
        x_range: Range<f32>,
        y_range: Range<f32>,
        hovered_tick: f32,
    ) {
        let Some((tick, incoming, outgoing)) = self.hovered_bucket(hovered_tick) else {
            return;
        };
        let style = self.chart.style;
        let text_color = style.get_palette().text_body;
        let guide_color = Color {
            a: 0.85,
            ..style.get_extension().buttons_color
        };
        let x = self
            .point(area, x_range.clone(), y_range.clone(), (tick, 0.0))
            .x;
        let guide = canvas::Path::new(|builder| {
            builder.move_to(Point::new(x, area.y));
            builder.line_to(Point::new(x, area.y + area.height));
        });
        frame.stroke(
            &guide,
            Stroke {
                style: canvas::stroke::Style::Solid(guide_color),
                width: 1.0,
                ..Default::default()
            },
        );

        for (value, color) in [
            (incoming, style.get_palette().secondary),
            (outgoing, style.get_palette().outgoing),
        ] {
            let point = self.point(area, x_range.clone(), y_range.clone(), (tick, value));
            frame.fill(&canvas::Path::circle(point, 3.5), color);
        }

        let incoming_label = incoming_translation(self.chart.language);
        let outgoing_label = outgoing_translation(self.chart.language);
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let content = format!(
            "t: {}\n{}: {}\n{}: {}",
            get_formatted_num_seconds(tick as u128),
            incoming_label,
            self.chart
                .data_repr
                .formatted_string(incoming.abs() as u128),
            outgoing_label,
            self.chart
                .data_repr
                .formatted_string(outgoing.abs() as u128),
        );

        let tooltip_size = Size::new(190.0, 58.0);
        let tooltip_x = if x + tooltip_size.width + 10.0 > area.x + area.width {
            x - tooltip_size.width - 8.0
        } else {
            x + 8.0
        };
        let tooltip_y = (area.y + 8.0).min(area.y + area.height - tooltip_size.height);
        let tooltip = canvas::Path::rounded_rectangle(
            Point::new(tooltip_x, tooltip_y),
            tooltip_size,
            4.0.into(),
        );
        frame.fill(
            &tooltip,
            Color {
                a: 0.95,
                ..style.get_extension().buttons_color
            },
        );
        frame.stroke(
            &tooltip,
            Stroke {
                style: canvas::stroke::Style::Solid(guide_color),
                width: 1.0,
                ..Default::default()
            },
        );
        frame.fill_text(Text {
            content,
            position: Point::new(tooltip_x + 8.0, tooltip_y + 8.0),
            color: text_color,
            size: 12.0.into(),
            font: SARASA_MONO,
            align_x: Alignment::Left,
            align_y: Vertical::Top,
            ..Default::default()
        });
    }

    fn draw_legend(&self, frame: &mut Frame, bounds: Rectangle) {
        if self.chart.thumbnail {
            return;
        }

        let style = self.chart.style;
        let labels = [
            (
                incoming_translation(self.chart.language),
                style.get_palette().secondary,
            ),
            (
                outgoing_translation(self.chart.language),
                style.get_palette().outgoing,
            ),
        ];
        let x = bounds.width - 155.0;
        let mut y = 15.0;

        for (label, color) in labels {
            let swatch = canvas::Path::rectangle(Point::new(x, y + 3.0), Size::new(25.0, 10.0));
            frame.fill(&swatch, color);
            frame.fill_text(Text {
                content: label.to_string(),
                position: Point::new(x + 33.0, y + 8.0),
                color: style.get_palette().text_body,
                size: 13.5.into(),
                font: SARASA_MONO,
                align_x: Alignment::Left,
                align_y: Vertical::Center,
                ..Default::default()
            });
            y += 20.0;
        }
    }
}

impl<Message> canvas::Program<Message, StyleType> for CanvasLineChart<'_> {
    type State = CanvasLineChartState;

    fn update(
        &self,
        state: &mut Self::State,
        event: &canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<iced::widget::Action<Message>> {
        match event {
            canvas::Event::Mouse(mouse::Event::CursorMoved { .. })
            | canvas::Event::Mouse(mouse::Event::CursorEntered)
            | canvas::Event::Mouse(mouse::Event::CursorLeft) => {
                let area = self.chart_area(Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: bounds.width,
                    height: bounds.height,
                });
                let hovered_tick = cursor
                    .position_in(bounds)
                    .and_then(|position| self.tick_at_x(area, self.x_axis_range(), position.x));
                if state.hovered_tick != hovered_tick {
                    state.hovered_tick = hovered_tick;
                    Some(iced::widget::Action::request_redraw())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _theme: &StyleType,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        if self.chart.ticks < 1 {
            return vec![frame.into_geometry()];
        }

        let local_bounds = Rectangle {
            x: 0.0,
            y: 0.0,
            width: bounds.width,
            height: bounds.height,
        };
        let area = self.chart_area(local_bounds);
        let x_range = self.x_axis_range();
        let y_range = self.y_axis_range();

        self.draw_grid(&mut frame, area, x_range.clone(), y_range.clone());
        self.draw_series(
            &mut frame,
            area,
            x_range.clone(),
            y_range.clone(),
            TrafficDirection::Incoming,
        );
        self.draw_series(
            &mut frame,
            area,
            x_range.clone(),
            y_range.clone(),
            TrafficDirection::Outgoing,
        );
        if let Some(hovered_tick) = state.hovered_tick {
            self.draw_hover(&mut frame, area, x_range, y_range, hovered_tick);
        }
        self.draw_legend(&mut frame, local_bounds);

        vec![frame.into_geometry()]
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if cursor.position_in(bounds).is_some() {
            mouse::Interaction::Crosshair
        } else {
            mouse::Interaction::default()
        }
    }
}

pub fn canvas_line_chart(chart: &TrafficChart) -> Element<'_, Message, StyleType> {
    let canvas = Canvas::new(CanvasLineChart::new(chart))
        .width(Length::Fill)
        .height(Length::Fill);

    Column::new()
        .height(Length::Fill)
        .push(canvas)
        .push(if chart.is_live_capture || chart.thumbnail {
            None
        } else {
            Some(Space::new().height(23))
        })
        .into()
}
