//! Module defining the initial page of the application.
//!
//! It contains elements to select network adapter and traffic filters.

use std::collections::HashSet;
use std::fmt::Write;

use iced::Length::FillPortion;
use iced::widget::scrollable::Direction;
use iced::widget::tooltip::Position;
use iced::widget::{
    Button, Checkbox, Column, Container, Row, Rule, Scrollable, Space, Text, TextInput, Tooltip,
    button, center,
};
use iced::{Alignment, Font, Length, Padding, alignment};

use crate::gui::components::button::button_open_file;
use crate::gui::sniffer::Sniffer;
use crate::gui::styles::button::ButtonType;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::scrollbar::ScrollbarType;
use crate::gui::styles::style_constants::{FONT_SIZE_SUBTITLE, FONT_SIZE_TITLE};
use crate::gui::styles::text::TextType;
use crate::gui::styles::text_input::TextInputType;
use crate::gui::styles::types::gradient_type::GradientType;
use crate::gui::types::export_pcap::ExportPcap;
use crate::gui::types::message::Message;
use crate::networking::types::capture_context::CaptureSource;
use crate::networking::types::filters::Filters;
use crate::networking::types::ip_collection::AddressCollection;
use crate::networking::types::port_collection::PortCollection;
use crate::translations::translations::{
    address_translation, addresses_translation, choose_adapters_translation,
    ip_version_translation, protocol_translation, select_filters_translation, start_translation,
};
use crate::translations::translations_3::{
    directory_translation, export_capture_translation, file_name_translation, port_translation,
};
use crate::translations::translations_4::import_capture_translation;
use crate::utils::formatted_strings::{get_invalid_filters_string, get_path_termination_string};
use crate::utils::types::file_info::FileInfo;
use crate::utils::types::icon::Icon;
use crate::{ConfigSettings, IpVersion, Language, Protocol, StyleType};

/// Computes the body of gui initial page
pub fn initial_page(sniffer: &Sniffer) -> Container<Message, StyleType> {
    let ConfigSettings {
        style,
        language,
        color_gradient,
        ..
    } = sniffer.configs.settings;
    let font = style.get_extension().font;

    let col_adapter = get_col_adapter(sniffer, font);
    let col_import_pcap = get_col_import_pcap(
        language,
        font,
        &sniffer.capture_source,
        &sniffer.import_pcap_path,
    );
    let col_capture_source = Column::new().push(col_adapter).push(col_import_pcap);

    let ip_active = &sniffer.filters.ip_versions;
    let col_ip_buttons = col_ip_buttons(ip_active, font, language);

    let protocol_active = &sniffer.filters.protocols;
    let col_protocol_buttons = col_protocol_buttons(protocol_active, font, language);

    let address_active = &sniffer.filters.address_str;
    let col_address_filter = col_address_input(address_active, font, language);

    let port_active = &sniffer.filters.port_str;
    let col_port_filter = col_port_input(port_active, font, language);

    let filters_pane = Column::new()
        .width(FillPortion(6))
        .padding(10)
        .spacing(15)
        .push(
            select_filters_translation(language)
                .font(font)
                .class(TextType::Title)
                .size(FONT_SIZE_TITLE),
        )
        .push(
            Row::new()
                .spacing(20)
                .push(col_ip_buttons)
                .push(col_protocol_buttons),
        )
        .push(
            Row::new()
                .spacing(20)
                .push(col_address_filter)
                .push(col_port_filter),
        )
        .push(Rule::horizontal(40))
        .push(get_export_pcap_group(
            &sniffer.capture_source,
            &sniffer.export_pcap,
            language,
            font,
        ))
        .push(
            Container::new(button_start(
                font,
                language,
                color_gradient,
                &sniffer.filters,
            ))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_y(Alignment::Start)
            .align_x(Alignment::Center),
        );

    let body = Column::new().push(Space::with_height(5)).push(
        Row::new()
            .push(col_capture_source)
            .push(Space::with_width(30))
            .push(filters_pane),
    );

    Container::new(body).height(Length::Fill)
}

fn col_ip_buttons(
    active_ip_filters: &HashSet<IpVersion>,
    font: Font,
    language: Language,
) -> Column<Message, StyleType> {
    let mut buttons_row = Row::new().spacing(5).padding(Padding::ZERO.left(5));
    for option in IpVersion::ALL {
        let is_active = active_ip_filters.contains(&option);
        let check_symbol = if is_active { "✔" } else { "✘" };
        buttons_row = buttons_row.push(
            Button::new(
                Text::new(format!("{option} {check_symbol}"))
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .font(font),
            )
            .width(80)
            .height(35)
            .class(if is_active {
                ButtonType::BorderedRoundSelected
            } else {
                ButtonType::BorderedRound
            })
            .on_press(Message::IpVersionSelection(option, !is_active)),
        );
    }

    Column::new()
        .width(Length::Fill)
        .spacing(7)
        .push(
            Text::new(ip_version_translation(language))
                .font(font)
                .class(TextType::Subtitle)
                .size(FONT_SIZE_SUBTITLE),
        )
        .push(buttons_row)
}

