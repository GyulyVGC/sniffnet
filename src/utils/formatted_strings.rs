use std::cmp::min;
use std::net::IpAddr;

use crate::networking::types::filters::Filters;
use crate::translations::translations::{
    address_translation, ip_version_translation, protocol_translation,
};
use crate::translations::translations_3::{invalid_filters_translation, port_translation};
use crate::Language;

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

pub fn get_invalid_filters_string(filters: &Filters, language: Language) -> String {
    let mut ret_val = format!("{}:", invalid_filters_translation(language));
    if !filters.ip_version_valid() {
        ret_val.push_str(&format!("\n • {}", ip_version_translation(language)));
    }
    if !filters.protocol_valid() {
        ret_val.push_str(&format!("\n • {}", protocol_translation(language)));
    }
    if !filters.address_valid() {
        ret_val.push_str(&format!("\n • {}", address_translation(language)));
    }
    if !filters.port_valid() {
        ret_val.push_str(&format!("\n • {}", port_translation(language)));
    }
    ret_val
}

/// Computes the string representing the active filters
pub fn get_active_filters_string(filters: &Filters, language: Language) -> String {
    let mut filters_string = String::new();
    if filters.ip_version_active() {
        filters_string.push_str(&format!(
            "• {}: {}\n",
            ip_version_translation(language),
            filters.pretty_print_ip()
        ));
    }
    if filters.protocol_active() {
        filters_string.push_str(&format!(
            "• {}: {}\n",
            protocol_translation(language),
            filters.pretty_print_protocol()
        ));
    }
    if filters.address_active() {
        filters_string.push_str(&format!(
            "• {}: {}\n",
            address_translation(language),
            filters.address_str
        ));
    }
    if filters.port_active() {
        filters_string.push_str(&format!(
            "• {}: {}\n",
            port_translation(language),
            filters.port_str
        ));
    }
    filters_string
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

pub fn get_socket_address(address: &String, port: Option<u16>) -> String {
    if let Some(res) = port {
        if address.contains(':') {
            // IPv6
            format!("[{address}]:{res}")
        } else {
            // IPv4
            format!("{address}:{res}")
        }
    } else {
        address.to_owned()
    }
}

pub fn get_path_termination_string(full_path: &str, i: usize) -> String {
    let chars = full_path.chars().collect::<Vec<char>>();
    if chars.is_empty() {
        return String::new();
    }
    let tot_len = chars.len();
    let slice_len = min(i, tot_len);
    let suspensions = if tot_len > i { "…" } else { "" };
    [
        suspensions,
        &chars[tot_len - slice_len..].iter().collect::<String>(),
        " ",
    ]
    .concat()
}
