use crate::gui::types::conf::deserialize_or_default;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

/// IANA-registered default IPFIX collector port.
pub const DEFAULT_IPFIX_PORT: u16 = 4739;
pub const DEFAULT_IPFIX_ADDR: IpAddr = IpAddr::V4(Ipv4Addr::UNSPECIFIED);

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(default)]
pub struct MyIpfixSocket {
    #[serde(deserialize_with = "deserialize_or_default")]
    addr: String,
    #[serde(deserialize_with = "deserialize_or_default")]
    port: String,
}

impl MyIpfixSocket {
    pub fn addr(&self) -> &str {
        &self.addr
    }

    pub fn port(&self) -> &str {
        &self.port
    }

    pub fn set_addr(&mut self, addr: String) {
        self.addr = addr;
    }

    pub fn set_port(&mut self, port: String) {
        self.port = port;
    }

    pub fn unspecified_addr(&self) -> Option<IpAddr> {
        if self.addr.is_empty() {
            return Some(DEFAULT_IPFIX_ADDR);
        }
        self.addr
            .parse::<IpAddr>()
            .ok()
            .filter(IpAddr::is_unspecified)
    }

    pub fn display_name(&self) -> String {
        let addr = if self.addr.is_empty() {
            &DEFAULT_IPFIX_ADDR.to_string()
        } else {
            &self.addr
        };
        let port = if self.port.is_empty() {
            &DEFAULT_IPFIX_PORT.to_string()
        } else {
            &self.port
        };

        if self.unspecified_addr().is_some() {
            format!("*:{port}")
        } else if addr.parse::<Ipv6Addr>().is_ok() {
            format!("[{addr}]:{port}")
        } else {
            format!("{addr}:{port}")
        }
    }

    pub fn socket_addr(&self) -> Result<SocketAddr, String> {
        let port = if self.port.is_empty() {
            DEFAULT_IPFIX_PORT
        } else {
            self.port
                .parse::<u16>()
                .map_err(|_| format!("Invalid port number: {}", self.port))?
        };
        let addr = if self.addr.is_empty() {
            DEFAULT_IPFIX_ADDR
        } else {
            self.addr
                .parse::<IpAddr>()
                .map_err(|_| format!("Invalid IP address: {}", self.addr))?
        };
        Ok(SocketAddr::new(addr, port))
    }
}

impl Default for MyIpfixSocket {
    fn default() -> Self {
        Self {
            addr: DEFAULT_IPFIX_ADDR.to_string(),
            port: DEFAULT_IPFIX_PORT.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_socket_addr() {
        let mut socket = MyIpfixSocket::default();
        assert_eq!(
            socket.socket_addr().unwrap(),
            SocketAddr::new(DEFAULT_IPFIX_ADDR, DEFAULT_IPFIX_PORT)
        );

        socket.set_addr("10.0.0.1".to_string());
        socket.set_port("1234".to_string());
        assert_eq!(
            socket.socket_addr().unwrap(),
            SocketAddr::new("10.0.0.1".parse().unwrap(), 1234)
        );

        socket.set_addr("::1".to_string());
        socket.set_port("5678".to_string());
        assert_eq!(
            socket.socket_addr().unwrap(),
            SocketAddr::new("::1".parse().unwrap(), 5678)
        );

        // invalid IP address
        socket.set_addr("invalid".to_string());
        assert_eq!(
            socket.socket_addr().unwrap_err(),
            "Invalid IP address: invalid"
        );

        // invalid port number
        socket.set_addr("10.0.0.1".to_string());
        socket.set_port("invalid".to_string());
        assert_eq!(
            socket.socket_addr().unwrap_err(),
            "Invalid port number: invalid"
        );

        // empty
        socket.set_addr("".to_string());
        socket.set_port("".to_string());
        assert_eq!(
            socket.socket_addr().unwrap(),
            SocketAddr::new(DEFAULT_IPFIX_ADDR, DEFAULT_IPFIX_PORT)
        );
    }

    #[test]
    fn test_display_name() {
        let mut socket = MyIpfixSocket::default();
        assert_eq!(socket.display_name(), format!("*:{DEFAULT_IPFIX_PORT}"));

        socket.set_addr("10.0.0.1".to_string());
        socket.set_port("1234".to_string());
        assert_eq!(socket.display_name(), "10.0.0.1:1234");

        socket.set_addr("::1".to_string());
        socket.set_port("5678".to_string());
        assert_eq!(socket.display_name(), "[::1]:5678");

        socket.set_addr("::".to_string());
        assert_eq!(socket.display_name(), "*:5678");

        // invalid is also accepted by display_name
        socket.set_addr("invalid".to_string());
        socket.set_port("invalid".to_string());
        assert_eq!(socket.display_name(), "invalid:invalid");

        // empty
        socket.set_addr("".to_string());
        socket.set_port("".to_string());
        assert_eq!(socket.display_name(), format!("*:{DEFAULT_IPFIX_PORT}"));
    }
}
