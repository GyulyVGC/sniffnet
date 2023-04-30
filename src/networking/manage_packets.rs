use std::sync::{Arc, Mutex};

use chrono::Local;
use dns_lookup::lookup_addr;
use etherparse::{Ethernet2Header, IpHeader, TransportHeader};
use maxminddb::Reader;
use pcap::{Active, Capture, Device};

use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::app_protocol::from_port_to_application_protocol;
use crate::networking::types::asn::Asn;
use crate::networking::types::data_info::DataInfo;
use crate::networking::types::info_address_port_pair::InfoAddressPortPair;
use crate::networking::types::traffic_type::TrafficType;
use crate::utils::asn::get_asn;
use crate::utils::countries::get_country_code;
use crate::{AppProtocol, InfoTraffic, IpVersion, TransProtocol};

/// This function analyzes the data link layer header passed as parameter and updates variables
/// passed by reference on the basis of the packet header content.
pub fn analyze_link_header(
    link_header: Option<Ethernet2Header>,
    mac_address1: &mut String,
    mac_address2: &mut String,
    skip_packet: &mut bool,
) {
    match link_header {
        Some(header) => {
            *mac_address1 = mac_from_dec_to_hex(header.source);
            *mac_address2 = mac_from_dec_to_hex(header.destination);
        }
        _ => {
            *skip_packet = true;
        }
    }
}

/// This function analyzes the network layer header passed as parameter and updates variables
/// passed by reference on the basis of the packet header content.
pub fn analyze_network_header(
    network_header: Option<IpHeader>,
    exchanged_bytes: &mut u128,
    network_protocol: &mut IpVersion,
    address1: &mut String,
    address2: &mut String,
    skip_packet: &mut bool,
) {
    match network_header {
        Some(IpHeader::Version4(ipv4header, _)) => {
            *network_protocol = IpVersion::IPv4;
            *address1 = format!("{:?}", ipv4header.source)
                .replace('[', "")
                .replace(']', "")
                .replace(',', ".")
                .replace(' ', "");
            *address2 = format!("{:?}", ipv4header.destination)
                .replace('[', "")
                .replace(']', "")
                .replace(',', ".")
                .replace(' ', "");
            *exchanged_bytes = u128::from(ipv4header.payload_len);
        }
        Some(IpHeader::Version6(ipv6header, _)) => {
            *network_protocol = IpVersion::IPv6;
            *address1 = ipv6_from_long_dec_to_short_hex(ipv6header.source);
            *address2 = ipv6_from_long_dec_to_short_hex(ipv6header.destination);
            *exchanged_bytes = u128::from(ipv6header.payload_length);
        }
        _ => {
            *skip_packet = true;
        }
    }
}

/// This function analyzes the transport layer header passed as parameter and updates variables
/// passed by reference on the basis of the packet header content.
pub fn analyze_transport_header(
    transport_header: Option<TransportHeader>,
    port1: &mut u16,
    port2: &mut u16,
    application_protocol: &mut AppProtocol,
    transport_protocol: &mut TransProtocol,
    skip_packet: &mut bool,
) {
    match transport_header {
        Some(TransportHeader::Udp(udp_header)) => {
            *port1 = udp_header.source_port;
            *port2 = udp_header.destination_port;
            *transport_protocol = TransProtocol::UDP;
            *application_protocol = from_port_to_application_protocol(*port1);
            if (*application_protocol).eq(&AppProtocol::Other) {
                *application_protocol = from_port_to_application_protocol(*port2);
            }
        }
        Some(TransportHeader::Tcp(tcp_header)) => {
            *port1 = tcp_header.source_port;
            *port2 = tcp_header.destination_port;
            *transport_protocol = TransProtocol::TCP;
            *application_protocol = from_port_to_application_protocol(*port1);
            if (*application_protocol).eq(&AppProtocol::Other) {
                *application_protocol = from_port_to_application_protocol(*port2);
            }
        }
        _ => {
            *skip_packet = true;
        }
    }
}

