use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::info_address_port_pair::InfoAddressPortPair;
use crate::networking::types::search_parameters::FilterInputType;
use crate::translations::translations::{
    address_translation, application_protocol_translation, bytes_translation, packets_translation,
    protocol_translation,
};
use crate::translations::translations_2::{
    country_translation, destination_translation, source_translation,
};
use crate::translations::translations_3::port_translation;
use crate::translations::types::language::Language;
use crate::utils::formatted_strings::get_formatted_bytes_string;

// total width: 1012.0

const ADDRESS_REPORT_WIDTH: f32 = 201.0;
const PORT_REPORT_WIDTH: f32 = 80.0;
const PROTO_REPORT_WIDTH: f32 = 90.0;
const APP_PROTO_REPORT_WIDTH: f32 = 90.0;
const PACKET_REPORT_WIDTH: f32 = 95.0;
const BYTE_REPORT_WIDTH: f32 = 95.0;
const COUNTRY_REPORT_WIDTH: f32 = 80.0;

// ----------------------------------------

const ADDRESS_REPORT_MAX_CHARS: u8 = 23;
const PORT_REPORT_MAX_CHARS: u8 = 8;
const PROTO_REPORT_MAX_CHARS: u8 = 9;
const APP_PROTO_REPORT_MAX_CHARS: u8 = 9;
const PACKET_REPORT_MAX_CHARS: u8 = 10;
const BYTE_REPORT_MAX_CHARS: u8 = 10;
const COUNTRY_REPORT_MAX_CHARS: u8 = 8;

#[derive(Eq, PartialEq)]
pub enum ReportCol {
    SrcIp,
    SrcPort,
    DstIp,
    DstPort,
    Proto,
    AppProto,
    Bytes,
    Packets,
    Country,
}

impl ReportCol {
    pub(crate) const ALL: [ReportCol; 9] = [
        ReportCol::Country,
        ReportCol::SrcIp,
        ReportCol::SrcPort,
        ReportCol::DstIp,
        ReportCol::DstPort,
        ReportCol::Proto,
        ReportCol::AppProto,
        ReportCol::Bytes,
        ReportCol::Packets,
    ];

    pub(crate) fn get_title(&self, language: Language) -> String {
        match self {
            ReportCol::SrcIp => format!(
                "{} ({})",
                address_translation(language),
                source_translation(language).to_ascii_lowercase()
            ),
            ReportCol::SrcPort => format!(
                "{} ({})",
                port_translation(language),
                source_translation(language).to_ascii_lowercase()
            ),
            ReportCol::DstIp => format!(
                "{} ({})",
                address_translation(language),
                destination_translation(language).to_ascii_lowercase()
            ),
            ReportCol::DstPort => format!(
                "{} ({})",
                port_translation(language),
                destination_translation(language).to_ascii_lowercase()
            ),
            ReportCol::Proto => protocol_translation(language).to_string(),
            ReportCol::AppProto => application_protocol_translation(language).to_string(),
            ReportCol::Bytes => {
                let mut str = bytes_translation(language).to_string();
                str.remove(0).to_ascii_uppercase().to_string() + &str
            }
            ReportCol::Packets => {
                let mut str = packets_translation(language).to_string();
                str.remove(0).to_ascii_uppercase().to_string() + &str
            }
            ReportCol::Country => country_translation(language).to_string(),
        }
    }

    pub(crate) fn get_value(&self, key: &AddressPortPair, val: &InfoAddressPortPair) -> String {
        match self {
            ReportCol::SrcIp => key.address1.clone(),
            ReportCol::SrcPort => {
                if let Some(port) = key.port1 {
                    port.to_string()
                } else {
                    "-".to_string()
                }
            }
            ReportCol::DstIp => key.address2.clone(),
            ReportCol::DstPort => {
                if let Some(port) = key.port2 {
                    port.to_string()
                } else {
                    "-".to_string()
                }
            }
            ReportCol::Proto => key.protocol.to_string(),
            ReportCol::AppProto => val.app_protocol.to_string(),
            ReportCol::Bytes => get_formatted_bytes_string(val.transmitted_bytes, 1),
            ReportCol::Packets => val.transmitted_packets.to_string(),
            ReportCol::Country => String::new(),
        }
    }

    pub(crate) fn get_width(&self) -> f32 {
        match self {
            ReportCol::SrcIp | ReportCol::DstIp => ADDRESS_REPORT_WIDTH,
            ReportCol::SrcPort | ReportCol::DstPort => PORT_REPORT_WIDTH,
            ReportCol::Proto => PROTO_REPORT_WIDTH,
            ReportCol::AppProto => APP_PROTO_REPORT_WIDTH,
            ReportCol::Bytes => BYTE_REPORT_WIDTH,
            ReportCol::Packets => PACKET_REPORT_WIDTH,
            ReportCol::Country => COUNTRY_REPORT_WIDTH,
        }
    }

    pub(crate) fn get_max_chars(&self) -> u8 {
        match self {
            ReportCol::SrcIp | ReportCol::DstIp => ADDRESS_REPORT_MAX_CHARS,
            ReportCol::SrcPort | ReportCol::DstPort => PORT_REPORT_MAX_CHARS,
            ReportCol::Proto => PROTO_REPORT_MAX_CHARS,
            ReportCol::AppProto => APP_PROTO_REPORT_MAX_CHARS,
            ReportCol::Bytes => BYTE_REPORT_MAX_CHARS,
            ReportCol::Packets => PACKET_REPORT_MAX_CHARS,
            ReportCol::Country => COUNTRY_REPORT_MAX_CHARS,
        }
    }

    pub(crate) fn get_filter_input_type(&self) -> FilterInputType {
        match self {
            ReportCol::SrcIp => FilterInputType::AddressSrc,
            ReportCol::DstIp => FilterInputType::AddressDst,
            ReportCol::SrcPort => FilterInputType::PortSrc,
            ReportCol::DstPort => FilterInputType::PortDst,
            ReportCol::Proto => FilterInputType::Proto,
            ReportCol::AppProto => FilterInputType::AppProto,
            ReportCol::Country => FilterInputType::Country,
            ReportCol::Bytes | ReportCol::Packets => panic!(),
        }
    }
}
