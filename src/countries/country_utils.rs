use iced::widget::Tooltip;
use iced::{Length, Renderer};
use iced_native::svg::Handle;
use iced_native::widget::tooltip::Position;
use iced_native::widget::Svg;
use maxminddb::{geoip2, MaxMindDBError, Reader};

use crate::countries::flags_pictures::{
    AD, AE, AF, AG, AI, AL, AM, AO, AQ, AR, AS, AT, AU, AW, AX, AZ, BA, BB, BD, BE, BF, BG, BH, BI,
    BJ, BL, BM, BN, BO, BQ, BR, BROADCAST, BS, BT, BV, BW, BY, BZ, CA, CC, CD, CF, CG, CH, CI, CK,
    CL, CM, CN, CO, COMPUTER, CR, CU, CV, CW, CX, CY, CZ, DE, DJ, DK, DM, DO, DZ, EC, EE, EG, EH,
    ER, ES, ET, FI, FJ, FK, FLAGS_WIDTH_BIG, FLAGS_WIDTH_SMALL, FM, FO, FR, GA, GB, GD, GE, GF, GG,
    GH, GI, GL, GM, GN, GP, GQ, GR, GS, GT, GU, GW, GY, HK, HM, HN, HOME, HR, HT, HU, ID, IE, IL,
    IM, IN, IO, IQ, IR, IS, IT, JE, JM, JO, JP, KE, KG, KH, KI, KM, KN, KP, KR, KW, KY, KZ, LA, LB,
    LC, LI, LK, LR, LS, LT, LU, LV, LY, MA, MC, MD, ME, MF, MG, MH, MK, ML, MM, MN, MO, MP, MQ, MR,
    MS, MT, MU, MULTICAST, MV, MW, MX, MY, MZ, NA, NC, NE, NF, NG, NI, NL, NO, NP, NR, NU, NZ, OM,
    PA, PE, PF, PG, PH, PK, PL, PM, PN, PR, PS, PT, PW, PY, QA, RE, RO, RS, RU, RW, SA, SB, SC, SD,
    SE, SG, SH, SI, SJ, SK, SL, SM, SN, SO, SR, SS, ST, SV, SX, SY, SZ, TC, TD, TF, TG, TH, TJ, TK,
    TL, TM, TN, TO, TR, TT, TV, TW, TZ, UA, UG, UM, UNKNOWN, US, UY, UZ, VA, VC, VE, VG, VI, VN,
    VU, WF, WS, YE, YT, ZA, ZM, ZW,
};
use crate::countries::types::country::Country;
use crate::gui::styles::style_constants::get_font;
use crate::gui::styles::types::element_type::ElementType;
use crate::gui::styles::types::style_tuple::StyleTuple;
use crate::gui::types::message::Message;
use crate::networking::types::traffic_type::TrafficType;
use crate::translations::translations_2::{
    local_translation, unknown_translation, your_network_adapter_translation,
};
use crate::{Language, StyleType};

pub const COUNTRY_MMDB: &[u8] = include_bytes!("../../resources/DB/GeoLite2-Country.mmdb");

pub fn get_country(address_to_lookup: &str, country_db_reader: &Reader<&[u8]>) -> Country {
    let country_result: Result<geoip2::Country, MaxMindDBError> =
        country_db_reader.lookup(address_to_lookup.parse().unwrap());
    if let Ok(res1) = country_result {
        if let Some(res2) = res1.country {
            if let Some(res3) = res2.iso_code {
                return Country::from_str(res3);
            }
        }
    }
    Country::ZZ // unknown
}

