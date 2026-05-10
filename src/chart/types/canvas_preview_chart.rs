//! Renderer-compatible preview chart for the initial page adapters.

use std::ops::Range;

use iced::advanced::graphics::geometry;
use iced::advanced::widget::{Tree, tree};
use iced::advanced::{Layout, Widget, layout, mouse, renderer};
use iced::widget::canvas::{self, Frame, Stroke};
use iced::{Color, Element, Length, Point, Rectangle, Size};

use crate::StyleType;
use crate::chart::types::chart_series::sample_spline;
use crate::chart::types::preview_chart::PreviewChart;
use crate::gui::styles::style_constants::CHARTS_LINE_BORDER;
use crate::gui::types::message::Message;

struct CanvasPreviewChart<'a> {
    chart: &'a PreviewChart,
}

impl<'a> CanvasPreviewChart<'a> {
    fn new(chart: &'a PreviewChart) -> Self {
        Self { chart }
    }

    fn x_axis_range(&self) -> Range<f32> {
        if self.chart.ticks == 1 {
            return 0.0..0.1;
        }

        let first_time_displayed = self.chart.ticks.saturating_sub(30);
        let last_time_displayed = self.chart.ticks - 1;
        #[allow(clippy::cast_precision_loss)]
        let range = first_time_displayed as f32..last_time_displayed as f32;
        range
    }

    fn y_axis_range(&self) -> Range<f32> {
        let max = self.chart.max_packets;
        let gap = if max == 0.0 { 1.0 } else { max * 0.1 };
        0.0..max + gap
    }

    fn series_points(&self) -> Vec<(f32, f32)> {
        match self.chart.packets.spline.keys() {
            [k] => vec![(0.0, k.value), (0.1, k.value)],
            _ => sample_spline(&self.chart.packets.spline, 1.0),
        }
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

    fn draw_chart<Renderer: geometry::Renderer>(
        &self,
        frame: &mut Frame<Renderer>,
        bounds: Rectangle,
    ) {
        // Keep tiny-skia damage tracking scoped to the whole chart widget.
        frame.fill_rectangle(
            bounds.position(),
            bounds.size(),
            Color {
                a: 0.0,
                ..self.chart.style.get_palette().primary
            },
        );

        if self.chart.ticks < 1 {
            return;
        }

        let area = Rectangle {
            x: bounds.x,
            y: bounds.y + 5.0,
            width: bounds.width,
            height: (bounds.height - 5.0).max(0.0),
        };
        let x_range = self.x_axis_range();
        let y_range = self.y_axis_range();
        let points = self.series_points();
        if points.is_empty() {
            return;
        }

        let style = self.chart.style;
        let color = style.get_palette().secondary;
        let axis_color = style.get_extension().buttons_color;
        let alpha = style.get_extension().alpha_chart_badge;
        let mapped: Vec<_> = points
            .iter()
            .map(|point| self.point(area, x_range.clone(), y_range.clone(), *point))
            .collect();
        let zero = self
            .point(area, x_range.clone(), y_range, (x_range.start, 0.0))
            .y;

        let axis = canvas::Path::line(
            Point::new(area.x, zero),
            Point::new(area.x + area.width, zero),
        );
        frame.stroke(
            &axis,
            Stroke {
                style: canvas::stroke::Style::Solid(axis_color),
                width: CHARTS_LINE_BORDER as f32,
                ..Default::default()
            },
        );

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
}

impl<Message, Renderer> Widget<Message, StyleType, Renderer> for CanvasPreviewChart<'_>
where
    Renderer: renderer::Renderer + geometry::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::stateless()
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Fixed(45.0),
        }
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, Length::Fill, Length::Fixed(45.0))
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        _theme: &StyleType,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let mut frame = Frame::with_bounds(renderer, *viewport);
        self.draw_chart(&mut frame, layout.bounds());
        renderer.draw_geometry(frame.into_geometry());
    }
}

pub fn canvas_preview_chart(chart: &PreviewChart) -> Element<'_, Message, StyleType> {
    CanvasPreviewChart::new(chart).into()
}

impl<'a, Message, Renderer> From<CanvasPreviewChart<'a>>
    for Element<'a, Message, StyleType, Renderer>
where
    Message: 'a,
    Renderer: 'a + renderer::Renderer + geometry::Renderer,
{
    fn from(chart: CanvasPreviewChart<'a>) -> Self {
        Element::new(chart)
    }
}
