use crate::mmdb::types::mmdb_asn_entry::MmdbAsnEntry;
use maxminddb::MaxMindDBError;

use crate::mmdb::types::mmdb_reader::MmdbReader;
use crate::networking::types::asn::Asn;

pub const ASN_MMDB: &[u8] = include_bytes!("../../resources/DB/GeoLite2-ASN.mmdb");

#[allow(clippy::module_name_repetitions)]
pub fn get_asn(address_to_lookup: &str, asn_db_reader: &MmdbReader) -> Asn {
    let asn_result: Result<MmdbAsnEntry, MaxMindDBError> = match asn_db_reader {
        MmdbReader::Default(reader) => reader.lookup(address_to_lookup.parse().unwrap()),
        MmdbReader::Custom(reader) => reader.lookup(address_to_lookup.parse().unwrap()),
    };
    if let Ok(res) = asn_result {
        return res.get_asn();
    }
    Asn::default()
}
