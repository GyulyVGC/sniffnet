use std::net::IpAddr;
use std::path::PathBuf;

use iced::Color;

use crate::networking::types::filters::Filters;
use crate::networking::types::traffic_direction::TrafficDirection;
use crate::translations::translations::{
    active_filters_translation, none_translation, open_report_translation,
};
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
            "{}:\n   {}",
            active_filters_translation(language),
            none_translation(language)
        )
    } else {
        let mut filters_string = String::new();
        if filters.ip.ne(&IpVersion::Other) {
            filters_string.push_str(&format!("{} ", filters.ip));
        }
        if filters.transport.ne(&TransProtocol::Other) {
            filters_string.push_str(&format!("{} ", filters.transport));
        }
        if filters.application.ne(&AppProtocol::Other) {
            filters_string.push_str(&format!("{} ", filters.application));
        }
        format!(
            "{}:\n   {filters_string}",
            active_filters_translation(language),
        )
    }
}

/// Returns the color to be used for a specific connection of the relevant connections table in gui run page
pub fn get_connection_color(traffic_direction: TrafficDirection, style: StyleType) -> Color {
    if traffic_direction == TrafficDirection::Outgoing {
        get_colors(style).outgoing
    } else {
        get_colors(style).incoming
    }
}

/// Returns a String representing a quantity of bytes with its proper multiple (K, M, G, T)
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
        format!("{n}  ")
    } else {
        // with multiple
        format!("{n:.1} {multiple_transmitted}")
    }
}

/// Returns a String representing a quantity of bytes with its proper multiple (KB, MB, GB, TB)
pub fn get_formatted_bytes_string_with_b(bytes: u128) -> String {
    let mut bytes_string = get_formatted_bytes_string(bytes).replace("  ", " ");
    bytes_string.push('B');
    bytes_string
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

pub fn get_open_report_tooltip(language: Language) -> String {
    let open_report_translation = open_report_translation(language).to_string();
    //open_report_translation.push_str(&format!(" [{}+O]", get_command_key()));
    let report_path = get_report_path().to_string_lossy().to_string();
    format!(
        "{:^len$}\n{report_path}",
        open_report_translation,
        len = report_path.len()
    )
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
 |   |_____/  |_| |_| |_| |_|   |_|   |_| |_|  \___|  \__|   |
 |                                                           |
 |                   ___________                             |
 |                  /___________\                            |
 |                 | ___________ |                           |
 |                 | |         | |                           |
 |                 | | v{APP_VERSION}  | |                           |
 |                 | |_________| |________________________   |
 |                 \_____________/   by Giuliano Bellini  )  |
 |                 / ''''''''''' \                       /   |
 |                / ::::::::::::: \                  =D-'    |
 |               (_________________)                         |
  \_________________________________________________________/
    "
    );
}

pub fn get_domain_from_r_dns(r_dns: String) -> String {
    if r_dns.parse::<IpAddr>().is_ok() || r_dns.is_empty() {
        // rDNS is equal to the corresponding IP address (can't be empty but checking it to be safe)
        r_dns
    } else {
        let parts: Vec<&str> = r_dns.split('.').collect();
        if parts.len() >= 2 {
            parts
                .get(parts.len() - 2..)
                .unwrap_or(&parts)
                .iter()
                .fold(Vec::new(), |mut vec, part| {
                    vec.push((*part).to_string());
                    vec
                })
                .join(".")
        } else {
            r_dns
        }
    }
}

pub fn get_socket_address(address: &String, port: u16) -> String {
    if address.contains(':') {
        // IPv6
        format!("[{address}]:{port}")
    } else {
        //IPv4
        format!("{address}:{port}")
    }
}
