use iced::{Color, Degrees, Gradient};
use serde::{Deserialize, Serialize};

use crate::gui::styles::types::palette::{mix_colors, Palette};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default, Serialize, Deserialize)]
pub enum GradientType {
    /// A harmonious color gradient
    Mild,
    /// A crazy yet good-looking color gradient
    Wild,
    /// No gradient applied
    #[default]
    None,
}

pub fn get_gradient_headers(
    colors: &Palette,
    gradient_type: GradientType,
    is_nightly: bool,
) -> Gradient {
    let mix = if is_nightly {
        Color::BLACK
    } else {
        Color::WHITE
    };
    Gradient::Linear(
        iced::gradient::Linear::new(Degrees(90.0))
            .add_stop(
                0.0,
                match gradient_type {
                    GradientType::Mild => mix_colors(mix, colors.secondary),
                    GradientType::Wild => colors.outgoing,
                    GradientType::None => colors.secondary,
                },
            )
            .add_stop(0.3, colors.secondary)
            .add_stop(0.7, colors.secondary)
            .add_stop(
                1.0,
                match gradient_type {
                    GradientType::Mild => mix_colors(mix, colors.secondary),
                    GradientType::Wild => colors.outgoing,
                    GradientType::None => colors.secondary,
                },
            ),
    )
}

pub fn get_gradient_buttons(
    colors: &Palette,
    gradient_type: GradientType,
    is_nightly: bool,
    alpha: f32,
) -> Gradient {
    let mix = if is_nightly {
        Color::BLACK
    } else {
        Color::WHITE
    };
    Gradient::Linear(
        iced::gradient::Linear::new(Degrees(135.0))
            .add_stop(
                0.0,
                Color {
                    a: alpha,
                    ..match gradient_type {
                        GradientType::Mild => mix_colors(mix, colors.secondary),
                        GradientType::Wild => colors.outgoing,
                        GradientType::None => colors.secondary,
                    }
                },
            )
            .add_stop(
                1.0,
                Color {
                    a: alpha,
                    ..colors.secondary
                },
            ),
    )
}

pub fn get_gradient_hovered_buttons(
    colors: &Palette,
    gradient_type: GradientType,
    is_nightly: bool,
) -> Gradient {
    let mix = if is_nightly {
        Color::BLACK
    } else {
        Color::WHITE
    };
    Gradient::Linear(
        iced::gradient::Linear::new(Degrees(135.0))
            .add_stop(0.0, colors.secondary)
            .add_stop(
                1.0,
                match gradient_type {
                    GradientType::Mild => mix_colors(mix, colors.secondary),
                    GradientType::Wild => colors.outgoing,
                    GradientType::None => colors.secondary,
                },
            ),
    )
}
