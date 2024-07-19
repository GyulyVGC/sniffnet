use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use chrono::Local;
use dns_lookup::lookup_addr;
use etherparse::{LaxPacketHeaders, LinkHeader, NetHeaders, TransportHeader};
use pcap::{Address, Device};

use crate::mmdb::asn::get_asn;
use crate::mmdb::country::get_country;
use crate::mmdb::types::mmdb_reader::MmdbReader;
use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::data_info_host::DataInfoHost;
use crate::networking::types::host::Host;
use crate::networking::types::icmp_type::{IcmpType, IcmpTypeV4, IcmpTypeV6};
use crate::networking::types::info_address_port_pair::InfoAddressPortPair;
use crate::networking::types::my_device::MyDevice;
use crate::networking::types::packet_filters_fields::PacketFiltersFields;
use crate::networking::types::service::Service;
use crate::networking::types::service_query::ServiceQuery;
use crate::networking::types::traffic_direction::TrafficDirection;
use crate::networking::types::traffic_type::TrafficType;
use crate::utils::formatted_strings::get_domain_from_r_dns;
use crate::IpVersion::{IPv4, IPv6};
use crate::{InfoTraffic, IpVersion, Protocol};

include!(concat!(env!("OUT_DIR"), "/services.rs"));

/// Calls methods to analyze link, network, and transport headers.
/// Returns the relevant collected information.
pub fn analyze_headers(
    headers: LaxPacketHeaders,
    mac_addresses: &mut (Option<String>, Option<String>),
    exchanged_bytes: &mut u128,
    icmp_type: &mut IcmpType,
    packet_filters_fields: &mut PacketFiltersFields,
) -> Option<AddressPortPair> {
    analyze_link_header(
        headers.link,
        &mut mac_addresses.0,
        &mut mac_addresses.1,
        exchanged_bytes,
    );

    if !analyze_network_header(
        headers.net,
        exchanged_bytes,
        &mut packet_filters_fields.ip_version,
        &mut packet_filters_fields.source,
        &mut packet_filters_fields.dest,
    ) {
        return None;
    }

    if !analyze_transport_header(
        headers.transport,
        &mut packet_filters_fields.sport,
        &mut packet_filters_fields.dport,
        &mut packet_filters_fields.protocol,
        icmp_type,
    ) {
        return None;
    }

    Some(AddressPortPair::new(
        packet_filters_fields.source.to_string(),
        packet_filters_fields.sport,
        packet_filters_fields.dest.to_string(),
        packet_filters_fields.dport,
        packet_filters_fields.protocol,
    ))
}

/// This function analyzes the data link layer header passed as parameter and updates variables
/// passed by reference on the basis of the packet header content.
/// Returns false if packet has to be skipped.
fn analyze_link_header(
    link_header: Option<LinkHeader>,
    mac_address1: &mut Option<String>,
    mac_address2: &mut Option<String>,
    exchanged_bytes: &mut u128,
) {
    if let Some(LinkHeader::Ethernet2(header)) = link_header {
        *exchanged_bytes += 14;
        *mac_address1 = Some(mac_from_dec_to_hex(header.source));
        *mac_address2 = Some(mac_from_dec_to_hex(header.destination));
    } else {
        *mac_address1 = None;
        *mac_address2 = None;
    }
}

/// This function analyzes the network layer header passed as parameter and updates variables
/// passed by reference on the basis of the packet header content.
/// Returns false if packet has to be skipped.
fn analyze_network_header(
    network_header: Option<NetHeaders>,
    exchanged_bytes: &mut u128,
    network_protocol: &mut IpVersion,
    address1: &mut IpAddr,
    address2: &mut IpAddr,
) -> bool {
    match network_header {
        Some(NetHeaders::Ipv4(ipv4header, _)) => {
            *network_protocol = IpVersion::IPv4;
            *address1 = IpAddr::from(ipv4header.source);
            *address2 = IpAddr::from(ipv4header.destination);
            *exchanged_bytes += u128::from(ipv4header.total_len);
            true
        }
        Some(NetHeaders::Ipv6(ipv6header, _)) => {
            *network_protocol = IpVersion::IPv6;
            *address1 = IpAddr::from(ipv6header.source);
            *address2 = IpAddr::from(ipv6header.destination);
            *exchanged_bytes += u128::from(40 + ipv6header.payload_length);
            true
        }
        _ => false,
    }
}

/// This function analyzes the transport layer header passed as parameter and updates variables
/// passed by reference on the basis of the packet header content.
/// Returns false if packet has to be skipped.
fn analyze_transport_header(
    transport_header: Option<TransportHeader>,
    port1: &mut Option<u16>,
    port2: &mut Option<u16>,
    protocol: &mut Protocol,
    icmp_type: &mut IcmpType,
) -> bool {
    match transport_header {
        Some(TransportHeader::Udp(udp_header)) => {
            *port1 = Some(udp_header.source_port);
            *port2 = Some(udp_header.destination_port);
            *protocol = Protocol::UDP;
            true
        }
        Some(TransportHeader::Tcp(tcp_header)) => {
            *port1 = Some(tcp_header.source_port);
            *port2 = Some(tcp_header.destination_port);
            *protocol = Protocol::TCP;
            true
        }
        Some(TransportHeader::Icmpv4(icmpv4_header)) => {
            *port1 = None;
            *port2 = None;
            *protocol = Protocol::ICMP;
            *icmp_type = IcmpTypeV4::from_etherparse(&icmpv4_header.icmp_type);
            true
        }
        Some(TransportHeader::Icmpv6(icmpv6_header)) => {
            *port1 = None;
            *port2 = None;
            *protocol = Protocol::ICMP;
            *icmp_type = IcmpTypeV6::from_etherparse(&icmpv6_header.icmp_type);
            true
        }
        _ => false,
    }
}

pub fn get_service(key: &AddressPortPair, traffic_direction: TrafficDirection) -> Service {
    if key.port1.is_none() || key.port2.is_none() || key.protocol == Protocol::ICMP {
        return Service::NotApplicable;
    }

    // to return the service associated with the highest score:
    // score = service_is_some * (port_is_well_known + bonus_direction)
    // service_is_some: 1 if some, 0 if unknown
    // port_is_well_known: 3 if well known, 1 if not
    // bonus_direction: +1 assigned to remote port
    let compute_service_score = |service: &Service, port: u16, bonus_direction: bool| {
        let service_is_some = u8::from(matches!(service, Service::Name(_)));
        let port_is_well_known = if port < 1024 { 3 } else { 1 };
        let bonus_direction = u8::from(bonus_direction);
        service_is_some * (port_is_well_known + bonus_direction)
    };

    let port1 = key.port1.unwrap();
    let port2 = key.port2.unwrap();

    let unknown = Service::Unknown;
    let service1 = SERVICES
        .get(&ServiceQuery(port1, key.protocol))
        .unwrap_or(&unknown);
    let service2 = SERVICES
        .get(&ServiceQuery(port2, key.protocol))
        .unwrap_or(&unknown);

    let score1 = compute_service_score(
        service1,
        port1,
        traffic_direction.ne(&TrafficDirection::Outgoing),
    );
    let score2 = compute_service_score(
        service2,
        port2,
        traffic_direction.eq(&TrafficDirection::Outgoing),
    );

    if score1 > score2 {
        *service1
    } else {
        *service2
    }
}

