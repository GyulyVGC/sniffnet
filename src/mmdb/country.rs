use maxminddb::{geoip2, MaxMindDBError};

use crate::countries::types::country::Country;
use crate::mmdb::types::mmdb_reader::MmdbReader;

pub const COUNTRY_MMDB: &[u8] = include_bytes!("../../resources/DB/GeoLite2-Country.mmdb");

#[allow(clippy::module_name_repetitions)]
pub fn get_country(address_to_lookup: &str, country_db_reader: &MmdbReader) -> Country {
    let country_result: Result<geoip2::Country, MaxMindDBError> = match country_db_reader {
        MmdbReader::Default(reader) => reader.lookup(address_to_lookup.parse().unwrap()),
        MmdbReader::Custom(reader) => reader.lookup(address_to_lookup.parse().unwrap()),
    };
    if let Ok(res1) = country_result {
        if let Some(res2) = res1.country {
            if let Some(res3) = res2.iso_code {
                return Country::from_str(res3);
            }
        }
    }
    Country::ZZ // unknown
}
