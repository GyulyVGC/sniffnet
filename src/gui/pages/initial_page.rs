//! Module defining the initial page of the application.
//!
//! It contains elements to select network adapter and traffic filters.

use std::fmt::Write;

use crate::gui::components::button::button_open_file;
use crate::gui::sniffer::Sniffer;
use crate::gui::styles::button::ButtonType;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::scrollbar::ScrollbarType;
use crate::gui::styles::style_constants::{FONT_SIZE_SUBTITLE, FONT_SIZE_TITLE};
use crate::gui::styles::text::TextType;
use crate::gui::styles::types::gradient_type::GradientType;
use crate::gui::types::export_pcap::ExportPcap;
use crate::gui::types::filters::Filters;
use crate::gui::types::message::Message;
use crate::gui::types::settings::Settings;
use crate::networking::types::capture_context::{CaptureSource, CaptureSourcePicklist};
use crate::translations::translations::{
    address_translation, addresses_translation, network_adapter_translation, start_translation,
};
use crate::translations::translations_3::{
    directory_translation, export_capture_translation, file_name_translation,
};
use crate::translations::translations_4::capture_file_translation;
use crate::translations::translations_5::{filter_traffic_translation, traffic_source_translation};
use crate::utils::formatted_strings::get_path_termination_string;
use crate::utils::types::file_info::FileInfo;
use crate::utils::types::icon::Icon;
use crate::{Language, StyleType};
use iced::Length::FillPortion;
use iced::widget::scrollable::Direction;
use iced::widget::{
    Button, Checkbox, Column, Container, PickList, Row, Scrollable, Space, Text, TextInput, button,
    center, vertical_space,
};
use iced::{Alignment, Font, Length, Padding, alignment};

/// Computes the body of gui initial page
pub fn initial_page(sniffer: &Sniffer) -> Container<'_, Message, StyleType> {
    let Settings {
        style,
        language,
        color_gradient,
        ..
    } = sniffer.conf.settings;
    let font = style.get_extension().font;
    let font_headers = style.get_extension().font_headers;

    let col_data_source = get_col_data_source(sniffer, font, language);

    let col_checkboxes = Column::new()
        .spacing(10)
        .push(get_filters_group(&sniffer.conf.filters, font, language))
        .push_maybe(get_export_pcap_group_maybe(
            sniffer.conf.capture_source_picklist,
            &sniffer.conf.export_pcap,
            language,
            font,
        ));

    let is_capture_source_consistent = sniffer.is_capture_source_consistent();
    let right_col = Column::new()
        .width(FillPortion(1))
        .padding(10)
        .push(Space::with_height(76))
        .push(col_checkboxes)
        .push(vertical_space())
        .push(button_start(
            font_headers,
            language,
            color_gradient,
            is_capture_source_consistent,
        ))
        .push(vertical_space());

    let body = Column::new().push(Space::with_height(5)).push(
        Row::new()
            .push(col_data_source)
            .push(Space::with_width(15))
            .push(right_col),
    );

    Container::new(body).height(Length::Fill)
}

fn button_start<'a>(
    font_headers: Font,
    language: Language,
    color_gradient: GradientType,
    is_capture_source_consistent: bool,
) -> Button<'a, Message, StyleType> {
    button(
        Text::new(start_translation(language))
            .font(font_headers)
            .size(FONT_SIZE_TITLE)
            .width(Length::Fill)
            .align_x(alignment::Alignment::Center)
            .align_y(alignment::Alignment::Center),
    )
    .padding(20)
    .width(Length::Fill)
    .class(ButtonType::Gradient(color_gradient))
    .on_press_maybe(if is_capture_source_consistent {
        Some(Message::Start)
    } else {
        None
    })
}