/// Function to insert the source and destination of a packet into the shared map containing the analyzed traffic.
pub fn modify_or_insert_in_map(
    info_traffic_mutex: &Arc<Mutex<InfoTraffic>>,
    key: AddressPortPair,
    mac_address1: String,
    mac_address2: String,
    exchanged_bytes: u128,
    traffic_type: TrafficType,
    application_protocol: AppProtocol,
    country_db_reader: &Reader<&[u8]>,
    asn_db_reader: &Reader<&[u8]>,
) -> InfoAddressPortPair {
    let now = Local::now();
    let very_long_address = key.address1.len() > 25 || key.address2.len() > 25;
    let mut info_traffic = info_traffic_mutex
        .lock()
        .expect("Error acquiring mutex\n\r");
    let len = info_traffic.map.len();
    let index = info_traffic.map.get_index_of(&key).unwrap_or(len);
    let (country, asn) = if index == len {
        // first occurrence of key => retrieve country code and asn
        (
            get_country_code(traffic_type, &key, country_db_reader),
            get_asn(traffic_type, &key, asn_db_reader),
        )
    } else {
        // this key already occurred
        (String::new(), Asn::default())
    };

    let new_info: InfoAddressPortPair = info_traffic
        .map
        .entry(key)
        .and_modify(|info| {
            info.transmitted_bytes += exchanged_bytes;
            info.transmitted_packets += 1;
            info.final_timestamp = now;
        })
        .or_insert(InfoAddressPortPair {
            mac_address1,
            mac_address2,
            transmitted_bytes: exchanged_bytes,
            transmitted_packets: 1,
            initial_timestamp: now,
            final_timestamp: now,
            app_protocol: application_protocol,
            very_long_address,
            traffic_type,
            country,
            asn,
            r_dns: None,
            index,
        })
        .clone();

    info_traffic.addresses_last_interval.insert(index);

    let host = new_info.get_host();
    if info_traffic.favorite_hosts.contains(&host) {
        info_traffic.favorites_last_interval.insert(host);
    }

    new_info
}

pub fn reverse_dns_lookup(
    info_traffic: Arc<Mutex<InfoTraffic>>,
    key: AddressPortPair,
    traffic_type: TrafficType,
) {
    let address_to_lookup = match traffic_type {
        TrafficType::Outgoing => key.address2.clone(),
        _ => key.address1.clone(),
    };

    // perform rDNS lookup
    let lookup_result = lookup_addr(&address_to_lookup.parse().unwrap());

    let mut info_traffic_lock = info_traffic.lock().unwrap();
    let new_info: InfoAddressPortPair = if let Ok(r_dns) = lookup_result {
        let actual_r_dns = Some(if !r_dns.is_empty() {
            r_dns
        } else {
            address_to_lookup
        });
        info_traffic_lock
            .map
            .entry(key)
            .and_modify(|info| info.r_dns = actual_r_dns)
            .or_insert(InfoAddressPortPair::default())
            .clone()
    } else {
        info_traffic_lock
            .map
            .entry(key.clone())
            .and_modify(|info| {
                info.r_dns = Some(address_to_lookup);
            })
            .or_insert(InfoAddressPortPair::default())
            .clone()
    };

    // insert the newly discovered host in the collection, with the data it exchanged so far
    info_traffic_lock
        .hosts
        .entry(new_info.get_host())
        .and_modify(|(data_info, _)| {
            if new_info.traffic_type == TrafficType::Outgoing {
                data_info.outgoing_packets += new_info.transmitted_packets;
                data_info.outgoing_bytes += new_info.transmitted_bytes;
            } else {
                data_info.incoming_packets += new_info.transmitted_packets;
                data_info.incoming_bytes += new_info.transmitted_bytes;
            }
        })
        .or_insert(if new_info.traffic_type == TrafficType::Outgoing {
            (
                DataInfo {
                    incoming_packets: 0,
                    outgoing_packets: new_info.transmitted_packets,
                    incoming_bytes: 0,
                    outgoing_bytes: new_info.transmitted_bytes,
                },
                false,
            )
        } else {
            (
                DataInfo {
                    incoming_packets: new_info.transmitted_packets,
                    outgoing_packets: 0,
                    incoming_bytes: new_info.transmitted_bytes,
                    outgoing_bytes: 0,
                },
                false,
            )
        });

    drop(info_traffic_lock);
}

/// Determines if the input address is a multicast address or not.
///
/// # Arguments
///
/// * `address` - string representing an IPv4 or IPv6 network address.
pub fn is_multicast_address(address: &str) -> bool {
    let mut ret_val = false;
    if address.contains(':') {
        //IPv6 address
        if address.starts_with("ff") {
            ret_val = true;
        }
    } else {
        //IPv4 address
        let first_group = address
            .split('.')
            .next()
            .unwrap()
            .to_string()
            .parse::<u8>()
            .unwrap();
        if (224..=239).contains(&first_group) {
            ret_val = true;
        }
    }
    ret_val
}

