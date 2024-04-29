use iced::advanced::layout::{self, Layout};
use iced::advanced::overlay;
use iced::advanced::renderer;
use iced::advanced::widget::{self, Widget};
use iced::advanced::{self, Clipboard, Shell};
use iced::alignment::{Alignment, Horizontal, Vertical};
use iced::widget::{button, horizontal_space, Column, Container, Row, Space, Text};
use iced::{event, mouse, Color, Element, Event, Font, Length, Point, Rectangle, Size, Vector};

use crate::gui::components::button::button_hide;
use crate::gui::styles::button::ButtonType;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::style_constants::FONT_SIZE_TITLE;
use crate::gui::styles::types::gradient_type::GradientType;
use crate::gui::types::message::Message;
use crate::translations::translations::{
    ask_clear_all_translation, ask_quit_translation, clear_all_translation,
    quit_analysis_translation, yes_translation,
};
use crate::{Language, StyleType};

pub fn get_exit_overlay(
    color_gradient: GradientType,
    font: Font,
    font_headers: Font,
    language: Language,
) -> Container<'static, Message, StyleType> {
    let row_buttons = confirm_button_row(language, font, Message::Reset);

    let content = Column::new()
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .push(get_modal_header(
            font,
            font_headers,
            color_gradient,
            language,
            quit_analysis_translation(language),
        ))
        .push(Space::with_height(20))
        .push(
            ask_quit_translation(language)
                .horizontal_alignment(Horizontal::Center)
                .font(font),
        )
        .push(row_buttons);

    Container::new(content)
        .height(160)
        .width(450)
        .style(ContainerType::Modal)
}

pub fn get_clear_all_overlay(
    color_gradient: GradientType,
    font: Font,
    font_headers: Font,
    language: Language,
) -> Container<'static, Message, StyleType> {
    let row_buttons = confirm_button_row(language, font, Message::ClearAllNotifications);

    let content = Column::new()
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .push(get_modal_header(
            font,
            font_headers,
            color_gradient,
            language,
            clear_all_translation(language),
        ))
        .push(Space::with_height(20))
        .push(
            ask_clear_all_translation(language)
                .horizontal_alignment(Horizontal::Center)
                .font(font),
        )
        .push(row_buttons);

    Container::new(content)
        .height(160)
        .width(450)
        .style(ContainerType::Modal)
}

fn get_modal_header(
    font: Font,
    font_headers: Font,
    color_gradient: GradientType,
    language: Language,
    title: &'static str,
) -> Container<'static, Message, StyleType> {
    Container::new(
        Row::new()
            .push(horizontal_space())
            .push(
                Text::new(title)
                    .font(font_headers)
                    .size(FONT_SIZE_TITLE)
                    .width(Length::FillPortion(6))
                    .horizontal_alignment(Horizontal::Center),
            )
            .push(
                Container::new(button_hide(Message::HideModal, language, font))
                    .width(Length::Fill)
                    .align_x(Horizontal::Center),
            ),
    )
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .height(40)
    .width(Length::Fill)
    .style(ContainerType::Gradient(color_gradient))
}

fn confirm_button_row(
    language: Language,
    font: Font,
    message: Message,
) -> Row<'static, Message, StyleType> {
    Row::new()
        .height(Length::Fill)
        .align_items(Alignment::Center)
        .push(
            button(
                yes_translation(language)
                    .font(font)
                    .vertical_alignment(Vertical::Center)
                    .horizontal_alignment(Horizontal::Center),
            )
            .padding(5)
            .height(40)
            .width(80)
            .style(ButtonType::Alert)
            .on_press(message),
        )
}

/// A widget that centers an overlay element over some base element
pub struct Modal<'a, Message, Theme, Renderer> {
    base: Element<'a, Message, Theme, Renderer>,
    overlay: Element<'a, Message, Theme, Renderer>,
    on_blur: Option<Message>,
}

impl<'a, Message, Theme, Renderer> Modal<'a, Message, Theme, Renderer> {
    /// Returns a new [`Modal`]
    pub fn new(
        base: impl Into<Element<'a, Message, Theme, Renderer>>,
        modal: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        Self {
            base: base.into(),
            overlay: modal.into(),
            on_blur: None,
        }
    }

