use crate::chart::types::donut_kind::DonutKind;
use crate::gui::styles::donut::Catalog;
use crate::gui::styles::style_constants::FONT_SIZE_BODY;
use crate::translations::types::language::Language;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::canvas::path::Arc;
use iced::widget::canvas::{Frame, Text};
use iced::widget::text::Shaping;
use iced::widget::{canvas, Canvas};
use iced::{mouse, Font, Point, Radians, Renderer};
use std::f32::consts;

pub struct DonutChart {
    kind: DonutKind,
    font: Font,
    language: Language,
}

impl DonutChart {
    pub fn new(kind: DonutKind, font: Font, language: Language) -> Self {
        Self {
            kind,
            font,
            language,
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

        let style = <Theme as Catalog>::style(theme, &<Theme as Catalog>::default());
        let colors = match self.kind {
            DonutKind::Total(..) => {
                vec![
                    style.incoming,
                    style.outgoing,
                    style.filtered_out,
                    style.dropped,
                ]
            }
            DonutKind::Ip => {
                vec![]
            }
            DonutKind::Proto => {
                vec![]
            }
        };

        for (i, angles) in self.kind.get_angles().iter().enumerate() {
            let path = canvas::Path::new(|builder| {
                builder.arc(Arc {
                    center,
                    radius,
                    start_angle: angles.0,
                    end_angle: angles.1,
                });
                builder.line_to(center);
                builder.close();
            });

            frame.fill(&path, colors[i]);
        }

        let inner_circle = canvas::Path::circle(center, radius - 6.0);
        frame.fill(&inner_circle, style.background);
        frame.fill_text(Text {
            content: self.kind.get_title().clone(),
            position: center,
            vertical_alignment: Vertical::Center,
            horizontal_alignment: Horizontal::Center,
            color: style.text_color,
            size: FONT_SIZE_BODY.into(),
            font: self.font,
            ..Default::default()
        });

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
    kind: DonutKind,
    font: Font,
    language: Language,
) -> Canvas<DonutChart, Message, Theme, Renderer> {
    let radius = 95.0;
    iced::widget::canvas(DonutChart::new(kind, font, language))
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
