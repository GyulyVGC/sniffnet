use db_ip::{CountryCode, DbIpDatabase};
use std::cmp::Ordering;
use std::collections::HashMap;

use iced::Color;
use thousands::Separable;

use crate::enums::traffic_type::TrafficType;
use crate::structs::address_port_pair::AddressPortPair;
use crate::structs::filters::Filters;
use crate::{get_colors, AppProtocol, IpVersion, StyleType, TransProtocol};

/// Application version number (to be displayed in gui footer)
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Computes the String representing the percentage of filtered bytes/packets
pub fn get_percentage_string(observed: u128, filtered: i128) -> String {
    if format!("{:.1}", 100.0 * (filtered) as f32 / observed as f32).eq("0.0") {
        "<0.1%".to_string()
    } else {
        format!("{:.1}%", 100.0 * (filtered) as f32 / observed as f32)
    }
}

/// Computes the String representing the active filters
pub fn get_active_filters_string(filters: &Filters) -> String {
    if filters.ip.eq(&IpVersion::Other)
        && filters.application.eq(&AppProtocol::Other)
        && filters.transport.eq(&TransProtocol::Other)
    {
        "Active filters:\n   none".to_string()
    } else {
        let mut ret_val = "Active filters:".to_string();
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
pub fn get_active_filters_string_nobr(filters: &Filters) -> String {
    let mut ret_val = "Active filters:".to_string();
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
    if traffic_type == TrafficType::Incoming || traffic_type == TrafficType::Multicast {
        get_colors(style).incoming
    } else {
        get_colors(style).outgoing
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

        let percentage_string =
            if format!("{:.1}", 100.0 * (*entry.1) as f32 / tot_packets as f32).eq("0.0") {
                "(<0.1%)".to_string()
            } else {
                format!("({:.1}%)", 100.0 * (*entry.1) as f32 / tot_packets as f32)
            };

        //to align digits
        let spaces_string_1 = " "
            .to_string()
            .repeat(9 + longest_num - num_string.len() - app_proto_string.len());
        let spaces_string_2 = " ".to_string().repeat(10 - percentage_string.len());

        ret_val.push_str(&format!(
            "   {}:{}{}{}{}  \n",
            app_proto_string, spaces_string_1, num_string, spaces_string_2, percentage_string
        ));
    }
    ret_val
}

/// Returns a String representing a quantity of bytes with their proper multiple (kB, MB, GB, TB)
pub fn get_formatted_bytes_string(bytes: u128) -> String {
    let mut multiple_transmitted = String::new();
    let mut n = bytes as f32;

    match bytes {
        0..=999 => {}
        1_000..=999_999 => {
            n /= 1000_f32;
            multiple_transmitted.push('k');
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
        format!("{n}  B")
    } else {
        // with multiple
        format!("{n:.1} {multiple_transmitted}B")
    }
}

pub fn get_country_code(
    db: &DbIpDatabase<CountryCode>,
    traffic_type: TrafficType,
    key: &AddressPortPair,
) -> String {
    match traffic_type {
        TrafficType::Incoming | TrafficType::Multicast => db
            .get(&key.address1.parse().unwrap())
            .unwrap()
            .to_string()
            .replace("ZZ", "//"),
        TrafficType::Outgoing => db
            .get(&key.address2.parse().unwrap())
            .unwrap()
            .to_string()
            .replace("ZZ", "//"),
        TrafficType::Other => String::new(),
    }
}
