use crate::enums::element_type::ElementType;
use crate::enums::message::Message;
use crate::structs::style_tuple::StyleTuple;
use crate::utility::style_constants::{get_font, get_font_headers, FONT_SIZE_TITLE};
use crate::utility::translations::{
    ask_clear_all_translation, ask_quit_translation, clear_all_translation, hide_translation,
    quit_analysis_translation, yes_translation,
};
use crate::{Language, StyleType};
use iced::alignment::{Alignment, Horizontal, Vertical};
use iced::widget::{
    button, horizontal_space, vertical_space, Column, Container, Row, Text, Tooltip,
};
use iced::{event, mouse, Color, Element, Event, Font, Length, Point, Rectangle, Size};
use iced_native::widget::tooltip::Position;
use iced_native::widget::{self, Tree};
use iced_native::{layout, overlay, renderer, Clipboard, Layout, Shell, Widget};

pub fn get_exit_overlay(
    style: StyleType,
    font: Font,
    language: Language,
) -> Container<'static, Message> {
    let row_buttons = confirm_button_row(language, font, style, Message::Reset);

    let content = Column::new()
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .push(get_modal_header(
            style,
            language,
            quit_analysis_translation(language),
        ))
        .push(vertical_space(Length::Fixed(20.0)))
        .push(ask_quit_translation(language).font(font))
        .push(vertical_space(Length::Fixed(20.0)))
        .push(row_buttons);

    Container::new(content)
        .height(Length::Fixed(150.0))
        .width(Length::Fixed(450.0))
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(style, ElementType::Standard),
        ))
}

pub fn get_clear_all_overlay(
    style: StyleType,
    font: Font,
    language: Language,
) -> Container<'static, Message> {
    let row_buttons = confirm_button_row(language, font, style, Message::ClearAllNotifications);

    let content = Column::new()
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .push(get_modal_header(
            style,
            language,
            clear_all_translation(language),
        ))
        .push(vertical_space(Length::Fixed(20.0)))
        .push(ask_clear_all_translation(language).font(font))
        .push(vertical_space(Length::Fixed(20.0)))
        .push(row_buttons);

    Container::new(content)
        .height(Length::Fixed(150.0))
        .width(Length::Fixed(450.0))
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(style, ElementType::Standard),
        ))
}

fn get_modal_header(
    style: StyleType,
    language: Language,
    title: String,
) -> Container<'static, Message> {
    let font = get_font(style);
    Container::new(
        Row::new()
            .push(horizontal_space(Length::FillPortion(1)))
            .push(
                Text::new(title)
                    .font(get_font_headers(style))
                    .size(FONT_SIZE_TITLE)
                    .width(Length::FillPortion(6))
                    .horizontal_alignment(Horizontal::Center),
            )
            .push(
                Container::new(
                    Tooltip::new(
                        button(
                            Text::new("x")
                                .font(font)
                                .horizontal_alignment(Horizontal::Center)
                                .size(15),
                        )
                        .padding(2)
                        .height(Length::Fixed(20.0))
                        .width(Length::Fixed(20.0))
                        .style(StyleTuple(style, ElementType::Standard).into())
                        .on_press(Message::HideModal(false)),
                        hide_translation(language),
                        Position::Right,
                    )
                    .font(font)
                    .style(<StyleTuple as Into<iced::theme::Container>>::into(
                        StyleTuple(style, ElementType::Tooltip),
                    )),
                )
                .width(Length::FillPortion(1))
                .align_x(Horizontal::Center),
            ),
    )
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .height(Length::Fixed(40.0))
    .width(Length::Fill)
    .style(<StyleTuple as Into<iced::theme::Container>>::into(
        StyleTuple(style, ElementType::Headers),
    ))
}

pub fn confirm_button_row(
    language: Language,
    font: Font,
    style: StyleType,
    message: Message,
) -> Row<'static, Message> {
    Row::new().push(
        button(
            yes_translation(language)
                .font(font)
                .vertical_alignment(Vertical::Center)
                .horizontal_alignment(Horizontal::Center),
        )
        .padding(5)
        .height(Length::Fixed(40.0))
        .width(Length::Fixed(80.0))
        .style(StyleTuple(style, ElementType::Alert).into())
        .on_press(message),
    )
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
