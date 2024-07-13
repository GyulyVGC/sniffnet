use crate::countries::types::country::Country;
use crate::mmdb::types::mmdb_country_entry::MmdbCountryEntry;
use crate::mmdb::types::mmdb_reader::MmdbReader;

pub const COUNTRY_MMDB: &[u8] = include_bytes!("../../resources/DB/GeoLite2-Country.mmdb");

#[allow(clippy::module_name_repetitions)]
pub fn get_country(address: &str, country_db_reader: &MmdbReader) -> Country {
    if let Ok(res) = country_db_reader.lookup::<MmdbCountryEntry>(address.parse().unwrap()) {
        return res.get_country();
    }
    Country::ZZ // unknown
}
