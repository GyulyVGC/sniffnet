//! The DNS analyzer page: a live log of the DNS messages parsed from captured
//! traffic (see [`crate::networking::dns`]).

use iced::widget::scrollable::Direction;
use iced::widget::{Column, Container, PickList, Row, Scrollable, Space, Text};
use iced::{Alignment, Length, Padding};

use crate::gui::components::tab::get_pages_tabs;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::rule::RuleType;
use crate::gui::styles::scrollbar::ScrollbarType;
use crate::gui::styles::style_constants::FONT_SIZE_FOOTER;
use crate::gui::styles::text::TextType;
use crate::gui::types::dns_state::{DnsEntry, DnsRCodeFilter, DnsTypeFilter};
use crate::gui::types::message::Message;
use crate::gui::types::settings::Settings;
use crate::utils::formatted_strings::{clip_text, get_formatted_timestamp};
use crate::utils::types::icon::Icon;
use crate::{RunningPage, Sniffer, StyleType};

// Column widths (in pixels) for the DNS log table.
const W_TIME: f32 = 160.0;
const W_QR: f32 = 45.0;
const W_DOMAIN: f32 = 240.0;
const W_TYPE: f32 = 55.0;
const W_RCODE: f32 = 85.0;
const W_LATENCY: f32 = 80.0;
const W_ANSWERS: f32 = 280.0;

// Maximum characters displayed per cell before clipping.
const MAX_DOMAIN_CHARS: usize = 38;
const MAX_ANSWERS_CHARS: usize = 48;

/// Number of domains shown in the "most queried" ranking.
const TOP_DOMAINS: usize = 5;

/// Builds the body of the DNS analyzer page.
pub fn dns_page(sniffer: &Sniffer) -> Container<'_, Message, StyleType> {
    let Settings { language, .. } = sniffer.conf.settings;

    let tabs = get_pages_tabs(RunningPage::Dns, language, sniffer.unread_notifications);

    let body = Column::new()
        .width(Length::Fill)
        .padding(10)
        .spacing(10)
        .align_x(Alignment::Center)
        .push(
            Container::new(dns_log(sniffer))
                .align_x(Alignment::Center)
                .padding(Padding::new(7.0).top(10).bottom(3))
                .width(947)
                .height(Length::Fill)
                .class(ContainerType::BorderedRound),
        );

    Container::new(Column::new().height(Length::Fill).push(tabs).push(body))
        .height(Length::Fill)
}

/// The DNS log: a header row plus a scrollable list of entries (newest first),
/// or an empty-state placeholder when no DNS traffic has been seen yet.
fn dns_log<'a>(sniffer: &Sniffer) -> Column<'a, Message, StyleType> {
    let filter = sniffer.dns_filter;
    // Newest first, with the active filters applied.
    let matching: Vec<&DnsEntry> = sniffer
        .dns_state
        .log
        .iter()
        .rev()
        .filter(|e| filter.matches(e))
        .collect();

    let col = Column::new()
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Start)
        .push(summary_row(sniffer.dns_state.len(), matching.len(), filter.is_active()))
        .push(ranking_section(sniffer))
        .push(Space::new().height(4))
        .push(filter_row(sniffer))
        .push(Space::new().height(4))
        .push(header_row())
        .push(RuleType::Standard.horizontal(5));

    if matching.is_empty() {
        let message = if sniffer.dns_state.is_empty() {
            "No DNS traffic captured yet"
        } else {
            "No DNS messages match the current filter"
        };
        return col.push(empty_state(message));
    }

    let mut scroll = Column::new().align_x(Alignment::Start);
    for entry in matching {
        scroll = scroll.push(log_row(entry));
    }

    col.push(
        Scrollable::with_direction(scroll, Direction::Vertical(ScrollbarType::properties()))
            .height(Length::Fill)
            .width(Length::Fill),
    )
}