/// Determines if the input address is a broadcast address or not.
///
/// # Arguments
///
/// * `address` - string representing an IPv4 or IPv6 network address.
pub fn is_broadcast_address(address: &str) -> bool {
    let mut ret_val = false;
    if !address.contains(':') {
        //IPv4 address
        let groups: Vec<u8> = address
            .split('.')
            .map(|str| str.parse::<u8>().unwrap())
            .collect();
        if *groups.first().unwrap() == 255
            && *groups.get(1).unwrap() == 255
            && *groups.get(2).unwrap() == 255
            && *groups.get(3).unwrap() == 255
        {
            ret_val = true;
        }
        // still missing a check for directed broadcast!
    }
    ret_val
}

/// Determines if the capture opening resolves into an Error
pub fn get_capture_result(device: &Device) -> (Option<String>, Option<Capture<Active>>) {
    let cap_result = Capture::from_device(&*device.name)
        .expect("Capture initialization error\n\r")
        .promisc(true)
        .snaplen(256) //limit stored packets slice dimension (to keep more in the buffer)
        .immediate_mode(true) //parse packets ASAP!
        .open();
    if cap_result.is_err() {
        let err_string = cap_result.err().unwrap().to_string();
        (Some(err_string), None)
    } else {
        (None, cap_result.ok())
    }
}

/// Converts a MAC address in its hexadecimal form
fn mac_from_dec_to_hex(mac_dec: [u8; 6]) -> String {
    let mut mac_hex = String::new();
    for n in &mac_dec {
        mac_hex.push_str(&format!("{:02x}:", n));
    }
    mac_hex.pop();
    mac_hex
}

/// Function to convert a long decimal ipv6 address to a
/// shorter compressed ipv6 address
///
/// # Arguments
///
/// * `ipv6_long` - Contains the 16 integer composing the not compressed decimal ipv6 address
///
/// # Example
///
/// ```
/// let result = ipv6_from_long_dec_to_short_hex([255,10,10,255,0,0,0,0,28,4,4,28,255,1,0,0]);
/// assert_eq!(result, "ff0a:aff::1c04:41c:ff01:0".to_string());
/// ```
fn ipv6_from_long_dec_to_short_hex(ipv6_long: [u8; 16]) -> String {
    //from hex to dec, paying attention to the correct number of digits
    let mut ipv6_hex = String::new();
    for i in 0..=15 {
        //even: first byte of the group
        if i % 2 == 0 {
            if *ipv6_long.get(i).unwrap() == 0 {
                continue;
            }
            ipv6_hex.push_str(&format!("{:x}", ipv6_long.get(i).unwrap()));
        }
        //odd: second byte of the group
        else if *ipv6_long.get(i - 1).unwrap() == 0 {
            ipv6_hex.push_str(&format!("{:x}:", ipv6_long.get(i).unwrap()));
        } else {
            ipv6_hex.push_str(&format!("{:02x}:", ipv6_long.get(i).unwrap()));
        }
    }
    ipv6_hex.pop();

    // search for the longest zero sequence in the ipv6 address
    let mut to_compress: Vec<&str> = ipv6_hex.split(':').collect();
    let mut longest_zero_sequence = 0; // max number of consecutive zeros
    let mut longest_zero_sequence_start = 0; // first index of the longest sequence of zeros
    let mut current_zero_sequence = 0;
    let mut current_zero_sequence_start = 0;
    let mut i = 0;
    for s in to_compress.clone() {
        if s.eq("0") {
            if current_zero_sequence == 0 {
                current_zero_sequence_start = i;
            }
            current_zero_sequence += 1;
        } else if current_zero_sequence != 0 {
            if current_zero_sequence > longest_zero_sequence {
                longest_zero_sequence = current_zero_sequence;
                longest_zero_sequence_start = current_zero_sequence_start;
            }
            current_zero_sequence = 0;
        }
        i += 1;
    }
    if current_zero_sequence != 0 {
        // to catch consecutive zeros at the end
        if current_zero_sequence > longest_zero_sequence {
            longest_zero_sequence = current_zero_sequence;
            longest_zero_sequence_start = current_zero_sequence_start;
        }
    }
    if longest_zero_sequence < 2 {
        // no compression needed
        return ipv6_hex;
    }

    //from longest sequence of consecutive zeros to '::'
    let mut ipv6_hex_compressed = String::new();
    for _ in 0..longest_zero_sequence {
        to_compress.remove(longest_zero_sequence_start);
    }
    i = 0;
    if longest_zero_sequence_start == 0 {
        ipv6_hex_compressed.push_str("::");
    }
    for s in to_compress {
        ipv6_hex_compressed.push_str(s);
        ipv6_hex_compressed.push(':');
        i += 1;
        if i == longest_zero_sequence_start {
            ipv6_hex_compressed.push(':');
        }
    }
    if ipv6_hex_compressed.ends_with("::") {
        return ipv6_hex_compressed;
    }
    ipv6_hex_compressed.pop();

    ipv6_hex_compressed
}

