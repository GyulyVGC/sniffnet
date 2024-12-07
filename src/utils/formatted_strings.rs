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

pub fn get_formatted_num_seconds(num_seconds: u128) -> String {
    match num_seconds {
        0..3600 => format!("{:02}:{:02}", num_seconds / 60, num_seconds % 60),
        _ => format!(
            "{:02}:{:02}:{:02}",
            num_seconds / 3600,
            (num_seconds % 3600) / 60,
            num_seconds % 60
        ),
    }
}

#[allow(dead_code)]
#[cfg(windows)]
pub fn get_logs_file_path() -> Option<String> {
    let mut conf = confy::get_configuration_file_path(crate::SNIFFNET_LOWERCASE, "logs").ok()?;
    conf.set_extension("txt");
    Some(conf.to_str()?.to_string())
}

#[cfg(all(windows, not(debug_assertions)))]
pub fn redirect_stdout_stderr_to_file(
) -> Option<(gag::Redirect<std::fs::File>, gag::Redirect<std::fs::File>)> {
    if let Ok(logs_file) = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(get_logs_file_path()?)
    {
        return Some((
            gag::Redirect::stdout(logs_file.try_clone().ok()?).ok()?,
            gag::Redirect::stderr(logs_file).ok()?,
        ));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formatted_num_seconds() {
        assert_eq!(get_formatted_num_seconds(0), "00:00");
        assert_eq!(get_formatted_num_seconds(1), "00:01");
        assert_eq!(get_formatted_num_seconds(28), "00:28");
        assert_eq!(get_formatted_num_seconds(59), "00:59");
        assert_eq!(get_formatted_num_seconds(60), "01:00");
        assert_eq!(get_formatted_num_seconds(61), "01:01");
        assert_eq!(get_formatted_num_seconds(119), "01:59");
        assert_eq!(get_formatted_num_seconds(120), "02:00");
        assert_eq!(get_formatted_num_seconds(121), "02:01");
        assert_eq!(get_formatted_num_seconds(3500), "58:20");
        assert_eq!(get_formatted_num_seconds(3599), "59:59");
        assert_eq!(get_formatted_num_seconds(3600), "01:00:00");
        assert_eq!(get_formatted_num_seconds(3601), "01:00:01");
        assert_eq!(get_formatted_num_seconds(3661), "01:01:01");
        assert_eq!(get_formatted_num_seconds(7139), "01:58:59");
        assert_eq!(get_formatted_num_seconds(7147), "01:59:07");
        assert_eq!(get_formatted_num_seconds(7199), "01:59:59");
        assert_eq!(get_formatted_num_seconds(7200), "02:00:00");
        assert_eq!(get_formatted_num_seconds(9999), "02:46:39");
        assert_eq!(get_formatted_num_seconds(36000), "10:00:00");
        assert_eq!(get_formatted_num_seconds(36001), "10:00:01");
        assert_eq!(get_formatted_num_seconds(36061), "10:01:01");
        assert_eq!(get_formatted_num_seconds(86400), "24:00:00");
        assert_eq!(get_formatted_num_seconds(123456789), "34293:33:09");
        assert_eq!(
            get_formatted_num_seconds(u128::MAX),
            "94522879700260684295381835397713392:04:15"
        );
    }

    #[cfg(windows)]
    #[test]
    fn test_logs_file_path() {
        let file_path = std::path::PathBuf::from(get_logs_file_path().unwrap());
        assert!(file_path.is_absolute());
        assert_eq!(file_path.file_name().unwrap(), "logs.txt");
    }
}
