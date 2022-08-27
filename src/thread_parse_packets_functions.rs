//! Module containing functions executed by the thread in charge of parsing sniffed packets and
//! inserting them in the shared map.

use std::cmp::Ordering::Equal;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Condvar, Mutex};
use chrono::{DateTime, Local};
use etherparse::{IpHeader, PacketHeaders, TransportHeader};
use pcap::{Capture, Device};
use crate::{AddressPort, AppProtocol, ReportInfo, Status, TransProtocol};


/// The calling thread enters in a loop in which it waits for network packets, parses them according
/// to the user specified filters, and inserts them into the shared map variable.
///
/// # Arguments
///
/// * `device` - Network adapter to be sniffed

/// * `lowest_port` - The lowest port number to be considered in the report. Specified by the user
/// through the ```-l``` option.
///
/// * `highest_port` - The highest port number to be considered in the report. Specified by the user
/// through the ```-h``` option.
///
/// * `network_layer` - A String representing the IP version to be filtered. Specified by the user through the
/// ```-n``` option.
///
/// * `transport_layer` - A String representing the transport protocol to be filtered. Specified by the user through the
/// ```-t``` option.
///
/// * `app_layer` - An AppProtocol representing the application protocol to be filtered. Specified by the user through the
/// ```--app``` option.
///
/// * `mutex_map` - Mutex to permit exclusive access to the shared tuple containing the parsed packets,
/// the total number of sniffed packets and the number of filtered packets.
///
/// * `status_pair` - Shared variable to check the application current status.
pub fn parse_packets_loop(device: Device, lowest_port: u16, highest_port: u16,
                          network_layer_filter: String, transport_layer_filter: String, app_layer: AppProtocol,
                          mutex_map: Arc<Mutex<(HashMap<AddressPort, ReportInfo>, u128, u128)>>,
                          status_pair: Arc<(Mutex<Status>, Condvar)>) {

    let cvar = &status_pair.1;

    let mut cap = Capture::from_device(device.clone())
        .expect("Capture initialization error\n\r")
        .promisc(true)
        .open()
        .expect("Capture initialization error\n\r");

    let mut my_interface_addresses = Vec::new();
    for address in device.addresses {
        my_interface_addresses.push(address.addr.to_string());
    }

    loop {
        let mut status = status_pair.0.lock().expect("Error acquiring mutex\n\r");
        status = cvar.wait_while(status, |s| *s == Status::Pause).expect("Error acquiring mutex\n\r");

        if *status == Status::Running {
            drop(status);
            match cap.next() {
                Err(_) => {
                    continue;
                }
                Ok(packet) => {
                    match PacketHeaders::from_ethernet_slice(&packet) {
                        Err(_) => {
                            continue;
                        }
                        Ok(value) => {
                            let mut address1 = "".to_string();
                            let mut address2 = "".to_string();
                            let mut network_layer = "".to_string();
                            let mut port1 = 0;
                            let mut port2 = 0;
                            let mut transport_layer = "".to_string();
                            let mut exchanged_bytes: u32 = 0;
                            let mut transport_protocol = TransProtocol::Other;
                            let mut application_protocol = AppProtocol::Other;
                            let mut skip_packet = false;
                            let mut reported_packet = false;

                            analyze_network_header(value.ip, &mut exchanged_bytes,
                                                   &mut network_layer, &mut address1, &mut address2,
                                                   &mut skip_packet);
                            if skip_packet {
                                continue;
                            }

                            analyze_transport_header(value.transport,
                                                     &mut transport_layer, &mut port1, &mut port2,
                                                     &mut application_protocol,
                                                     &mut transport_protocol, &mut skip_packet);
                            if skip_packet {
                                continue;
                            }

                            let mut map_sniffed_filtered = mutex_map.lock().expect("Error acquiring mutex\n\r");
                            //increment number of sniffed packets
                            map_sniffed_filtered.1 += 1;
                            drop(map_sniffed_filtered);

                            let key1: AddressPort = AddressPort::new(address1.clone(), port1,
                                                                     my_interface_addresses.contains(&address1));
                            let key2: AddressPort = AddressPort::new(address2.clone(), port2,
                                                                     my_interface_addresses.contains(&address2));

                            if network_layer_filter.cmp(&network_layer) == Equal || network_layer_filter.cmp(&"no filter".to_string()) == Equal {
                                if transport_layer_filter.cmp(&transport_layer) == Equal || transport_layer_filter.cmp(&"no filter".to_string()) == Equal {
                                    if application_protocol.eq(&app_layer) || app_layer.eq(&AppProtocol::Other) {

                                        if port1 >= lowest_port && port1 <= highest_port {
                                            modify_or_insert_source_in_map(mutex_map.clone(), key1,
                                                                           exchanged_bytes, transport_protocol,
                                                                           application_protocol);
                                            reported_packet = true;
                                        }

                                        if port2 >= lowest_port && port2 <= highest_port {
                                            modify_or_insert_destination_in_map(mutex_map.clone(), key2,
                                                                                exchanged_bytes, transport_protocol,
                                                                                application_protocol);
                                            reported_packet = true;
                                        }

                                        if reported_packet {
                                            //increment number of reported packets
                                            mutex_map.lock().expect("Error acquiring mutex\n\r").2 += 1;
                                        }

                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

}


/// This function analyzes the network layer header passed as parameter and updates variables
/// passed by reference on the basis of the packet header content.
///
/// # Arguments
///
/// * `network_header` - An `Option` containing the `IpHeader` enum obtained through etherparse.
///
/// * `exchanged_bytes` - Parameter initialized with a default value; it is passed by reference
/// and it will be overwritten with the IP payload dimension.
///
/// * `network_layer` - Parameter initialized with a default value; it is passed by reference
/// and it will be overwritten with a String representing the IP version ("ipv4" or "ipv6").
///
/// * `address1` - Parameter initialized with a default value; it is passed by reference
/// and will be overwritten with the source IP address of the packet.
///
/// * `address2` - Parameter initialized with a default value; it is passed by reference
/// and will be overwritten with the destination IP address of the packet.
///
/// * `skip_packet` - Boolean flag initialized with a `false` value; it is passed by reference
/// and will be overwritten to `true` if the `network_header` is invalid; in this case the
/// packet will not be considered.
fn analyze_network_header(network_header: Option<IpHeader>, exchanged_bytes: &mut u32,
                          network_layer: &mut String, address1: &mut String,
                          address2: &mut String, skip_packet: &mut bool) {
    match network_header {
        Some(IpHeader::Version4(ipv4header, _)) => {
            *network_layer = "ipv4".to_string();
            *address1 = format!("{:?}", ipv4header.source)
                .replace("[","")
                .replace("]","")
                .replace(",",".")
                .replace(" ","");
            *address2 = format!("{:?}", ipv4header.destination)
                .replace("[","")
                .replace("]","")
                .replace(",",".")
                .replace(" ","");
            *exchanged_bytes = ipv4header.payload_len as u32;
        }
        Some(IpHeader::Version6(ipv6header, _)) => {
            *network_layer = "ipv6".to_string();
            *address1 = ipv6_from_long_dec_to_short_hex(ipv6header.source);
            *address2 = ipv6_from_long_dec_to_short_hex(ipv6header.destination);
            *exchanged_bytes = ipv6header.payload_length as u32;
        }
        _ => {
            *skip_packet = true;
        }
    }
}


/// This function analyzes the transport layer header passed as parameter and updates variables
/// passed by reference on the basis of the packet header content.
///
/// # Arguments
///
/// * `transport_header` - An `Option` containing the `TransportHeader` enum obtained through etherparse.
///
/// * `transport_layer` - Parameter initialized with a default value; it is passed by reference
/// and it will be overwritten with a String representing the transport layer
/// protocol ("tcp" or "udp").
///
/// * `port1` - Parameter initialized with a default value; it is passed by reference
/// and will be overwritten with the source port of the packet.
///
/// * `port2` - Parameter initialized with a default value; it is passed by reference
/// and will be overwritten with the destination port of the packet.
///
/// * `application_protocol` - Parameter initialized with a `None` value; it is passed by reference
/// and will be overwritten with the application layer protocol (obtained from port numbers).
///
/// * `transport_protocol` - Parameter initialized with a default value; it is passed by reference
/// and will be overwritten with the observed transport layer protocol.
///
/// * `skip_packet` - Boolean flag initialized with a `false` value; it is passed by reference
/// and will be overwritten to `true` if the `transport_header` is invalid; in this case the
/// packet will not be considered.
fn analyze_transport_header(transport_header: Option<TransportHeader>,
                            transport_layer: &mut String, port1: &mut u16, port2: &mut u16,
                            application_protocol: &mut AppProtocol,
                            transport_protocol: &mut TransProtocol, skip_packet: &mut bool) {
    match transport_header {
        Some(TransportHeader::Udp(udp_header)) => {
            *transport_layer = "udp".to_string();
            *port1 = udp_header.source_port;
            *port2 = udp_header.destination_port;
            *transport_protocol = TransProtocol::UDP;
            *application_protocol = from_port_to_application_protocol(*port1);
            if (*application_protocol).eq(&AppProtocol::Other) {
                *application_protocol = from_port_to_application_protocol(*port2);
            }
        }
        Some(TransportHeader::Tcp(tcp_header)) => {
            *transport_layer = "tcp".to_string();
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


/// Function to insert the source of a packet into the shared map containing the analyzed traffic.
///
/// # Arguments
///
/// * `mutex_map` - Mutex to permit exclusive access to the shared tuple containing the parsed packets,
/// the total number of sniffed packets and the number of filtered packets.
///
/// * `key` - An `AddressPort` element representing the source of the packet. It corresponds to the map key part.
///
/// * `exchanged_bytes` - IP payload dimension of the observed packet.
///
/// * `transport_protocol` - Transport layer protocol carried by the observed packet.
///
/// * `application_protocol` - Application layer protocol (obtained from port numbers).
fn modify_or_insert_source_in_map(mutex_map: Arc<Mutex<(HashMap<AddressPort, ReportInfo>, u128, u128)>>,
                                  key: AddressPort, exchanged_bytes: u32, transport_protocol: TransProtocol,
                                  application_protocol: AppProtocol) {
    let now_ugly: DateTime<Local> = Local::now();
    let now = now_ugly.format("%d/%m/%Y %H:%M:%S").to_string();
    mutex_map.lock().expect("Error acquiring mutex\n\r").0.entry(key).and_modify(|info| {
        info.transmitted_bytes += exchanged_bytes;
        info.transmitted_packets += 1;
        info.final_timestamp = now.clone();
        info.trans_protocols.insert(transport_protocol);
        if application_protocol.ne(&AppProtocol::Other) {
            info.app_protocols.insert(application_protocol);
        }
    })
        .or_insert(ReportInfo {
            transmitted_bytes: exchanged_bytes,
            transmitted_packets: 1,
            received_bytes: 0,
            received_packets: 0,
            initial_timestamp: now.clone(),
            final_timestamp: now.clone(),
            trans_protocols: HashSet::from([transport_protocol]),
            app_protocols: if application_protocol.ne(&AppProtocol::Other) {
                                HashSet::from([application_protocol])
                            }
                            else {
                                HashSet::new()
                            }
        }
            );
}


/// Function to insert the destination of a packet into the shared map containing the analyzed traffic.
///
/// # Arguments
///
/// * `mutex_map` - Mutex to permit exclusive access to the shared tuple containing the parsed packets,
/// the total number of sniffed packets and the number of filtered packets.
///
/// * `key` - An `AddressPort` element representing the destination of the packet.
/// It corresponds to the map key part.
///
/// * `exchanged_bytes` - IP payload dimension of the observed packet.
///
/// * `transport_protocol` - Transport layer protocol carried by the observed packet.
///
/// * `application_protocol` - Application layer protocol (obtained from port numbers).
fn modify_or_insert_destination_in_map(mutex_map: Arc<Mutex<(HashMap<AddressPort, ReportInfo>, u128, u128)>>,
                                       key: AddressPort, exchanged_bytes: u32, transport_protocol: TransProtocol,
                                       application_protocol: AppProtocol) {
    let now_ugly: DateTime<Local> = Local::now();
    let now = now_ugly.format("%d/%m/%Y %H:%M:%S").to_string();
    mutex_map.lock().expect("Error acquiring mutex\n\r").0.entry(key).and_modify(|info| {
        info.received_bytes += exchanged_bytes;
        info.received_packets += 1;
        info.final_timestamp = now.clone();
        info.trans_protocols.insert(transport_protocol);
        if application_protocol.ne(&AppProtocol::Other) {
            info.app_protocols.insert(application_protocol);
        }
    })
        .or_insert(ReportInfo {
            transmitted_bytes: 0,
            transmitted_packets: 0,
            received_bytes: exchanged_bytes,
            received_packets: 1,
            initial_timestamp: now.clone(),
            final_timestamp: now.clone(),
            trans_protocols: HashSet::from([transport_protocol]),
            app_protocols: if application_protocol.ne(&AppProtocol::Other) {
                                HashSet::from([application_protocol])
                            }
                            else {
                                HashSet::new()
                            }
        });
}


/// Given an integer in the range `0..=65535`, this function returns an `Option<AppProtocol>` containing
/// the respective application protocol represented by a value of the `AppProtocol` enum.
/// Only the most common application layer protocols are considered; if a unknown port number
/// is provided, this function returns `None`.
///
/// # Arguments
///
/// * `port` - An integer representing the transport layer port to be mapped to
/// an application layer protocol.
///
/// # Examples
///
/// ```
/// let x = from_port_to_application_protocol(25);
/// //Simple Mail Transfer Protocol
/// assert_eq!(x, Option::Some(AppProtocol::SMTP));
///
/// let y = from_port_to_application_protocol(1999);
/// //Unknown port-to-protocol mapping
/// assert_eq!(y, Option::None);
/// ```
fn from_port_to_application_protocol(port: u16) -> AppProtocol {
    match port {
        20..=21 => AppProtocol::FTP,
        22 => AppProtocol::SSH,
        23 => AppProtocol::Telnet,
        25 => AppProtocol::SMTP,
        53 => AppProtocol::DNS,
        67..=68 => AppProtocol::DHCP,
        69 => AppProtocol::TFTP,
        80 | 8080 => AppProtocol::HTTP,
        110 => AppProtocol::POP,
        123 => AppProtocol::NTP,
        137..=139 => AppProtocol::NetBIOS,
        143 => AppProtocol::IMAP,
        161..=162 => AppProtocol::SNMP,
        179 => AppProtocol::BGP,
        389 => AppProtocol::LDAP,
        443 => AppProtocol::HTTPS,
        636 => AppProtocol::LDAPS,
        989..=990 => AppProtocol::FTPS,
        1900 => AppProtocol::SSDP,
        5222 => AppProtocol::XMPP,
        5353 => AppProtocol::mDNS,
        _ => {AppProtocol::Other}
    }
}


// Test for this function at the end of this file (run with cargo test)
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
pub fn ipv6_from_long_dec_to_short_hex(ipv6_long: [u8;16]) -> String {

    //from hex to dec, paying attention to the correct number of digits
    let mut ipv6_hex = "".to_string();
    for i in 0..=15 {
        //pari: primo byte del gruppo
        if i % 2 == 0 {
            if *ipv6_long.get(i).unwrap() == 0 {
                continue;
            }
            ipv6_hex.push_str(&format!("{:x}", ipv6_long.get(i).unwrap()));
        }

        //dispari: secondo byte del gruppo
        else {
            if *ipv6_long.get(i-1).unwrap() == 0 {
                ipv6_hex.push_str(&format!("{:x}:", ipv6_long.get(i).unwrap()));
            }
            else {
                ipv6_hex.push_str(&format!("{:02x}:", ipv6_long.get(i).unwrap()));
            }
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
        }
        else {
            if current_zero_sequence != 0 {
                if current_zero_sequence > longest_zero_sequence {
                    longest_zero_sequence = current_zero_sequence;
                    longest_zero_sequence_start = current_zero_sequence_start;
                }
                current_zero_sequence = 0;
            }
        }
        i += 1;
    }
    if current_zero_sequence != 0 { // to catch consecutive zeros at the end
        if current_zero_sequence > longest_zero_sequence {
            longest_zero_sequence = current_zero_sequence;
            longest_zero_sequence_start = current_zero_sequence_start;
        }
    }
    if longest_zero_sequence < 2 { // no compression needed
        return ipv6_hex;
    }

    //from longest sequence of consecutive zeros to '::'
    let mut ipv6_hex_compressed = "".to_string();
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
        return  ipv6_hex_compressed;
    }
    ipv6_hex_compressed.pop();

    ipv6_hex_compressed
}


#[cfg(test)]
mod ipv6_format_tests {

    use crate::thread_parse_packets_functions::ipv6_from_long_dec_to_short_hex;

    #[test]
    fn simple_test() {
        let result = ipv6_from_long_dec_to_short_hex([255,10,10,255,255,10,10,255,255,10,10,255,255,10,10,255,]);
        assert_eq!(result, "ff0a:aff:ff0a:aff:ff0a:aff:ff0a:aff".to_string());
    }

    #[test]
    fn zeros_in_the_middle() {
        let result = ipv6_from_long_dec_to_short_hex([255,10,10,255,0,0,0,0,28,4,4,28,255,1,0,0]);
        assert_eq!(result, "ff0a:aff::1c04:41c:ff01:0".to_string());
    }

    #[test]
    fn leading_zeros() {
        let result = ipv6_from_long_dec_to_short_hex([0,0,0,0,0,0,0,0,28,4,4,28,255,1,0,10]);
        assert_eq!(result, "::1c04:41c:ff01:a".to_string());
    }

    #[test]
    fn tail_one_after_zeros() {
        let result = ipv6_from_long_dec_to_short_hex([28,4,4,28,255,1,0,10,0,0,0,0,0,0,0,1]);
        assert_eq!(result, "1c04:41c:ff01:a::1".to_string());
    }

    #[test]
    fn tail_zeros() {
        let result = ipv6_from_long_dec_to_short_hex([28,4,4,28,255,1,0,10,0,0,0,0,0,0,0,0]);
        assert_eq!(result, "1c04:41c:ff01:a::".to_string());
    }

    #[test]
    fn multiple_zero_sequences_first_longer() {
        let result = ipv6_from_long_dec_to_short_hex([32,0,0,0,0,0,0,0,1,1,0,0,0,0,0,1]);
        assert_eq!(result, "2000::101:0:0:1".to_string());
    }

    #[test]
    fn multiple_zero_sequences_first_longer_head() {
        let result = ipv6_from_long_dec_to_short_hex([0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,1]);
        assert_eq!(result, "::101:0:0:1".to_string());
    }

    #[test]
    fn multiple_zero_sequences_second_longer() {
        let result = ipv6_from_long_dec_to_short_hex([1,0,0,0,0,0,0,1,0,0,0,0,0,0,3,118]);
        assert_eq!(result, "100:0:0:1::376".to_string());
    }

    #[test]
    fn multiple_zero_sequences_second_longer_tail() {
        let result = ipv6_from_long_dec_to_short_hex([32,0,0,0,0,0,0,1,1,1,0,0,0,0,0,0]);
        assert_eq!(result, "2000:0:0:1:101::".to_string());
    }

    #[test]
    fn multiple_zero_sequences_equal_length() {
        let result = ipv6_from_long_dec_to_short_hex([118,3,0,0,0,0,0,1,1,1,0,0,0,0,0,1]);
        assert_eq!(result, "7603::1:101:0:0:1".to_string());
    }

    #[test]
    fn all_zeros() {
        let result = ipv6_from_long_dec_to_short_hex([0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
        assert_eq!(result, "::".to_string());
    }

    #[test]
    fn x_all_zeros() {
        let result = ipv6_from_long_dec_to_short_hex([161,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
        assert_eq!(result, "a100::".to_string());
    }

    #[test]
    fn all_zeros_x() {
        let result = ipv6_from_long_dec_to_short_hex([0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,176]);
        assert_eq!(result, "::b0".to_string());
    }

    #[test]
    fn many_zeros_but_no_compression() {
        let result = ipv6_from_long_dec_to_short_hex([0,16, 16,0, 0,1, 7,0, 0,2, 216,0, 1,0, 0,1]);
        assert_eq!(result, "10:1000:1:700:2:d800:100:1".to_string());
    }

}