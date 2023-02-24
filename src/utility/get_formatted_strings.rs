use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::PathBuf;

use iced::Color;
use thousands::Separable;

use crate::enums::traffic_type::TrafficType;
use crate::structs::filters::Filters;
use crate::utility::translations::{active_filters_translation, none_translation};
use crate::{get_colors, AppProtocol, IpVersion, Language, StyleType, TransProtocol};

/// Application version number (to be displayed in gui footer)
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Computes the String representing the percentage of filtered bytes/packets
pub fn get_percentage_string(observed: u128, filtered: u128) -> String {
    #[allow(clippy::cast_precision_loss)]
    let filtered_float = filtered as f32;
    #[allow(clippy::cast_precision_loss)]
    let observed_float = observed as f32;
    if format!("{:.1}", 100.0 * filtered_float / observed_float).eq("0.0") {
        "<0.1%".to_string()
    } else {
        format!("{:.1}%", 100.0 * filtered_float / observed_float)
    }
}

/// Computes the String representing the active filters
pub fn get_active_filters_string(filters: &Filters, language: Language) -> String {
    if filters.ip.eq(&IpVersion::Other)
        && filters.application.eq(&AppProtocol::Other)
        && filters.transport.eq(&TransProtocol::Other)
    {
        format!(
            "{}\n   {}",
            active_filters_translation(language),
            none_translation(language)
        )
    } else {
        let mut ret_val = active_filters_translation(language).to_string();
        if filters.ip.ne(&IpVersion::Other) {
            ret_val.push_str(&format!("\n   {}", filters.ip));
        }
        if filters.transport.ne(&TransProtocol::Other) {
            ret_val.push_str(&format!("\n   {}", filters.transport));
        }
        if filters.application.ne(&AppProtocol::Other) {
            ret_val.push_str(&format!("\n   {}", filters.application));
        }
        ret_val
    }
}

/// Computes the String representing the active filters, without line breaks
pub fn get_active_filters_string_nobr(filters: &Filters, language: Language) -> String {
    let mut ret_val = active_filters_translation(language).to_string();
    if filters.ip.ne(&IpVersion::Other) {
        ret_val.push_str(&format!(" {}", filters.ip));
    }
    if filters.transport.ne(&TransProtocol::Other) {
        ret_val.push_str(&format!(" {}", filters.transport));
    }
    if filters.application.ne(&AppProtocol::Other) {
        ret_val.push_str(&format!(" {}", filters.application));
    }
    ret_val
}

/// Returns the color to be used for a specific connection of the relevant connections table in gui run page
pub fn get_connection_color(traffic_type: TrafficType, style: StyleType) -> Color {
    if traffic_type == TrafficType::Outgoing {
        get_colors(style).outgoing
    } else {
        get_colors(style).incoming
    }
}

/// Given the map of app layer protocols with the relative sniffed packets count,
/// the function generates the corresponding String
/// to be displayed in gui run page.
///
/// # Arguments
///
/// * `app_count` - Map of app layer protocols with the relative sniffed packets count
///
/// * `tot_packets` - Total number of sniffed packets
pub fn get_app_count_string(app_count: &HashMap<AppProtocol, u128>, tot_packets: u128) -> String {
    let mut ret_val = String::new();

    if app_count.is_empty() {
        return ret_val;
    }

    let mut sorted_app_count: Vec<(&AppProtocol, &u128)> = app_count.iter().collect();
    sorted_app_count.sort_by(|&(p1, a), &(p2, b)| {
        if p1.eq(&AppProtocol::Other) {
            Ordering::Greater
        } else if p2.eq(&AppProtocol::Other) {
            Ordering::Less
        } else {
            b.cmp(a)
        }
    });

    //compute the length of the longest packet count string, used to align text
    let mut longest_num = sorted_app_count
        .get(0)
        .unwrap()
        .1
        .separate_with_spaces()
        .len();
    match app_count.get(&AppProtocol::Other) {
        None => {}
        Some(x) => {
            if x.separate_with_spaces().len() > longest_num {
                longest_num = x.separate_with_spaces().len();
            }
        }
    }

    for entry in sorted_app_count {
        let app_proto_string = format!("{:?}", entry.0);

        let num_string = entry.1.separate_with_spaces().to_string();

        #[allow(clippy::cast_precision_loss)]
        let num_app_float = *entry.1 as f32;
        #[allow(clippy::cast_precision_loss)]
        let num_tot_float = tot_packets as f32;
        let percentage_string = if format!("{:.1}", 100.0 * num_app_float / num_tot_float).eq("0.0")
        {
            "(<0.1%)".to_string()
        } else {
            format!("({:.1}%)", 100.0 * num_app_float / num_tot_float)
        };

        //to align digits
        let spaces_string_1 = " "
            .to_string()
            .repeat(9 + longest_num - num_string.len() - app_proto_string.len());
        let spaces_string_2 = " ".to_string().repeat(10 - percentage_string.len());

        ret_val.push_str(&format!(
            "   {app_proto_string}:{spaces_string_1}{num_string}{spaces_string_2}{percentage_string}  \n",
        ));
    }
    ret_val
}

/// Returns a String representing a quantity of bytes with their proper multiple (KB, MB, GB, TB)
pub fn get_formatted_bytes_string(bytes: u128) -> String {
    let mut multiple_transmitted = String::new();
    #[allow(clippy::cast_precision_loss)]
    let mut n = bytes as f32;

    match bytes {
        0..=999 => {}
        1_000..=999_999 => {
            n /= 1000_f32;
            multiple_transmitted.push('K');
        } // kilo
        1_000_000..=999_999_999 => {
            n /= 1_000_000_f32;
            multiple_transmitted.push('M');
        } // mega
        1_000_000_000..=999_999_999_999 => {
            n /= 1_000_000_000_f32;
            multiple_transmitted.push('G');
        } // giga
        _ => {
            n /= 1_000_000_000_000_f32;
            multiple_transmitted.push('T');
        } // tera
    }

    if multiple_transmitted.is_empty() {
        // no multiple
        format!("{n}   ")
    } else {
        // with multiple
        format!("{n:.1} {multiple_transmitted} ")
    }
}

pub fn get_report_path() -> PathBuf {
    if let Ok(mut config_path) = confy::get_configuration_file_path("sniffnet", "file") {
        config_path.pop();
        config_path.push("report.txt");
        config_path
    } else {
        let mut report_path = PathBuf::from(std::env::var_os("HOME").unwrap());
        report_path.push("sniffnet_report.txt");
        report_path
    }
}

pub fn print_cli_welcome_message() {
    print!(
        r"
  /---------------------------------------------------------\
 |     _____           _    __    __                  _      |
 |    / ____|         (_)  / _|  / _|                | |     |
 |   | (___    _ __    _  | |_  | |_   _ __     ___  | |_    |
 |    \___ \  | '_ \  | | |  _| |  _| | '_ \   / _ \ | __|   |
 |    ____) | | | | | | | | |   | |   | | | | |  __/ | |_    |
 |   |_____/  |_| |_| |_| |_|   |_|   |_| |_|  \___|  \__|   |"
    );
    print!(
        r"
 |                                                           |
 |                   ___________                             |
 |                  /___________\                            |
 |                 | ___________ |                           |
 |                 | |         | |                           |
 |                 | | v{}  | |                           |
 |                 | |_________| |________________________   |
 |                 \_____________/   by Giuliano Bellini  )  |
 |                 / ''''''''''' \                       /   |
 |                / ::::::::::::: \                  =D-'    |
 |               (_________________)                         |
  \_________________________________________________________/
    ",
        APP_VERSION
    );
}
