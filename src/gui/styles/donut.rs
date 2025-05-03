use crate::gui::styles::types::style_type::StyleType;
use iced::Color;

#[derive(Default)]
pub enum DonutType {
    #[default]
    Standard,
}

impl DonutType {
    #[allow(clippy::unused_self)]
    fn active(&self, style: &StyleType) -> Style {
        let colors = style.get_palette();
        let ext = style.get_extension();
        let primary = colors.primary;
        let buttons = ext.buttons_color;
        let background = Color {
            r: primary.r + (buttons.r - primary.r) * ext.alpha_round_containers,
            g: primary.g + (buttons.g - primary.g) * ext.alpha_round_containers,
            b: primary.b + (buttons.b - primary.b) * ext.alpha_round_containers,
            a: 1.0,
        };
        Style {
            background,
            incoming: colors.secondary,
            outgoing: colors.outgoing,
            text_color: colors.text_body,
            filtered_out: ext.buttons_color,
            dropped: ext.red_alert_color,
        }
    }
}

impl Catalog for StyleType {
    type Class<'a> = DonutType;

    fn default<'a>() -> Self::Class<'a> {
        Self::Class::default()
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class.active(self)
    }
}

pub struct Style {
    pub(crate) background: Color,
    pub(crate) text_color: Color,
    pub(crate) incoming: Color,
    pub(crate) outgoing: Color,
    pub(crate) filtered_out: Color,
    pub(crate) dropped: Color,
}

pub trait Catalog: Sized {
    type Class<'a>;

    fn default<'a>() -> Self::Class<'a>;

    fn style(&self, class: &Self::Class<'_>) -> Style;
}
