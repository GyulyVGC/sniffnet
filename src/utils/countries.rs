use iced::widget::{image::Handle, Image};
use iced::Length;
use maxminddb::{geoip2, MaxMindDBError, Reader};

use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::traffic_type::TrafficType;

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

pub const FLAGS_WIDTH: f32 = 15.0;

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

pub fn get_flag_from_language_code(language: &str) -> Image {
    Image::new(Handle::from_memory(Vec::from(match language {
        "ZH" => CN,
        "DE" => DE,
        "ES" => ES,
        "FR" => FR,
        "EN" => GB,
        "IT" => IT,
        "KO" => KR,
        "PL" => PL,
        "PT" => PT,
        "RO" => RO,
        "RU" => RU,
        "TR" => TR,
        "UK" => UA,
        "EL" => GR,
        "FA" => IR,
        _ => UNKNOWN,
    })))
    .width(Length::Fixed(FLAGS_WIDTH))
}

pub fn get_flag_from_country_code(country: &str) -> Image {
    #![allow(clippy::too_many_lines)]
    Image::new(Handle::from_memory(Vec::from(match country {
        "AD" => AD,
        "AE" => AE,
        "AF" => AF,
        "AG" => AG,
        "AI" => AI,
        "AL" => AL,
        "AM" => AM,
        "AO" => AO,
        "AQ" => AQ,
        "AR" => AR,
        "AS" => AS,
        "AT" => AT,
        "AU" => AU,
        "AW" => AW,
        "AX" => AX,
        "AZ" => AZ,
        "BA" => BA,
        "BB" => BB,
        "BD" => BD,
        "BE" => BE,
        "BF" => BF,
        "BG" => BG,
        "BH" => BH,
        "BI" => BI,
        "BJ" => BJ,
        "BL" => BL,
        "BM" => BM,
        "BN" => BN,
        "BO" => BO,
        "BQ" => BQ,
        "BR" => BR,
        "BS" => BS,
        "BT" => BT,
        "BV" => BV,
        "BW" => BW,
        "BY" => BY,
        "BZ" => BZ,
        "CA" => CA,
        "CC" => CC,
        "CD" => CD,
        "CF" => CF,
        "CG" => CG,
        "CH" => CH,
        "CI" => CI,
        "CK" => CK,
        "CL" => CL,
        "CM" => CM,
        "CN" => CN,
        "CO" => CO,
        "CR" => CR,
        "CU" => CU,
        "CV" => CV,
        "CW" => CW,
        "CX" => CX,
        "CY" => CY,
        "CZ" => CZ,
        "DE" => DE,
        "DJ" => DJ,
        "DK" => DK,
        "DM" => DM,
        "DO" => DO,
        "DZ" => DZ,
        "EC" => EC,
        "EE" => EE,
        "EG" => EG,
        "EH" => EH,
        "ER" => ER,
        "ES" => ES,
        "ET" => ET,
        "FI" => FI,
        "FJ" => FJ,
        "FK" => FK,
        "FM" => FM,
        "FO" => FO,
        "FR" => FR,
        "GA" => GA,
        "GB" => GB,
        "GD" => GD,
        "GE" => GE,
        "GF" => GF,
        "GG" => GG,
        "GH" => GH,
        "GI" => GI,
        "GL" => GL,
        "GM" => GM,
        "GN" => GN,
        "GP" => GP,
        "GQ" => GQ,
        "GR" => GR,
        "GS" => GS,
        "GT" => GT,
        "GU" => GU,
        "GW" => GW,
        "GY" => GY,
        "HK" => HK,
        "HM" => HM,
        "HN" => HN,
        "HR" => HR,
        "HT" => HT,
        "HU" => HU,
        "ID" => ID,
        "IE" => IE,
        "IL" => IL,
        "IM" => IM,
        "IN" => IN,
        "IO" => IO,
        "IQ" => IQ,
        "IR" => IR,
        "IS" => IS,
        "IT" => IT,
        "JE" => JE,
        "JM" => JM,
        "JO" => JO,
        "JP" => JP,
        "KE" => KE,
        "KG" => KG,
        "KH" => KH,
        "KI" => KI,
        "KM" => KM,
        "KN" => KN,
        "KP" => KP,
        "KR" => KR,
        "KW" => KW,
        "KY" => KY,
        "KZ" => KZ,
        "LA" => LA,
        "LB" => LB,
        "LC" => LC,
        "LI" => LI,
        "LK" => LK,
        "LR" => LR,
        "LS" => LS,
        "LT" => LT,
        "LU" => LU,
        "LV" => LV,
        "LY" => LY,
        "MA" => MA,
        "MC" => MC,
        "MD" => MD,
        "ME" => ME,
        "MF" => MF,
        "MG" => MG,
        "MH" => MH,
        "MK" => MK,
        "ML" => ML,
        "MM" => MM,
        "MN" => MN,
        "MO" => MO,
        "MP" => MP,
        "MQ" => MQ,
        "MR" => MR,
        "MS" => MS,
        "MT" => MT,
        "MU" => MU,
        "MV" => MV,
        "MW" => MW,
        "MX" => MX,
        "MY" => MY,
        "MZ" => MZ,
        "NA" => NA,
        "NC" => NC,
        "NE" => NE,
        "NF" => NF,
        "NG" => NG,
        "NI" => NI,
        "NL" => NL,
        "NO" => NO,
        "NP" => NP,
        "NR" => NR,
        "NU" => NU,
        "NZ" => NZ,
        "OM" => OM,
        "PA" => PA,
        "PE" => PE,
        "PF" => PF,
        "PG" => PG,
        "PH" => PH,
        "PK" => PK,
        "PL" => PL,
        "PM" => PM,
        "PN" => PN,
        "PR" => PR,
        "PS" => PS,
        "PT" => PT,
        "PW" => PW,
        "PY" => PY,
        "QA" => QA,
        "RE" => RE,
        "RO" => RO,
        "RS" => RS,
        "RU" => RU,
        "RW" => RW,
        "SA" => SA,
        "SB" => SB,
        "SC" => SC,
        "SD" => SD,
        "SE" => SE,
        "SG" => SG,
        "SH" => SH,
        "SI" => SI,
        "SJ" => SJ,
        "SK" => SK,
        "SL" => SL,
        "SM" => SM,
        "SN" => SN,
        "SO" => SO,
        "SR" => SR,
        "SS" => SS,
        "ST" => ST,
        "SV" => SV,
        "SX" => SX,
        "SY" => SY,
        "SZ" => SZ,
        "TC" => TC,
        "TD" => TD,
        "TF" => TF,
        "TG" => TG,
        "TH" => TH,
        "TJ" => TJ,
        "TK" => TK,
        "TL" => TL,
        "TM" => TM,
        "TN" => TN,
        "TO" => TO,
        "TR" => TR,
        "TT" => TT,
        "TV" => TV,
        "TW" => TW,
        "TZ" => TZ,
        "UA" => UA,
        "UG" => UG,
        "UM" => UM,
        "US" => US,
        "UY" => UY,
        "UZ" => UZ,
        "VA" => VA,
        "VC" => VC,
        "VE" => VE,
        "VG" => VG,
        "VI" => VI,
        "VN" => VN,
        "VU" => VU,
        "WF" => WF,
        "WS" => WS,
        "YE" => YE,
        "YT" => YT,
        "ZA" => ZA,
        "ZM" => ZM,
        "ZW" => ZW,
        _ => UNKNOWN,
    })))
    .width(Length::Fixed(FLAGS_WIDTH))
}