/// Function to insert the source and destination of a packet into the shared map containing the analyzed traffic.
pub fn modify_or_insert_in_map(
    info_traffic_mutex: &Arc<Mutex<InfoTraffic>>,
    key: &AddressPortPair,
    my_device: &MyDevice,
    mac_addresses: (Option<String>, Option<String>),
    icmp_type: IcmpType,
    exchanged_bytes: u128,
) -> InfoAddressPortPair {
    let now = Local::now();
    let mut traffic_direction = TrafficDirection::default();
    let mut service = Service::Unknown;

    if !info_traffic_mutex.lock().unwrap().map.contains_key(key) {
        // first occurrence of key

        // update device addresses
        let mut my_interface_addresses = Vec::new();
        for dev in Device::list().expect("Error retrieving device list\r\n") {
            if dev.name.eq(&my_device.name) {
                let mut my_interface_addresses_mutex = my_device.addresses.lock().unwrap();
                my_interface_addresses_mutex.clone_from(&dev.addresses);
                drop(my_interface_addresses_mutex);
                my_interface_addresses = dev.addresses;
                break;
            }
        }
        // determine traffic direction
        let source_ip = &key.address1;
        let destination_ip = &key.address2;
        traffic_direction = get_traffic_direction(
            source_ip,
            destination_ip,
            key.port1,
            key.port2,
            &my_interface_addresses,
        );
        // determine upper layer service
        service = get_service(key, traffic_direction);
    };

    let mut info_traffic = info_traffic_mutex
        .lock()
        .expect("Error acquiring mutex\n\r");

    let new_info: InfoAddressPortPair = info_traffic
        .map
        .entry(key.clone())
        .and_modify(|info| {
            info.transmitted_bytes += exchanged_bytes;
            info.transmitted_packets += 1;
            info.final_timestamp = now;
            if key.protocol.eq(&Protocol::ICMP) {
                info.icmp_types
                    .entry(icmp_type)
                    .and_modify(|n| *n += 1)
                    .or_insert(1);
            }
        })
        .or_insert_with(|| InfoAddressPortPair {
            mac_address1: mac_addresses.0,
            mac_address2: mac_addresses.1,
            transmitted_bytes: exchanged_bytes,
            transmitted_packets: 1,
            initial_timestamp: now,
            final_timestamp: now,
            service,
            traffic_direction,
            icmp_types: if key.protocol.eq(&Protocol::ICMP) {
                HashMap::from([(icmp_type, 1)])
            } else {
                HashMap::new()
            },
        })
        .clone();

    if let Some(host_info) = info_traffic
        .addresses_resolved
        .get(&get_address_to_lookup(key, new_info.traffic_direction))
        .cloned()
    {
        if info_traffic.favorite_hosts.contains(&host_info.1) {
            info_traffic.favorites_last_interval.insert(host_info.1);
        }
    }

    new_info
}

pub fn reverse_dns_lookup(
    info_traffic: &Arc<Mutex<InfoTraffic>>,
    key: &AddressPortPair,
    traffic_direction: TrafficDirection,
    my_device: &MyDevice,
    country_db_reader: &Arc<MmdbReader>,
    asn_db_reader: &Arc<MmdbReader>,
) {
    let address_to_lookup = get_address_to_lookup(key, traffic_direction);
    let my_interface_addresses = my_device.addresses.lock().unwrap().clone();

    // perform rDNS lookup
    let lookup_result = lookup_addr(&address_to_lookup.parse().unwrap());

    // get new host info and build the new host
    let traffic_type = get_traffic_type(
        &address_to_lookup,
        &my_interface_addresses,
        traffic_direction,
    );
    let is_loopback = is_loopback(&address_to_lookup);
    let is_local = is_local_connection(&address_to_lookup, &my_interface_addresses);
    let country = get_country(&address_to_lookup, country_db_reader);
    let asn = get_asn(&address_to_lookup, asn_db_reader);
    let r_dns = if let Ok(result) = lookup_result {
        if result.is_empty() {
            address_to_lookup.clone()
        } else {
            result
        }
    } else {
        address_to_lookup.clone()
    };
    let new_host = Host {
        domain: get_domain_from_r_dns(r_dns.clone()),
        asn,
        country,
    };

    let mut info_traffic_lock = info_traffic.lock().unwrap();
    // collect the data exchanged from the same address so far and remove the address from the collection of addresses waiting a rDNS
    let other_data = info_traffic_lock
        .addresses_waiting_resolution
        .remove(&address_to_lookup)
        .unwrap_or_default();
    // insert the newly resolved host in the collections, with the data it exchanged so far
    info_traffic_lock
        .addresses_resolved
        .insert(address_to_lookup, (r_dns, new_host.clone()));
    info_traffic_lock
        .hosts
        .entry(new_host.clone())
        .and_modify(|data_info_host| {
            data_info_host.data_info += other_data;
        })
        .or_insert_with(|| DataInfoHost {
            data_info: other_data,
            is_favorite: false,
            is_loopback,
            is_local,
            traffic_type,
        });
    // check if the newly resolved host was featured in the favorites (possible in case of already existing host)
    if info_traffic_lock.favorite_hosts.contains(&new_host) {
        info_traffic_lock.favorites_last_interval.insert(new_host);
    }

    drop(info_traffic_lock);
}

/// Returns the traffic direction observed (incoming or outgoing)
fn get_traffic_direction(
    source_ip: &String,
    destination_ip: &String,
    source_port: Option<u16>,
    dest_port: Option<u16>,
    my_interface_addresses: &[Address],
) -> TrafficDirection {
    let my_interface_addresses_string: Vec<String> = my_interface_addresses
        .iter()
        .map(|address| address.addr.to_string())
        .collect();

    // first let's handle TCP and UDP loopback
    if is_loopback(source_ip) && is_loopback(destination_ip) {
        if let (Some(sport), Some(dport)) = (source_port, dest_port) {
            return if sport > dport {
                TrafficDirection::Outgoing
            } else {
                TrafficDirection::Incoming
            };
        }
    }

    if my_interface_addresses_string.contains(source_ip) {
        // source is local
        TrafficDirection::Outgoing
    } else if source_ip.ne("0.0.0.0") && source_ip.ne("::") {
        // source not local and different from 0.0.0.0 and different from ::
        TrafficDirection::Incoming
    } else if !my_interface_addresses_string.contains(destination_ip) {
        // source is 0.0.0.0 or :: (local not yet assigned an IP) and destination is not local
        TrafficDirection::Outgoing
    } else {
        TrafficDirection::Incoming
    }
}

/// Returns the traffic type observed (unicast, multicast or broadcast)
/// It refers to the remote host
pub fn get_traffic_type(
    destination_ip: &str,
    my_interface_addresses: &[Address],
    traffic_direction: TrafficDirection,
) -> TrafficType {
    if traffic_direction.eq(&TrafficDirection::Outgoing) {
        if is_multicast_address(destination_ip) {
            TrafficType::Multicast
        } else if is_broadcast_address(destination_ip, my_interface_addresses) {
            TrafficType::Broadcast
        } else {
            TrafficType::Unicast
        }
    } else {
        TrafficType::Unicast
    }
}

