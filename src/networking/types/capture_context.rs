use pcap::{Active, Address, Capture, Error, Packet, Savefile, Stat};

use crate::networking::types::my_device::MyDevice;
use crate::networking::types::my_link_type::MyLinkType;
use crate::translations::translations::network_adapter_translation;
use crate::translations::translations_3::file_name_translation;
use crate::translations::types::language::Language;

pub enum CaptureContext {
    Live(Live),
    LiveWithSavefile(LiveWithSavefile),
    Offline(Offline),
    Error(String),
}

impl CaptureContext {
    pub fn new(source: &CaptureSource, pcap_out_path: Option<&String>) -> Self {
        let cap_type = match CaptureType::from_source(source, pcap_out_path) {
            Ok(c) => c,
            Err(e) => return Self::Error(e.to_string()),
        };
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

    pub fn consume(self) -> (CaptureType, Option<Savefile>) {
        match self {
            Self::Live(on) => (CaptureType::Live(on.cap), None),
            Self::LiveWithSavefile(onws) => (CaptureType::Live(onws.live.cap), Some(onws.savefile)),
            Self::Offline(off) => (CaptureType::Offline(off.cap), None),
            Self::Error(_) => panic!(),
        }
    }

    pub fn my_link_type(&self) -> MyLinkType {
        match self {
            Self::Live(on) => MyLinkType::from_pcap_link_type(on.cap.get_datalink()),
            Self::LiveWithSavefile(onws) => {
                MyLinkType::from_pcap_link_type(onws.live.cap.get_datalink())
            }
            Self::Offline(off) => MyLinkType::from_pcap_link_type(off.cap.get_datalink()),
            Self::Error(_) => MyLinkType::default(),
        }
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
    pub fn next_packet(&mut self) -> Result<Packet, Error> {
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

    fn from_source(source: &CaptureSource, pcap_out_path: Option<&String>) -> Result<Self, Error> {
        match source {
            CaptureSource::Device(device) => {
                let inactive = Capture::from_device(device.to_pcap_device())?;
                let cap = inactive
                    .promisc(true)
                    .buffer_size(2_000_000) // 2MB buffer
                    .snaplen(if pcap_out_path.is_some() {
                        i32::from(u16::MAX)
                    } else {
                        200 // limit stored packets slice dimension (to keep more in the buffer)
                    })
                    .immediate_mode(true) // parse packets ASAP
                    .timeout(150) // ensure UI is updated even if no packets are captured
                    .open()?;
                Ok(Self::Live(cap))
            }
            CaptureSource::File(file) => Ok(Self::Offline(Capture::from_file(&file.path)?)),
        }
    }
}

#[derive(Clone)]
pub enum CaptureSource {
    Device(MyDevice),
    File(MyPcapImport),
}

impl CaptureSource {
    pub fn title(&self, language: Language) -> &str {
        match self {
            Self::Device(_) => network_adapter_translation(language),
            Self::File(_) => file_name_translation(language),
        }
    }

    pub fn get_addresses(&self) -> &Vec<Address> {
        match self {
            Self::Device(device) => device.get_addresses(),
            Self::File(file) => &file.addresses,
        }
    }

    pub fn set_addresses(&mut self, addresses: Vec<Address>) {
        if let Self::Device(device) = self {
            device.set_addresses(addresses);
        }
    }

    pub fn get_link_type(&self) -> MyLinkType {
        match self {
            Self::Device(device) => device.get_link_type(),
            Self::File(file) => file.link_type,
        }
    }

    pub fn set_link_type(&mut self, link_type: MyLinkType) {
        match self {
            Self::Device(device) => device.set_link_type(link_type),
            Self::File(file) => file.link_type = link_type,
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            Self::Device(device) => device.get_name().clone(),
            Self::File(file) => file.path.clone(),
        }
    }

    #[cfg(target_os = "windows")]
    pub fn get_desc(&self) -> Option<String> {
        match self {
            Self::Device(device) => device.get_desc().cloned(),
            Self::File(_) => None,
        }
    }
}

#[derive(Clone)]
pub struct MyPcapImport {
    path: String,
    link_type: MyLinkType,
    addresses: Vec<Address>, // this is always empty!
}

impl MyPcapImport {
    pub fn new(path: String) -> Self {
        Self {
            path,
            link_type: MyLinkType::default(),
            addresses: vec![],
        }
    }
}
