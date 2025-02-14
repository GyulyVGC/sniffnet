use crate::chart::types::donut_chart::Status;
use crate::gui::styles::button::ButtonType;
use crate::gui::styles::types::style_type::StyleType;
use iced::Color;

#[derive(Default)]
pub enum DonutType {
    #[default]
    Standard,
}

impl DonutType {
    fn active(&self, style: &StyleType) -> Style {
        let colors = style.get_palette();
        let ext = style.get_extension();
        Style {
            background: Color::TRANSPARENT,
            incoming: colors.secondary,
            outgoing: colors.outgoing,
            rail: colors.primary,
            text_color: colors.text_body,
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
    pub(crate) incoming: Color,
    pub(crate) outgoing: Color,
    pub(crate) rail: Color,
    pub(crate) text_color: Color,
}

/// The theme catalog of a [`ProgressBar`].
pub trait Catalog: Sized {
    /// The item class of the [`Catalog`].
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &Self::Class<'_>) -> Style;
}
