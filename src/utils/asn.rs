use maxminddb::{geoip2, MaxMindDBError, Reader};

use crate::networking::types::asn::Asn;

pub const ASN_MMDB: &[u8] = include_bytes!("../../resources/DB/GeoLite2-ASN.mmdb");

pub fn asn(address_to_lookup: &str, asn_db_reader: &Reader<&[u8]>) -> Asn {
    let asn_result: Result<geoip2::Asn, MaxMindDBError> =
        asn_db_reader.lookup(address_to_lookup.parse().unwrap());
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
