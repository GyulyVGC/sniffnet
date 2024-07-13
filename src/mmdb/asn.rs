use crate::mmdb::types::mmdb_asn_entry::MmdbAsnEntry;
use crate::mmdb::types::mmdb_reader::MmdbReader;
use crate::networking::types::asn::Asn;

pub const ASN_MMDB: &[u8] = include_bytes!("../../resources/DB/GeoLite2-ASN.mmdb");

#[allow(clippy::module_name_repetitions)]
pub fn get_asn(address: &str, asn_db_reader: &MmdbReader) -> Asn {
    if let Ok(res) = asn_db_reader.lookup::<MmdbAsnEntry>(address.parse().unwrap()) {
        return res.get_asn();
    }
    Asn::default()
}
