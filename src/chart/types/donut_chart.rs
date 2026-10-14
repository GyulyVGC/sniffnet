use crate::gui::styles::donut::Catalog;
use crate::gui::styles::style_constants::{FONT_SIZE_FOOTER, FONT_SIZE_SUBTITLE, SARASA_MONO};
use crate::networking::types::data_representation::DataRepr;
use iced::alignment::Vertical;
use iced::widget::canvas::path::Arc;
use iced::widget::canvas::{Frame, Stroke, Text};
use iced::widget::text::Alignment;
use iced::widget::{Canvas, canvas};
use iced::{Length, Radians, Renderer, mouse};
use std::f32::consts;

pub struct DonutChart {
    data_repr: DataRepr,
    incoming: u128,
    outgoing: u128,
    dropped: Option<u128>,
    thumbnail: bool,
}

impl DonutChart {
    fn new(
        data_repr: DataRepr,
        incoming: u128,
        outgoing: u128,
        dropped: Option<u128>,
        thumbnail: bool,
    ) -> Self {
        Self {
            data_repr,
            incoming,
            outgoing,
            dropped,
            thumbnail,
        }
    }

    fn total(&self) -> u128 {
        self.incoming + self.outgoing + self.dropped.unwrap_or_default()
    }

    fn title(&self) -> String {
        let total = self.total();
        self.data_repr.formatted_string(total)
    }

    fn angles(&self) -> [(Radians, Radians); 3] {
        #[allow(clippy::cast_precision_loss)]
        let mut values = [
            self.incoming as f32,
            self.outgoing as f32,
            self.dropped.unwrap_or_default() as f32,
        ];
        let total: f32 = values.iter().sum();
        if total == 0.0 {
            let angle = Radians(-consts::FRAC_PI_2);
            return [(angle, angle); 3];
        }
        let min_val = 2.0 * total / 100.0;
        let mut diff = 0.0;

        for value in &mut values {
            if *value > 0.0 && *value < min_val {
                diff += min_val - *value;
                *value = min_val;
            }
        }
        // remove the diff from the max value
        if diff > 0.0 {
            let _ = values
                .iter_mut()
                .max_by(|a, b| a.total_cmp(b))
                .map(|max| *max -= diff);
        }

        let mut start_angle = Radians(-consts::FRAC_PI_2);
        values.map(|value| {
            let start = start_angle;
            let end = start + Radians(consts::TAU) * value / total;
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

        let thickness = 6.0;
        let radius = (frame.width().min(frame.height()) / 2.0) * 0.9 - thickness / 2.0;

        let style = <Theme as Catalog>::style(theme, &<Theme as Catalog>::default());
        let colors = [style.incoming, style.outgoing, style.dropped];

        for ((start_angle, end_angle), color) in self.angles().into_iter().zip(colors) {
            if start_angle == end_angle {
                continue;
            }
            let path = canvas::Path::new(|builder| {
                // build path using just an arc with thickness, because iced's paths are limited:
                // - no single close path can be constructed as the one we want (drawing an arc starts a new sub-path)
                // - counter-clockwise arcs are not supported
                builder.arc(Arc {
                    center,
                    radius,
                    start_angle,
                    end_angle,
                });
            });

            let stroke = Stroke {
                style: canvas::stroke::Style::Solid(color),
                width: thickness,
                ..Default::default()
            };
            frame.stroke(&path, stroke);
        }

        frame.fill_text(Text {
            content: self.title().clone(),
            position: center,
            color: style.text_color,
            size: if self.thumbnail {
                FONT_SIZE_FOOTER
            } else {
                FONT_SIZE_SUBTITLE
            }
            .into(),
            font: SARASA_MONO,
            align_x: Alignment::Center,
            align_y: Vertical::Center,
            ..Default::default()
        });

        vec![frame.into_geometry()]
    }
}

pub fn donut_chart<Message, Theme: Catalog>(
    data_repr: DataRepr,
    incoming: u128,
    outgoing: u128,
    dropped: Option<u128>,
    thumbnail: bool,
) -> Canvas<DonutChart, Message, Theme, Renderer> {
    let size = if thumbnail {
        Length::Fill
    } else {
        Length::Fixed(110.0)
    };
    iced::widget::canvas(DonutChart::new(
        data_repr, incoming, outgoing, dropped, thumbnail,
    ))
    .width(size)
    .height(size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_donut_chart() {
        for dropped in [None, Some(0)] {
            for thumbnail in [false, true] {
                let chart = DonutChart::new(DataRepr::bytes(true), 0, 0, dropped, thumbnail);
                assert_eq!(chart.title(), "0 B/s");
                for (start, end) in chart.angles() {
                    assert!(start.0.is_finite());
                    assert!(end.0.is_finite());
                    assert_eq!(start, end);
                }
            }
        }
    }

    #[test]
    fn test_donut_chart_angles() {
        for thumbnail in [false, true] {
            for per_second in [false, true] {
                let chart =
                    DonutChart::new(DataRepr::bytes(per_second), 100, 200, Some(100), thumbnail);
                let angles = chart.angles();
                assert!((angles[0].1.0 - angles[0].0.0 - consts::FRAC_PI_2).abs() < 0.000_001);
                assert!(
                    (angles[1].1.0 - angles[1].0.0 - 2.0 * consts::FRAC_PI_2).abs() < 0.000_001
                );
                assert!((angles[2].1.0 - angles[2].0.0 - consts::FRAC_PI_2).abs() < 0.000_001);
            }
        }
    }
}
