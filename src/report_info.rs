//! Module defining the `ReportInfo` struct, useful to format the output report file and
//! to keep track of statistics about the sniffed traffic.

use std::collections::HashSet;
use std::fmt;
use thousands::Separable;

/// Struct useful to format the output report file and to keep track of statistics about the sniffed traffic.
///
/// Each ReportInfo struct is associated to a single address:port pair.
pub struct ReportInfo {
    /// Amount of bytes transmitted between the pair.
    pub transmitted_bytes: u128,
    /// Amount of packets transmitted between the pair.
    pub transmitted_packets: u128,
    /// First occurrence of information exchange featuring the associate address:port pair as a source or destination.
    pub initial_timestamp: String,
    /// Last occurrence of information exchange featuring the associate address:port pair as a source or destination.
    pub final_timestamp: String,
    /// Set of transport layer protocols carried through the associate address:port pair.
    pub trans_protocols: HashSet<TransProtocol>,
    /// Set of application layer protocols carried through the associate address:port pair.
    pub app_protocols: HashSet<AppProtocol>,
}

impl fmt::Display for ReportInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut multiple_transmitted = "".to_string();
        let mut n = self.transmitted_bytes as f32;
        let transport_level_protocols;
        let application_level_protocols;

        match self.transmitted_bytes {
            0..=999 => {},
            1000..=999999 => {n /= 1000 as f32; multiple_transmitted.push('k'); },
            1000000..=999999999 => {n /= 1000000 as f32; multiple_transmitted.push('M');},
            _ => {n /= 1000000000 as f32; multiple_transmitted.push('G'); }
        }

        transport_level_protocols = format!("{:?}", self.trans_protocols)
            .replace("{", "")
            .replace("}", "");

        application_level_protocols = match self.app_protocols.len() {
            0 => {
                "unable to identify any level 7 protocol".to_string()
            }
            _ => {
                format!("{:?}", self.app_protocols)
                    .replace("{", "")
                    .replace("}", "")
            }
        };

        let set_precision = |prefix: &String, &n| {
            if !prefix.is_empty() // no multiple
            {
                if n < 10.0 {2}
                else if n < 100.0 {1}
                else {0}
            } else {0}
        };

        let precision: usize = set_precision(&multiple_transmitted, &n);

        write!(f, " \t\tExchanged Bytes: {:.*} {}B\n\
                    \t\tExchanged packets: {}\n\
                    \t\tInitial Timestamp: {}\n\
                    \t\tFinal Timestamp: {}\n\
                    \t\tTransport layer protocols: {}\n\
                    \t\tApplication layer protocols: {}\n\n",
               precision, n, multiple_transmitted, self.transmitted_packets.separate_with_underscores(),
               self.initial_timestamp, self.final_timestamp,
               transport_level_protocols, application_level_protocols
        )
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