/// Determines if the input address is a multicast address or not.
///
/// # Arguments
///
/// * `address` - string representing an IPv4 or IPv6 network address.
fn is_multicast_address(address: &str) -> bool {
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
fn is_broadcast_address(address: &str, my_interface_addresses: &[Address]) -> bool {
    if address.eq("255.255.255.255") {
        return true;
    }
    // check if directed broadcast
    let my_broadcast_addresses: Vec<String> = my_interface_addresses
        .iter()
        .map(|address| {
            address
                .broadcast_addr
                .unwrap_or_else(|| "255.255.255.255".parse().unwrap())
                .to_string()
        })
        .collect();
    if my_broadcast_addresses.contains(&address.to_string()) {
        return true;
    }
    false
}

fn is_loopback(address_to_lookup: &str) -> bool {
    IpAddr::from_str(address_to_lookup)
        .unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED))
        .is_loopback()
}

/// Determines if the connection is local
pub fn is_local_connection(address_to_lookup: &str, my_interface_addresses: &Vec<Address>) -> bool {
    let mut ret_val = false;

    let address_to_lookup_type = if address_to_lookup.contains(':') {
        IPv6
    } else {
        IPv4
    };

    for address in my_interface_addresses {
        match address.addr {
            IpAddr::V4(local_addr) if address_to_lookup_type.eq(&IPv4) => {
                // check if the two IPv4 addresses are in the same subnet
                let address_to_lookup_parsed: Ipv4Addr = address_to_lookup
                    .parse()
                    .unwrap_or_else(|_| Ipv4Addr::from(0));
                // remote is link local?
                if address_to_lookup_parsed.is_link_local() {
                    ret_val = true;
                }
                // is the same subnet?
                else if let Some(IpAddr::V4(netmask)) = address.netmask {
                    let mut local_subnet = Vec::new();
                    let mut remote_subnet = Vec::new();
                    let netmask_digits = netmask.octets();
                    let local_addr_digits = local_addr.octets();
                    let remote_addr_digits = address_to_lookup_parsed.octets();
                    for (i, netmask_digit) in netmask_digits.iter().enumerate() {
                        local_subnet.push(netmask_digit & local_addr_digits[i]);
                        remote_subnet.push(netmask_digit & remote_addr_digits[i]);
                    }
                    if local_subnet == remote_subnet {
                        ret_val = true;
                    }
                }
            }
            IpAddr::V6(local_addr) if address_to_lookup_type.eq(&IPv6) => {
                // check if the two IPv6 addresses are in the same subnet
                let address_to_lookup_parsed: Ipv6Addr = address_to_lookup
                    .parse()
                    .unwrap_or_else(|_| Ipv6Addr::from(0));
                // remote is link local?
                if address_to_lookup.starts_with("fe80") {
                    ret_val = true;
                }
                // is the same subnet?
                else if let Some(IpAddr::V6(netmask)) = address.netmask {
                    let mut local_subnet = Vec::new();
                    let mut remote_subnet = Vec::new();
                    let netmask_digits = netmask.octets();
                    let local_addr_digits = local_addr.octets();
                    let remote_addr_digits = address_to_lookup_parsed.octets();
                    for (i, netmask_digit) in netmask_digits.iter().enumerate() {
                        local_subnet.push(netmask_digit & local_addr_digits[i]);
                        remote_subnet.push(netmask_digit & remote_addr_digits[i]);
                    }
                    if local_subnet == remote_subnet {
                        ret_val = true;
                    }
                }
            }
            _ => {}
        }
    }

    ret_val
}

/// Determines if the address passed as parameter belong to the chosen adapter
pub fn is_my_address(local_address: &String, my_interface_addresses: &Vec<Address>) -> bool {
    for address in my_interface_addresses {
        if address.addr.to_string().eq(local_address) {
            return true;
        }
    }
    is_loopback(local_address)
}

/// Converts a MAC address in its hexadecimal form
fn mac_from_dec_to_hex(mac_dec: [u8; 6]) -> String {
    let mut mac_hex = String::new();
    for n in &mac_dec {
        mac_hex.push_str(&format!("{n:02x}:"));
    }
    mac_hex.pop();
    mac_hex
}

