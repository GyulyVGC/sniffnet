//! Module defining the `DataInfo` struct, which represents incoming and outgoing packets and bytes.

/// Amount of exchanged data (packets and bytes) incoming and outgoing
#[derive(Clone, Default)]
pub struct DataInfo {
    /// Incoming packets
    pub incoming_packets: u128,
    /// Outgoing packets
    pub outgoing_packets: u128,
    /// Incoming bytes
    pub incoming_bytes: u128,
    /// Outgoing bytes
    pub outgoing_bytes: u128,
}

impl DataInfo {
    pub fn tot_packets(&self) -> u128 {
        self.incoming_packets + self.outgoing_packets
    }

    pub fn tot_bytes(&self) -> u128 {
        self.incoming_bytes + self.outgoing_bytes
    }
}