/// Record-type and response-code filter dropdowns.
fn filter_row<'a>(sniffer: &Sniffer) -> Row<'a, Message, StyleType> {
    let type_pick = PickList::new(
        &DnsTypeFilter::ALL[..],
        Some(sniffer.dns_filter.record_type),
        Message::DnsTypeFilterSelection,
    )
    .padding([2, 7]);

    let rcode_pick = PickList::new(
        &DnsRCodeFilter::ALL[..],
        Some(sniffer.dns_filter.rcode),
        Message::DnsRCodeFilterSelection,
    )
    .padding([2, 7]);

    Row::new()
        .padding([0, 2])
        .spacing(10)
        .align_y(Alignment::Center)
        .push(Text::new("Filter:").size(FONT_SIZE_FOOTER).class(TextType::Subtitle))
        .push(type_pick)
        .push(rcode_pick)
}

fn summary_row<'a>(total: usize, shown: usize, filter_active: bool) -> Row<'a, Message, StyleType> {
    let label = if filter_active {
        format!("DNS messages captured: {total} (showing {shown})")
    } else {
        format!("DNS messages captured: {total}")
    };
    Row::new()
        .padding([0, 2])
        .align_y(Alignment::Center)
        .push(Text::new(label).class(TextType::Title))
}

/// A vertical "most queried domains" ranking: a title followed by one domain
/// per line, to avoid horizontal overflow.
fn ranking_section<'a>(sniffer: &Sniffer) -> Column<'a, Message, StyleType> {
    let top = sniffer.dns_state.top_domains(TOP_DOMAINS);
    let mut col = Column::new().padding([0, 2]).spacing(1).align_x(Alignment::Start);
    if top.is_empty() {
        return col;
    }
    col = col.push(Text::new("Top domains").size(FONT_SIZE_FOOTER).class(TextType::Subtitle));
    for (rank, (domain, count)) in top.into_iter().enumerate() {
        col = col.push(
            Text::new(format!("{}. {} ({})", rank + 1, clip_text(&domain, 60), count))
                .size(FONT_SIZE_FOOTER),
        );
    }
    col
}

fn header_row<'a>() -> Row<'a, Message, StyleType> {
    let titles = [
        ("Time", W_TIME),
        ("Q/R", W_QR),
        ("Domain", W_DOMAIN),
        ("Type", W_TYPE),
        ("RCODE", W_RCODE),
        ("Latency", W_LATENCY),
        ("Answer(s)", W_ANSWERS),
    ];
    let mut row = Row::new().padding([0, 2]).align_y(Alignment::Center);
    for (title, width) in titles {
        row = row.push(
            Container::new(Text::new(title).class(TextType::Title))
                .align_x(Alignment::Center)
                .width(width),
        );
    }
    row
}

fn log_row<'a>(entry: &DnsEntry) -> Row<'a, Message, StyleType> {
    // Responses and queries are colored differently for quick scanning.
    let text_type = if entry.is_response {
        TextType::Incoming
    } else {
        TextType::Outgoing
    };

    let qtype = entry
        .qtype
        .map(|t| t.to_string())
        .unwrap_or_else(|| "-".to_string());
    let qr = if entry.is_response { "R" } else { "Q" };

    // Latency only applies to responses matched to their query.
    let latency = if entry.is_response {
        entry
            .latency_ms
            .map(|ms| format!("{ms:.1} ms"))
            .unwrap_or_else(|| "-".to_string())
    } else {
        String::new()
    };

    let cells = [
        (get_formatted_timestamp(entry.timestamp), W_TIME),
        (qr.to_string(), W_QR),
        (clip_text(&entry.domain, MAX_DOMAIN_CHARS), W_DOMAIN),
        (qtype, W_TYPE),
        (entry.rcode.to_string(), W_RCODE),
        (latency, W_LATENCY),
        (clip_text(&entry.answers, MAX_ANSWERS_CHARS), W_ANSWERS),
    ];

    let mut row = Row::new().padding([1, 2]).align_y(Alignment::Center);
    for (value, width) in cells {
        row = row.push(
            Container::new(Text::new(value).size(FONT_SIZE_FOOTER).class(text_type))
                .align_x(Alignment::Center)
                .width(width),
        );
    }
    row
}

fn empty_state<'a>(message: &'a str) -> Column<'a, Message, StyleType> {
    Column::new()
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .align_x(Alignment::Center)
        .push(Space::new().height(Length::Fill))
        .push(Icon::Globe.to_text().size(60))
        .push(Space::new().height(15))
        .push(Text::new(message))
        .push(Space::new().height(Length::FillPortion(2)))
}
