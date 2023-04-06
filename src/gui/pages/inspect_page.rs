use crate::gui::components::tab::get_pages_tabs;
use crate::gui::styles::types::element_type::ElementType;
use crate::gui::styles::types::style_tuple::StyleTuple;
use crate::gui::types::message::Message;
use crate::{RunningPage, Sniffer};
use iced::widget::{Column, Container};
use iced::{Alignment, Length};

/// Computes the body of gui inspect page
pub fn inspect_page(sniffer: &Sniffer) -> Container<Message> {
    //let font = get_font(sniffer.style);

    let body = Column::new()
        .width(Length::Fill)
        .padding(5)
        .spacing(5)
        .align_items(Alignment::Center);

    let mut tab_and_body = Column::new().height(Length::Fill);

    let tabs = get_pages_tabs(
        [
            RunningPage::Overview,
            RunningPage::Inspect,
            RunningPage::Notifications,
        ],
        &["d ", "5 ", "7 "],
        &[
            Message::ChangeRunningPage(RunningPage::Overview),
            Message::TickInit,
            Message::ChangeRunningPage(RunningPage::Notifications),
        ],
        RunningPage::Inspect,
        sniffer.style,
        sniffer.language,
        sniffer.unread_notifications,
    );

    tab_and_body = tab_and_body.push(tabs);

    Container::new(Column::new().push(tab_and_body.push(body)))
        .height(Length::Fill)
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(sniffer.style, ElementType::Standard),
        ))
}
