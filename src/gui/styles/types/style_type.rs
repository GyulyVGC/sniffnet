use crate::get_colors;
use crate::gui::styles::container::ContainerType;
use iced::application::Appearance;
use iced::widget::{container, text};
use iced::{application, Background, Color};
use serde::{Deserialize, Serialize};

use crate::gui::styles::types::custom_palette::ExtraStyles;

/// Used to specify the kind of style of the application
#[derive(Clone, Copy, Serialize, Deserialize, Debug, Hash, PartialEq)]
#[serde(tag = "style", content = "name")]
pub enum StyleType {
    Night,
    Day,
    DeepSea,
    MonAmour,
    Custom(ExtraStyles),
}

impl Default for StyleType {
    fn default() -> Self {
        Self::Night
    }
}

impl application::StyleSheet for StyleType {
    type Style = ();

    fn appearance(&self, _: &Self::Style) -> Appearance {
        let colors = get_colors(*self);
        Appearance {
            background_color: colors.primary,
            text_color: colors.text_body,
        }
    }
}

impl StyleType {
    pub fn is_nightly(self) -> bool {
        match self {
            StyleType::Night | StyleType::DeepSea => true,
            StyleType::Day | StyleType::MonAmour => false,
            StyleType::Custom(style) => style.is_nightly(),
        }
    }
}
