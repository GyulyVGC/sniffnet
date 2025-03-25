use crate::chart::types::chart_type::ChartType;
use crate::gui::styles::donut::Catalog;
use crate::gui::styles::style_constants::FONT_SIZE_SUBTITLE;
use crate::networking::types::byte_multiple::ByteMultiple;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::canvas::path::Arc;
use iced::widget::canvas::{Frame, Text};
use iced::widget::{canvas, Canvas};
use iced::{mouse, Font, Radians, Renderer};
use std::f32::consts;

pub struct DonutChart {
    chart_type: ChartType,
    incoming: u128,
    outgoing: u128,
    filtered_out: u128,
    dropped: u128,
    font: Font,
}

impl DonutChart {
    fn new(
        chart_type: ChartType,
        incoming: u128,
        outgoing: u128,
        filtered_out: u128,
        dropped: u128,
        font: Font,
    ) -> Self {
        Self {
            chart_type,
            incoming,
            outgoing,
            filtered_out,
            dropped,
            font,
        }
    }

    fn values(&self) -> [u128; 4] {
        let inc = self.incoming;
        let out = self.outgoing;
        let filtered_out = self.filtered_out;
        let dropped = self.dropped;
        [inc, out, filtered_out, dropped]
    }

    fn title(&self) -> String {
        let total = self.values().iter().sum();
        if self.chart_type.eq(&ChartType::Bytes) {
            ByteMultiple::formatted_string(total)
        } else {
            total.to_string()
        }
    }

    fn angles(&self) -> [(Radians, Radians); 4] {
        let mut values = self.values();
        let total: u128 = values.iter().sum();
        let min_val = 2 * total / 100;
        let mut diff = 0;

        for value in &mut values {
            if *value != 0 && *value < min_val {
                diff += min_val - *value;
                *value = min_val;
            }
        }
        // remove the diff from the max value
        if diff > 0 {
            let _ = values.iter_mut().max().map(|max| *max -= diff);
        }

        let mut start_angle = Radians(-consts::FRAC_PI_2);
        values.map(|value| {
            let start = start_angle;
            #[allow(clippy::cast_precision_loss)]
            let end = start + Radians(consts::TAU) * (value as f32) / (total as f32);
            start_angle = end;
            (start, end)
        })
    }
}

impl<Message, Theme: Catalog> canvas::Program<Message, Theme> for DonutChart {
    type State = ();

    fn draw(
        &self,
        (): &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: iced::Rectangle,
        _: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let center = frame.center();
        let radius = (frame.width().min(frame.height()) / 2.0) * 0.9;

        let style = <Theme as Catalog>::style(theme, &<Theme as Catalog>::default());
        let colors = [
            style.incoming,
            style.outgoing,
            style.filtered_out,
            style.dropped,
        ];

        for ((start_angle, end_angle), color) in self.angles().into_iter().zip(colors) {
            let path = canvas::Path::new(|builder| {
                builder.arc(Arc {
                    center,
                    radius,
                    start_angle,
                    end_angle,
                });
                builder.line_to(center);
                builder.close();
            });

            frame.fill(&path, color);
        }

        let inner_circle = canvas::Path::circle(center, radius - 6.0);
        frame.fill(&inner_circle, style.background);
        frame.fill_text(Text {
            content: self.title().clone(),
            position: center,
            vertical_alignment: Vertical::Center,
            horizontal_alignment: Horizontal::Center,
            color: style.text_color,
            size: FONT_SIZE_SUBTITLE.into(),
            font: self.font,
            ..Default::default()
        });

        vec![frame.into_geometry()]
    }
}

pub fn donut_chart<Message, Theme: Catalog>(
    chart_type: ChartType,
    incoming: u128,
    outgoing: u128,
    filtered_out: u128,
    dropped: u128,
    font: Font,
) -> Canvas<DonutChart, Message, Theme, Renderer> {
    iced::widget::canvas(DonutChart::new(
        chart_type,
        incoming,
        outgoing,
        filtered_out,
        dropped,
        font,
    ))
    .width(110)
    .height(110)
}
