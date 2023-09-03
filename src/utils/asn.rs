use maxminddb::{geoip2, MaxMindDBError, Reader};

use crate::networking::types::asn::Asn;

pub const ASN_MMDB: &[u8] = include_bytes!("../../resources/DB/GeoLite2-ASN.mmdb");

pub fn mmdb_asn_reader(mmdb_asn_path: String) -> (Reader<&'static [u8]>, Option<Reader<Vec<u8>>>) {
    let default_reader = maxminddb::Reader::from_source(ASN_MMDB).unwrap();
    if mmdb_asn_path.is_empty() {
        (default_reader, None)
    } else {
        let custom_reader_result = maxminddb::Reader::open_readfile(mmdb_asn_path);
        if let Ok(custom_reader) = custom_reader_result {
            return (default_reader, Some(custom_reader));
        }
        (default_reader, None)
    }
}

pub fn asn(
    address_to_lookup: &str,
    asn_db_readers: &(Reader<&[u8]>, Option<Reader<Vec<u8>>>),
) -> Asn {
    let (default_reader, custom_reader) = asn_db_readers;
    let asn_result: Result<geoip2::Asn, MaxMindDBError> = if custom_reader.is_some() {
        custom_reader
            .as_ref()
            .unwrap()
            .lookup(address_to_lookup.parse().unwrap())
    } else {
        default_reader.lookup(address_to_lookup.parse().unwrap())
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
