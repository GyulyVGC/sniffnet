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

#[cfg(test)]
mod tests {
    use crate::countries::types::country::Country;
    use crate::mmdb::country::{get_country, COUNTRY_MMDB};
    use crate::mmdb::types::mmdb_reader::MmdbReader;

    #[test]
    fn test_get_country_with_default_reader() {
        let reader_1 = MmdbReader::from(&String::from("unknown path"), COUNTRY_MMDB);
        assert!(matches!(reader_1, MmdbReader::Default(_)));
        let reader_2 = MmdbReader::from(&String::new(), COUNTRY_MMDB);
        assert!(matches!(reader_2, MmdbReader::Default(_)));
        let reader_3 = MmdbReader::from(&String::from("resources/repository/hr.png"), COUNTRY_MMDB);
        assert!(matches!(reader_3, MmdbReader::Default(_)));
        let reader_4 = MmdbReader::from(
            &String::from("resources/DB/GeoLite2-Country.mmdb"),
            COUNTRY_MMDB,
        );
        assert!(matches!(reader_4, MmdbReader::Custom(_)));
        let reader_5 = MmdbReader::from(&String::from("resources/DB/GeoLite2-Country.mmdb"), &[]);
        assert!(matches!(reader_5, MmdbReader::Custom(_)));

        for reader in vec![reader_1, reader_2, reader_3, reader_4, reader_5] {
            // known IP
            let res = get_country("8.8.8.8", &reader);
            assert_eq!(res, Country::US);

            // another known IP
            let res = get_country("78.35.248.93", &reader);
            assert_eq!(res, Country::DE);

            // known IPv6
            let res = get_country("2806:230:2057::", &reader);
            assert_eq!(res, Country::MX);

            // unknown IP
            let res = get_country("127.0.0.1", &reader);
            assert_eq!(res, Country::ZZ);

            // unknown IPv6
            let res = get_country("::1", &reader);
            assert_eq!(res, Country::ZZ);
        }
    }

    #[test]
    fn test_get_country_with_custom_ipinfo_single_reader() {
        let reader_1 = MmdbReader::from(
            &String::from("resources/test/ipinfo_country_sample.mmdb"),
            COUNTRY_MMDB,
        );
        let reader_2 = MmdbReader::from(
            &String::from("resources/test/ipinfo_country_sample.mmdb"),
            &[],
        );

        for reader in vec![reader_1, reader_2] {
            assert!(matches!(reader, MmdbReader::Custom(_)));

            // known IP
            let res = get_country("2.2.146.0", &reader);
            assert_eq!(res, Country::GB);

            // another known IP
            let res = get_country("23.193.112.81", &reader);
            assert_eq!(res, Country::US);

            // known IPv6
            let res = get_country("2a0e:1d80::", &reader);
            assert_eq!(res, Country::RO);

            // unknown IP
            let res = get_country("127.0.0.1", &reader);
            assert_eq!(res, Country::ZZ);

            // unknown IPv6
            let res = get_country("::1", &reader);
            assert_eq!(res, Country::ZZ);
        }
    }

    #[test]
    fn test_get_country_with_custom_ipinfo_combined_reader() {
        let reader_1 = MmdbReader::from(
            &String::from("resources/test/ipinfo_country_asn_sample.mmdb"),
            COUNTRY_MMDB,
        );
        let reader_2 = MmdbReader::from(
            &String::from("resources/test/ipinfo_country_asn_sample.mmdb"),
            &[],
        );

        for reader in vec![reader_1, reader_2] {
            assert!(matches!(reader, MmdbReader::Custom(_)));

            // known IP
            let res = get_country("31.171.144.141", &reader);
            assert_eq!(res, Country::IT);

            // another known IP
            let res = get_country("103.112.220.111", &reader);
            assert_eq!(res, Country::TH);

            // known IPv6
            let res = get_country("2a02:6ea0:f001::", &reader);
            assert_eq!(res, Country::AR);

            // unknown IP
            let res = get_country("127.0.0.1", &reader);
            assert_eq!(res, Country::ZZ);

            // unknown IPv6
            let res = get_country("::1", &reader);
            assert_eq!(res, Country::ZZ);
        }
    }
}
