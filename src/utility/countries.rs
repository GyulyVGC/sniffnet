use crate::enums::traffic_type::TrafficType;
use crate::structs::address_port_pair::AddressPortPair;
use iced::widget::{image::Handle, Image};
use iced::Length;
use maxminddb::{geoip2, MaxMindDBError, Reader};

pub const COUNTRY_MMDB: &[u8] = include_bytes!("../../resources/DB/GeoLite2-Country.mmdb");

pub fn get_country_code(
    traffic_type: TrafficType,
    key: &AddressPortPair,
    country_db_reader: &Reader<&[u8]>,
) -> String {
    let address_to_lookup = match traffic_type {
        TrafficType::Outgoing => &key.address2,
        _ => &key.address1,
    };

    let country_result: Result<geoip2::Country, MaxMindDBError> =
        country_db_reader.lookup(address_to_lookup.parse().unwrap());
    if let Ok(res1) = country_result {
        if let Some(res2) = res1.country {
            if let Some(res3) = res2.iso_code {
                return res3.to_string().replace("ZZ", "//");
            }
        }
    }
    String::new()
}

pub const FLAGS_WIDTH: u16 = 15;

pub const AD: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/andorra-16x16-32921.png");
pub const AE: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/united-16x16-33114.png");
pub const AF: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/afghanistan-16x16-32928.png");
pub const AG: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/antigua-16x16-32910.png");
pub const AI: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/anguilla-16x16-32924.png");
pub const AL: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/albania-16x16-32909.png");
pub const AM: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/armenia-16x16-32925.png");
pub const AO: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/angola-16x16-32914.png");
pub const AQ: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/antarctica-16x16-33151.png");
pub const AR: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/argentina-16x16-32919.png");
pub const AS: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/american-16x16-32917.png");
pub const AT: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/austria-16x16-32920.png");
pub const AU: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/austallia-16x16-32912.png");
pub const AW: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/aruba-16x16-32923.png");
pub const AX: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/aland-16x16-32908.png");
pub const AZ: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/azerbaijan-16x16-32926.png");
pub const BA: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/bosnia-16x16-32932.png");
pub const BB: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/barbados-16x16-32913.png");
pub const BD: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/bangladesh-16x16-32916.png");
pub const BE: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/belgium-16x16-32911.png");
pub const BF: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/burkina-16x16-32934.png");
pub const BG: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/bulgaria-16x16-32973.png");
pub const BH: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/bahrain-16x16-32974.png");
pub const BI: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/burundi-16x16-32935.png");
pub const BJ: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/benin-16x16-32922.png");
pub const BL: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/saint-16x16-33068.png");
pub const BM: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/bermuda-16x16-32929.png");
pub const BN: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/brunei-16x16-32944.png");
pub const BO: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/bolivia-16x16-32945.png");
pub const BQ: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/bonaire-16x16-32930.png");
pub const BR: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/brazil-16x16-32937.png");
pub const BS: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/bahamas-16x16-32915.png");
pub const BT: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/bhutan-16x16-32931.png");
pub const BV: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/bouvet-16x16-33156.png");
pub const BW: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/bostwana-16x16-32933.png");
pub const BY: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/belarus-16x16-32918.png");
pub const BZ: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/belize-16x16-32927.png");
pub const CA: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/canada-16x16-32938.png");
pub const CC: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/cocos-16x16-32947.png");
pub const CD: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/democratic-16x16-32952.png");
pub const CF: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/central-16x16-32940.png");
pub const CG: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/republic-16x16-33061.png");
pub const CH: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/switzerland-16x16-33095.png");
pub const CI: &[u8] = include_bytes!("../../resources/countries_flags/png-16/cote-16x16-32949.png");
pub const CK: &[u8] = include_bytes!("../../resources/countries_flags/png-16/cook-16x16-32954.png");
pub const CL: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/chile-16x16-32939.png");
pub const CM: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/cameroon-16x16-32936.png");
pub const CN: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/china-16x16-32942.png");
pub const CO: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/colombia-16x16-32946.png");
pub const CR: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/costa-16x16-32948.png");
pub const CU: &[u8] = include_bytes!("../../resources/countries_flags/png-16/cuba-16x16-32951.png");
pub const CV: &[u8] = include_bytes!("../../resources/countries_flags/png-16/cabo-16x16-32941.png");
pub const CW: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/curacao-16x16-32950.png");
pub const CX: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/christmas-16x16-32943.png");
pub const CY: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/cyprus-16x16-32953.png");
pub const CZ: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/czech-16x16-32956.png");
pub const DE: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/germany-16x16-32989.png");
pub const DJ: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/djibouti-16x16-32957.png");
pub const DK: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/denmark-16x16-32955.png");
pub const DM: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/dominica-16x16-32960.png");
pub const DO: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/dominican-16x16-32993.png");
pub const DZ: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/algeria-16x16-32972.png");
pub const EC: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/ecuador-16x16-32962.png");
pub const EE: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/estonia-16x16-32959.png");
pub const EG: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/egypt-16x16-32961.png");
pub const EH: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/western-16x16-33139.png");
pub const ER: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/eritrea-16x16-32964.png");
pub const ES: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/spain-16x16-33105.png");
pub const ET: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/ethiopia-16x16-32958.png");
pub const FI: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/finland-16x16-32966.png");
pub const FJ: &[u8] = include_bytes!("../../resources/countries_flags/png-16/fiji-16x16-32970.png");
pub const FK: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/falkland-16x16-32963.png");
pub const FM: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/federated-16x16-32969.png");
pub const FO: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/faroe-16x16-32965.png");
pub const FR: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/france-16x16-32967.png");
pub const GA: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/gabon-16x16-32968.png");
pub const GB: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/united-16x16-33115.png");
pub const GD: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/grenada-16x16-33002.png");
pub const GE: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/georgia-16x16-32979.png");
pub const GF: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/france-16x16-32967.png");
pub const GG: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/guernsey-16x16-32980.png");
pub const GH: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/ghana-16x16-32990.png");
pub const GI: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/gibraltar-16x16-32992.png");
pub const GL: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/greenland-16x16-33003.png");
pub const GM: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/gambia-16x16-32976.png");
pub const GN: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/guinea-16x16-33008.png");
pub const GP: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/guadeloupe-16x16-32982.png");
pub const GQ: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/equatorial-16x16-32971.png");
pub const GR: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/greece-16x16-32991.png");
pub const GS: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/south-16x16-33088.png");
pub const GT: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/guatemala-16x16-33009.png");
pub const GU: &[u8] = include_bytes!("../../resources/countries_flags/png-16/guam-16x16-33006.png");
pub const GW: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/guinea-16x16-32983.png");
pub const GY: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/guyana-16x16-33007.png");
pub const HK: &[u8] = include_bytes!("../../resources/countries_flags/png-16/hong-16x16-32987.png");
pub const HM: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/heard-16x16-33153.png");
pub const HN: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/honduras-16x16-33020.png");
pub const HR: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/croatia-16x16-32995.png");
pub const HT: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/haiti-16x16-32984.png");
pub const HU: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/hungary-16x16-33019.png");
pub const ID: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/indonesia-16x16-32994.png");
pub const IE: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/ireland-16x16-32996.png");
pub const IL: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/israel-16x16-32997.png");
pub const IM: &[u8] = include_bytes!("../../resources/countries_flags/png-16/isle-16x16-32998.png");
pub const IN: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/india-16x16-32988.png");
pub const IO: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/british-16x16-33159.png");
pub const IQ: &[u8] = include_bytes!("../../resources/countries_flags/png-16/iraq-16x16-33017.png");
pub const IR: &[u8] = include_bytes!("../../resources/countries_flags/png-16/iran-16x16-33022.png");
pub const IS: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/iceland-16x16-33152.png");
pub const IT: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/italy-16x16-32999.png");
pub const JE: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/jersey-16x16-33004.png");
pub const JM: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/jamaica-16x16-33000.png");
pub const JO: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/jordan-16x16-33005.png");
pub const JP: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/japan-16x16-33001.png");
pub const KE: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/kenya-16x16-33010.png");
pub const KG: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/kyrgyzstan-16x16-33014.png");
pub const KH: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/cambodia-16x16-32978.png");
pub const KI: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/kiribati-16x16-33012.png");
pub const KM: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/comoros-16x16-32981.png");
pub const KN: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/saint-16x16-33070.png");
pub const KP: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/north-16x16-33045.png");
pub const KR: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/south-16x16-33086.png");
pub const KW: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/kuwait-16x16-33013.png");
pub const KY: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/cayman-16x16-32986.png");
pub const KZ: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/kazakhstan-16x16-33021.png");
pub const LA: &[u8] = include_bytes!("../../resources/countries_flags/png-16/laos-16x16-33018.png");
pub const LB: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/lebanon-16x16-33015.png");
pub const LC: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/saint-16x16-33071.png");
pub const LI: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/liechtenstein-16x16-33016.png");
pub const LK: &[u8] = include_bytes!("../../resources/countries_flags/png-16/sri-16x16-33091.png");
pub const LR: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/liberia-16x16-33032.png");
pub const LS: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/lesotho-16x16-33029.png");
pub const LT: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/lithuania-16x16-33043.png");
pub const LU: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/luxembourg-16x16-33041.png");
pub const LV: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/latvia-16x16-33031.png");
pub const LY: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/libya-16x16-33026.png");
pub const MA: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/morocco-16x16-33027.png");
pub const MC: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/monaco-16x16-33053.png");
pub const MD: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/moldova-16x16-33055.png");
pub const ME: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/montenegro-16x16-33056.png");
pub const MF: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/saint-16x16-33068.png");
pub const MG: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/madagascar-16x16-33042.png");
pub const MH: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/marshall-16x16-33054.png");
pub const MK: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/former-16x16-33023.png");
pub const ML: &[u8] = include_bytes!("../../resources/countries_flags/png-16/mali-16x16-33025.png");
pub const MM: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/myanmar-16x16-33118.png");
pub const MN: &[u8] = include_bytes!("../../resources/countries_flags/png-16/mn-16x16-33117.png");
pub const MO: &[u8] = include_bytes!("../../resources/countries_flags/png-16/mo-16x16-33127.png");
pub const MP: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/northern-16x16-33128.png");
pub const MQ: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/martinique-16x16-33119.png");
pub const MR: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/mauritania-16x16-33125.png");
pub const MS: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/montserrat-16x16-33126.png");
pub const MT: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/malta-16x16-33120.png");
pub const MU: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/mauritius-16x16-33121.png");
pub const MV: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/maldives-16x16-33122.png");
pub const MW: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/malawi-16x16-33116.png");
pub const MX: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/mexico-16x16-33133.png");
pub const MY: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/malaysia-16x16-33134.png");
pub const MZ: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/mozambique-16x16-33033.png");
pub const NA: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/namibia-16x16-33132.png");
pub const NC: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/france-16x16-32967.png");
pub const NE: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/niger-16x16-33038.png");
pub const NF: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/norfolk-16x16-33044.png");
pub const NG: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/nigeria-16x16-33039.png");
pub const NI: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/nicaragua-16x16-33037.png");
pub const NL: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/netherlands-16x16-33035.png");
pub const NO: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/norway-16x16-33155.png");
pub const NP: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/nepal-16x16-33028.png");
pub const NR: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/nauru-16x16-33030.png");
pub const NU: &[u8] = include_bytes!("../../resources/countries_flags/png-16/niue-16x16-33040.png");
pub const NZ: &[u8] = include_bytes!("../../resources/countries_flags/png-16/new-16x16-33036.png");
pub const OM: &[u8] = include_bytes!("../../resources/countries_flags/png-16/oman-16x16-33046.png");
pub const PA: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/panama-16x16-33049.png");
pub const PE: &[u8] = include_bytes!("../../resources/countries_flags/png-16/peru-16x16-33051.png");
pub const PF: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/french-16x16-33024.png");
pub const PG: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/papua-16x16-33050.png");
pub const PH: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/philippines-16x16-33052.png");
pub const PK: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/pakistan-16x16-33047.png");
pub const PL: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/poland-16x16-33057.png");
pub const PM: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/france-16x16-32967.png");
pub const PN: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/itcairn-16x16-33034.png");
pub const PR: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/puerto-16x16-33059.png");
pub const PS: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/state-16x16-33089.png");
pub const PT: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/portugal-16x16-33058.png");
pub const PW: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/palau-16x16-33048.png");
pub const PY: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/paraguay-16x16-33066.png");
pub const QA: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/qatar-16x16-33060.png");
pub const RE: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/france-16x16-32967.png");
pub const RO: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/romania-16x16-33063.png");
pub const RS: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/serbia-16x16-33099.png");
pub const RU: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/russia-16x16-33064.png");
pub const RW: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/rwanda-16x16-33067.png");
pub const SA: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/saudi-16x16-33076.png");
pub const SB: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/solomon-16x16-33083.png");
pub const SC: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/seychelles-16x16-33078.png");
pub const SD: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/sudan-16x16-33090.png");
pub const SE: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/sweden-16x16-33096.png");
pub const SG: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/singapore-16x16-33080.png");
pub const SH: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/united-16x16-33115.png");
pub const SI: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/slovenia-16x16-33084.png");
pub const SJ: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/svalbard-16x16-33093.png");
pub const SK: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/slovakia-16x16-33082.png");
pub const SL: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/sierra-16x16-33079.png");
pub const SM: &[u8] = include_bytes!("../../resources/countries_flags/png-16/san-16x16-33074.png");
pub const SN: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/senegal-16x16-33077.png");
pub const SO: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/somalia-16x16-33085.png");
pub const SR: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/suriname-16x16-33092.png");
pub const SS: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/south-16x16-33087.png");
pub const ST: &[u8] = include_bytes!("../../resources/countries_flags/png-16/sao-16x16-33075.png");
pub const SV: &[u8] = include_bytes!("../../resources/countries_flags/png-16/el-16x16-33011.png");
pub const SX: &[u8] = include_bytes!("../../resources/countries_flags/png-16/sint-16x16-33081.png");
pub const SY: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/syrian-16x16-33097.png");
pub const SZ: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/swaziland-16x16-33094.png");
pub const TC: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/turks-16x16-33112.png");
pub const TD: &[u8] = include_bytes!("../../resources/countries_flags/png-16/chad-16x16-32985.png");
pub const TF: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/french-16x16-32977.png");
pub const TG: &[u8] = include_bytes!("../../resources/countries_flags/png-16/togo-16x16-33106.png");
pub const TH: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/thailand-16x16-33102.png");
pub const TJ: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/tajikistan-16x16-33100.png");
pub const TK: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/tokelau-16x16-33104.png");
pub const TL: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/timor-16x16-33103.png");
pub const TM: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/turkmenistan-16x16-33111.png");
pub const TN: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/tunisia-16x16-33110.png");
pub const TO: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/tonga-16x16-33107.png");
pub const TR: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/turkey-16x16-33109.png");
pub const TT: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/trinidad-16x16-33108.png");
pub const TV: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/tuvalu-16x16-33113.png");
pub const TW: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/taiwan-16x16-33098.png");
pub const TZ: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/tanzania-16x16-33101.png");
pub const UA: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/ukraine-16x16-33145.png");
pub const UG: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/uganda-16x16-33129.png");
pub const UM: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/united-16x16-33135.png");
pub const US: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/united-16x16-33137.png");
pub const UY: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/uruguay-16x16-33140.png");
pub const UZ: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/uzbekistan-16x16-33141.png");
pub const VA: &[u8] = include_bytes!("../../resources/countries_flags/png-16/holy-16x16-33136.png");
pub const VC: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/saint-16x16-33131.png");
pub const VE: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/venezuela-16x16-33138.png");
pub const VG: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/virgin-16x16-33147.png");
pub const VI: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/virgin-16x16-33149.png");
pub const VN: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/vietnam-16x16-33148.png");
pub const VU: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/vanuatu-16x16-33142.png");
pub const WF: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/wallis-16x16-33144.png");
pub const WS: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/samoa-16x16-33124.png");
pub const YE: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/yemen-16x16-33143.png");
pub const YT: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/mayotte-16x16-33123.png");
pub const ZA: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/south-16x16-33130.png");
pub const ZM: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/zambia-16x16-33146.png");
pub const ZW: &[u8] =
    include_bytes!("../../resources/countries_flags/png-16/zimbabwe-16x16-33150.png");
