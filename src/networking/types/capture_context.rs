use pcap::{Active, Capture, Savefile};

use crate::networking::types::my_device::MyDevice;
use crate::networking::types::my_link_type::MyLinkType;

pub enum CaptureContext {
    Online(Online),
    OnlineWithSavefile(OnlineWithSavefile),
    Error(String),
}

impl CaptureContext {
    pub fn new(device: &MyDevice, pcap_path: &Option<String>) -> Self {
        let cap_res = Capture::from_device(device.to_pcap_device())
            .expect("Capture initialization error\n\r")
            .promisc(true)
            .snaplen(if pcap_path.is_some() {
                i32::from(u16::MAX)
            } else {
                256 //limit stored packets slice dimension (to keep more in the buffer)
            })
            .immediate_mode(true) //parse packets ASAP!
            .open();

        if let Err(e) = &cap_res {
            return Self::Error(e.to_string());
        }

        let cap = cap_res.unwrap();

        if let Some(path) = pcap_path {
            let savefile_res = cap.savefile(path);
            if let Err(e) = savefile_res {
                Self::Error(e.to_string())
            } else {
                Self::new_online_with_savefile(cap, savefile_res.unwrap())
            }
        } else {
            Self::new_online(cap)
        }
    }

    fn new_online(cap: Capture<Active>) -> Self {
        Self::Online(Online { cap })
    }

    fn new_online_with_savefile(cap: Capture<Active>, savefile: Savefile) -> Self {
        Self::OnlineWithSavefile(OnlineWithSavefile {
            online: Online { cap },
            savefile,
        })
    }

    pub fn error(&self) -> Option<&str> {
        match self {
            Self::Error(e) => Some(e),
            _ => None,
        }
    }

    pub fn consume(self) -> (Capture<Active>, Option<Savefile>) {
        match self {
            Self::Online(o) => (o.cap, None),
            Self::OnlineWithSavefile(ows) => (ows.online.cap, Some(ows.savefile)),
            Self::Error(_) => panic!(),
        }
    }

    pub fn my_link_type(&self) -> MyLinkType {
        match self {
            Self::Online(o) => MyLinkType::from_pcap_link_type(o.cap.get_datalink()),
            Self::OnlineWithSavefile(ows) => {
                MyLinkType::from_pcap_link_type(ows.online.cap.get_datalink())
            }
            Self::Error(_) => MyLinkType::default(),
        }
    }
}

pub struct Online {
    cap: Capture<Active>,
}

pub struct OnlineWithSavefile {
    online: Online,
    savefile: Savefile,
}
