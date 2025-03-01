use crate::gui::styles::donut::Catalog;
use crate::gui::styles::style_constants::FONT_SIZE_BODY;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::canvas::path::Arc;
use iced::widget::canvas::{Frame, Text};
use iced::widget::text::Shaping;
use iced::widget::{canvas, Canvas};
use iced::{mouse, Font, Point, Radians, Renderer};
use std::f32::consts;

pub struct DonutChart {
    percentage: f32,
    label: String,
    font: Font,
}

impl DonutChart {
    pub fn new(percentage: f32, label: impl Into<String>, font: Font) -> Self {
        let label = label.into();
        Self {
            percentage,
            label,
            font,
        }
    }
}

impl<Message, Theme: Catalog> canvas::Program<Message, Theme> for DonutChart {
    type State = ();

    fn draw(
        &self,
        _: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: iced::Rectangle,
        _: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let center = frame.center();
        let radius = frame.width().min(frame.height()) / 2.0;

        let start_angle1 = Radians(-consts::FRAC_PI_2);
        let end_angle1 = Radians(consts::TAU) * self.percentage / 100.0 + start_angle1;

        let style = <Theme as Catalog>::style(theme, &<Theme as Catalog>::default());

        // let circle_at_angle = |angle: Radians| {
        //     canvas::Path::circle(
        //         Point::new(
        //             center.x + fixed_radius * angle.0.cos(),
        //             center.y + fixed_radius * angle.0.sin(),
        //         ),
        //         inner_ball_radius,
        //     )
        // };

        let circle = canvas::Path::circle(center, radius);

        let incoming = canvas::Path::new(|builder| {
            builder.arc(Arc {
                center,
                radius,
                start_angle: start_angle1,
                end_angle: end_angle1,
            });
            builder.line_to(center);
            builder.close();
        });

        let outgoing = canvas::Path::new(|builder| {
            builder.arc(Arc {
                center,
                radius,
                start_angle: end_angle1,
                end_angle: start_angle1 + Radians(consts::PI * 2.0),
            });
            builder.line_to(center);
            builder.close();
        });

        let inner_circle = canvas::Path::circle(center, radius - 6.0);

        frame.fill_text(Text {
            content: if self.label.is_empty() {
                format!("{:.2}%", self.percentage)
            } else {
                self.label.clone()
            },
            position: center,
            vertical_alignment: Vertical::Center,
            horizontal_alignment: Horizontal::Center,
            color: style.text_color,
            size: FONT_SIZE_BODY.into(),
            font: self.font,
            ..Default::default()
        });

        frame.fill(&incoming, style.incoming);
        frame.fill(&outgoing, style.outgoing);
        frame.fill(&inner_circle, style.background);

        vec![frame.into_geometry()]
    }
}

/// Creates a radial progress bar widget.
///
/// This function returns a `Canvas` widget that displays a radial progress bar
/// with a specified percentage and content. If the content is empty, the
/// percentage will be displayed by default.
///
/// # Example
///
/// ```rust
/// use atoms::widgets::radial_progress_bar;
///
/// let progress = radial_progress_bar(0.75, "75% Complete");
/// let default_progress = radial_progress_bar(0.75, "");
/// ```
pub fn donut_chart<Message, Theme: Catalog>(
    percentage: f32,
    label: impl Into<String>,
    font: Font,
) -> Canvas<DonutChart, Message, Theme, Renderer> {
    let radius = 95.0;
    iced::widget::canvas(DonutChart::new(percentage, label, font))
        .width(radius)
        .height(radius)
}

/// The status of a [`ProgressBar`].
#[derive(Debug, Clone, Copy)]
pub enum Status {
    /// The progress bar is idle.
    Idle,
    /// The progress bar is currently progressing.
    Progressing,
    /// The progress bar has finished.
    Finished,
    /// The progress bar has failed.
    Failed,
}
