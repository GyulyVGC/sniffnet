use maxminddb::MaxMindDBError;

use crate::countries::types::country::Country;
use crate::mmdb::types::mmdb_country_entry::MmdbCountryEntry;
use crate::mmdb::types::mmdb_reader::MmdbReader;

pub const COUNTRY_MMDB: &[u8] = include_bytes!("../../resources/DB/GeoLite2-Country.mmdb");

#[allow(clippy::module_name_repetitions)]
pub fn get_country(address_to_lookup: &str, country_db_reader: &MmdbReader) -> Country {
    let country_result: Result<MmdbCountryEntry, MaxMindDBError> = match country_db_reader {
        MmdbReader::Default(reader) => reader.lookup(address_to_lookup.parse().unwrap()),
        MmdbReader::Custom(reader) => reader.lookup(address_to_lookup.parse().unwrap()),
    };
    if let Ok(res) = country_result {
        return res.get_country();
    }
    Country::ZZ // unknown
}