fn get_col_data_source(
    sniffer: &Sniffer,
    font: Font,
    language: Language,
) -> Column<'_, Message, StyleType> {
    let current_option = if sniffer.conf.capture_source_picklist == CaptureSourcePicklist::Device {
        network_adapter_translation(language)
    } else {
        capture_file_translation(language)
    };
    let picklist = PickList::new(
        [
            network_adapter_translation(language),
            capture_file_translation(language),
        ],
        Some(current_option),
        move |option| {
            if option == network_adapter_translation(language) {
                Message::SetCaptureSource(CaptureSourcePicklist::Device)
            } else {
                Message::SetCaptureSource(CaptureSourcePicklist::File)
            }
        },
    )
    .padding([2, 7])
    .font(font);

    let mut col = Column::new()
        .align_x(Alignment::Center)
        .padding(Padding::new(10.0).top(30))
        .spacing(30)
        .height(Length::Fill)
        .width(FillPortion(1))
        .push(
            Row::new()
                .spacing(10)
                .push(
                    Text::new(traffic_source_translation(language))
                        .font(font)
                        .class(TextType::Title)
                        .size(FONT_SIZE_TITLE),
                )
                .push(picklist),
        );

    match &sniffer.conf.capture_source_picklist {
        CaptureSourcePicklist::Device => {
            col = col.push(get_col_adapter(sniffer, font, language));
        }
        CaptureSourcePicklist::File => {
            col = col.push(get_col_import_pcap(
                language,
                font,
                &sniffer.capture_source,
                &sniffer.conf.import_pcap_path,
            ));
        }
    }

    col
}

fn get_col_adapter(
    sniffer: &Sniffer,
    font: Font,
    language: Language,
) -> Column<'_, Message, StyleType> {
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
        .spacing(5)
        .height(Length::Fill)
        .push(if dev_str_list.is_empty() {
            Into::<iced::Element<Message, StyleType>>::into(center(
                Icon::get_hourglass(sniffer.dots_pulse.0.len()).size(60),
            ))
        } else {
            Scrollable::with_direction(
                dev_str_list.into_iter().fold(
                    Column::new().padding(Padding::ZERO.right(13)).spacing(5),
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
        .align_x(alignment::Alignment::Center)
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
    .padding(Padding::ZERO.right(13));

    Column::new().spacing(5).push(button)
}

fn get_filters_group<'a>(
    filters: &Filters,
    font: Font,
    language: Language,
) -> Container<'a, Message, StyleType> {
    let expanded = filters.expanded();
    let bpf = filters.bpf();

    let caption = filter_traffic_translation(language);
    let checkbox = Checkbox::new(caption, expanded)
        .on_toggle(move |_| Message::ToggleFilters)
        .size(18)
        .font(font);

    let mut ret_val = Column::new().spacing(10).push(checkbox);

    if expanded {
        let input = TextInput::new("", bpf)
            .on_input(Message::BpfFilter)
            .padding([2, 5])
            .font(font);
        let inner_col = Column::new()
            .spacing(10)
            .padding(Padding::ZERO.left(26))
            .push(
                Row::new()
                    .align_y(Alignment::Center)
                    .spacing(5)
                    .push(Text::new("BPF:").font(font))
                    .push(input),
            );
        ret_val = ret_val.push(inner_col);
    }

    Container::new(ret_val)
        .padding(15)
        .width(Length::Fill)
        .class(ContainerType::BorderedRound)
}

fn get_export_pcap_group_maybe<'a>(
    cs_pick: CaptureSourcePicklist,
    export_pcap: &ExportPcap,
    language: Language,
    font: Font,
) -> Option<Container<'a, Message, StyleType>> {
    if cs_pick == CaptureSourcePicklist::File {
        return None;
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
            .padding(Padding::ZERO.left(26))
            .push(
                Row::new()
                    .align_y(Alignment::Center)
                    .spacing(5)
                    .push(Text::new(format!("{}:", file_name_translation(language))).font(font))
                    .push(
                        TextInput::new(ExportPcap::DEFAULT_FILE_NAME, file_name)
                            .on_input(Message::OutputPcapFile)
                            .padding([2, 5])
                            .font(font),
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

    Some(
        Container::new(ret_val)
            .padding(15)
            .width(Length::Fill)
            .class(ContainerType::BorderedRound),
    )
}
