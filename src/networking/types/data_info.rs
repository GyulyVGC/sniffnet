//! Module defining the `DataInfo` struct, which represents incoming and outgoing packets and bytes.

use std::ops::AddAssign;

use crate::networking::types::traffic_direction::TrafficDirection;

/// Amount of exchanged data (packets and bytes) incoming and outgoing
#[derive(Clone, Default, Copy)]
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

    pub fn add_packet(&mut self, bytes: u128, traffic_direction: TrafficDirection) {
        if traffic_direction.eq(&TrafficDirection::Outgoing) {
            self.outgoing_packets += 1;
            self.outgoing_bytes += bytes;
        } else {
            self.incoming_packets += 1;
            self.incoming_bytes += bytes;
        }
    }

    pub fn new_with_first_packet(bytes: u128, traffic_direction: TrafficDirection) -> Self {
        if traffic_direction.eq(&TrafficDirection::Outgoing) {
            Self {
                incoming_packets: 0,
                outgoing_packets: 1,
                incoming_bytes: 0,
                outgoing_bytes: bytes,
            }
        } else {
            Self {
                incoming_packets: 1,
                outgoing_packets: 0,
                incoming_bytes: bytes,
                outgoing_bytes: 0,
            }
        }
    }
}

impl AddAssign for DataInfo {
    fn add_assign(&mut self, rhs: Self) {
        self.incoming_packets += rhs.incoming_packets;
        self.outgoing_packets += rhs.outgoing_packets;
        self.incoming_bytes += rhs.incoming_bytes;
        self.outgoing_bytes += rhs.outgoing_bytes;
    }
}
