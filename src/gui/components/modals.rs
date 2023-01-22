use crate::enums::element_type::ElementType;
use crate::enums::message::Message;
use crate::structs::style_tuple::StyleTuple;
use crate::utility::style_constants::{FONT_SIZE_TITLE, INCONSOLATA_BOLD};
use crate::StyleType;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Column, Container, Row, Text};
use iced::Font;
use iced_native::alignment::Alignment;
use iced_native::widget::{self, button, horizontal_space, vertical_space, Tree};
use iced_native::{
    event, layout, mouse, overlay, renderer, Clipboard, Color, Element, Event, Layout, Length,
    Point, Rectangle, Shell, Size, Widget,
};

pub fn get_exit_overlay(style: StyleType, font: Font) -> Container<'static, Message> {
    let row_buttons = Row::new().push(
        button(
            Text::new("Yes")
                .font(font)
                .vertical_alignment(Vertical::Center)
                .horizontal_alignment(Horizontal::Center),
        )
        .padding(5)
        .height(Length::Units(40))
        .width(Length::Units(80))
        .style(StyleTuple(style, ElementType::Alert).into())
        .on_press(Message::Reset),
    );

    let content = Column::new()
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .push(get_modal_header(style))
        .push(vertical_space(Length::Units(20)))
        .push(Text::new("Are you sure you want to quit this analysis?").font(font))
        .push(vertical_space(Length::Units(20)))
        .push(row_buttons);

    Container::new(content)
        .height(Length::Units(150))
        .width(Length::Units(450))
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(style, ElementType::Standard),
        ))
}

fn get_modal_header(style: StyleType) -> Container<'static, Message> {
    Container::new(
        Row::new()
            .push(horizontal_space(Length::FillPortion(1)))
            .push(
                Text::new("Quit analysis")
                    .font(INCONSOLATA_BOLD)
                    .size(FONT_SIZE_TITLE)
                    .width(Length::FillPortion(6))
                    .horizontal_alignment(Horizontal::Center),
            )
            .push(
                Container::new(
                    button(
                        Text::new("x")
                            .font(INCONSOLATA_BOLD)
                            .horizontal_alignment(Horizontal::Center)
                            .size(15),
                    )
                    .padding(2)
                    .height(Length::Units(20))
                    .width(Length::Units(20))
                    .style(StyleTuple(style, ElementType::Standard).into())
                    .on_press(Message::HideModal(false)),
                )
                .width(Length::FillPortion(1))
                .align_x(Horizontal::Center),
            ),
    )
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .height(Length::Units(40))
    .width(Length::Fill)
    .style(<StyleTuple as Into<iced::theme::Container>>::into(
        StyleTuple(style, ElementType::Headers),
    ))
}

/// A widget that centers a modal element over some base element
pub struct Modal<'a, Message, Renderer> {
    base: Element<'a, Message, Renderer>,
    modal: Element<'a, Message, Renderer>,
    on_blur: Option<Message>,
}

impl<'a, Message, Renderer> Modal<'a, Message, Renderer> {
    /// Returns a new [`Modal`]
    pub fn new(
        base: impl Into<Element<'a, Message, Renderer>>,
        modal: impl Into<Element<'a, Message, Renderer>>,
    ) -> Self {
        Self {
            base: base.into(),
            modal: modal.into(),
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

impl<'a, Message, Renderer> Widget<Message, Renderer> for Modal<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer,
    Message: Clone,
{
    fn width(&self) -> Length {
        self.base.as_widget().width()
    }

    fn height(&self) -> Length {
        self.base.as_widget().height()
    }

    fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        self.base.as_widget().layout(renderer, limits)
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &<Renderer as iced_native::Renderer>::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        self.base.as_widget().draw(
            &state.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor_position,
            viewport,
        );
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.base), Tree::new(&self.modal)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.base, &self.modal]);
    }

    fn operate(
        &self,
        state: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation<Message>,
    ) {
        self.base
            .as_widget()
            .operate(&mut state.children[0], layout, renderer, operation);
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        self.base.as_widget_mut().on_event(
            &mut state.children[0],
            event,
            layout,
            cursor_position,
            renderer,
            clipboard,
            shell,
        )
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.base.as_widget().mouse_interaction(
            &state.children[0],
            layout,
            cursor_position,
            viewport,
            renderer,
        )
    }

    fn overlay<'b>(
        &'b mut self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        _renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Renderer>> {
        Some(overlay::Element::new(
            layout.position(),
            Box::new(Overlay {
                content: &mut self.modal,
                tree: &mut state.children[1],
                size: layout.bounds().size(),
                on_blur: self.on_blur.clone(),
            }),
        ))
    }
}

struct Overlay<'a, 'b, Message, Renderer> {
    content: &'b mut Element<'a, Message, Renderer>,
    tree: &'b mut Tree,
    size: Size,
    on_blur: Option<Message>,
}

impl<'a, 'b, Message, Renderer> overlay::Overlay<Message, Renderer>
    for Overlay<'a, 'b, Message, Renderer>
where
    Renderer: iced_native::Renderer,
    Message: Clone,
{
    fn layout(&self, renderer: &Renderer, _bounds: Size, position: Point) -> layout::Node {
        let limits = layout::Limits::new(Size::ZERO, self.size)
            .width(Length::Fill)
            .height(Length::Fill);

        let mut child = self.content.as_widget().layout(renderer, &limits);
        child.align(Alignment::Center, Alignment::Center, limits.max());

        let mut node = layout::Node::with_children(self.size, vec![child]);
        node.move_to(position);

        node
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
    ) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border_radius: renderer::BorderRadius::from(0.0),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
            Color {
                a: 0.8, // background opacity
                ..Color::BLACK
            },
        );

        self.content.as_widget().draw(
            self.tree,
            renderer,
            theme,
            style,
            layout.children().next().unwrap(),
            cursor_position,
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

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        let content_bounds = layout.children().next().unwrap().bounds();

        if let Some(message) = self.on_blur.as_ref() {
            if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = &event {
                if !content_bounds.contains(cursor_position) {
                    shell.publish(message.clone());
                    return event::Status::Captured;
                }
            }
        }

        self.content.as_widget_mut().on_event(
            self.tree,
            event,
            layout.children().next().unwrap(),
            cursor_position,
            renderer,
            clipboard,
            shell,
        )
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            self.tree,
            layout.children().next().unwrap(),
            cursor_position,
            viewport,
            renderer,
        )
    }
}

impl<'a, Message, Renderer> From<Modal<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: 'a + iced_native::Renderer,
    Message: 'a + Clone,
{
    fn from(modal: Modal<'a, Message, Renderer>) -> Self {
        Element::new(modal)
    }
}
