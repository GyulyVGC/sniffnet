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

#[cfg(test)]
mod tests {
    use crate::mmdb::asn::{get_asn, ASN_MMDB};
    use crate::mmdb::types::mmdb_reader::MmdbReader;

    #[test]
    fn test_get_asn_with_default_reader() {
        let reader_1 = MmdbReader::from(&String::from("unknown path"), ASN_MMDB);
        assert!(matches!(reader_1, MmdbReader::Default(_)));
        let reader_2 = MmdbReader::from(&String::new(), ASN_MMDB);
        assert!(matches!(reader_2, MmdbReader::Default(_)));
        let reader_3 = MmdbReader::from(&String::from("resources/repository/hr.png"), ASN_MMDB);
        assert!(matches!(reader_3, MmdbReader::Default(_)));
        let reader_4 = MmdbReader::from(&String::from("resources/DB/GeoLite2-ASN.mmdb"), ASN_MMDB);
        assert!(matches!(reader_4, MmdbReader::Custom(_)));
        let reader_5 = MmdbReader::from(&String::from("resources/DB/GeoLite2-ASN.mmdb"), &[]);
        assert!(matches!(reader_5, MmdbReader::Custom(_)));

        for reader in vec![reader_1, reader_2, reader_3, reader_4, reader_5] {
            // known IP
            let res = get_asn("8.8.8.8", &reader);
            assert_eq!(res.code, "15169");
            assert_eq!(res.name, "GOOGLE");

            // another known IP
            let res = get_asn("78.35.248.93", &reader);
            assert_eq!(res.code, "8422");
            assert_eq!(
                res.name,
                "NetCologne Gesellschaft fur Telekommunikation mbH"
            );

            // known IPv6
            let res = get_asn("2806:230:2057::", &reader);
            assert_eq!(res.code, "11888");
            assert_eq!(res.name, "Television Internacional, S.A. de C.V.");

            // unknown IP
            let res = get_asn("127.0.0.1", &reader);
            assert_eq!(res.code, "");
            assert_eq!(res.name, "");

            // unknown IPv6
            let res = get_asn("::1", &reader);
            assert_eq!(res.code, "");
            assert_eq!(res.name, "");
        }
    }

    #[test]
    fn test_get_asn_with_custom_ipinfo_single_reader() {
        let reader_1 = MmdbReader::from(
            &String::from("resources/test/ipinfo_asn_sample.mmdb"),
            ASN_MMDB,
        );
        let reader_2 =
            MmdbReader::from(&String::from("resources/test/ipinfo_asn_sample.mmdb"), &[]);

        for reader in vec![reader_1, reader_2] {
            assert!(matches!(reader, MmdbReader::Custom(_)));

            // known IP
            let res = get_asn("61.8.0.0", &reader);
            assert_eq!(res.code, "AS1221");
            assert_eq!(res.name, "Telstra Limited");

            // another known IP
            let res = get_asn("206.180.34.99", &reader);
            assert_eq!(res.code, "AS63344");
            assert_eq!(res.name, "The Reynolds and Reynolds Company");

            // known IPv6
            let res = get_asn("2806:230:2057::", &reader);
            assert_eq!(res.code, "AS11888");
            assert_eq!(res.name, "Television Internacional, S.A. de C.V.");

            // unknown IP
            let res = get_asn("127.0.0.1", &reader);
            assert_eq!(res.code, "");
            assert_eq!(res.name, "");

            // unknown IPv6
            let res = get_asn("::1", &reader);
            assert_eq!(res.code, "");
            assert_eq!(res.name, "");
        }
    }

    #[test]
    fn test_get_asn_with_custom_ipinfo_combined_reader() {
        let reader_1 = MmdbReader::from(
            &String::from("resources/test/ipinfo_country_asn_sample.mmdb"),
            ASN_MMDB,
        );
        let reader_2 = MmdbReader::from(
            &String::from("resources/test/ipinfo_country_asn_sample.mmdb"),
            &[],
        );

        for reader in vec![reader_1, reader_2] {
            assert!(matches!(reader, MmdbReader::Custom(_)));

            // known IP
            let res = get_asn("31.171.144.141", &reader);
            assert_eq!(res.code, "AS197742");
            assert_eq!(res.name, "IBB Energie AG");

            // another known IP
            let res = get_asn("103.112.220.111", &reader);
            assert_eq!(res.code, "AS134077");
            assert_eq!(res.name, "Magik Pivot Company Limited");

            // known IPv6
            let res = get_asn("2a02:6ea0:f001::", &reader);
            assert_eq!(res.code, "AS60068");
            assert_eq!(res.name, "Datacamp Limited");

            // unknown IP
            let res = get_asn("127.0.0.1", &reader);
            assert_eq!(res.code, "");
            assert_eq!(res.name, "");

            // unknown IPv6
            let res = get_asn("::1", &reader);
            assert_eq!(res.code, "");
            assert_eq!(res.name, "");
        }
    }
}