pub const UNKNOWN: &[u8] = include_bytes!("../../resources/countries_flags/png-16/question.png");

pub fn get_flag(country: &str) -> Image {
    match country {
        "AD" => Image::new(Handle::from_memory(Vec::from(AD))).width(Length::Units(FLAGS_WIDTH)),
        "AE" => Image::new(Handle::from_memory(Vec::from(AE))).width(Length::Units(FLAGS_WIDTH)),
        "AF" => Image::new(Handle::from_memory(Vec::from(AF))).width(Length::Units(FLAGS_WIDTH)),
        "AG" => Image::new(Handle::from_memory(Vec::from(AG))).width(Length::Units(FLAGS_WIDTH)),
        "AI" => Image::new(Handle::from_memory(Vec::from(AI))).width(Length::Units(FLAGS_WIDTH)),
        "AL" => Image::new(Handle::from_memory(Vec::from(AL))).width(Length::Units(FLAGS_WIDTH)),
        "AM" => Image::new(Handle::from_memory(Vec::from(AM))).width(Length::Units(FLAGS_WIDTH)),
        "AO" => Image::new(Handle::from_memory(Vec::from(AO))).width(Length::Units(FLAGS_WIDTH)),
        "AQ" => Image::new(Handle::from_memory(Vec::from(AQ))).width(Length::Units(FLAGS_WIDTH)),
        "AR" => Image::new(Handle::from_memory(Vec::from(AR))).width(Length::Units(FLAGS_WIDTH)),
        "AS" => Image::new(Handle::from_memory(Vec::from(AS))).width(Length::Units(FLAGS_WIDTH)),
        "AT" => Image::new(Handle::from_memory(Vec::from(AT))).width(Length::Units(FLAGS_WIDTH)),
        "AU" => Image::new(Handle::from_memory(Vec::from(AU))).width(Length::Units(FLAGS_WIDTH)),
        "AW" => Image::new(Handle::from_memory(Vec::from(AW))).width(Length::Units(FLAGS_WIDTH)),
        "AX" => Image::new(Handle::from_memory(Vec::from(AX))).width(Length::Units(FLAGS_WIDTH)),
        "AZ" => Image::new(Handle::from_memory(Vec::from(AZ))).width(Length::Units(FLAGS_WIDTH)),
        "BA" => Image::new(Handle::from_memory(Vec::from(BA))).width(Length::Units(FLAGS_WIDTH)),
        "BB" => Image::new(Handle::from_memory(Vec::from(BB))).width(Length::Units(FLAGS_WIDTH)),
        "BD" => Image::new(Handle::from_memory(Vec::from(BD))).width(Length::Units(FLAGS_WIDTH)),
        "BE" => Image::new(Handle::from_memory(Vec::from(BE))).width(Length::Units(FLAGS_WIDTH)),
        "BF" => Image::new(Handle::from_memory(Vec::from(BF))).width(Length::Units(FLAGS_WIDTH)),
        "BG" => Image::new(Handle::from_memory(Vec::from(BG))).width(Length::Units(FLAGS_WIDTH)),
        "BH" => Image::new(Handle::from_memory(Vec::from(BH))).width(Length::Units(FLAGS_WIDTH)),
        "BI" => Image::new(Handle::from_memory(Vec::from(BI))).width(Length::Units(FLAGS_WIDTH)),
        "BJ" => Image::new(Handle::from_memory(Vec::from(BJ))).width(Length::Units(FLAGS_WIDTH)),
        "BL" => Image::new(Handle::from_memory(Vec::from(BL))).width(Length::Units(FLAGS_WIDTH)),
        "BM" => Image::new(Handle::from_memory(Vec::from(BM))).width(Length::Units(FLAGS_WIDTH)),
        "BN" => Image::new(Handle::from_memory(Vec::from(BN))).width(Length::Units(FLAGS_WIDTH)),
        "BO" => Image::new(Handle::from_memory(Vec::from(BO))).width(Length::Units(FLAGS_WIDTH)),
        "BQ" => Image::new(Handle::from_memory(Vec::from(BQ))).width(Length::Units(FLAGS_WIDTH)),
        "BR" => Image::new(Handle::from_memory(Vec::from(BR))).width(Length::Units(FLAGS_WIDTH)),
        "BS" => Image::new(Handle::from_memory(Vec::from(BS))).width(Length::Units(FLAGS_WIDTH)),
        "BT" => Image::new(Handle::from_memory(Vec::from(BT))).width(Length::Units(FLAGS_WIDTH)),
        "BV" => Image::new(Handle::from_memory(Vec::from(BV))).width(Length::Units(FLAGS_WIDTH)),
        "BW" => Image::new(Handle::from_memory(Vec::from(BW))).width(Length::Units(FLAGS_WIDTH)),
        "BY" => Image::new(Handle::from_memory(Vec::from(BY))).width(Length::Units(FLAGS_WIDTH)),
        "BZ" => Image::new(Handle::from_memory(Vec::from(BZ))).width(Length::Units(FLAGS_WIDTH)),
        "CA" => Image::new(Handle::from_memory(Vec::from(CA))).width(Length::Units(FLAGS_WIDTH)),
        "CC" => Image::new(Handle::from_memory(Vec::from(CC))).width(Length::Units(FLAGS_WIDTH)),
        "CD" => Image::new(Handle::from_memory(Vec::from(CD))).width(Length::Units(FLAGS_WIDTH)),
        "CF" => Image::new(Handle::from_memory(Vec::from(CF))).width(Length::Units(FLAGS_WIDTH)),
        "CG" => Image::new(Handle::from_memory(Vec::from(CG))).width(Length::Units(FLAGS_WIDTH)),
        "CH" => Image::new(Handle::from_memory(Vec::from(CH))).width(Length::Units(FLAGS_WIDTH)),
        "CI" => Image::new(Handle::from_memory(Vec::from(CI))).width(Length::Units(FLAGS_WIDTH)),
        "CK" => Image::new(Handle::from_memory(Vec::from(CK))).width(Length::Units(FLAGS_WIDTH)),
        "CL" => Image::new(Handle::from_memory(Vec::from(CL))).width(Length::Units(FLAGS_WIDTH)),
        "CM" => Image::new(Handle::from_memory(Vec::from(CM))).width(Length::Units(FLAGS_WIDTH)),
        "CN" => Image::new(Handle::from_memory(Vec::from(CN))).width(Length::Units(FLAGS_WIDTH)),
        "CO" => Image::new(Handle::from_memory(Vec::from(CO))).width(Length::Units(FLAGS_WIDTH)),
        "CR" => Image::new(Handle::from_memory(Vec::from(CR))).width(Length::Units(FLAGS_WIDTH)),
        "CU" => Image::new(Handle::from_memory(Vec::from(CU))).width(Length::Units(FLAGS_WIDTH)),
        "CV" => Image::new(Handle::from_memory(Vec::from(CV))).width(Length::Units(FLAGS_WIDTH)),
        "CW" => Image::new(Handle::from_memory(Vec::from(CW))).width(Length::Units(FLAGS_WIDTH)),
        "CX" => Image::new(Handle::from_memory(Vec::from(CX))).width(Length::Units(FLAGS_WIDTH)),
        "CY" => Image::new(Handle::from_memory(Vec::from(CY))).width(Length::Units(FLAGS_WIDTH)),
        "CZ" => Image::new(Handle::from_memory(Vec::from(CZ))).width(Length::Units(FLAGS_WIDTH)),
        "DE" => Image::new(Handle::from_memory(Vec::from(DE))).width(Length::Units(FLAGS_WIDTH)),
        "DJ" => Image::new(Handle::from_memory(Vec::from(DJ))).width(Length::Units(FLAGS_WIDTH)),
        "DK" => Image::new(Handle::from_memory(Vec::from(DK))).width(Length::Units(FLAGS_WIDTH)),
        "DM" => Image::new(Handle::from_memory(Vec::from(DM))).width(Length::Units(FLAGS_WIDTH)),
        "DO" => Image::new(Handle::from_memory(Vec::from(DO))).width(Length::Units(FLAGS_WIDTH)),
        "DZ" => Image::new(Handle::from_memory(Vec::from(DZ))).width(Length::Units(FLAGS_WIDTH)),
        "EC" => Image::new(Handle::from_memory(Vec::from(EC))).width(Length::Units(FLAGS_WIDTH)),
        "EE" => Image::new(Handle::from_memory(Vec::from(EE))).width(Length::Units(FLAGS_WIDTH)),
        "EG" => Image::new(Handle::from_memory(Vec::from(EG))).width(Length::Units(FLAGS_WIDTH)),
        "EH" => Image::new(Handle::from_memory(Vec::from(EH))).width(Length::Units(FLAGS_WIDTH)),
        "ER" => Image::new(Handle::from_memory(Vec::from(ER))).width(Length::Units(FLAGS_WIDTH)),
        "ES" => Image::new(Handle::from_memory(Vec::from(ES))).width(Length::Units(FLAGS_WIDTH)),
        "ET" => Image::new(Handle::from_memory(Vec::from(ET))).width(Length::Units(FLAGS_WIDTH)),
        "FI" => Image::new(Handle::from_memory(Vec::from(FI))).width(Length::Units(FLAGS_WIDTH)),
        "FJ" => Image::new(Handle::from_memory(Vec::from(FJ))).width(Length::Units(FLAGS_WIDTH)),
        "FK" => Image::new(Handle::from_memory(Vec::from(FK))).width(Length::Units(FLAGS_WIDTH)),
        "FM" => Image::new(Handle::from_memory(Vec::from(FM))).width(Length::Units(FLAGS_WIDTH)),
        "FO" => Image::new(Handle::from_memory(Vec::from(FO))).width(Length::Units(FLAGS_WIDTH)),
        "FR" => Image::new(Handle::from_memory(Vec::from(FR))).width(Length::Units(FLAGS_WIDTH)),
        "GA" => Image::new(Handle::from_memory(Vec::from(GA))).width(Length::Units(FLAGS_WIDTH)),
        "GB" | "EN" => {
            Image::new(Handle::from_memory(Vec::from(GB))).width(Length::Units(FLAGS_WIDTH))
        }
        "GD" => Image::new(Handle::from_memory(Vec::from(GD))).width(Length::Units(FLAGS_WIDTH)),
        "GE" => Image::new(Handle::from_memory(Vec::from(GE))).width(Length::Units(FLAGS_WIDTH)),
        "GF" => Image::new(Handle::from_memory(Vec::from(GF))).width(Length::Units(FLAGS_WIDTH)),
        "GG" => Image::new(Handle::from_memory(Vec::from(GG))).width(Length::Units(FLAGS_WIDTH)),
        "GH" => Image::new(Handle::from_memory(Vec::from(GH))).width(Length::Units(FLAGS_WIDTH)),
        "GI" => Image::new(Handle::from_memory(Vec::from(GI))).width(Length::Units(FLAGS_WIDTH)),
        "GL" => Image::new(Handle::from_memory(Vec::from(GL))).width(Length::Units(FLAGS_WIDTH)),
        "GM" => Image::new(Handle::from_memory(Vec::from(GM))).width(Length::Units(FLAGS_WIDTH)),
        "GN" => Image::new(Handle::from_memory(Vec::from(GN))).width(Length::Units(FLAGS_WIDTH)),
        "GP" => Image::new(Handle::from_memory(Vec::from(GP))).width(Length::Units(FLAGS_WIDTH)),
        "GQ" => Image::new(Handle::from_memory(Vec::from(GQ))).width(Length::Units(FLAGS_WIDTH)),
        "GR" => Image::new(Handle::from_memory(Vec::from(GR))).width(Length::Units(FLAGS_WIDTH)),
        "GS" => Image::new(Handle::from_memory(Vec::from(GS))).width(Length::Units(FLAGS_WIDTH)),
        "GT" => Image::new(Handle::from_memory(Vec::from(GT))).width(Length::Units(FLAGS_WIDTH)),
        "GU" => Image::new(Handle::from_memory(Vec::from(GU))).width(Length::Units(FLAGS_WIDTH)),
        "GW" => Image::new(Handle::from_memory(Vec::from(GW))).width(Length::Units(FLAGS_WIDTH)),
        "GY" => Image::new(Handle::from_memory(Vec::from(GY))).width(Length::Units(FLAGS_WIDTH)),
        "HK" => Image::new(Handle::from_memory(Vec::from(HK))).width(Length::Units(FLAGS_WIDTH)),
        "HM" => Image::new(Handle::from_memory(Vec::from(HM))).width(Length::Units(FLAGS_WIDTH)),
        "HN" => Image::new(Handle::from_memory(Vec::from(HN))).width(Length::Units(FLAGS_WIDTH)),
        "HR" => Image::new(Handle::from_memory(Vec::from(HR))).width(Length::Units(FLAGS_WIDTH)),
        "HT" => Image::new(Handle::from_memory(Vec::from(HT))).width(Length::Units(FLAGS_WIDTH)),
        "HU" => Image::new(Handle::from_memory(Vec::from(HU))).width(Length::Units(FLAGS_WIDTH)),
        "ID" => Image::new(Handle::from_memory(Vec::from(ID))).width(Length::Units(FLAGS_WIDTH)),
        "IE" => Image::new(Handle::from_memory(Vec::from(IE))).width(Length::Units(FLAGS_WIDTH)),
        "IL" => Image::new(Handle::from_memory(Vec::from(IL))).width(Length::Units(FLAGS_WIDTH)),
        "IM" => Image::new(Handle::from_memory(Vec::from(IM))).width(Length::Units(FLAGS_WIDTH)),
        "IN" => Image::new(Handle::from_memory(Vec::from(IN))).width(Length::Units(FLAGS_WIDTH)),
        "IO" => Image::new(Handle::from_memory(Vec::from(IO))).width(Length::Units(FLAGS_WIDTH)),
        "IQ" => Image::new(Handle::from_memory(Vec::from(IQ))).width(Length::Units(FLAGS_WIDTH)),
        "IR" => Image::new(Handle::from_memory(Vec::from(IR))).width(Length::Units(FLAGS_WIDTH)),
        "IS" => Image::new(Handle::from_memory(Vec::from(IS))).width(Length::Units(FLAGS_WIDTH)),
        "IT" => Image::new(Handle::from_memory(Vec::from(IT))).width(Length::Units(FLAGS_WIDTH)),
        "JE" => Image::new(Handle::from_memory(Vec::from(JE))).width(Length::Units(FLAGS_WIDTH)),
        "JM" => Image::new(Handle::from_memory(Vec::from(JM))).width(Length::Units(FLAGS_WIDTH)),
        "JO" => Image::new(Handle::from_memory(Vec::from(JO))).width(Length::Units(FLAGS_WIDTH)),
        "JP" => Image::new(Handle::from_memory(Vec::from(JP))).width(Length::Units(FLAGS_WIDTH)),
        "KE" => Image::new(Handle::from_memory(Vec::from(KE))).width(Length::Units(FLAGS_WIDTH)),
        "KG" => Image::new(Handle::from_memory(Vec::from(KG))).width(Length::Units(FLAGS_WIDTH)),
        "KH" => Image::new(Handle::from_memory(Vec::from(KH))).width(Length::Units(FLAGS_WIDTH)),
        "KI" => Image::new(Handle::from_memory(Vec::from(KI))).width(Length::Units(FLAGS_WIDTH)),
        "KM" => Image::new(Handle::from_memory(Vec::from(KM))).width(Length::Units(FLAGS_WIDTH)),
        "KN" => Image::new(Handle::from_memory(Vec::from(KN))).width(Length::Units(FLAGS_WIDTH)),
        "KP" => Image::new(Handle::from_memory(Vec::from(KP))).width(Length::Units(FLAGS_WIDTH)),
        "KR" => Image::new(Handle::from_memory(Vec::from(KR))).width(Length::Units(FLAGS_WIDTH)),
        "KW" => Image::new(Handle::from_memory(Vec::from(KW))).width(Length::Units(FLAGS_WIDTH)),
        "KY" => Image::new(Handle::from_memory(Vec::from(KY))).width(Length::Units(FLAGS_WIDTH)),
        "KZ" => Image::new(Handle::from_memory(Vec::from(KZ))).width(Length::Units(FLAGS_WIDTH)),
        "LA" => Image::new(Handle::from_memory(Vec::from(LA))).width(Length::Units(FLAGS_WIDTH)),
        "LB" => Image::new(Handle::from_memory(Vec::from(LB))).width(Length::Units(FLAGS_WIDTH)),
        "LC" => Image::new(Handle::from_memory(Vec::from(LC))).width(Length::Units(FLAGS_WIDTH)),
        "LI" => Image::new(Handle::from_memory(Vec::from(LI))).width(Length::Units(FLAGS_WIDTH)),
        "LK" => Image::new(Handle::from_memory(Vec::from(LK))).width(Length::Units(FLAGS_WIDTH)),
        "LR" => Image::new(Handle::from_memory(Vec::from(LR))).width(Length::Units(FLAGS_WIDTH)),
        "LS" => Image::new(Handle::from_memory(Vec::from(LS))).width(Length::Units(FLAGS_WIDTH)),
        "LT" => Image::new(Handle::from_memory(Vec::from(LT))).width(Length::Units(FLAGS_WIDTH)),
        "LU" => Image::new(Handle::from_memory(Vec::from(LU))).width(Length::Units(FLAGS_WIDTH)),
        "LV" => Image::new(Handle::from_memory(Vec::from(LV))).width(Length::Units(FLAGS_WIDTH)),
        "LY" => Image::new(Handle::from_memory(Vec::from(LY))).width(Length::Units(FLAGS_WIDTH)),
        "MA" => Image::new(Handle::from_memory(Vec::from(MA))).width(Length::Units(FLAGS_WIDTH)),
        "MC" => Image::new(Handle::from_memory(Vec::from(MC))).width(Length::Units(FLAGS_WIDTH)),
        "MD" => Image::new(Handle::from_memory(Vec::from(MD))).width(Length::Units(FLAGS_WIDTH)),
        "ME" => Image::new(Handle::from_memory(Vec::from(ME))).width(Length::Units(FLAGS_WIDTH)),
        "MF" => Image::new(Handle::from_memory(Vec::from(MF))).width(Length::Units(FLAGS_WIDTH)),
        "MG" => Image::new(Handle::from_memory(Vec::from(MG))).width(Length::Units(FLAGS_WIDTH)),
        "MH" => Image::new(Handle::from_memory(Vec::from(MH))).width(Length::Units(FLAGS_WIDTH)),
        "MK" => Image::new(Handle::from_memory(Vec::from(MK))).width(Length::Units(FLAGS_WIDTH)),
        "ML" => Image::new(Handle::from_memory(Vec::from(ML))).width(Length::Units(FLAGS_WIDTH)),
        "MM" => Image::new(Handle::from_memory(Vec::from(MM))).width(Length::Units(FLAGS_WIDTH)),
        "MN" => Image::new(Handle::from_memory(Vec::from(MN))).width(Length::Units(FLAGS_WIDTH)),
        "MO" => Image::new(Handle::from_memory(Vec::from(MO))).width(Length::Units(FLAGS_WIDTH)),
        "MP" => Image::new(Handle::from_memory(Vec::from(MP))).width(Length::Units(FLAGS_WIDTH)),
        "MQ" => Image::new(Handle::from_memory(Vec::from(MQ))).width(Length::Units(FLAGS_WIDTH)),
        "MR" => Image::new(Handle::from_memory(Vec::from(MR))).width(Length::Units(FLAGS_WIDTH)),
        "MS" => Image::new(Handle::from_memory(Vec::from(MS))).width(Length::Units(FLAGS_WIDTH)),
        "MT" => Image::new(Handle::from_memory(Vec::from(MT))).width(Length::Units(FLAGS_WIDTH)),
        "MU" => Image::new(Handle::from_memory(Vec::from(MU))).width(Length::Units(FLAGS_WIDTH)),
        "MV" => Image::new(Handle::from_memory(Vec::from(MV))).width(Length::Units(FLAGS_WIDTH)),
        "MW" => Image::new(Handle::from_memory(Vec::from(MW))).width(Length::Units(FLAGS_WIDTH)),
        "MX" => Image::new(Handle::from_memory(Vec::from(MX))).width(Length::Units(FLAGS_WIDTH)),
        "MY" => Image::new(Handle::from_memory(Vec::from(MY))).width(Length::Units(FLAGS_WIDTH)),
        "MZ" => Image::new(Handle::from_memory(Vec::from(MZ))).width(Length::Units(FLAGS_WIDTH)),
        "NA" => Image::new(Handle::from_memory(Vec::from(NA))).width(Length::Units(FLAGS_WIDTH)),
        "NC" => Image::new(Handle::from_memory(Vec::from(NC))).width(Length::Units(FLAGS_WIDTH)),
        "NE" => Image::new(Handle::from_memory(Vec::from(NE))).width(Length::Units(FLAGS_WIDTH)),
        "NF" => Image::new(Handle::from_memory(Vec::from(NF))).width(Length::Units(FLAGS_WIDTH)),
        "NG" => Image::new(Handle::from_memory(Vec::from(NG))).width(Length::Units(FLAGS_WIDTH)),
        "NI" => Image::new(Handle::from_memory(Vec::from(NI))).width(Length::Units(FLAGS_WIDTH)),
        "NL" => Image::new(Handle::from_memory(Vec::from(NL))).width(Length::Units(FLAGS_WIDTH)),
        "NO" => Image::new(Handle::from_memory(Vec::from(NO))).width(Length::Units(FLAGS_WIDTH)),
        "NP" => Image::new(Handle::from_memory(Vec::from(NP))).width(Length::Units(FLAGS_WIDTH)),
        "NR" => Image::new(Handle::from_memory(Vec::from(NR))).width(Length::Units(FLAGS_WIDTH)),
        "NU" => Image::new(Handle::from_memory(Vec::from(NU))).width(Length::Units(FLAGS_WIDTH)),
        "NZ" => Image::new(Handle::from_memory(Vec::from(NZ))).width(Length::Units(FLAGS_WIDTH)),
        "OM" => Image::new(Handle::from_memory(Vec::from(OM))).width(Length::Units(FLAGS_WIDTH)),
        "PA" => Image::new(Handle::from_memory(Vec::from(PA))).width(Length::Units(FLAGS_WIDTH)),
        "PE" => Image::new(Handle::from_memory(Vec::from(PE))).width(Length::Units(FLAGS_WIDTH)),
        "PF" => Image::new(Handle::from_memory(Vec::from(PF))).width(Length::Units(FLAGS_WIDTH)),
        "PG" => Image::new(Handle::from_memory(Vec::from(PG))).width(Length::Units(FLAGS_WIDTH)),
        "PH" => Image::new(Handle::from_memory(Vec::from(PH))).width(Length::Units(FLAGS_WIDTH)),
        "PK" => Image::new(Handle::from_memory(Vec::from(PK))).width(Length::Units(FLAGS_WIDTH)),
        "PL" => Image::new(Handle::from_memory(Vec::from(PL))).width(Length::Units(FLAGS_WIDTH)),
        "PM" => Image::new(Handle::from_memory(Vec::from(PM))).width(Length::Units(FLAGS_WIDTH)),
        "PN" => Image::new(Handle::from_memory(Vec::from(PN))).width(Length::Units(FLAGS_WIDTH)),
        "PR" => Image::new(Handle::from_memory(Vec::from(PR))).width(Length::Units(FLAGS_WIDTH)),
        "PS" => Image::new(Handle::from_memory(Vec::from(PS))).width(Length::Units(FLAGS_WIDTH)),
        "PT" => Image::new(Handle::from_memory(Vec::from(PT))).width(Length::Units(FLAGS_WIDTH)),
        "PW" => Image::new(Handle::from_memory(Vec::from(PW))).width(Length::Units(FLAGS_WIDTH)),
        "PY" => Image::new(Handle::from_memory(Vec::from(PY))).width(Length::Units(FLAGS_WIDTH)),
        "QA" => Image::new(Handle::from_memory(Vec::from(QA))).width(Length::Units(FLAGS_WIDTH)),
        "RE" => Image::new(Handle::from_memory(Vec::from(RE))).width(Length::Units(FLAGS_WIDTH)),
        "RO" => Image::new(Handle::from_memory(Vec::from(RO))).width(Length::Units(FLAGS_WIDTH)),
        "RS" => Image::new(Handle::from_memory(Vec::from(RS))).width(Length::Units(FLAGS_WIDTH)),
        "RU" => Image::new(Handle::from_memory(Vec::from(RU))).width(Length::Units(FLAGS_WIDTH)),
        "RW" => Image::new(Handle::from_memory(Vec::from(RW))).width(Length::Units(FLAGS_WIDTH)),
        "SA" => Image::new(Handle::from_memory(Vec::from(SA))).width(Length::Units(FLAGS_WIDTH)),
        "SB" => Image::new(Handle::from_memory(Vec::from(SB))).width(Length::Units(FLAGS_WIDTH)),
        "SC" => Image::new(Handle::from_memory(Vec::from(SC))).width(Length::Units(FLAGS_WIDTH)),
        "SD" => Image::new(Handle::from_memory(Vec::from(SD))).width(Length::Units(FLAGS_WIDTH)),
        "SE" => Image::new(Handle::from_memory(Vec::from(SE))).width(Length::Units(FLAGS_WIDTH)),
        "SG" => Image::new(Handle::from_memory(Vec::from(SG))).width(Length::Units(FLAGS_WIDTH)),
        "SH" => Image::new(Handle::from_memory(Vec::from(SH))).width(Length::Units(FLAGS_WIDTH)),
        "SI" => Image::new(Handle::from_memory(Vec::from(SI))).width(Length::Units(FLAGS_WIDTH)),
        "SJ" => Image::new(Handle::from_memory(Vec::from(SJ))).width(Length::Units(FLAGS_WIDTH)),
        "SK" => Image::new(Handle::from_memory(Vec::from(SK))).width(Length::Units(FLAGS_WIDTH)),
        "SL" => Image::new(Handle::from_memory(Vec::from(SL))).width(Length::Units(FLAGS_WIDTH)),
        "SM" => Image::new(Handle::from_memory(Vec::from(SM))).width(Length::Units(FLAGS_WIDTH)),
        "SN" => Image::new(Handle::from_memory(Vec::from(SN))).width(Length::Units(FLAGS_WIDTH)),
        "SO" => Image::new(Handle::from_memory(Vec::from(SO))).width(Length::Units(FLAGS_WIDTH)),
        "SR" => Image::new(Handle::from_memory(Vec::from(SR))).width(Length::Units(FLAGS_WIDTH)),
        "SS" => Image::new(Handle::from_memory(Vec::from(SS))).width(Length::Units(FLAGS_WIDTH)),
        "ST" => Image::new(Handle::from_memory(Vec::from(ST))).width(Length::Units(FLAGS_WIDTH)),
        "SV" => Image::new(Handle::from_memory(Vec::from(SV))).width(Length::Units(FLAGS_WIDTH)),
        "SX" => Image::new(Handle::from_memory(Vec::from(SX))).width(Length::Units(FLAGS_WIDTH)),
        "SY" => Image::new(Handle::from_memory(Vec::from(SY))).width(Length::Units(FLAGS_WIDTH)),
        "SZ" => Image::new(Handle::from_memory(Vec::from(SZ))).width(Length::Units(FLAGS_WIDTH)),
        "TC" => Image::new(Handle::from_memory(Vec::from(TC))).width(Length::Units(FLAGS_WIDTH)),
        "TD" => Image::new(Handle::from_memory(Vec::from(TD))).width(Length::Units(FLAGS_WIDTH)),
        "TF" => Image::new(Handle::from_memory(Vec::from(TF))).width(Length::Units(FLAGS_WIDTH)),
        "TG" => Image::new(Handle::from_memory(Vec::from(TG))).width(Length::Units(FLAGS_WIDTH)),
        "TH" => Image::new(Handle::from_memory(Vec::from(TH))).width(Length::Units(FLAGS_WIDTH)),
        "TJ" => Image::new(Handle::from_memory(Vec::from(TJ))).width(Length::Units(FLAGS_WIDTH)),
        "TK" => Image::new(Handle::from_memory(Vec::from(TK))).width(Length::Units(FLAGS_WIDTH)),
        "TL" => Image::new(Handle::from_memory(Vec::from(TL))).width(Length::Units(FLAGS_WIDTH)),
        "TM" => Image::new(Handle::from_memory(Vec::from(TM))).width(Length::Units(FLAGS_WIDTH)),
        "TN" => Image::new(Handle::from_memory(Vec::from(TN))).width(Length::Units(FLAGS_WIDTH)),
        "TO" => Image::new(Handle::from_memory(Vec::from(TO))).width(Length::Units(FLAGS_WIDTH)),
        "TR" => Image::new(Handle::from_memory(Vec::from(TR))).width(Length::Units(FLAGS_WIDTH)),
        "TT" => Image::new(Handle::from_memory(Vec::from(TT))).width(Length::Units(FLAGS_WIDTH)),
        "TV" => Image::new(Handle::from_memory(Vec::from(TV))).width(Length::Units(FLAGS_WIDTH)),
        "TW" => Image::new(Handle::from_memory(Vec::from(TW))).width(Length::Units(FLAGS_WIDTH)),
        "TZ" => Image::new(Handle::from_memory(Vec::from(TZ))).width(Length::Units(FLAGS_WIDTH)),
        "UA" => Image::new(Handle::from_memory(Vec::from(UA))).width(Length::Units(FLAGS_WIDTH)),
        "UG" => Image::new(Handle::from_memory(Vec::from(UG))).width(Length::Units(FLAGS_WIDTH)),
        "UM" => Image::new(Handle::from_memory(Vec::from(UM))).width(Length::Units(FLAGS_WIDTH)),
        "US" => Image::new(Handle::from_memory(Vec::from(US))).width(Length::Units(FLAGS_WIDTH)),
        "UY" => Image::new(Handle::from_memory(Vec::from(UY))).width(Length::Units(FLAGS_WIDTH)),
        "UZ" => Image::new(Handle::from_memory(Vec::from(UZ))).width(Length::Units(FLAGS_WIDTH)),
        "VA" => Image::new(Handle::from_memory(Vec::from(VA))).width(Length::Units(FLAGS_WIDTH)),
        "VC" => Image::new(Handle::from_memory(Vec::from(VC))).width(Length::Units(FLAGS_WIDTH)),
        "VE" => Image::new(Handle::from_memory(Vec::from(VE))).width(Length::Units(FLAGS_WIDTH)),
        "VG" => Image::new(Handle::from_memory(Vec::from(VG))).width(Length::Units(FLAGS_WIDTH)),
        "VI" => Image::new(Handle::from_memory(Vec::from(VI))).width(Length::Units(FLAGS_WIDTH)),
        "VN" => Image::new(Handle::from_memory(Vec::from(VN))).width(Length::Units(FLAGS_WIDTH)),
        "VU" => Image::new(Handle::from_memory(Vec::from(VU))).width(Length::Units(FLAGS_WIDTH)),
        "WF" => Image::new(Handle::from_memory(Vec::from(WF))).width(Length::Units(FLAGS_WIDTH)),
        "WS" => Image::new(Handle::from_memory(Vec::from(WS))).width(Length::Units(FLAGS_WIDTH)),
        "YE" => Image::new(Handle::from_memory(Vec::from(YE))).width(Length::Units(FLAGS_WIDTH)),
        "YT" => Image::new(Handle::from_memory(Vec::from(YT))).width(Length::Units(FLAGS_WIDTH)),
        "ZA" => Image::new(Handle::from_memory(Vec::from(ZA))).width(Length::Units(FLAGS_WIDTH)),
        "ZM" => Image::new(Handle::from_memory(Vec::from(ZM))).width(Length::Units(FLAGS_WIDTH)),
        "ZW" => Image::new(Handle::from_memory(Vec::from(ZW))).width(Length::Units(FLAGS_WIDTH)),
        _ => Image::new(Handle::from_memory(Vec::from(UNKNOWN))).width(Length::Units(15)),
    }
}
