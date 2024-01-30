use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::info_address_port_pair::InfoAddressPortPair;
use crate::networking::types::search_parameters::FilterInputType;
use crate::translations::translations::{
    address_translation, application_protocol_translation, bytes_translation, packets_translation,
    protocol_translation,
};
use crate::translations::translations_2::{destination_translation, source_translation};
use crate::translations::translations_3::port_translation;
use crate::translations::types::language::Language;
use crate::utils::formatted_strings::get_formatted_bytes_string;

// total width: 1012.0

const LARGE_COL_WIDTH: f32 = 221.0;
const SMALL_COL_WIDTH: f32 = 95.0;

const LARGE_COL_MAX_CHARS: usize = 25;
const SMALL_COL_MAX_CHARS: usize = 10;

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
}

impl ReportCol {
    pub(crate) const ALL: [ReportCol; 8] = [
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
                source_translation(language).to_lowercase()
            ),
            ReportCol::SrcPort => format!(
                "{} ({})",
                port_translation(language),
                source_translation(language).to_lowercase()
            ),
            ReportCol::DstIp => format!(
                "{} ({})",
                address_translation(language),
                destination_translation(language).to_lowercase()
            ),
            ReportCol::DstPort => format!(
                "{} ({})",
                port_translation(language),
                destination_translation(language).to_lowercase()
            ),
            ReportCol::Proto => protocol_translation(language).to_string(),
            ReportCol::AppProto => application_protocol_translation(language).to_string(),
            ReportCol::Bytes => {
                let mut str = bytes_translation(language).to_string();
                str.remove(0).to_uppercase().to_string() + &str
            }
            ReportCol::Packets => {
                let mut str = packets_translation(language).to_string();
                str.remove(0).to_uppercase().to_string() + &str
            }
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
        }
    }

    pub(crate) fn get_width(&self) -> f32 {
        match self {
            ReportCol::SrcIp | ReportCol::DstIp => LARGE_COL_WIDTH,
            _ => SMALL_COL_WIDTH,
        }
    }

    pub(crate) fn get_max_chars(&self, language_opt: Option<Language>) -> usize {
        let reduction_factor = if [Language::JA, Language::KO, Language::ZH]
            .contains(&language_opt.unwrap_or(Language::EN))
        {
            2
        } else {
            1
        };
        match self {
            ReportCol::SrcIp | ReportCol::DstIp => LARGE_COL_MAX_CHARS / reduction_factor,
            _ => SMALL_COL_MAX_CHARS / reduction_factor,
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
            ReportCol::Bytes | ReportCol::Packets => FilterInputType::Country, // just to not panic...
        }
    }
}