fn col_protocol_buttons(
    active_protocol_filters: &HashSet<Protocol>,
    font: Font,
    language: Language,
) -> Column<Message, StyleType> {
    let mut buttons_row = Row::new().spacing(5).padding(Padding::ZERO.left(5));
    for option in Protocol::ALL {
        let is_active = active_protocol_filters.contains(&option);
        let check_symbol = if is_active { "✔" } else { "✘" };
        buttons_row = buttons_row.push(
            Button::new(
                Text::new(format!("{option} {check_symbol}"))
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .font(font),
            )
            .width(80)
            .height(35)
            .class(if is_active {
                ButtonType::BorderedRoundSelected
            } else {
                ButtonType::BorderedRound
            })
            .on_press(Message::ProtocolSelection(option, !is_active)),
        );
    }

    Column::new()
        .width(Length::Fill)
        .spacing(7)
        .push(
            Text::new(protocol_translation(language))
                .font(font)
                .class(TextType::Subtitle)
                .size(FONT_SIZE_SUBTITLE),
        )
        .push(buttons_row)
}

fn col_address_input(value: &str, font: Font, language: Language) -> Column<Message, StyleType> {
    let is_error = if value.is_empty() {
        false
    } else {
        AddressCollection::new(value).is_none()
    };
    let input_row = Row::new().padding(Padding::ZERO.left(5)).push(
        TextInput::new(AddressCollection::PLACEHOLDER_STR, value)
            .padding([3, 5])
            .on_input(Message::AddressFilter)
            .font(font)
            .width(310)
            .class(if is_error {
                TextInputType::Error
            } else {
                TextInputType::Standard
            }),
    );

    Column::new()
        .width(Length::Fill)
        .spacing(7)
        .push(
            Text::new(address_translation(language))
                .font(font)
                .class(TextType::Subtitle)
                .size(FONT_SIZE_SUBTITLE),
        )
        .push(input_row)
}

fn col_port_input(value: &str, font: Font, language: Language) -> Column<Message, StyleType> {
    let is_error = if value.is_empty() {
        false
    } else {
        PortCollection::new(value).is_none()
    };
    let input_row = Row::new().padding(Padding::ZERO.left(5)).push(
        TextInput::new(PortCollection::PLACEHOLDER_STR, value)
            .padding([3, 5])
            .on_input(Message::PortFilter)
            .font(font)
            .width(180)
            .class(if is_error {
                TextInputType::Error
            } else {
                TextInputType::Standard
            }),
    );

    Column::new()
        .width(Length::Fill)
        .spacing(7)
        .push(
            Text::new(port_translation(language))
                .font(font)
                .class(TextType::Subtitle)
                .size(FONT_SIZE_SUBTITLE),
        )
        .push(input_row)
}

fn button_start(
    font: Font,
    language: Language,
    color_gradient: GradientType,
    filters: &Filters,
) -> Tooltip<Message, StyleType> {
    let mut content = button(
        Icon::Rocket
            .to_text()
            .size(25)
            .align_x(alignment::Alignment::Center)
            .align_y(alignment::Alignment::Center),
    )
    .padding(10)
    .height(80)
    .width(160)
    .class(ButtonType::Gradient(color_gradient));

    let mut tooltip = start_translation(language).to_string();
    //tooltip.push_str(" [⏎]");
    let mut position = Position::Top;

    if filters.are_valid() {
        content = content.on_press(Message::Start);
    } else {
        tooltip = get_invalid_filters_string(filters, language);
        position = Position::FollowCursor;
    }

    Tooltip::new(content, Text::new(tooltip).font(font), position)
        .gap(5)
        .class(ContainerType::Tooltip)
}