#[cfg(test)]
mod tests {
    use crate::networking::manage_packets::{ipv6_from_long_dec_to_short_hex, mac_from_dec_to_hex};

    #[test]
    fn mac_simple_test() {
        let result = mac_from_dec_to_hex([255, 255, 10, 177, 9, 15]);
        assert_eq!(result, "ff:ff:0a:b1:09:0f".to_string());
    }

    #[test]
    fn mac_all_zero_test() {
        let result = mac_from_dec_to_hex([0, 0, 0, 0, 0, 0]);
        assert_eq!(result, "00:00:00:00:00:00".to_string());
    }

    #[test]
    fn ipv6_simple_test() {
        let result = ipv6_from_long_dec_to_short_hex([
            255, 10, 10, 255, 255, 10, 10, 255, 255, 10, 10, 255, 255, 10, 10, 255,
        ]);
        assert_eq!(result, "ff0a:aff:ff0a:aff:ff0a:aff:ff0a:aff".to_string());
    }

    #[test]
    fn ipv6_zeros_in_the_middle() {
        let result = ipv6_from_long_dec_to_short_hex([
            255, 10, 10, 255, 0, 0, 0, 0, 28, 4, 4, 28, 255, 1, 0, 0,
        ]);
        assert_eq!(result, "ff0a:aff::1c04:41c:ff01:0".to_string());
    }

    #[test]
    fn ipv6_leading_zeros() {
        let result =
            ipv6_from_long_dec_to_short_hex([0, 0, 0, 0, 0, 0, 0, 0, 28, 4, 4, 28, 255, 1, 0, 10]);
        assert_eq!(result, "::1c04:41c:ff01:a".to_string());
    }

    #[test]
    fn ipv6_tail_one_after_zeros() {
        let result =
            ipv6_from_long_dec_to_short_hex([28, 4, 4, 28, 255, 1, 0, 10, 0, 0, 0, 0, 0, 0, 0, 1]);
        assert_eq!(result, "1c04:41c:ff01:a::1".to_string());
    }

    #[test]
    fn ipv6_tail_zeros() {
        let result =
            ipv6_from_long_dec_to_short_hex([28, 4, 4, 28, 255, 1, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(result, "1c04:41c:ff01:a::".to_string());
    }

    #[test]
    fn ipv6_multiple_zero_sequences_first_longer() {
        let result =
            ipv6_from_long_dec_to_short_hex([32, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1]);
        assert_eq!(result, "2000::101:0:0:1".to_string());
    }

    #[test]
    fn ipv6_multiple_zero_sequences_first_longer_head() {
        let result =
            ipv6_from_long_dec_to_short_hex([0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1]);
        assert_eq!(result, "::101:0:0:1".to_string());
    }

    #[test]
    fn ipv6_multiple_zero_sequences_second_longer() {
        let result =
            ipv6_from_long_dec_to_short_hex([1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 3, 118]);
        assert_eq!(result, "100:0:0:1::376".to_string());
    }

    #[test]
    fn ipv6_multiple_zero_sequences_second_longer_tail() {
        let result =
            ipv6_from_long_dec_to_short_hex([32, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0]);
        assert_eq!(result, "2000:0:0:1:101::".to_string());
    }

    #[test]
    fn ipv6_multiple_zero_sequences_equal_length() {
        let result =
            ipv6_from_long_dec_to_short_hex([118, 3, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1]);
        assert_eq!(result, "7603::1:101:0:0:1".to_string());
    }

    #[test]
    fn ipv6_all_zeros() {
        let result =
            ipv6_from_long_dec_to_short_hex([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(result, "::".to_string());
    }

    #[test]
    fn ipv6_x_all_zeros() {
        let result =
            ipv6_from_long_dec_to_short_hex([161, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(result, "a100::".to_string());
    }

    #[test]
    fn ipv6_all_zeros_x() {
        let result =
            ipv6_from_long_dec_to_short_hex([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 176]);
        assert_eq!(result, "::b0".to_string());
    }

    #[test]
    fn ipv6_many_zeros_but_no_compression() {
        let result =
            ipv6_from_long_dec_to_short_hex([0, 16, 16, 0, 0, 1, 7, 0, 0, 2, 216, 0, 1, 0, 0, 1]);
        assert_eq!(result, "10:1000:1:700:2:d800:100:1".to_string());
    }
}
