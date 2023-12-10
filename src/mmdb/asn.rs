use maxminddb::{geoip2, MaxMindDBError};

use crate::mmdb::types::mmdb_reader::MmdbReader;
use crate::networking::types::asn::Asn;

pub const ASN_MMDB: &[u8] = include_bytes!("../../resources/DB/GeoLite2-ASN.mmdb");

#[allow(clippy::module_name_repetitions)]
pub fn get_asn(address_to_lookup: &str, asn_db_reader: &MmdbReader) -> Asn {
    let asn_result: Result<geoip2::Asn, MaxMindDBError> = match asn_db_reader {
        MmdbReader::Default(reader) => reader.lookup(address_to_lookup.parse().unwrap()),
        MmdbReader::Custom(reader) => reader.lookup(address_to_lookup.parse().unwrap()),
    };
    if let Ok(res) = asn_result {
        if res.autonomous_system_number.is_some() && res.autonomous_system_organization.is_some() {
            return Asn {
                number: res.autonomous_system_number.unwrap(),
                name: res.autonomous_system_organization.unwrap().to_string(),
            };
        }
    }
    Asn::default()
}