fn get_col_adapter(sniffer: &Sniffer, font: Font) -> Column<Message, StyleType> {
    let ConfigSettings { language, .. } = sniffer.configs.settings;

    let mut dev_str_list = vec![];
    for my_dev in &sniffer.my_devices {
        let mut title = String::new();
        #[allow(unused_mut)]
        let mut subtitle: Option<&String> = None;
        let name = my_dev.get_name();
        match my_dev.get_desc() {
            None => {
                title.push_str(name);
            }
            Some(description) => {
                #[cfg(not(target_os = "windows"))]
                {
                    let _ = writeln!(title, "{name}");
                    subtitle = Some(description);
                }
                #[cfg(target_os = "windows")]
                title.push_str(description);
            }
        }
        let mut addrs_str = String::new();
        let num_addresses = my_dev.get_addresses().len();
        match num_addresses {
            0 => {}
            1 => {
                let _ = write!(addrs_str, "{}:", address_translation(language));
            }
            _ => {
                let _ = write!(addrs_str, "{}:", addresses_translation(language));
            }
        }

        for addr in my_dev.get_addresses() {
            let address_string = addr.addr.to_string();
            let _ = write!(addrs_str, "\n   {address_string}");
        }
        dev_str_list.push((name, title, subtitle, addrs_str));
    }

    Column::new()
        .padding(10)
        .spacing(5)
        .height(Length::Fill)
        .width(FillPortion(4))
        .push(
            choose_adapters_translation(language)
                .font(font)
                .class(TextType::Title)
                .size(FONT_SIZE_TITLE),
        )
        .push(if dev_str_list.is_empty() {
            Into::<iced::Element<Message, StyleType>>::into(center(
                Icon::get_hourglass(sniffer.dots_pulse.0.len()).size(60),
            ))
        } else {
            Scrollable::with_direction(
                dev_str_list.into_iter().fold(
                    Column::new().padding(13).spacing(5),
                    |scroll_adapters, (name, title, subtitle, addrs)| {
                        let addrs_text = if addrs.is_empty() {
                            None
                        } else {
                            Some(Text::new(addrs).font(font))
                        };
                        scroll_adapters.push(
                            Button::new(
                                Column::new()
                                    .spacing(5)
                                    .push(
                                        Text::new(title)
                                            .font(font)
                                            .class(TextType::Subtitle)
                                            .size(FONT_SIZE_SUBTITLE),
                                    )
                                    .push_maybe(subtitle.map(|sub| Text::new(sub).font(font)))
                                    .push_maybe(addrs_text),
                            )
                            .padding([20, 30])
                            .width(Length::Fill)
                            .class(
                                if let CaptureSource::Device(device) = &sniffer.capture_source {
                                    if name == device.get_name() {
                                        ButtonType::BorderedRoundSelected
                                    } else {
                                        ButtonType::BorderedRound
                                    }
                                } else {
                                    ButtonType::BorderedRound
                                },
                            )
                            .on_press(Message::DeviceSelection(name.to_string())),
                        )
                    },
                ),
                Direction::Vertical(ScrollbarType::properties()),
            )
            .into()
        })
}

fn get_col_import_pcap<'a>(
    language: Language,
    font: Font,
    cs: &CaptureSource,
    path: &String,
) -> Column<'a, Message, StyleType> {
    let is_import_pcap_set = matches!(cs, CaptureSource::File(_));

    let button_row = Row::new()
        .align_y(Alignment::Center)
        .push(Text::new(get_path_termination_string(path, 25)).font(font))
        .push(button_open_file(
            path.clone(),
            FileInfo::PcapImport,
            language,
            font,
            true,
            Message::SetPcapImport,
        ));

    let content = Column::new()
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .spacing(5)
        .push(button_row);

    let button = Container::new(
        Button::new(content)
            .width(Length::Fill)
            .padding([20, 30])
            .class(if is_import_pcap_set {
                ButtonType::BorderedRoundSelected
            } else {
                ButtonType::BorderedRound
            })
            .on_press(Message::SetPcapImport(path.to_string())),
    )
    .padding(13);

    Column::new()
        .padding(10)
        .spacing(5)
        .width(FillPortion(4))
        .push(
            Text::new(import_capture_translation(language))
                .font(font)
                .class(TextType::Title)
                .size(FONT_SIZE_TITLE),
        )
        .push(button)
}

fn get_export_pcap_group<'a>(
    cs: &CaptureSource,
    export_pcap: &ExportPcap,
    language: Language,
    font: Font,
) -> Container<'a, Message, StyleType> {
    if matches!(cs, CaptureSource::File(_)) {
        return Container::new(Space::with_height(Length::Fill));
    }

    let enabled = export_pcap.enabled();
    let file_name = export_pcap.file_name();
    let directory = export_pcap.directory();

    let caption = export_capture_translation(language);
    let checkbox = Checkbox::new(caption, enabled)
        .on_toggle(move |_| Message::ToggleExportPcap)
        .size(18)
        .font(font);

    let mut ret_val = Column::new().spacing(10).push(checkbox);

    if enabled {
        let inner_col = Column::new()
            .spacing(10)
            .padding(Padding::ZERO.left(45))
            .push(
                Row::new()
                    .align_y(Alignment::Center)
                    .spacing(5)
                    .push(Text::new(format!("{}:", file_name_translation(language))).font(font))
                    .push(
                        TextInput::new(ExportPcap::DEFAULT_FILE_NAME, file_name)
                            .on_input(Message::OutputPcapFile)
                            .padding([2, 5])
                            .font(font)
                            .width(200),
                    ),
            )
            .push(
                Row::new()
                    .align_y(Alignment::Center)
                    .spacing(5)
                    .push(Text::new(format!("{}:", directory_translation(language))).font(font))
                    .push(Text::new(get_path_termination_string(directory, 25)).font(font))
                    .push(button_open_file(
                        directory.to_owned(),
                        FileInfo::Directory,
                        language,
                        font,
                        true,
                        Message::OutputPcapDir,
                    )),
            );
        ret_val = ret_val.push(inner_col);
    }

    Container::new(
        Container::new(ret_val)
            .padding(10)
            .width(Length::Fill)
            .class(ContainerType::BorderedRound),
    )
    .height(Length::Fill)
    .align_y(Alignment::Start)
}
