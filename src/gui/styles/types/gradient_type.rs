use iced::{Color, Degrees, Gradient};
use crate::gui::styles::types::palette::{mix_colors, Palette};

pub enum GradientType {
    /// An harmonious color gradient
    Mild,
    /// A crazy, good-looking color gradient
    Wild,
}

const MID_GRAY: Color = Color {
    r: 0.5,
    g: 0.5,
    b: 0.5,
    a: 1.0
};

pub fn get_gradient(colors: &Palette, gradient_type: GradientType) -> Gradient {
    Gradient::Linear(
        iced::gradient::Linear::new(Degrees(180.0))
            .add_stop(0.0, match gradient_type {
                GradientType::Mild => {MID_GRAY}
                GradientType::Wild => {colors.outgoing}
            })
            .add_stop(0.3, colors.secondary)
            .add_stop(0.7, colors.secondary)
            .add_stop(1.0, match gradient_type {
                GradientType::Mild => {MID_GRAY}
                GradientType::Wild => {colors.outgoing}
            })
    )
}
