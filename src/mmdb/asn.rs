use crate::mmdb::types::mmdb_asn_entry::MmdbAsnEntry;
use crate::mmdb::types::mmdb_reader::MmdbReader;
use crate::networking::types::asn::Asn;
use std::net::IpAddr;

pub const ASN_MMDB: &[u8] = include_bytes!("../../resources/DB/GeoLite2-ASN.mmdb");

#[allow(clippy::module_name_repetitions)]
pub fn get_asn(address: &IpAddr, asn_db_reader: &MmdbReader) -> Asn {
    if let Ok(Some(res)) = asn_db_reader.lookup::<MmdbAsnEntry>(*address) {
        return res.get_asn();
    }
    Asn::default()
}

#[cfg(test)]
mod tests {
    use crate::mmdb::asn::{ASN_MMDB, get_asn};
    use crate::mmdb::types::mmdb_reader::MmdbReader;
    use std::net::IpAddr;
    use std::str::FromStr;

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
            let res = get_asn(&IpAddr::from([8, 8, 8, 8]), &reader);
            assert_eq!(res.code, "15169");
            assert_eq!(res.name, "GOOGLE");

            // another known IP
            let res = get_asn(&IpAddr::from([78, 35, 248, 93]), &reader);
            assert_eq!(res.code, "8422");
            assert_eq!(
                res.name,
                "NetCologne Gesellschaft fur Telekommunikation mbH"
            );

            // known IPv6
            let res = get_asn(&IpAddr::from_str("2806:230:2057::").unwrap(), &reader);
            assert_eq!(res.code, "11888");
            assert_eq!(res.name, "Television Internacional, S.A. de C.V.");

            // unknown IP
            let res = get_asn(&IpAddr::from([127, 0, 0, 1]), &reader);
            assert_eq!(res.code, "");
            assert_eq!(res.name, "");

            // unknown IPv6
            let res = get_asn(&IpAddr::from_str("::1").unwrap(), &reader);
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
            let res = get_asn(&IpAddr::from([185, 72, 2, 28]), &reader);
            assert_eq!(res.code, "AS202583");
            assert_eq!(res.name, "AVATEL TELECOM, SA");

            // another known IP
            let res = get_asn(&IpAddr::from([89, 187, 198, 0]), &reader);
            assert_eq!(res.code, "AS210367");
            assert_eq!(res.name, "Krajska zdravotni, a.s.");

            // known IPv6
            let res = get_asn(&IpAddr::from_str("2408:8957:6280::").unwrap(), &reader);
            assert_eq!(res.code, "AS17622");
            assert_eq!(res.name, "China Unicom Guangzhou network");

            // unknown IP
            let res = get_asn(&IpAddr::from([127, 0, 0, 1]), &reader);
            assert_eq!(res.code, "");
            assert_eq!(res.name, "");

            // unknown IPv6
            let res = get_asn(&IpAddr::from_str("::1").unwrap(), &reader);
            assert_eq!(res.code, "");
            assert_eq!(res.name, "");
        }
    }

    #[test]
    fn test_get_asn_with_custom_ipinfo_combined_reader() {
        let reader_1 = MmdbReader::from(
            &String::from("resources/test/ipinfo_lite_sample.mmdb"),
            ASN_MMDB,
        );
        let reader_2 =
            MmdbReader::from(&String::from("resources/test/ipinfo_lite_sample.mmdb"), &[]);

        for reader in vec![reader_1, reader_2] {
            assert!(matches!(reader, MmdbReader::Custom(_)));

            // known IP
            let res = get_asn(&IpAddr::from([1, 0, 65, 1]), &reader);
            assert_eq!(res.code, "AS18144");
            assert_eq!(res.name, "Enecom,Inc.");

            // another known IP
            let res = get_asn(&IpAddr::from([1, 6, 230, 0]), &reader);
            assert_eq!(res.code, "AS4755");
            assert_eq!(res.name, "TATA Communications formerly VSNL is Leading ISP");

            // known IPv6
            // let res = get_asn(&IpAddr::from_str("2a02:6ea0:f001::").unwrap(), &reader);
            // assert_eq!(res.code, "AS60068");
            // assert_eq!(res.name, "Datacamp Limited");

            // unknown IP
            let res = get_asn(&IpAddr::from([127, 0, 0, 1]), &reader);
            assert_eq!(res.code, "");
            assert_eq!(res.name, "");

            // unknown IPv6
            let res = get_asn(&IpAddr::from_str("::1").unwrap(), &reader);
            assert_eq!(res.code, "");
            assert_eq!(res.name, "");
        }
    }
}
