use std::cmp::min;
use std::net::IpAddr;

use crate::Language;
use crate::networking::types::filters::Filters;
use crate::translations::translations::{
    address_translation, ip_version_translation, protocol_translation,
};
use crate::translations::translations_3::{invalid_filters_translation, port_translation};
use crate::utils::types::timestamp::Timestamp;
use chrono::{Local, TimeZone};
use std::fmt::Write;

/// Application version number (to be displayed in gui footer)
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

// /// Computes the String representing the percentage of filtered bytes/packets
// pub fn get_percentage_string(observed: u128, filtered: u128) -> String {
//     #[allow(clippy::cast_precision_loss)]
//     let filtered_float = filtered as f32;
//     #[allow(clippy::cast_precision_loss)]
//     let observed_float = observed as f32;
//     if format!("{:.1}", 100.0 * filtered_float / observed_float).eq("0.0") {
//         "<0.1%".to_string()
//     } else {
//         format!("{:.1}%", 100.0 * filtered_float / observed_float)
//     }
// }

pub fn get_invalid_filters_string(filters: &Filters, language: Language) -> String {
    let mut ret_val = format!("{}:", invalid_filters_translation(language));
    if !filters.ip_version_valid() {
        let _ = write!(ret_val, "\n • {}", ip_version_translation(language));
    }
    if !filters.protocol_valid() {
        let _ = write!(ret_val, "\n • {}", protocol_translation(language));
    }
    if !filters.address_valid() {
        let _ = write!(ret_val, "\n • {}", address_translation(language));
    }
    if !filters.port_valid() {
        let _ = write!(ret_val, "\n • {}", port_translation(language));
    }
    ret_val
}

/// Computes the string representing the active filters
pub fn get_active_filters_string(filters: &Filters, language: Language) -> String {
    let mut filters_string = String::new();
    if filters.ip_version_active() {
        let _ = writeln!(
            filters_string,
            "• {}: {}",
            ip_version_translation(language),
            filters.pretty_print_ip()
        );
    }
    if filters.protocol_active() {
        let _ = writeln!(
            filters_string,
            "• {}: {}",
            protocol_translation(language),
            filters.pretty_print_protocol()
        );
    }
    if filters.address_active() {
        let _ = writeln!(
            filters_string,
            "• {}: {}",
            address_translation(language),
            filters.address_str
        );
    }
    if filters.port_active() {
        let _ = writeln!(
            filters_string,
            "• {}: {}",
            port_translation(language),
            filters.port_str
        );
    }
    filters_string
}

pub fn print_cli_welcome_message() {
    let ver = APP_VERSION;
    print!(
        "\n\
╭────────────────────────────────────────────────────────────────────╮\n\
│                                                                    │\n\
│                           Sniffnet {ver}                           │\n\
│                                                                    │\n\
│           → Website: https://sniffnet.net                          │\n\
│           → GitHub:  https://github.com/GyulyVGC/sniffnet          │\n\
│                                                                    │\n\
╰────────────────────────────────────────────────────────────────────╯\n\n"
    );
}

pub fn get_domain_from_r_dns(r_dns: String) -> String {
    if r_dns.parse::<IpAddr>().is_ok() || r_dns.is_empty() {
        // rDNS is equal to the corresponding IP address (can't be empty but checking it to be safe)
        r_dns
    } else {
        let parts: Vec<&str> = r_dns.split('.').collect();
        let len = parts.len();
        if len >= 2 {
            let last = parts.get(len - 1).unwrap_or(&"");
            let second_last = parts.get(len - 2).unwrap_or(&"");
            if last.len() > 3 || second_last.len() > 3 {
                format!("{second_last}.{last}")
            } else {
                let third_last_opt = len.checked_sub(3).and_then(|i| parts.get(i));
                match third_last_opt {
                    Some(third_last) => format!("{third_last}.{second_last}.{last}"),
                    None => format!("{second_last}.{last}"),
                }
            }
        } else {
            r_dns
        }
    }
}

pub fn get_socket_address(address: &IpAddr, port: Option<u16>) -> String {
    if let Some(res) = port {
        if address.is_ipv6() {
            // IPv6
            format!("[{address}]:{res}")
        } else {
            // IPv4
            format!("{address}:{res}")
        }
    } else {
        address.to_string()
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

pub fn get_formatted_timestamp(t: Timestamp) -> String {
    let date_opt = t
        .to_usecs()
        .and_then(|usecs| Local.timestamp_micros(usecs).latest());
    if let Some(date) = date_opt {
        date.format("%Y/%m/%d %H:%M:%S").to_string()
    } else {
        "?".to_string()
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
pub fn redirect_stdout_stderr_to_file()
-> Option<(gag::Redirect<std::fs::File>, gag::Redirect<std::fs::File>)> {
    if let Ok(logs_file) = std::fs::OpenOptions::new()
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

    #[test]
    fn test_get_domain_from_r_dns() {
        let f = |s: &str| get_domain_from_r_dns(s.to_string());
        assert_eq!(f(""), "");
        assert_eq!(f("8.8.8.8"), "8.8.8.8");
        assert_eq!(f("a.b.c.d"), "b.c.d");
        assert_eq!(f("ciao.xyz"), "ciao.xyz");
        assert_eq!(f("bye.ciao.xyz"), "ciao.xyz");
        assert_eq!(f("ciao.bye.xyz"), "ciao.bye.xyz");
        assert_eq!(f("hola.ciao.bye.xyz"), "ciao.bye.xyz");
        assert_eq!(f(".bye.xyz"), ".bye.xyz");
        assert_eq!(f("bye.xyz"), "bye.xyz");
        assert_eq!(f("hola.ciao.b"), "ciao.b");
        assert_eq!(f("hola.b.ciao"), "b.ciao");
        assert_eq!(f("ciao."), "ciao.");
        assert_eq!(f("ciao.."), "ciao..");
        assert_eq!(f(".ciao."), "ciao.");
        assert_eq!(f("ciao.bye."), "ciao.bye.");
        assert_eq!(f("ciao..."), "..");
        assert_eq!(f("..bye"), "..bye");
        assert_eq!(f("ciao..bye"), "ciao..bye");
        assert_eq!(f("..ciao"), ".ciao");
        assert_eq!(f("bye..ciao"), ".ciao");
        assert_eq!(f("."), ".");
        assert_eq!(f(".."), "..");
        assert_eq!(f("..."), "..");
        assert_eq!(f("no_dots_in_this"), "no_dots_in_this");
    }
}