#[allow(clippy::too_many_lines)]
fn get_flag_from_country(
    country: Country,
    width: f32,
    is_local: bool,
    traffic_type: TrafficType,
    language: Language,
) -> (Svg<Renderer>, String) {
    let mut tooltip = country.to_string();
    let svg = Svg::new(Handle::from_memory(Vec::from(match country {
        Country::AD => AD,
        Country::AE => AE,
        Country::AF => AF,
        Country::AG => AG,
        Country::AI => AI,
        Country::AL => AL,
        Country::AM => AM,
        Country::AO => AO,
        Country::AQ => AQ,
        Country::AR => AR,
        Country::AS => AS,
        Country::AT => AT,
        Country::AU => AU,
        Country::AW => AW,
        Country::AX => AX,
        Country::AZ => AZ,
        Country::BA => BA,
        Country::BB => BB,
        Country::BD => BD,
        Country::BE => BE,
        Country::BF => BF,
        Country::BG => BG,
        Country::BH => BH,
        Country::BI => BI,
        Country::BJ => BJ,
        Country::BL => BL,
        Country::BM => BM,
        Country::BN => BN,
        Country::BO => BO,
        Country::BQ => BQ,
        Country::BR => BR,
        Country::BS => BS,
        Country::BT => BT,
        Country::BV => BV,
        Country::BW => BW,
        Country::BY => BY,
        Country::BZ => BZ,
        Country::CA => CA,
        Country::CC => CC,
        Country::CD => CD,
        Country::CF => CF,
        Country::CG => CG,
        Country::CH => CH,
        Country::CI => CI,
        Country::CK => CK,
        Country::CL => CL,
        Country::CM => CM,
        Country::CN => CN,
        Country::CO => CO,
        Country::CR => CR,
        Country::CU => CU,
        Country::CV => CV,
        Country::CW => CW,
        Country::CX => CX,
        Country::CY => CY,
        Country::CZ => CZ,
        Country::DE => DE,
        Country::DJ => DJ,
        Country::DK => DK,
        Country::DM => DM,
        Country::DO => DO,
        Country::DZ => DZ,
        Country::EC => EC,
        Country::EE => EE,
        Country::EG => EG,
        Country::EH => EH,
        Country::ER => ER,
        Country::ES => ES,
        Country::ET => ET,
        Country::FI => FI,
        Country::FJ => FJ,
        Country::FK => FK,
        Country::FM => FM,
        Country::FO => FO,
        Country::FR => FR,
        Country::GA => GA,
        Country::GB => GB,
        Country::GD => GD,
        Country::GE => GE,
        Country::GF => GF,
        Country::GG => GG,
        Country::GH => GH,
        Country::GI => GI,
        Country::GL => GL,
        Country::GM => GM,
        Country::GN => GN,
        Country::GP => GP,
        Country::GQ => GQ,
        Country::GR => GR,
        Country::GS => GS,
        Country::GT => GT,
        Country::GU => GU,
        Country::GW => GW,
        Country::GY => GY,
        Country::HK => HK,
        Country::HM => HM,
        Country::HN => HN,
        Country::HR => HR,
        Country::HT => HT,
        Country::HU => HU,
        Country::ID => ID,
        Country::IE => IE,
        Country::IL => IL,
        Country::IM => IM,
        Country::IN => IN,
        Country::IO => IO,
        Country::IQ => IQ,
        Country::IR => IR,
        Country::IS => IS,
        Country::IT => IT,
        Country::JE => JE,
        Country::JM => JM,
        Country::JO => JO,
        Country::JP => JP,
        Country::KE => KE,
        Country::KG => KG,
        Country::KH => KH,
        Country::KI => KI,
        Country::KM => KM,
        Country::KN => KN,
        Country::KP => KP,
        Country::KR => KR,
        Country::KW => KW,
        Country::KY => KY,
        Country::KZ => KZ,
        Country::LA => LA,
        Country::LB => LB,
        Country::LC => LC,
        Country::LI => LI,
        Country::LK => LK,
        Country::LR => LR,
        Country::LS => LS,
        Country::LT => LT,
        Country::LU => LU,
        Country::LV => LV,
        Country::LY => LY,
        Country::MA => MA,
        Country::MC => MC,
        Country::MD => MD,
        Country::ME => ME,
        Country::MF => MF,
        Country::MG => MG,
        Country::MH => MH,
        Country::MK => MK,
        Country::ML => ML,
        Country::MM => MM,
        Country::MN => MN,
        Country::MO => MO,
        Country::MP => MP,
        Country::MQ => MQ,
        Country::MR => MR,
        Country::MS => MS,
        Country::MT => MT,
        Country::MU => MU,
        Country::MV => MV,
        Country::MW => MW,
        Country::MX => MX,
        Country::MY => MY,
        Country::MZ => MZ,
        Country::NA => NA,
        Country::NC => NC,
        Country::NE => NE,
        Country::NF => NF,
        Country::NG => NG,
        Country::NI => NI,
        Country::NL => NL,
        Country::NO => NO,
        Country::NP => NP,
        Country::NR => NR,
        Country::NU => NU,
        Country::NZ => NZ,
        Country::OM => OM,
        Country::PA => PA,
        Country::PE => PE,
        Country::PF => PF,
        Country::PG => PG,
        Country::PH => PH,
        Country::PK => PK,
        Country::PL => PL,
        Country::PM => PM,
        Country::PN => PN,
        Country::PR => PR,
        Country::PS => PS,
        Country::PT => PT,
        Country::PW => PW,
        Country::PY => PY,
        Country::QA => QA,
        Country::RE => RE,
        Country::RO => RO,
        Country::RS => RS,
        Country::RU => RU,
        Country::RW => RW,
        Country::SA => SA,
        Country::SB => SB,
        Country::SC => SC,
        Country::SD => SD,
        Country::SE => SE,
        Country::SG => SG,
        Country::SH => SH,
        Country::SI => SI,
        Country::SJ => SJ,
        Country::SK => SK,
        Country::SL => SL,
        Country::SM => SM,
        Country::SN => SN,
        Country::SO => SO,
        Country::SR => SR,
        Country::SS => SS,
        Country::ST => ST,
        Country::SV => SV,
        Country::SX => SX,
        Country::SY => SY,
        Country::SZ => SZ,
        Country::TC => TC,
        Country::TD => TD,
        Country::TF => TF,
        Country::TG => TG,
        Country::TH => TH,
        Country::TJ => TJ,
        Country::TK => TK,
        Country::TL => TL,
        Country::TM => TM,
        Country::TN => TN,
        Country::TO => TO,
        Country::TR => TR,
        Country::TT => TT,
        Country::TV => TV,
        Country::TW => TW,
        Country::TZ => TZ,
        Country::UA => UA,
        Country::UG => UG,
        Country::UM => UM,
        Country::US => US,
        Country::UY => UY,
        Country::UZ => UZ,
        Country::VA => VA,
        Country::VC => VC,
        Country::VE => VE,
        Country::VG => VG,
        Country::VI => VI,
        Country::VN => VN,
        Country::VU => VU,
        Country::WF => WF,
        Country::WS => WS,
        Country::YE => YE,
        Country::YT => YT,
        Country::ZA => ZA,
        Country::ZM => ZM,
        Country::ZW => ZW,
        Country::ZZ => {
            if is_local {
                tooltip = local_translation(language);
                HOME
            } else if traffic_type.eq(&TrafficType::Multicast) {
                tooltip = "Multicast".to_string();
                MULTICAST
            } else if traffic_type.eq(&TrafficType::Broadcast) {
                tooltip = "Broadcast".to_string();
                BROADCAST
            } else {
                tooltip = unknown_translation(language);
                UNKNOWN
            }
        }
    })))
    .width(Length::Fixed(width))
    .height(Length::Fixed(width * 0.75));

    (svg, tooltip)
}

