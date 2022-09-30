//! Module defining the `IndoAddressPortPair` struct, useful to format the output report file and
//! to keep track of statistics about the sniffed traffic.

use std::fmt;

/// Struct useful to format the output report file and to keep track of statistics about the sniffed traffic.
///
/// Each InfoAddressPortPair struct is associated to a single address:port pair.
pub struct InfoAddressPortPair {
    /// Amount of bytes transmitted between the pair.
    pub transmitted_bytes: u128,
    /// Amount of packets transmitted between the pair.
    pub transmitted_packets: u128,
    /// First occurrence of information exchange featuring the associate address:port pair as a source or destination.
    pub initial_timestamp: String,
    /// Last occurrence of information exchange featuring the associate address:port pair as a source or destination.
    pub final_timestamp: String,
    ///  Transport layer protocol carried through the associate address:port pair (TCP or UPD).
    pub trans_protocol: TransProtocol,
    /// Set of application layer protocols carried through the associate address:port pair.
    pub app_protocol: AppProtocol,
    /// Check if source or destination is an IPv6 address longer than 25 bytes (used for Display
    pub very_long_address: bool
}

impl fmt::Display for InfoAddressPortPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut multiple_transmitted = "".to_string();
        let mut n = self.transmitted_bytes as f32;

        match self.transmitted_bytes {
            0..=999 => {},
            1_000..=999_999 => {n /= 1000_f32; multiple_transmitted.push('k'); }, // kilo
            1_000_000..=999_999_999 => {n /= 1_000_000_f32; multiple_transmitted.push('M');}, // mega
            1_000_000_000..=999_999_999_999 => {n /= 1_000_000_000_f32; multiple_transmitted.push('G');}, // giga
            _ => {n /= 1_000_000_000_000_f32; multiple_transmitted.push('T');} // tera
        }

        let bytes_string = if !multiple_transmitted.is_empty() { // with multiple
                format!("{:.1} {}B", n, multiple_transmitted)
            } else { // no multiple
                format!("{}  B", n)
            };

        if self.very_long_address {
            write!(f, "   {}   |{:^9}|{:>10}  |{:>10}  | {} | {} |",
                   self.trans_protocol, self.app_protocol.to_string(),
                   self.transmitted_packets, bytes_string,
                   self.initial_timestamp, self.final_timestamp)
        }
        else {
            write!(f, "   {}   |{:^9}|{:>10}  |{:>10}  | {} | {} |{}",
                   self.trans_protocol, self.app_protocol.to_string(),
                   self.transmitted_packets, bytes_string,
                   self.initial_timestamp, self.final_timestamp, " ".repeat(40))
        }

    }
}


/// Enum representing the possible observed values of transport layer protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransProtocol {
    /// Transmission Control Protocol
    TCP,
    /// User Datagram Protocol
    UDP,
    /// Not identified
    Other
}


impl fmt::Display for TransProtocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


/// Enum representing the possible observed values of application layer protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppProtocol {
    /// File Transfer Protocol
    FTP,
    /// Secure Shell
    SSH,
    /// Telnet
    Telnet,
    /// Simple Mail Transfer Protocol
    SMTP,
    /// Terminal Access Controller Access-Control System
    TACACS,
    /// Domain Name System
    DNS,
    /// Dynamic Host Configuration Protocol
    DHCP,
    /// Trivial File Transfer Protocol
    TFTP,
    /// Hypertext Transfer Protocol
    HTTP,
    /// Post Office Protocol
    POP,
    /// Network Time Protocol
    NTP,
    /// NetBIOS
    NetBIOS,
    /// Post Office Protocol 3 over TLS/SSL
    POP3S,
    /// Internet Message Access Protocol
    IMAP,
    /// Simple Network Management Protocol
    SNMP,
    /// Border Gateway Protocol
    BGP,
    /// Lightweight Directory Access Protocol
    LDAP,
    ///Hypertext Transfer Protocol over TLS/SSL
    HTTPS,
    /// Lightweight Directory Access Protocol over TLS/SSL
    LDAPS,
    /// File Transfer Protocol over TLS/SSL
    FTPS,
    /// Multicast DNS
    #[allow(non_camel_case_types)]
    mDNS,
    ///Internet Message Access Protocol over TLS/SSL
    IMAPS,
    /// Simple Service Discovery Protocol
    SSDP,
    /// Extensible Messaging and Presence Protocol |
    XMPP,
    /// not identified
    Other
}


impl fmt::Display for AppProtocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}