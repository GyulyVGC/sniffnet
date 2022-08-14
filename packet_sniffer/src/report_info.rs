use std::collections::HashSet;
use std::fmt;

pub struct ReportInfo {
    pub transmitted_bytes: u32,
    pub transmitted_packets: u32,
    pub received_bytes: u32,
    pub received_packets: u32,
    pub initial_timestamp: String,
    pub final_timestamp: String,
    pub trans_protocols: HashSet<TransProtocol>,
    pub app_protocols: HashSet<AppProtocol>,
}

impl fmt::Display for ReportInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut multiple_transmitted = "".to_string();
        let mut multiple_received = "".to_string();
        let mut n = self.transmitted_bytes as f32;
        let mut m = self.received_bytes as f32;
        let transport_level_protocols;
        let application_level_protocols;

        match self.transmitted_bytes {
            0..=1000 => {},
            1001..=1000000 => {n /= 1000 as f32; multiple_transmitted.push('k'); },
            1000001..=1000000000 => {n /= 1000000 as f32; multiple_transmitted.push('M');},
            _ => {n /= 1000000000 as f32; multiple_transmitted.push('G'); }
        }

        match self.received_bytes {
            0..=1000 => {},
            1001..=1000000 => {m /= 1000 as f32; multiple_received.push('k'); },
            1000001..=1000000000 => {m /= 1000000 as f32; multiple_received.push('M');},
            _ => {m /= 1000000000 as f32; multiple_received.push('G'); }
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

        write!(f, "\tSent data\n\
                    \t\tSent Bytes: {:.2} {}B\n\
                    \t\tSent packets: {}\n\
                    \tReceived data\n\
                    \t\tReceived Bytes: {:.2} {}B\n\
                    \t\tReceived packets: {}\n\
                    \tTimestamps\n\
                    \t\tInitial Timestamp: {}\n\
                    \t\tFinal Timestamp: {}\n\
                    \tProtocols\n\
                    \t\tTransport layer protocols: {}\n\
                    \t\tApplication layer protocols: {}\n",
               n, multiple_transmitted, self.transmitted_packets,
               m, multiple_received, self.received_packets,
               self.initial_timestamp, self.final_timestamp,
               transport_level_protocols, application_level_protocols
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransProtocol { TCP, UDP, Other }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppProtocol { FTP, SSH, Telnet, SMTP, DNS, DHCP, TFTP, HTTP, POP,
                        NTP, NetBIOS, IMAP, SNMP, BGP, LDAP, HTTPS, LDAPS, FTPS }