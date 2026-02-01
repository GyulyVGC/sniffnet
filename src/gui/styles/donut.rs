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
        Style {
            incoming: colors.secondary,
            outgoing: colors.outgoing,
            text_color: colors.text_body,
            dropped: ext.buttons_color,
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
    pub(crate) text_color: Color,
    pub(crate) incoming: Color,
    pub(crate) outgoing: Color,
    pub(crate) dropped: Color,
}

pub trait Catalog: Sized {
    type Class<'a>;

    fn default<'a>() -> Self::Class<'a>;

    fn style(&self, class: &Self::Class<'_>) -> Style;
}
