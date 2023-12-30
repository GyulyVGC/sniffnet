use std::net::IpAddr;

use crate::networking::types::filters::Filters;
use crate::translations::translations::{
    address_translation, ip_version_translation, protocol_translation,
};
use crate::translations::translations_3::{invalid_filters_translation, port_translation};
use crate::Language;

/// Application version number (to be displayed in gui footer)
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

// pub const PCAP_FILE_NAME: &str = "sniffnet.pcap";

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

// /// Returns the default report path
// pub fn get_default_report_file_path() -> String {
//     return if let Ok(mut config_path) = confy::get_configuration_file_path(SNIFFNET_LOWERCASE, "file") {
//         config_path.pop();
//         config_path.push(PCAP_FILE_NAME);
//         config_path.to_string_lossy().to_string()
//     } else {
//         let mut path = PathBuf::from(std::env::var_os("HOME").unwrap());
//         path.push(PCAP_FILE_NAME);
//         path.to_string_lossy().to_string()
//     };
// }

// /// Returns the file to use for the output PCAP report
// /// It tries and fallbacks in the order: custom path, configs path, home directory path
// // /// This function also updates the custom path text input TODO!
// pub fn set_report_file_to_use(custom_path: &str) -> File {
//     if let Ok(custom_file) = File::create(custom_path) {
//         return custom_file;
//     } else if let Ok(mut config_path) =
//         confy::get_configuration_file_path(SNIFFNET_LOWERCASE, "file")
//     {
//         config_path.pop();
//         config_path.push(PCAP_FILE_NAME);
//         if let Ok(file) = File::create(config_path) {
//             return file;
//         }
//     }
//     let mut path = PathBuf::from(std::env::var_os("HOME").unwrap());
//     path.push(PCAP_FILE_NAME);
//     File::create(path).unwrap()
// }

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
