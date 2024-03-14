use crate::networking::types::my_link_type::MyLinkType;
use pcap::{Active, Capture, Savefile};

pub enum CaptureContext {
    Online(Online),
    OnlineWithSavefile(OnlineWithSavefile),
    Error(String),
}

impl CaptureContext {
    pub fn new_online(cap: Capture<Active>) -> CaptureContext {
        CaptureContext::Online(Online { cap })
    }

    pub fn new_online_with_savefile(cap: Capture<Active>, savefile: Savefile) -> CaptureContext {
        CaptureContext::OnlineWithSavefile(OnlineWithSavefile {
            online: Online { cap },
            savefile,
        })
    }

    pub fn error(&self) -> Option<String> {
        match self {
            CaptureContext::Error(e) => Some(e.clone()),
            _ => None,
        }
    }

    pub fn consume(self) -> (Capture<Active>, Option<Savefile>) {
        match self {
            CaptureContext::Online(o) => (o.cap, None),
            CaptureContext::OnlineWithSavefile(ows) => (ows.online.cap, Some(ows.savefile)),
            CaptureContext::Error(_) => panic!(),
        }
    }

    pub fn my_link_type(&self) -> Option<MyLinkType> {
        match self {
            CaptureContext::Online(o) => {
                Some(MyLinkType::from_pcap_link_type(o.cap.get_datalink()))
            }
            CaptureContext::OnlineWithSavefile(ows) => Some(MyLinkType::from_pcap_link_type(
                ows.online.cap.get_datalink(),
            )),
            CaptureContext::Error(_) => None,
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
