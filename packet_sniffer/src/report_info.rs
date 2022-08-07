use std::collections::HashSet;
use std::fmt;

pub struct ReportInfo {
    pub transmitted_bytes: u32,
    // pub received_bytes: u32 ???
    pub initial_timestamp: String,
    pub final_timestamp: String,
    pub trans_protocols: HashSet<TransProtocol>,
}

impl ReportInfo {

    pub fn new () -> Self {
        ReportInfo {
            transmitted_bytes: 0,
            initial_timestamp: "".to_string(),
            final_timestamp: "".to_string(),
            trans_protocols: HashSet::new(),
        }
    }

}

impl fmt::Display for ReportInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut multiple = "".to_string();
        let mut n = self.transmitted_bytes as f32;

        match transmitted_bytes {
            0 .. 1000 => {},
            1000 .. 1000000 => {n /= 1000; multiple.push('k'); },
            1000000 .. 1000000000 => {n /= 1000000; multiple.push('M');},
            _ => {n /= 1000000000; multiple.push('G'); }
        }
        write!(f, "Transmitted: {:.2} {}B\n\
                    Initial Timestamp: {}\n\
                    Final Timestamp: {}\n\
                    trans_protocols: {}\n", n, multiple, self.initial_timestamp, self.final_timestamp, self.trans_protocols)
    }
}

#[derive(Display, Copy)]
pub enum TransProtocol { Other, TCP, UDP, }