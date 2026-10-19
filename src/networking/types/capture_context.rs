use crate::gui::types::conf::Conf;
use crate::gui::types::filters::Filters;
use crate::gui::types::ipfix_socket::MyIpfixSocket;
use crate::location;
use crate::networking::types::my_device::MyDevice;
use crate::translations::translations::network_adapter_translation;
use crate::translations::translations_4::capture_file_translation;
use crate::translations::translations_6::ipfix_collector_translation;
use crate::translations::types::language::Language;
use crate::utils::error_logger::{ErrorLogger, Location};
use pcap::{Active, Address, Capture, Device, Error, Packet, Savefile, Stat};
use serde::{Deserialize, Serialize};
use sniffnet_packet_parser::LinkType;
use std::collections::HashSet;
use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::time::{Duration, Instant};

pub enum CaptureContext {
    Live(Live),
    LiveWithSavefile(LiveWithSavefile),
    Offline(Offline),
    Ipfix(UdpSocket),
    Error(String),
}

/// A problem the capture backend has encountered
pub enum CaptureError {
    /// The capture failed to start (fatal)
    Fatal(String),
    /// The IPFIX collector is running but what reaches it isn't decodable at the moment (warning)
    IpfixUndecodable,
}

impl CaptureContext {
    pub fn new(source: &CaptureSource, pcap_out_path: Option<&String>, filters: &Filters) -> Self {
        let mut cap_type = match source {
            CaptureSource::Device(device) => {
                match CaptureType::from_device(device, pcap_out_path) {
                    Ok(c) => c,
                    Err(e) => return Self::Error(e),
                }
            }
            CaptureSource::File(file) => match CaptureType::from_file(file) {
                Ok(c) => c,
                Err(e) => return Self::Error(e),
            },
            CaptureSource::Ipfix(ipfix) => {
                return Self::new_ipfix(&ipfix.socket).unwrap_or_else(Self::Error);
            }
        };

        // only apply BPF filter if it is active, and return an error if it fails to apply
        if filters.is_some_filter_active()
            && let Err(e) = cap_type.set_bpf(filters.bpf())
        {
            return Self::Error(e.to_string());
        }

        let cap = match cap_type {
            CaptureType::Live(cap) => cap,
            CaptureType::Offline(cap) => return Self::new_offline(cap),
        };

        if let Some(out_path) = pcap_out_path {
            let savefile_res = cap.savefile(out_path);
            match savefile_res {
                Ok(s) => Self::new_live_with_savefile(cap, s),
                Err(e) => Self::Error(e.to_string()),
            }
        } else {
            Self::new_live(cap)
        }
    }

    fn new_live(cap: Capture<Active>) -> Self {
        Self::Live(Live { cap })
    }

    fn new_live_with_savefile(cap: Capture<Active>, savefile: Savefile) -> Self {
        Self::LiveWithSavefile(LiveWithSavefile {
            live: Live { cap },
            savefile,
        })
    }

    fn new_offline(cap: Capture<pcap::Offline>) -> Self {
        Self::Offline(Offline { cap })
    }

    pub fn error(&self) -> Option<&str> {
        match self {
            Self::Error(e) => Some(e),
            _ => None,
        }
    }

    pub fn consume(self) -> (Option<CaptureType>, Option<Savefile>) {
        match self {
            Self::Live(on) => (Some(CaptureType::Live(on.cap)), None),
            Self::LiveWithSavefile(onws) => {
                (Some(CaptureType::Live(onws.live.cap)), Some(onws.savefile))
            }
            Self::Offline(off) => (Some(CaptureType::Offline(off.cap)), None),
            Self::Ipfix(_) | Self::Error(_) => (None, None),
        }
    }

    pub fn link_type(&self) -> Option<LinkType> {
        match self {
            Self::Live(on) => Some(LinkType::from_pcap(on.cap.get_datalink())),
            Self::LiveWithSavefile(onws) => Some(LinkType::from_pcap(onws.live.cap.get_datalink())),
            Self::Offline(off) => Some(LinkType::from_pcap(off.cap.get_datalink())),
            Self::Ipfix(_) | Self::Error(_) => None,
        }
    }

    fn new_ipfix(ipfix_socket: &MyIpfixSocket) -> Result<Self, String> {
        const IPFIX_READ_TIMEOUT: Duration = Duration::from_millis(150);
        const RETRY_BIND_DELAY: Duration = Duration::from_millis(20);
        let socket_addr = ipfix_socket.socket_addr()?;

        // restarting a capture on the same port races the just finished collector
        // retry across its read window rather than failing on the socket that is about to go away
        let give_up_at = Instant::now() + IPFIX_READ_TIMEOUT * 3;
        let bind_socket = |socket_addr: SocketAddr| loop {
            match UdpSocket::bind(socket_addr) {
                Err(e)
                    if e.kind() == std::io::ErrorKind::AddrInUse && Instant::now() < give_up_at =>
                {
                    std::thread::sleep(RETRY_BIND_DELAY);
                }
                result => return result,
            }
        };

        let socket = bind_socket(socket_addr).map_err(|e| e.to_string())?;
        socket
            .set_read_timeout(Some(IPFIX_READ_TIMEOUT))
            .map_err(|e| e.to_string())?;

        Ok(Self::Ipfix(socket))
    }
}

