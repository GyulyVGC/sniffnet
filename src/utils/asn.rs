use maxminddb::{geoip2, MaxMindDBError, Reader};

use crate::networking::types::asn::Asn;

pub const ASN_MMDB: &[u8] = include_bytes!("../../resources/DB/GeoLite2-ASN.mmdb");

pub enum MmdbReader {
    Default(Reader<&'static [u8]>),
    Custom(Reader<Vec<u8>>),
}

pub fn mmdb_reader(mmdb_path: &String, default_mmdb: &'static [u8]) -> MmdbReader {
    let default_reader = maxminddb::Reader::from_source(default_mmdb).unwrap();
    if mmdb_path.is_empty() {
        MmdbReader::Default(default_reader)
    } else {
        let custom_reader_result = maxminddb::Reader::open_readfile(mmdb_path);
        if let Ok(custom_reader) = custom_reader_result {
            return MmdbReader::Custom(custom_reader);
        }
        MmdbReader::Default(default_reader)
    }
}

pub fn asn(address_to_lookup: &str, asn_db_reader: &MmdbReader) -> Asn {
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
