use iced::advanced::layout::{self, Layout};
use iced::advanced::overlay;
use iced::advanced::renderer;
use iced::advanced::widget::{self, Widget};
use iced::advanced::{self, Clipboard, Shell};
use iced::alignment::{Alignment, Horizontal, Vertical};
use iced::widget::tooltip::Position;
use iced::widget::{
    button, horizontal_space, vertical_space, Column, Container, Row, Text, Tooltip,
};
use iced::{
    event, mouse, BorderRadius, Color, Element, Event, Font, Length, Point, Rectangle, Size,
};

use crate::gui::styles::button::{ButtonStyleTuple, ButtonType};
use crate::gui::styles::container::{ContainerStyleTuple, ContainerType};
use crate::gui::styles::style_constants::{get_font, get_font_headers, FONT_SIZE_TITLE};
use crate::gui::styles::types::gradient_type::GradientType;
use crate::gui::types::message::Message;
use crate::translations::translations::{
    ask_clear_all_translation, ask_quit_translation, clear_all_translation, hide_translation,
    quit_analysis_translation, yes_translation,
};
use crate::{Language, StyleType};

pub fn get_exit_overlay(
    style: StyleType,
    color_gradient: GradientType,
    font: Font,
    language: Language,
) -> Container<'static, Message> {
    let row_buttons = confirm_button_row(language, font, style, Message::Reset);

    let content = Column::new()
        .padding(0)
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .push(get_modal_header(
            style,
            color_gradient,
            language,
            quit_analysis_translation(language),
        ))
        .push(vertical_space(Length::Fixed(20.0)))
        .push(
            ask_quit_translation(language)
                .horizontal_alignment(Horizontal::Center)
                .font(font),
        )
        .push(row_buttons);

    Container::new(content)
        .height(Length::Fixed(160.0))
        .width(Length::Fixed(450.0))
        .style(ContainerType::Modal)
}

pub fn get_clear_all_overlay(
    style: StyleType,
    color_gradient: GradientType,
    font: Font,
    language: Language,
) -> Container<'static, Message> {
    let row_buttons = confirm_button_row(language, font, style, Message::ClearAllNotifications);

    let content = Column::new()
        .padding(0)
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .push(get_modal_header(
            style,
            color_gradient,
            language,
            clear_all_translation(language),
        ))
        .push(vertical_space(Length::Fixed(20.0)))
        .push(
            ask_clear_all_translation(language)
                .horizontal_alignment(Horizontal::Center)
                .font(font),
        )
        .push(row_buttons);

    Container::new(content)
        .height(Length::Fixed(160.0))
        .width(Length::Fixed(450.0))
        .style(ContainerType::Modal)
}

fn get_modal_header(
    style: StyleType,
    color_gradient: GradientType,
    language: Language,
    title: String,
) -> Container<'static, Message> {
    let font = get_font(style);
    let tooltip = hide_translation(language).to_string();
    //tooltip.push_str(" [esc]");
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
                            Text::new("×")
                                .font(font)
                                .vertical_alignment(Vertical::Center)
                                .horizontal_alignment(Horizontal::Center)
                                .size(15),
                        )
                        .padding(2)
                        .height(Length::Fixed(20.0))
                        .width(Length::Fixed(20.0))
                        .style(ButtonStyleTuple(style, ButtonType::Standard).into())
                        .on_press(Message::HideModal),
                        tooltip,
                        Position::Right,
                    )
                    .font(font)
                    .style(
                        ContainerType::Tooltip, ),
                )
                .width(Length::FillPortion(1))
                .align_x(Horizontal::Center),
            ),
    )
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .height(Length::Fixed(40.0))
    .width(Length::Fill)
    .style(ContainerType::Gradient(color_gradient))
}

fn confirm_button_row(
    language: Language,
    font: Font,
    style: StyleType,
    message: Message,
) -> Row<'static, Message> {
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
            .height(Length::Fixed(40.0))
            .width(Length::Fixed(80.0))
            .style(ButtonStyleTuple(style, ButtonType::Alert).into())
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
    Renderer: advanced::Renderer,
    Message: Clone,
{
    fn children(&self) -> Vec<widget::Tree> {
        vec![
            widget::Tree::new(&self.base),
            widget::Tree::new(&self.modal),
        ]
    }

    fn diff(&self, tree: &mut widget::Tree) {
        tree.diff_children(&[&self.base, &self.modal]);
    }

    fn width(&self) -> Length {
        self.base.as_widget().width()
    }

    fn height(&self) -> Length {
        self.base.as_widget().height()
    }

    fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        self.base.as_widget().layout(renderer, limits)
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
        theme: &<Renderer as advanced::Renderer>::Theme,
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

struct Overlay<'a, 'b, Message, Renderer> {
    content: &'b mut Element<'a, Message, Renderer>,
    tree: &'b mut widget::Tree,
    size: Size,
    on_blur: Option<Message>,
}

impl<'a, 'b, Message, Renderer> overlay::Overlay<Message, Renderer>
    for Overlay<'a, 'b, Message, Renderer>
where
    Renderer: advanced::Renderer,
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
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border_radius: BorderRadius::default(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
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
    ) -> Option<overlay::Element<'c, Message, Renderer>> {
        self.content
            .as_widget_mut()
            .overlay(self.tree, layout.children().next().unwrap(), renderer)
    }
}

impl<'a, Message, Renderer> From<Modal<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: 'a + advanced::Renderer,
    Message: 'a + Clone,
{
    fn from(modal: Modal<'a, Message, Renderer>) -> Self {
        Element::new(modal)
    }
}