    /// Sets the message that will be produces when the background
    /// of the [`Modal`] is pressed
    pub fn on_blur(self, on_blur: Message) -> Self {
        Self {
            on_blur: Some(on_blur),
            ..self
        }
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Modal<'a, Message, Theme, Renderer>
where
    Renderer: advanced::Renderer,
    Message: Clone,
{
    fn children(&self) -> Vec<widget::Tree> {
        vec![
            widget::Tree::new(&self.base),
            widget::Tree::new(&self.overlay),
        ]
    }

    fn diff(&self, tree: &mut widget::Tree) {
        tree.diff_children(&[&self.base, &self.overlay]);
    }

    fn size(&self) -> Size<Length> {
        self.base.as_widget().size()
    }

    fn layout(
        &self,
        tree: &mut widget::Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.base
            .as_widget()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn on_event(
        &mut self,
        state: &mut widget::Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        self.base.as_widget_mut().on_event(
            &mut state.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        )
    }

    fn draw(
        &self,
        state: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.base.as_widget().draw(
            &state.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }

    fn overlay<'b>(
        &'b mut self,
        state: &'b mut widget::Tree,
        layout: Layout<'_>,
        _renderer: &Renderer,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        Some(overlay::Element::new(Box::new(Overlay {
            position: layout.position() + translation,
            content: &mut self.overlay,
            tree: &mut state.children[1],
            size: layout.bounds().size(),
            on_blur: self.on_blur.clone(),
        })))
    }

    fn mouse_interaction(
        &self,
        state: &widget::Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.base.as_widget().mouse_interaction(
            &state.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn operate(
        &self,
        state: &mut widget::Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation<Message>,
    ) {
        self.base
            .as_widget()
            .operate(&mut state.children[0], layout, renderer, operation);
    }
}

struct Overlay<'a, 'b, Message, Theme, Renderer> {
    position: Point,
    content: &'b mut Element<'a, Message, Theme, Renderer>,
    tree: &'b mut widget::Tree,
    size: Size,
    on_blur: Option<Message>,
}

impl<'a, 'b, Message, Theme, Renderer> overlay::Overlay<Message, Theme, Renderer>
    for Overlay<'a, 'b, Message, Theme, Renderer>
where
    Renderer: advanced::Renderer,
    Message: Clone,
{
    fn layout(&mut self, renderer: &Renderer, _bounds: Size) -> layout::Node {
        let limits = layout::Limits::new(Size::ZERO, self.size)
            .width(Length::Fill)
            .height(Length::Fill);

        let child = self
            .content
            .as_widget()
            .layout(self.tree, renderer, &limits)
            .align(Alignment::Center, Alignment::Center, limits.max());

        layout::Node::with_children(self.size, vec![child]).move_to(self.position)
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        let content_bounds = layout.children().next().unwrap().bounds();

        if let Some(message) = self.on_blur.as_ref() {
            if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = &event {
                if !cursor.is_over(content_bounds) {
                    shell.publish(message.clone());
                    return event::Status::Captured;
                }
            }
        }

        self.content.as_widget_mut().on_event(
            self.tree,
            event,
            layout.children().next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            &layout.bounds(),
        )
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                ..renderer::Quad::default()
            },
            Color {
                a: 0.80,
                ..Color::BLACK
            },
        );

        self.content.as_widget().draw(
            self.tree,
            renderer,
            theme,
            style,
            layout.children().next().unwrap(),
            cursor,
            &layout.bounds(),
        );
    }

    fn operate(
        &mut self,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation<Message>,
    ) {
        self.content.as_widget().operate(
            self.tree,
            layout.children().next().unwrap(),
            renderer,
            operation,
        );
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            self.tree,
            layout.children().next().unwrap(),
            cursor,
            viewport,
            renderer,
        )
    }

    fn overlay<'c>(
        &'c mut self,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'c, Message, Theme, Renderer>> {
        self.content.as_widget_mut().overlay(
            self.tree,
            layout.children().next().unwrap(),
            renderer,
            Vector::ZERO,
        )
    }
}

impl<'a, Message, Theme, Renderer> From<Modal<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Theme: 'a,
    Message: 'a + Clone,
    Renderer: 'a + advanced::Renderer,
{
    fn from(modal: Modal<'a, Message, Theme, Renderer>) -> Self {
        Element::new(modal)
    }
}
