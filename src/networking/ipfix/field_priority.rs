use crate::networking::ipfix::ie;

/// Rank of the IE that supplied the value currently held in each record slot.
/// Used for slots that can be filled by multiple IEs, to ensure that the most preferred one is kept.
#[derive(Default)]
pub(super) struct FieldPriority {
    pub(super) bytes_delta: u8,
    pub(super) packets_delta: u8,
    pub(super) bytes_total: u8,
    pub(super) packets_total: u8,
    pub(super) flow_start: u8,
    pub(super) flow_end: u8,
    pub(super) src_mac: u8,
    pub(super) dst_mac: u8,
}

pub(super) fn bytes_delta_rank(ie_id: u16) -> u8 {
    match ie_id {
        ie::LAYER2_OCTET_DELTA_COUNT => 4,
        ie::POST_LAYER2_OCTET_DELTA_COUNT => 3,
        ie::OCTET_DELTA_COUNT => 2,
        ie::POST_OCTET_DELTA_COUNT => 1,
        _ => 0,
    }
}

pub(super) fn packets_delta_rank(ie_id: u16) -> u8 {
    match ie_id {
        ie::PACKET_DELTA_COUNT => 2,
        ie::POST_PACKET_DELTA_COUNT => 1,
        _ => 0,
    }
}

pub(super) fn bytes_total_rank(ie_id: u16) -> u8 {
    match ie_id {
        ie::LAYER2_OCTET_TOTAL_COUNT => 4,
        ie::POST_LAYER2_OCTET_TOTAL_COUNT => 3,
        ie::OCTET_TOTAL_COUNT => 2,
        ie::POST_OCTET_TOTAL_COUNT => 1,
        _ => 0,
    }
}

pub(super) fn packets_total_rank(ie_id: u16) -> u8 {
    match ie_id {
        ie::PACKET_TOTAL_COUNT => 2,
        ie::POST_PACKET_TOTAL_COUNT => 1,
        _ => 0,
    }
}

pub(super) fn timestamp_rank(ie_id: u16) -> u8 {
    match ie_id {
        ie::FLOW_START_NANOSECONDS | ie::FLOW_END_NANOSECONDS => 4,
        ie::FLOW_START_MICROSECONDS | ie::FLOW_END_MICROSECONDS => 3,
        ie::FLOW_START_MILLISECONDS | ie::FLOW_END_MILLISECONDS => 2,
        ie::FLOW_START_SECONDS | ie::FLOW_END_SECONDS => 1,
        _ => 0,
    }
}

pub(super) fn mac_rank(ie_id: u16) -> u8 {
    match ie_id {
        ie::SOURCE_MAC_ADDRESS | ie::DESTINATION_MAC_ADDRESS => 2,
        ie::POST_SOURCE_MAC_ADDRESS | ie::POST_DESTINATION_MAC_ADDRESS => 1,
        _ => 0,
    }
}