pub struct Live {
    cap: Capture<Active>,
}

pub struct LiveWithSavefile {
    live: Live,
    savefile: Savefile,
}

pub struct Offline {
    cap: Capture<pcap::Offline>,
}

pub enum CaptureType {
    Live(Capture<Active>),
    Offline(Capture<pcap::Offline>),
}

impl CaptureType {
    // TODO: consider calling PCAP's dispatch() instead of next_packet() (needs benchmarking first)
    pub fn next_packet(&mut self) -> Result<Packet<'_>, Error> {
        match self {
            Self::Live(on) => on.next_packet(),
            Self::Offline(off) => off.next_packet(),
        }
    }

    pub fn stats(&mut self) -> Result<Stat, Error> {
        match self {
            Self::Live(on) => on.stats(),
            Self::Offline(off) => off.stats(),
        }
    }

    fn from_device(device: &MyDevice, pcap_out_path: Option<&String>) -> Result<Self, String> {
        let inactive = Capture::from_device(device.to_pcap_device()).map_err(|e| e.to_string())?;
        let cap = inactive
            .promisc(false)
            .buffer_size(2_000_000) // 2MB buffer -> 10k packets of 200 bytes
            .snaplen(if pcap_out_path.is_some() {
                i32::from(u16::MAX)
            } else {
                200 // limit stored packets slice dimension (to keep more in the buffer)
            })
            .immediate_mode(false)
            .timeout(150) // ensure UI is updated even if no packets are captured
            .open()
            .map_err(|e| e.to_string())?;
        Ok(Self::Live(cap))
    }

    fn from_file(file: &MyPcapImport) -> Result<Self, String> {
        Ok(Self::Offline(
            Capture::from_file(&file.path).map_err(|e| e.to_string())?,
        ))
    }

    fn set_bpf(&mut self, bpf: &str) -> Result<(), Error> {
        match self {
            Self::Live(cap) => cap.filter(bpf, true),
            Self::Offline(cap) => cap.filter(bpf, true),
        }
    }

    pub fn pause(&mut self) {
        if let Self::Live(cap) = self {
            let _ = cap.filter("less 2", true).log_err(location!());
        }
    }

    pub fn resume(&mut self, filters: &Filters) {
        if let Self::Live(cap) = self {
            if filters.is_some_filter_active() {
                let _ = cap.filter(filters.bpf(), true).log_err(location!());
            } else if cap.filter("", true).log_err(location!()).is_err() {
                let _ = cap.filter("greater 0", true).log_err(location!());
            }
        }
    }
}

#[derive(Clone)]
pub enum CaptureSource {
    Device(MyDevice),
    File(MyPcapImport),
    Ipfix(MyIpfixCollector),
}

impl CaptureSource {
    pub fn from_conf(conf: &Conf) -> Self {
        match conf.capture_source_picklist {
            CaptureSourcePicklist::Device => {
                let device = conf.device.to_my_device();
                Self::Device(device)
            }
            CaptureSourcePicklist::File => {
                let path = conf.import_pcap_path.clone();
                Self::File(MyPcapImport::new(path))
            }
            CaptureSourcePicklist::Ipfix => {
                let socket = conf.ipfix_socket.clone();
                Self::Ipfix(MyIpfixCollector::new(socket))
            }
        }
    }

    pub fn title(&self, language: Language) -> &str {
        match self {
            Self::Device(_) => network_adapter_translation(language),
            Self::File(_) => capture_file_translation(language),
            Self::Ipfix(_) => ipfix_collector_translation(language),
        }
    }

    /// Addresses the collector listens on when bound to the unspecified address
    /// (only used for display purposes)
    pub fn get_ipfix_unspecified_bound_addresses(&self) -> &[Address] {
        match self {
            Self::Ipfix(collector) => &collector.unspecified_bound_addresses,
            Self::Device(_) | Self::File(_) => &[],
        }
    }

    pub fn get_addresses(&self) -> &[Address] {
        match self {
            Self::Device(device) => device.get_addresses(),
            Self::File(_) | Self::Ipfix(_) => &[],
        }
    }

    pub fn set_addresses(&mut self) {
        match self {
            Self::Device(my_device) => {
                let mut addresses = Vec::new();
                for dev in Device::list().log_err(location!()).unwrap_or_default() {
                    if matches!(
                        my_device.get_link_type(),
                        Some(LinkType::LinuxSll(_) | LinkType::LinuxSll2(_))
                    ) {
                        addresses.extend(dev.addresses);
                    } else if dev.name.eq(my_device.get_name()) {
                        addresses.extend(dev.addresses);
                        break;
                    }
                }
                my_device.set_addresses(addresses);
            }
            Self::Ipfix(collector) => collector.set_unspecified_bound_addresses(),
            Self::File(_) => {}
        }
    }

