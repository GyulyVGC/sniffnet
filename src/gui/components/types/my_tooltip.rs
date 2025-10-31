use iced::widget::{Tooltip, Container};
use iced::widget::tooltip::Position;
use iced::Element;

use crate::gui::styles::container::ContainerType;
use crate::gui::types::message::Message;
use crate::gui::styles::types::style_type::StyleType;

pub struct MyTooltip<'a> {
    content: Element<'a, Message, StyleType>,
    label: Element<'a, Message, StyleType>,
    position: Position,
    gap: f32,
    padding: f32,
    enabled: bool,
    style: ContainerType,
    snap_within_viewport: bool,
}

impl<'a> MyTooltip<'a> {
    pub fn new(
        content: impl Into<Element<'a, Message, StyleType>>,
        label: impl Into<Element<'a, Message, StyleType>>,
    ) -> Self {
        Self {
            content: content.into(),
            label: label.into(),
            position: Position::Right,
            gap: 0.0,
            padding: 5.0,
            enabled: true,
            style: ContainerType::Tooltip,
            snap_within_viewport: false,
        }
    }

    pub fn position(mut self, position: Position) -> Self {
        self.position = position;
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn style(mut self, style: ContainerType) -> Self {
        self.style = style;
        self
    }

    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    pub fn snap_within_viewport(mut self) -> Self {
        self.snap_within_viewport = true;
        self
    }

    pub fn build(self) -> Element<'a, Message, StyleType> {
        if self.enabled {
            Tooltip::new(self.content, self.label, self.position)
                .gap(self.gap)
                .class(self.style)
                .padding(self.padding)
                .snap_within_viewport(self.snap_within_viewport)
                .into()
        } else {
            Container::new(self.content)
                .class(ContainerType::Transparent)
                .into()
        }
    }
}