pub fn get_flag_tooltip(
    country: Country,
    width: f32,
    is_local: bool,
    traffic_type: TrafficType,
    language: Language,
    style: StyleType,
) -> Tooltip<'static, Message> {
    let (content, tooltip) =
        get_flag_from_country(country, width, is_local, traffic_type, language);

    let mut tooltip = Tooltip::new(content, tooltip, Position::FollowCursor)
        .font(get_font(style))
        .snap_within_viewport(true)
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(style, ElementType::Tooltip),
        ));

    if width == FLAGS_WIDTH_SMALL {
        tooltip = tooltip.padding(3);
    }

    tooltip
}

pub fn get_computer_tooltip(
    is_my_address: bool,
    traffic_type: TrafficType,
    language: Language,
    style: StyleType,
) -> Tooltip<'static, Message> {
    let content = Svg::new(Handle::from_memory(Vec::from(
        match (is_my_address, traffic_type) {
            (true, _) => COMPUTER,
            (false, TrafficType::Multicast) => MULTICAST,
            (false, TrafficType::Broadcast) => BROADCAST,
            (false, TrafficType::Unicast) => UNKNOWN,
        },
    )))
    .width(Length::Fixed(FLAGS_WIDTH_BIG))
    .height(Length::Fixed(FLAGS_WIDTH_BIG * 0.75));

    let tooltip = match (is_my_address, traffic_type) {
        (true, _) => your_network_adapter_translation(language),
        (false, TrafficType::Multicast) => "Multicast".to_string(),
        (false, TrafficType::Broadcast) => "Broadcast".to_string(),
        (false, TrafficType::Unicast) => unknown_translation(language),
    };

    Tooltip::new(content, tooltip, Position::FollowCursor)
        .font(get_font(style))
        .snap_within_viewport(true)
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(style, ElementType::Tooltip),
        ))
}