    pub fn get_link_type(&self) -> Option<LinkType> {
        match self {
            Self::Device(device) => device.get_link_type(),
            Self::File(file) => file.link_type,
            Self::Ipfix(_) => None,
        }
    }

    pub fn set_link_type(&mut self, link_type: Option<LinkType>) {
        match self {
            Self::Device(device) => device.set_link_type(link_type),
            Self::File(file) => file.link_type = link_type,
            Self::Ipfix(_) => {}
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            Self::Device(device) => device.get_name().clone(),
            Self::File(file) => file.path.clone(),
            Self::Ipfix(collector) => collector.socket.display_name(),
        }
    }

    #[cfg(target_os = "windows")]
    pub fn get_desc(&self) -> Option<String> {
        match self {
            Self::Device(device) => device.get_desc().cloned(),
            Self::File(_) | Self::Ipfix(_) => None,
        }
    }

    pub fn supports_link_type(&self) -> bool {
        match self {
            Self::Device(_) | Self::File(_) => true,
            Self::Ipfix(_) => false,
        }
    }

    pub fn supports_filters(&self) -> bool {
        match self {
            Self::Device(_) | Self::File(_) => true,
            Self::Ipfix(_) => false,
        }
    }

    pub fn supports_export_pcap(&self) -> bool {
        match self {
            Self::Device(_) => true,
            Self::Ipfix(_) | Self::File(_) => false,
        }
    }

    pub fn supports_latency(&self) -> bool {
        match self {
            Self::Device(_) => true,
            Self::Ipfix(_) | Self::File(_) => false,
        }
    }

    pub fn supports_live_chart(&self) -> bool {
        match self {
            Self::Device(_) | Self::Ipfix(_) => true,
            Self::File(_) => false,
        }
    }

    pub fn supports_exporters(&self) -> bool {
        match self {
            Self::Ipfix(_) => true,
            Self::Device(_) | Self::File(_) => false,
        }
    }

    pub fn supports_programs(&self) -> bool {
        match self {
            Self::Device(_) => true,
            Self::File(_) | Self::Ipfix(_) => false,
        }
    }

    pub fn supports_notification_sound(&self) -> bool {
        match self {
            Self::Device(_) | Self::Ipfix(_) => true,
            Self::File(_) => false,
        }
    }
}

#[derive(Clone)]
pub struct MyIpfixCollector {
    socket: MyIpfixSocket,
    /// Addresses the collector listens on when bound to the unspecified address
    /// (only used for display purposes)
    unspecified_bound_addresses: Vec<Address>,
}

impl MyIpfixCollector {
    pub fn new(socket: MyIpfixSocket) -> Self {
        Self {
            socket,
            unspecified_bound_addresses: Vec::new(),
        }
    }

    fn set_unspecified_bound_addresses(&mut self) {
        let Some(bound_to) = self.socket.unspecified_addr() else {
            self.unspecified_bound_addresses.clear();
            return;
        };

        let mut seen = HashSet::new();
        self.unspecified_bound_addresses = Device::list()
            .log_err(location!())
            .unwrap_or_default()
            .into_iter()
            .flat_map(|dev| dev.addresses)
            // a socket bound to an unspecified IP only receives datagrams of the same family
            .filter(|addr| addr.addr.is_ipv4() == bound_to.is_ipv4())
            // exporters can't reach loopback, multicast, broadcast, and link-local addresses
            .filter(|addr| match addr.addr {
                IpAddr::V4(v4) => {
                    !v4.is_loopback()
                        && !v4.is_link_local()
                        && !v4.is_unspecified()
                        && !v4.is_broadcast()
                        && !v4.is_multicast()
                }
                IpAddr::V6(v6) => {
                    !v6.is_loopback()
                        && !v6.is_unicast_link_local()
                        && !v6.is_unspecified()
                        && !v6.is_multicast()
                }
            })
            // remove duplicates
            .filter(|addr| seen.insert(addr.addr))
            .collect();
    }

    /// Update the collector's bind address and port to the actual ones used by the socket
    /// (particularly relevant for port = 0, which lets the OS pick a free port)
    pub(crate) fn set_actually_bind_addr(&mut self, local_addr: SocketAddr) {
        self.socket.set_addr(local_addr.ip().to_string());
        self.socket.set_port(local_addr.port().to_string());
    }
}

#[derive(Clone)]
pub struct MyPcapImport {
    path: String,
    link_type: Option<LinkType>,
}

impl MyPcapImport {
    pub fn new(path: String) -> Self {
        Self {
            path,
            link_type: None,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Copy, Default, Serialize, Deserialize)]
pub enum CaptureSourcePicklist {
    #[default]
    Device,
    File,
    Ipfix,
}

impl CaptureSourcePicklist {
    pub fn supports_filters(self) -> bool {
        match self {
            Self::Device | Self::File => true,
            Self::Ipfix => false,
        }
    }

    pub fn supports_export_pcap(self) -> bool {
        match self {
            Self::Device => true,
            Self::Ipfix | Self::File => false,
        }
    }
}