pub fn get_address_to_lookup(key: &AddressPortPair, traffic_direction: TrafficDirection) -> String {
    match traffic_direction {
        TrafficDirection::Outgoing => key.address2.clone(),
        TrafficDirection::Incoming => key.address1.clone(),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::net::IpAddr;

    use pcap::Address;

    use crate::networking::manage_packets::{
        get_service, get_traffic_direction, get_traffic_type, is_local_connection,
        mac_from_dec_to_hex,
    };
    use crate::networking::types::address_port_pair::AddressPortPair;
    use crate::networking::types::service_query::ServiceQuery;
    use crate::networking::types::traffic_direction::TrafficDirection;
    use crate::networking::types::traffic_type::TrafficType;
    use crate::Protocol;
    use crate::Service;

    include!(concat!(env!("OUT_DIR"), "/services.rs"));

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
        let result = IpAddr::from([
            255, 10, 10, 255, 255, 10, 10, 255, 255, 10, 10, 255, 255, 10, 10, 255,
        ]);
        assert_eq!(
            result.to_string(),
            "ff0a:aff:ff0a:aff:ff0a:aff:ff0a:aff".to_string()
        );
    }

    #[test]
    fn ipv6_zeros_in_the_middle() {
        let result =
            IpAddr::from([255, 10, 10, 255, 0, 0, 0, 0, 28, 4, 4, 28, 255, 1, 0, 0]).to_string();
        assert_eq!(result, "ff0a:aff::1c04:41c:ff01:0".to_string());
    }

    #[test]
    fn ipv6_leading_zeros() {
        let result =
            IpAddr::from([0, 0, 0, 0, 0, 0, 0, 0, 28, 4, 4, 28, 255, 1, 0, 10]).to_string();
        assert_eq!(result, "::1c04:41c:ff01:a".to_string());
    }

    #[test]
    fn ipv6_tail_one_after_zeros() {
        let result =
            IpAddr::from([28, 4, 4, 28, 255, 1, 0, 10, 0, 0, 0, 0, 0, 0, 0, 1]).to_string();
        assert_eq!(result, "1c04:41c:ff01:a::1".to_string());
    }

    #[test]
    fn ipv6_tail_zeros() {
        let result =
            IpAddr::from([28, 4, 4, 28, 255, 1, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0]).to_string();
        assert_eq!(result, "1c04:41c:ff01:a::".to_string());
    }

    #[test]
    fn ipv6_multiple_zero_sequences_first_longer() {
        let result = IpAddr::from([32, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1]).to_string();
        assert_eq!(result, "2000::101:0:0:1".to_string());
    }

    #[test]
    fn ipv6_multiple_zero_sequences_first_longer_head() {
        let result = IpAddr::from([0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1]).to_string();
        assert_eq!(result, "::101:0:0:1".to_string());
    }

    #[test]
    fn ipv6_multiple_zero_sequences_second_longer() {
        let result = IpAddr::from([1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 3, 118]).to_string();
        assert_eq!(result, "100:0:0:1::376".to_string());
    }

    #[test]
    fn ipv6_multiple_zero_sequences_second_longer_tail() {
        let result = IpAddr::from([32, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0]).to_string();
        assert_eq!(result, "2000:0:0:1:101::".to_string());
    }

    #[test]
    fn ipv6_multiple_zero_sequences_equal_length() {
        let result = IpAddr::from([118, 3, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1]).to_string();
        assert_eq!(result, "7603::1:101:0:0:1".to_string());
    }

    #[test]
    fn ipv6_all_zeros() {
        let result = IpAddr::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).to_string();
        assert_eq!(result, "::".to_string());
    }

    #[test]
    fn ipv6_x_all_zeros() {
        let result = IpAddr::from([161, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).to_string();
        assert_eq!(result, "a100::".to_string());
    }

    #[test]
    fn ipv6_all_zeros_x() {
        let result = IpAddr::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 176]).to_string();
        assert_eq!(result, "::b0".to_string());
    }

    #[test]
    fn ipv6_many_zeros_but_no_compression() {
        let result = IpAddr::from([0, 16, 16, 0, 0, 1, 7, 0, 0, 2, 216, 0, 1, 0, 0, 1]).to_string();
        assert_eq!(result, "10:1000:1:700:2:d800:100:1".to_string());
    }

    #[test]
    fn traffic_direction_ipv4_test() {
        let mut address_vec: Vec<Address> = Vec::new();
        let my_address_v4 = Address {
            addr: IpAddr::V4("172.20.10.9".parse().unwrap()),
            netmask: Some(IpAddr::V4("255.255.255.240".parse().unwrap())),
            broadcast_addr: Some(IpAddr::V4("172.20.10.15".parse().unwrap())),
            dst_addr: None,
        };
        let my_address_v6 = Address {
            addr: IpAddr::V6("fe80::8b1:1234:5678:d065".parse().unwrap()),
            netmask: Some(IpAddr::V6("ffff:ffff:ffff:ffff::".parse().unwrap())),
            broadcast_addr: None,
            dst_addr: None,
        };
        address_vec.push(my_address_v4);
        address_vec.push(my_address_v6);

        let result1 = get_traffic_direction(
            &"172.20.10.9".to_string(),
            &"99.88.77.00".to_string(),
            Some(99),
            Some(99),
            &address_vec,
        );
        assert_eq!(result1, TrafficDirection::Outgoing);
        let result2 = get_traffic_direction(
            &"172.20.10.10".to_string(),
            &"172.20.10.9".to_string(),
            Some(99),
            Some(99),
            &address_vec,
        );
        assert_eq!(result2, TrafficDirection::Incoming);
        let result3 = get_traffic_direction(
            &"172.20.10.9".to_string(),
            &"0.0.0.0".to_string(),
            Some(99),
            Some(99),
            &address_vec,
        );
        assert_eq!(result3, TrafficDirection::Outgoing);
        let result4 = get_traffic_direction(
            &"0.0.0.0".to_string(),
            &"172.20.10.9".to_string(),
            Some(99),
            Some(99),
            &address_vec,
        );
        assert_eq!(result4, TrafficDirection::Incoming);
        let result4 = get_traffic_direction(
            &"0.0.0.0".to_string(),
            &"172.20.10.10".to_string(),
            Some(99),
            Some(99),
            &address_vec,
        );
        assert_eq!(result4, TrafficDirection::Outgoing);
    }

    #[test]
    fn traffic_type_multicast_ipv4_test() {
        let result1 = get_traffic_type("227.255.255.0", &[], TrafficDirection::Outgoing);
        assert_eq!(result1, TrafficType::Multicast);
        let result2 = get_traffic_type("239.255.255.255", &[], TrafficDirection::Outgoing);
        assert_eq!(result2, TrafficType::Multicast);
        let result3 = get_traffic_type("224.0.0.0", &[], TrafficDirection::Outgoing);
        assert_eq!(result3, TrafficType::Multicast);
        let result4 = get_traffic_type("223.255.255.255", &[], TrafficDirection::Outgoing);
        assert_eq!(result4, TrafficType::Unicast);
        let result5 = get_traffic_type("240.0.0.0", &[], TrafficDirection::Outgoing);
        assert_eq!(result5, TrafficType::Unicast);

        let result6 = get_traffic_type("227.255.255.0", &[], TrafficDirection::Incoming);
        assert_eq!(result6, TrafficType::Unicast);
        let result7 = get_traffic_type("239.255.255.255", &[], TrafficDirection::Incoming);
        assert_eq!(result7, TrafficType::Unicast);
        let result8 = get_traffic_type("224.0.0.0", &[], TrafficDirection::Incoming);
        assert_eq!(result8, TrafficType::Unicast);
        let result9 = get_traffic_type("223.255.255.255", &[], TrafficDirection::Incoming);
        assert_eq!(result9, TrafficType::Unicast);
        let result10 = get_traffic_type("240.0.0.0", &[], TrafficDirection::Incoming);
        assert_eq!(result10, TrafficType::Unicast);
    }

    #[test]
    fn traffic_type_multicast_ipv6_test() {
        let result1 = get_traffic_type("ff::", &[], TrafficDirection::Outgoing);
        assert_eq!(result1, TrafficType::Multicast);
        let result2 = get_traffic_type("fe80:1234::", &[], TrafficDirection::Outgoing);
        assert_eq!(result2, TrafficType::Unicast);
        let result3 = get_traffic_type("ffff:ffff:ffff::", &[], TrafficDirection::Outgoing);
        assert_eq!(result3, TrafficType::Multicast);

        let result4 = get_traffic_type("ff::", &[], TrafficDirection::Incoming);
        assert_eq!(result4, TrafficType::Unicast);
        let result5 = get_traffic_type("fe80:1234::", &[], TrafficDirection::Incoming);
        assert_eq!(result5, TrafficType::Unicast);
        let result6 = get_traffic_type("ffff:ffff:ffff::", &[], TrafficDirection::Incoming);
        assert_eq!(result6, TrafficType::Unicast);
    }

    #[test]
    fn traffic_type_host_local_broadcast_test() {
        let result1 = get_traffic_type("255.255.255.255", &[], TrafficDirection::Outgoing);
        assert_eq!(result1, TrafficType::Broadcast);
        let result2 = get_traffic_type("255.255.255.255", &[], TrafficDirection::Incoming);
        assert_eq!(result2, TrafficType::Unicast);
        let result3 = get_traffic_type("255.255.255.254", &[], TrafficDirection::Outgoing);
        assert_eq!(result3, TrafficType::Unicast);

        let mut address_vec: Vec<Address> = Vec::new();
        let my_address = Address {
            addr: IpAddr::V4("172.20.10.9".parse().unwrap()),
            netmask: Some(IpAddr::V4("255.255.255.240".parse().unwrap())),
            broadcast_addr: Some(IpAddr::V4("172.20.10.15".parse().unwrap())),
            dst_addr: None,
        };
        address_vec.push(my_address);

        let result1 = get_traffic_type("255.255.255.255", &address_vec, TrafficDirection::Outgoing);
        assert_eq!(result1, TrafficType::Broadcast);
        let result2 = get_traffic_type("255.255.255.255", &address_vec, TrafficDirection::Incoming);
        assert_eq!(result2, TrafficType::Unicast);
    }

    #[test]
    fn traffic_type_host_directed_broadcast_test() {
        let result1 = get_traffic_type("172.20.10.15", &[], TrafficDirection::Outgoing);
        assert_eq!(result1, TrafficType::Unicast);
        let result2 = get_traffic_type("172.20.10.15", &[], TrafficDirection::Incoming);
        assert_eq!(result2, TrafficType::Unicast);

        let mut address_vec: Vec<Address> = Vec::new();
        let my_address = Address {
            addr: IpAddr::V4("172.20.10.9".parse().unwrap()),
            netmask: Some(IpAddr::V4("255.255.255.240".parse().unwrap())),
            broadcast_addr: Some(IpAddr::V4("172.20.10.15".parse().unwrap())),
            dst_addr: None,
        };
        address_vec.push(my_address);

        let result1 = get_traffic_type("172.20.10.15", &address_vec, TrafficDirection::Outgoing);
        assert_eq!(result1, TrafficType::Broadcast);
        let result2 = get_traffic_type("172.20.10.15", &address_vec, TrafficDirection::Incoming);
        assert_eq!(result2, TrafficType::Unicast);
    }

    #[test]
    fn is_local_connection_ipv4_test() {
        let mut address_vec: Vec<Address> = Vec::new();
        let my_address_v4 = Address {
            addr: IpAddr::V4("172.20.10.9".parse().unwrap()),
            netmask: Some(IpAddr::V4("255.255.255.240".parse().unwrap())),
            broadcast_addr: Some(IpAddr::V4("172.20.10.15".parse().unwrap())),
            dst_addr: None,
        };
        let my_address_v6 = Address {
            addr: IpAddr::V6("fe80::8b1:1234:5678:d065".parse().unwrap()),
            netmask: Some(IpAddr::V6("ffff:ffff:ffff:ffff::".parse().unwrap())),
            broadcast_addr: None,
            dst_addr: None,
        };
        address_vec.push(my_address_v4);
        address_vec.push(my_address_v6);

        let result1 = is_local_connection("104.18.43.158", &address_vec);
        assert_eq!(result1, false);

        let result2 = is_local_connection("172.20.10.15", &address_vec);
        assert_eq!(result2, true);

        let result3 = is_local_connection("172.20.10.16", &address_vec);
        assert_eq!(result3, false);

        let result4 = is_local_connection("172.20.10.0", &address_vec);
        assert_eq!(result4, true);

        let result5 = is_local_connection("172.20.10.7", &address_vec);
        assert_eq!(result5, true);

        let result6 = is_local_connection("172.20.10.99", &address_vec);
        assert_eq!(result6, false);
    }

    #[test]
    fn is_local_connection_ipv6_test() {
        let mut address_vec: Vec<Address> = Vec::new();
        let my_address_v4 = Address {
            addr: IpAddr::V4("172.20.10.9".parse().unwrap()),
            netmask: Some(IpAddr::V4("255.255.255.240".parse().unwrap())),
            broadcast_addr: Some(IpAddr::V4("172.20.10.15".parse().unwrap())),
            dst_addr: None,
        };
        let my_address_v6 = Address {
            addr: IpAddr::V6("fe90:8b1:1234:5678:d065::1234".parse().unwrap()),
            netmask: Some(IpAddr::V6("ffff:ffff:ffff:ff11::".parse().unwrap())),
            broadcast_addr: None,
            dst_addr: None,
        };
        address_vec.push(my_address_v4);
        address_vec.push(my_address_v6);

        let result1 = is_local_connection("fe90:8b1:1234:5611:d065::1234", &address_vec);
        assert_eq!(result1, false);

        let result2 = is_local_connection("fe90:8b1:1234:5610:d065::1234", &address_vec);
        assert_eq!(result2, true);

        let result3 = is_local_connection("ff90:8b1:1234:5610:d065::1234", &address_vec);
        assert_eq!(result3, false);

        let result4 = is_local_connection("fe90:8b1:1234:5610:ffff:eeee:9876:1234", &address_vec);
        assert_eq!(result4, true);
    }

    #[test]
    fn is_local_connection_ipv4_2_test() {
        let mut address_vec: Vec<Address> = Vec::new();
        let my_address_v4 = Address {
            addr: IpAddr::V4("172.20.10.9".parse().unwrap()),
            netmask: Some(IpAddr::V4("255.255.255.0".parse().unwrap())),
            broadcast_addr: Some(IpAddr::V4("172.20.10.15".parse().unwrap())),
            dst_addr: None,
        };
        let my_address_v6 = Address {
            addr: IpAddr::V6("fe80::8b1:1234:5678:d065".parse().unwrap()),
            netmask: Some(IpAddr::V6("ffff:ffff:ffff:ffff::".parse().unwrap())),
            broadcast_addr: None,
            dst_addr: None,
        };
        address_vec.push(my_address_v4);
        address_vec.push(my_address_v6);

        let result1 = is_local_connection("255.255.255.255", &address_vec);
        assert_eq!(result1, false);

        let result2 = is_local_connection("172.20.10.9", &address_vec);
        assert_eq!(result2, true);

        let result3 = is_local_connection("172.20.10.9", &address_vec);
        assert_eq!(result3, true);

        let result4 = is_local_connection("172.20.10.9", &address_vec);
        assert_eq!(result4, true);

        let result5 = is_local_connection("172.20.10.7", &address_vec);
        assert_eq!(result5, true);

        let result6 = is_local_connection("172.20.10.99", &address_vec);
        assert_eq!(result6, true);

        let result7 = is_local_connection("172.20.11.0", &address_vec);
        assert_eq!(result7, false);

        let result8 = is_local_connection("172.20.9.255", &address_vec);
        assert_eq!(result8, false);
    }

    #[test]
    fn is_local_connection_ipv4_multicast_test() {
        let mut address_vec: Vec<Address> = Vec::new();
        let my_address_v4 = Address {
            addr: IpAddr::V4("172.20.10.9".parse().unwrap()),
            netmask: Some(IpAddr::V4("255.255.255.240".parse().unwrap())),
            broadcast_addr: Some(IpAddr::V4("172.20.10.15".parse().unwrap())),
            dst_addr: None,
        };
        let my_address_v6 = Address {
            addr: IpAddr::V6("fe80::8b1:1234:5678:d065".parse().unwrap()),
            netmask: Some(IpAddr::V6("ffff:ffff:ffff:ffff::".parse().unwrap())),
            broadcast_addr: None,
            dst_addr: None,
        };
        address_vec.push(my_address_v4);
        address_vec.push(my_address_v6);

        let result1 = is_local_connection("224.0.0.251", &address_vec);
        assert_eq!(result1, false);
    }

    #[test]
    fn is_local_connection_ipv6_multicast_test() {
        let mut address_vec: Vec<Address> = Vec::new();
        let my_address_v4 = Address {
            addr: IpAddr::V4("172.20.10.9".parse().unwrap()),
            netmask: Some(IpAddr::V4("255.255.255.240".parse().unwrap())),
            broadcast_addr: Some(IpAddr::V4("172.20.10.15".parse().unwrap())),
            dst_addr: None,
        };
        let my_address_v6 = Address {
            addr: IpAddr::V6("fe80::8b1:1234:5678:d065".parse().unwrap()),
            netmask: Some(IpAddr::V6("ffff:ffff:ffff:ffff::".parse().unwrap())),
            broadcast_addr: None,
            dst_addr: None,
        };
        address_vec.push(my_address_v4);
        address_vec.push(my_address_v6);

        let result1 = is_local_connection("ff::1234", &address_vec);
        assert_eq!(result1, false);
    }

    #[test]
    fn is_local_connection_ipv4_link_local_test() {
        let mut address_vec: Vec<Address> = Vec::new();
        let my_address_v4 = Address {
            addr: IpAddr::V4("172.20.10.9".parse().unwrap()),
            netmask: Some(IpAddr::V4("255.255.255.240".parse().unwrap())),
            broadcast_addr: Some(IpAddr::V4("172.20.10.15".parse().unwrap())),
            dst_addr: None,
        };
        let my_address_v6 = Address {
            addr: IpAddr::V6("fe80::8b1:1234:5678:d065".parse().unwrap()),
            netmask: Some(IpAddr::V6("ffff:ffff:ffff:ffff::".parse().unwrap())),
            broadcast_addr: None,
            dst_addr: None,
        };
        address_vec.push(my_address_v4);
        address_vec.push(my_address_v6);

        let result1 = is_local_connection("224.0.1.2", &address_vec);
        assert_eq!(result1, false);

        let result2 = is_local_connection("169.254.17.199", &address_vec);
        assert_eq!(result2, true);

        let result3 = is_local_connection("169.255.17.199", &address_vec);
        assert_eq!(result3, false);
    }

    #[test]
    fn is_local_connection_ipv6_link_local_test() {
        let mut address_vec: Vec<Address> = Vec::new();
        let my_address_v4 = Address {
            addr: IpAddr::V4("172.20.10.9".parse().unwrap()),
            netmask: Some(IpAddr::V4("255.255.255.240".parse().unwrap())),
            broadcast_addr: Some(IpAddr::V4("172.20.10.15".parse().unwrap())),
            dst_addr: None,
        };
        let my_address_v6 = Address {
            addr: IpAddr::V6("fe90::8b1:1234:5678:d065".parse().unwrap()),
            netmask: Some(IpAddr::V6("ffff:ffff:ffff:ffff::".parse().unwrap())),
            broadcast_addr: None,
            dst_addr: None,
        };
        address_vec.push(my_address_v4);
        address_vec.push(my_address_v6);

        let result1 = is_local_connection("ff88::", &address_vec);
        assert_eq!(result1, false);

        let result2 = is_local_connection("fe80::8b1:1234:5678:d065", &address_vec);
        assert_eq!(result2, true);

        let result3 = is_local_connection("fe70::8b1:1234:5678:d065", &address_vec);
        assert_eq!(result3, false);
    }

    #[test]
    fn test_get_service_simple_only_one_valid() {
        let unknown_port = Some(65000);
        for p in [Protocol::TCP, Protocol::UDP] {
            assert!(SERVICES
                .get(&ServiceQuery(unknown_port.unwrap(), p))
                .is_none());
            for d in [TrafficDirection::Incoming, TrafficDirection::Outgoing] {
                let key = AddressPortPair::new(
                    String::new(),
                    unknown_port,
                    String::new(),
                    unknown_port,
                    p,
                );
                assert_eq!(get_service(&key, d), Service::Unknown);

                for (p1, p2) in [
                    (unknown_port, Some(22)),
                    (Some(22), unknown_port),
                    (Some(22), Some(22)),
                ] {
                    let key = AddressPortPair::new(String::new(), p1, String::new(), p2, p);
                    assert_eq!(get_service(&key, d), Service::Name("ssh"));
                }

                for (p1, p2) in [
                    (unknown_port, Some(443)),
                    (Some(443), unknown_port),
                    (Some(443), Some(443)),
                ] {
                    let key = AddressPortPair::new(String::new(), p1, String::new(), p2, p);
                    assert_eq!(get_service(&key, d), Service::Name("https"));
                }

                for (p1, p2) in [
                    (unknown_port, Some(80)),
                    (Some(80), unknown_port),
                    (Some(80), Some(80)),
                ] {
                    let key = AddressPortPair::new(String::new(), p1, String::new(), p2, p);
                    assert_eq!(get_service(&key, d), Service::Name("http"));
                }

                for (p1, p2) in [
                    (unknown_port, Some(1900)),
                    (Some(1900), unknown_port),
                    (Some(1900), Some(1900)),
                ] {
                    let key = AddressPortPair::new(String::new(), p1, String::new(), p2, p);
                    assert_eq!(get_service(&key, d), Service::Name("upnp"));
                }
            }
        }
    }

    #[test]
    fn test_get_service_well_known_ports_always_win() {
        let valid_but_not_well_known = Some(1030);
        for p in [Protocol::TCP, Protocol::UDP] {
            assert_eq!(
                SERVICES
                    .get(&ServiceQuery(valid_but_not_well_known.unwrap(), p))
                    .unwrap(),
                &Service::Name("iad1")
            );
            for d in [TrafficDirection::Incoming, TrafficDirection::Outgoing] {
                let key = AddressPortPair::new(
                    String::new(),
                    valid_but_not_well_known,
                    String::new(),
                    valid_but_not_well_known,
                    p,
                );
                assert_eq!(get_service(&key, d), Service::Name("iad1"));

                for (p1, p2) in [
                    (valid_but_not_well_known, Some(67)),
                    (Some(67), valid_but_not_well_known),
                    (Some(67), Some(67)),
                ] {
                    let key = AddressPortPair::new(String::new(), p1, String::new(), p2, p);
                    assert_eq!(get_service(&key, d), Service::Name("dhcps"));
                }

                for (p1, p2) in [
                    (valid_but_not_well_known, Some(179)),
                    (Some(179), valid_but_not_well_known),
                    (Some(179), Some(179)),
                ] {
                    let key = AddressPortPair::new(String::new(), p1, String::new(), p2, p);
                    assert_eq!(get_service(&key, d), Service::Name("bgp"));
                }

                for (p1, p2) in [
                    (valid_but_not_well_known, Some(53)),
                    (Some(53), valid_but_not_well_known),
                    (Some(53), Some(53)),
                ] {
                    let key = AddressPortPair::new(String::new(), p1, String::new(), p2, p);
                    assert_eq!(get_service(&key, d), Service::Name("domain"));
                }

                for (p1, p2) in [
                    (valid_but_not_well_known, Some(1022)),
                    (Some(1022), valid_but_not_well_known),
                    (Some(1022), Some(1022)),
                ] {
                    let key = AddressPortPair::new(String::new(), p1, String::new(), p2, p);
                    assert_eq!(get_service(&key, d), Service::Name("exp2"));
                }
            }
        }
    }

    #[test]
    fn test_get_service_direction_bonus_matters() {
        let smtp = Some(25);
        let tacacs = Some(49);
        let netmagic = Some(1196);
        let tgp = Some(1223);

        for p in [Protocol::TCP, Protocol::UDP] {
            for d in [TrafficDirection::Incoming, TrafficDirection::Outgoing] {
                for (p1, p2) in [(smtp, tacacs), (tacacs, smtp)] {
                    let key = AddressPortPair::new(String::new(), p1, String::new(), p2, p);
                    assert_eq!(
                        get_service(&key, d),
                        Service::Name(match (p1, d) {
                            (source, TrafficDirection::Incoming) if source == tacacs => "tacacs",
                            (source, TrafficDirection::Outgoing) if source == tacacs => "smtp",
                            (source, TrafficDirection::Incoming) if source == smtp => "smtp",
                            (source, TrafficDirection::Outgoing) if source == smtp => "tacacs",
                            _ => panic!(),
                        })
                    );
                }

                for (p1, p2) in [(netmagic, tgp), (tgp, netmagic)] {
                    let key = AddressPortPair::new(String::new(), p1, String::new(), p2, p);
                    assert_eq!(
                        get_service(&key, d),
                        Service::Name(match (p1, d) {
                            (source, TrafficDirection::Incoming) if source == netmagic =>
                                "netmagic",
                            (source, TrafficDirection::Outgoing) if source == netmagic => "tgp",
                            (source, TrafficDirection::Incoming) if source == tgp => "tgp",
                            (source, TrafficDirection::Outgoing) if source == tgp => "netmagic",
                            _ => panic!(),
                        })
                    );
                }
            }
        }
    }

    #[test]
    fn test_get_service_different_tcp_udp() {
        for p in [Protocol::TCP, Protocol::UDP] {
            for d in [TrafficDirection::Incoming, TrafficDirection::Outgoing] {
                let key =
                    AddressPortPair::new(String::new(), Some(5353), String::new(), Some(5353), p);
                assert_eq!(
                    get_service(&key, d),
                    Service::Name(match p {
                        Protocol::TCP => "mdns",
                        Protocol::UDP => "zeroconf",
                        Protocol::ICMP => panic!(),
                    })
                );

                let key = AddressPortPair::new(String::new(), Some(15), String::new(), Some(15), p);
                assert_eq!(
                    get_service(&key, d),
                    match p {
                        Protocol::TCP => Service::Name("netstat"),
                        Protocol::UDP => Service::Unknown,
                        Protocol::ICMP => panic!(),
                    }
                );

                let key =
                    AddressPortPair::new(String::new(), Some(64738), String::new(), Some(64738), p);
                assert_eq!(
                    get_service(&key, d),
                    match p {
                        Protocol::TCP => Service::Unknown,
                        Protocol::UDP => Service::Name("murmur"),
                        Protocol::ICMP => panic!(),
                    }
                );

                for (p1, p2) in [(Some(5353), Some(53)), (Some(53), Some(5353))] {
                    let key = AddressPortPair::new(String::new(), p1, String::new(), p2, p);
                    assert_eq!(get_service(&key, d), Service::Name("domain"));
                }
            }
        }
    }

    #[test]
    fn test_get_service_not_applicable() {
        for p in Protocol::ALL {
            for d in [TrafficDirection::Incoming, TrafficDirection::Outgoing] {
                for (p1, p2) in [(None, Some(443)), (None, None), (Some(443), None)] {
                    let key = AddressPortPair::new(String::new(), p1, String::new(), p2, p);
                    assert_eq!(get_service(&key, d), Service::NotApplicable);
                }
            }
        }
    }

    #[test]
    fn test_get_service_unknown() {
        let unknown_port_1 = Some(39332);
        let unknown_port_2 = Some(23679);
        for p in [Protocol::TCP, Protocol::UDP] {
            assert!(SERVICES
                .get(&ServiceQuery(unknown_port_1.unwrap(), p))
                .is_none());
            assert!(SERVICES
                .get(&ServiceQuery(unknown_port_2.unwrap(), p))
                .is_none());
            for d in [TrafficDirection::Incoming, TrafficDirection::Outgoing] {
                for (p1, p2) in [
                    (unknown_port_1, unknown_port_2),
                    (unknown_port_2, unknown_port_1),
                    (unknown_port_1, unknown_port_1),
                    (unknown_port_2, unknown_port_2),
                ] {
                    let key = AddressPortPair::new(String::new(), p1, String::new(), p2, p);
                    assert_eq!(get_service(&key, d), Service::Unknown);
                }
            }
        }
    }

    #[test]
    fn test_all_services_map_key_and_values_are_valid() {
        assert_eq!(SERVICES.len(), 12078);
        let mut distinct_services = HashSet::new();
        for (sq, s) in &SERVICES {
            // only tcp or udp
            assert!(sq.1 == Protocol::TCP || sq.1 == Protocol::UDP);
            // no unknown or not applicable services
            let name = match *s {
                Service::Name(name) => name,
                _ => panic!(),
            };
            // name is valid...
            assert!(
                !["", "unknown", "-"].contains(&name)
                    && name.is_ascii()
                    && !name.starts_with('#')
                    && !name.contains(' ')
                    && !name.contains('?')
            );
            // just to count and verify number of distinct services
            distinct_services.insert(name.to_string());
        }
        assert_eq!(distinct_services.len(), 6450);
    }

    #[test]
    fn test_service_names_of_old_application_protocols() {
        for p in [Protocol::TCP, Protocol::UDP] {
            // FTP
            assert_eq!(
                SERVICES.get(&ServiceQuery(20, p)).unwrap(),
                &Service::Name("ftp-data")
            );
            assert_eq!(
                SERVICES.get(&ServiceQuery(21, p)).unwrap(),
                &Service::Name("ftp")
            );

            // SSH
            assert_eq!(
                SERVICES.get(&ServiceQuery(22, p)).unwrap(),
                &Service::Name("ssh")
            );

            // Telnet
            assert_eq!(
                SERVICES.get(&ServiceQuery(23, p)).unwrap(),
                &Service::Name("telnet")
            );

            // SMTP
            assert_eq!(
                SERVICES.get(&ServiceQuery(25, p)).unwrap(),
                &Service::Name("smtp")
            );

            // TACACS
            assert_eq!(
                SERVICES.get(&ServiceQuery(49, p)).unwrap(),
                &Service::Name("tacacs")
            );

            // DNS
            assert_eq!(
                SERVICES.get(&ServiceQuery(53, p)).unwrap(),
                &Service::Name("domain")
            );

            // DHCP
            assert_eq!(
                SERVICES.get(&ServiceQuery(67, p)).unwrap(),
                &Service::Name("dhcps")
            );
            assert_eq!(
                SERVICES.get(&ServiceQuery(68, p)).unwrap(),
                &Service::Name("dhcpc")
            );

            // TFTP
            assert_eq!(
                SERVICES.get(&ServiceQuery(69, p)).unwrap(),
                &Service::Name("tftp")
            );

            // HTTP
            assert_eq!(
                SERVICES.get(&ServiceQuery(80, p)).unwrap(),
                &Service::Name("http")
            );

            // POP
            assert_eq!(
                SERVICES.get(&ServiceQuery(109, p)).unwrap(),
                &Service::Name("pop2")
            );
            assert_eq!(
                SERVICES.get(&ServiceQuery(110, p)).unwrap(),
                &Service::Name("pop3")
            );

            // NTP
            assert_eq!(
                SERVICES.get(&ServiceQuery(123, p)).unwrap(),
                &Service::Name("ntp")
            );

            // NetBIOS
            assert_eq!(
                SERVICES.get(&ServiceQuery(137, p)).unwrap(),
                &Service::Name("netbios-ns")
            );
            assert_eq!(
                SERVICES.get(&ServiceQuery(138, p)).unwrap(),
                &Service::Name("netbios-dgm")
            );
            assert_eq!(
                SERVICES.get(&ServiceQuery(139, p)).unwrap(),
                &Service::Name("netbios-ssn")
            );

            // IMAP
            assert_eq!(
                SERVICES.get(&ServiceQuery(143, p)).unwrap(),
                &Service::Name("imap")
            );
            assert_eq!(
                SERVICES.get(&ServiceQuery(220, p)).unwrap(),
                &Service::Name("imap3")
            );

            // SNMP
            assert_eq!(
                SERVICES.get(&ServiceQuery(161, p)).unwrap(),
                &Service::Name("snmp")
            );
            assert_eq!(
                SERVICES.get(&ServiceQuery(162, p)).unwrap(),
                &Service::Name("snmptrap")
            );
            assert_eq!(
                SERVICES.get(&ServiceQuery(199, p)).unwrap(),
                &Service::Name("smux")
            );

            // BGP
            assert_eq!(
                SERVICES.get(&ServiceQuery(179, p)).unwrap(),
                &Service::Name("bgp")
            );

            // LDAP
            assert_eq!(
                SERVICES.get(&ServiceQuery(389, p)).unwrap(),
                &Service::Name("ldap")
            );

            // HTTPS
            assert_eq!(
                SERVICES.get(&ServiceQuery(443, p)).unwrap(),
                &Service::Name("https")
            );

            // FTPS
            assert_eq!(
                SERVICES.get(&ServiceQuery(989, p)).unwrap(),
                &Service::Name("ftps-data")
            );
            assert_eq!(
                SERVICES.get(&ServiceQuery(990, p)).unwrap(),
                &Service::Name("ftps")
            );

            // IMAPS
            assert_eq!(
                SERVICES.get(&ServiceQuery(993, p)).unwrap(),
                &Service::Name("imaps")
            );

            // POP3S
            assert_eq!(
                SERVICES.get(&ServiceQuery(995, p)).unwrap(),
                &Service::Name("pop3s")
            );

            // SSDP
            assert_eq!(
                SERVICES.get(&ServiceQuery(1900, p)).unwrap(),
                &Service::Name("upnp")
            );

            // XMPP
            assert_eq!(
                SERVICES.get(&ServiceQuery(5222, p)).unwrap(),
                &Service::Name("xmpp-client")
            );
        }

        // HTTP
        assert_eq!(
            SERVICES.get(&ServiceQuery(8080, Protocol::TCP)).unwrap(),
            &Service::Name("http-proxy")
        );
        assert_eq!(
            SERVICES.get(&ServiceQuery(8080, Protocol::UDP)).unwrap(),
            &Service::Name("http-alt")
        );

        // LDAPS
        assert_eq!(
            SERVICES.get(&ServiceQuery(636, Protocol::TCP)).unwrap(),
            &Service::Name("ldapssl")
        );
        assert_eq!(
            SERVICES.get(&ServiceQuery(636, Protocol::UDP)).unwrap(),
            &Service::Name("ldaps")
        );

        // mDNS
        assert_eq!(
            SERVICES.get(&ServiceQuery(5353, Protocol::TCP)).unwrap(),
            &Service::Name("mdns")
        );
        assert_eq!(
            SERVICES.get(&ServiceQuery(5353, Protocol::UDP)).unwrap(),
            &Service::Name("zeroconf")
        );
    }

    #[test]
    fn test_other_service_names() {
        for p in [Protocol::TCP, Protocol::UDP] {
            assert!(SERVICES.get(&ServiceQuery(4, p)).is_none());
            assert!(SERVICES.get(&ServiceQuery(6, p)).is_none());
            assert_eq!(
                SERVICES.get(&ServiceQuery(7, p)).unwrap(),
                &Service::Name("echo")
            );
            assert!(SERVICES.get(&ServiceQuery(5811, p)).is_none());
            assert_eq!(
                SERVICES.get(&ServiceQuery(6004, p)).unwrap(),
                &Service::Name("X11:4")
            );
            assert_eq!(
                SERVICES.get(&ServiceQuery(7777, p)).unwrap(),
                &Service::Name("cbt")
            );
            assert!(SERVICES.get(&ServiceQuery(65000, p)).is_none());
        }

        assert_eq!(
            SERVICES.get(&ServiceQuery(15, Protocol::TCP)).unwrap(),
            &Service::Name("netstat")
        );
        assert!(SERVICES.get(&ServiceQuery(15, Protocol::UDP)).is_none());

        assert_eq!(
            SERVICES.get(&ServiceQuery(26, Protocol::TCP)).unwrap(),
            &Service::Name("rsftp")
        );
        assert!(SERVICES.get(&ServiceQuery(26, Protocol::UDP)).is_none());

        assert_eq!(
            SERVICES.get(&ServiceQuery(87, Protocol::TCP)).unwrap(),
            &Service::Name("priv-term-l")
        );
        assert!(SERVICES.get(&ServiceQuery(87, Protocol::UDP)).is_none());

        assert_eq!(
            SERVICES.get(&ServiceQuery(106, Protocol::TCP)).unwrap(),
            &Service::Name("pop3pw")
        );
        assert_eq!(
            SERVICES.get(&ServiceQuery(106, Protocol::UDP)).unwrap(),
            &Service::Name("3com-tsmux")
        );

        assert!(SERVICES.get(&ServiceQuery(1028, Protocol::TCP)).is_none());
        assert_eq!(
            SERVICES.get(&ServiceQuery(1028, Protocol::UDP)).unwrap(),
            &Service::Name("ms-lsa")
        );

        assert_eq!(
            SERVICES.get(&ServiceQuery(1029, Protocol::TCP)).unwrap(),
            &Service::Name("ms-lsa")
        );
        assert_eq!(
            SERVICES.get(&ServiceQuery(1029, Protocol::UDP)).unwrap(),
            &Service::Name("solid-mux")
        );

        assert_eq!(
            SERVICES.get(&ServiceQuery(5820, Protocol::TCP)).unwrap(),
            &Service::Name("autopassdaemon")
        );
        assert!(SERVICES.get(&ServiceQuery(5820, Protocol::UDP)).is_none());

        assert_eq!(
            SERVICES.get(&ServiceQuery(5900, Protocol::TCP)).unwrap(),
            &Service::Name("vnc")
        );
        assert_eq!(
            SERVICES.get(&ServiceQuery(5900, Protocol::UDP)).unwrap(),
            &Service::Name("rfb")
        );

        assert_eq!(
            SERVICES.get(&ServiceQuery(5938, Protocol::TCP)).unwrap(),
            &Service::Name("teamviewer")
        );
        assert!(SERVICES.get(&ServiceQuery(5938, Protocol::UDP)).is_none());

        assert_eq!(
            SERVICES.get(&ServiceQuery(8888, Protocol::TCP)).unwrap(),
            &Service::Name("sun-answerbook")
        );
        assert_eq!(
            SERVICES.get(&ServiceQuery(8888, Protocol::UDP)).unwrap(),
            &Service::Name("ddi-udp-1")
        );

        assert_eq!(
            SERVICES.get(&ServiceQuery(23294, Protocol::TCP)).unwrap(),
            &Service::Name("5afe-dir")
        );
        assert!(SERVICES.get(&ServiceQuery(23294, Protocol::UDP)).is_none());

        assert!(SERVICES.get(&ServiceQuery(48899, Protocol::TCP)).is_none());
        assert_eq!(
            SERVICES.get(&ServiceQuery(48899, Protocol::UDP)).unwrap(),
            &Service::Name("tc_ads_discovery")
        );

        assert_eq!(
            SERVICES.get(&ServiceQuery(62078, Protocol::TCP)).unwrap(),
            &Service::Name("iphone-sync")
        );
        assert!(SERVICES.get(&ServiceQuery(62078, Protocol::UDP)).is_none());

        assert!(SERVICES.get(&ServiceQuery(64738, Protocol::TCP)).is_none());
        assert_eq!(
            SERVICES.get(&ServiceQuery(64738, Protocol::UDP)).unwrap(),
            &Service::Name("murmur")
        );
    }
}
