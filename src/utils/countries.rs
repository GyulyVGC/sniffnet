use iced::widget::{svg::Handle, Svg, Tooltip};
use iced::{Length, Renderer};
use iced_native::widget::tooltip::Position;
use maxminddb::{geoip2, MaxMindDBError, Reader};

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

pub fn get_country_code(address_to_lookup: &str, country_db_reader: &Reader<&[u8]>) -> String {
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

pub const FLAGS_WIDTH_SMALL: f32 = 20.0;
pub const FLAGS_WIDTH_BIG: f32 = 37.5;

pub const AD: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ad.svg");
pub const AE: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ae.svg");
pub const AF: &[u8] = include_bytes!("../../resources/countries_flags/4x3/af.svg");
pub const AG: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ag.svg");
pub const AI: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ai.svg");
pub const AL: &[u8] = include_bytes!("../../resources/countries_flags/4x3/al.svg");
pub const AM: &[u8] = include_bytes!("../../resources/countries_flags/4x3/am.svg");
pub const AO: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ao.svg");
pub const AQ: &[u8] = include_bytes!("../../resources/countries_flags/4x3/aq.svg");
pub const AR: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ar.svg");
pub const AS: &[u8] = include_bytes!("../../resources/countries_flags/4x3/as.svg");
pub const AT: &[u8] = include_bytes!("../../resources/countries_flags/4x3/at.svg");
pub const AU: &[u8] = include_bytes!("../../resources/countries_flags/4x3/au.svg");
pub const AW: &[u8] = include_bytes!("../../resources/countries_flags/4x3/aw.svg");
pub const AX: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ax.svg");
pub const AZ: &[u8] = include_bytes!("../../resources/countries_flags/4x3/az.svg");
pub const BA: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ba.svg");
pub const BB: &[u8] = include_bytes!("../../resources/countries_flags/4x3/bb.svg");
pub const BD: &[u8] = include_bytes!("../../resources/countries_flags/4x3/bd.svg");
pub const BE: &[u8] = include_bytes!("../../resources/countries_flags/4x3/be.svg");
pub const BF: &[u8] = include_bytes!("../../resources/countries_flags/4x3/bf.svg");
pub const BG: &[u8] = include_bytes!("../../resources/countries_flags/4x3/bg.svg");
pub const BH: &[u8] = include_bytes!("../../resources/countries_flags/4x3/bh.svg");
pub const BI: &[u8] = include_bytes!("../../resources/countries_flags/4x3/bi.svg");
pub const BJ: &[u8] = include_bytes!("../../resources/countries_flags/4x3/bj.svg");
pub const BL: &[u8] = include_bytes!("../../resources/countries_flags/4x3/bl.svg");
pub const BM: &[u8] = include_bytes!("../../resources/countries_flags/4x3/bm.svg");
pub const BN: &[u8] = include_bytes!("../../resources/countries_flags/4x3/bn.svg");
pub const BO: &[u8] = include_bytes!("../../resources/countries_flags/4x3/bo.svg");
pub const BQ: &[u8] = include_bytes!("../../resources/countries_flags/4x3/bq.svg");
pub const BR: &[u8] = include_bytes!("../../resources/countries_flags/4x3/br.svg");
pub const BS: &[u8] = include_bytes!("../../resources/countries_flags/4x3/bs.svg");
pub const BT: &[u8] = include_bytes!("../../resources/countries_flags/4x3/bt.svg");
pub const BV: &[u8] = include_bytes!("../../resources/countries_flags/4x3/bv.svg");
pub const BW: &[u8] = include_bytes!("../../resources/countries_flags/4x3/bw.svg");
pub const BY: &[u8] = include_bytes!("../../resources/countries_flags/4x3/by.svg");
pub const BZ: &[u8] = include_bytes!("../../resources/countries_flags/4x3/bz.svg");
pub const CA: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ca.svg");
pub const CC: &[u8] = include_bytes!("../../resources/countries_flags/4x3/cc.svg");
pub const CD: &[u8] = include_bytes!("../../resources/countries_flags/4x3/cd.svg");
pub const CF: &[u8] = include_bytes!("../../resources/countries_flags/4x3/cf.svg");
pub const CG: &[u8] = include_bytes!("../../resources/countries_flags/4x3/cg.svg");
pub const CH: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ch.svg");
pub const CI: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ci.svg");
pub const CK: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ck.svg");
pub const CL: &[u8] = include_bytes!("../../resources/countries_flags/4x3/cl.svg");
pub const CM: &[u8] = include_bytes!("../../resources/countries_flags/4x3/cm.svg");
pub const CN: &[u8] = include_bytes!("../../resources/countries_flags/4x3/cn.svg");
pub const CO: &[u8] = include_bytes!("../../resources/countries_flags/4x3/co.svg");
pub const CR: &[u8] = include_bytes!("../../resources/countries_flags/4x3/cr.svg");
pub const CU: &[u8] = include_bytes!("../../resources/countries_flags/4x3/cu.svg");
pub const CV: &[u8] = include_bytes!("../../resources/countries_flags/4x3/cv.svg");
pub const CW: &[u8] = include_bytes!("../../resources/countries_flags/4x3/cw.svg");
pub const CX: &[u8] = include_bytes!("../../resources/countries_flags/4x3/cx.svg");
pub const CY: &[u8] = include_bytes!("../../resources/countries_flags/4x3/cy.svg");
pub const CZ: &[u8] = include_bytes!("../../resources/countries_flags/4x3/cz.svg");
pub const DE: &[u8] = include_bytes!("../../resources/countries_flags/4x3/de.svg");
pub const DJ: &[u8] = include_bytes!("../../resources/countries_flags/4x3/dj.svg");
pub const DK: &[u8] = include_bytes!("../../resources/countries_flags/4x3/dk.svg");
pub const DM: &[u8] = include_bytes!("../../resources/countries_flags/4x3/dm.svg");
pub const DO: &[u8] = include_bytes!("../../resources/countries_flags/4x3/do.svg");
pub const DZ: &[u8] = include_bytes!("../../resources/countries_flags/4x3/dz.svg");
pub const EC: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ec.svg");
pub const EE: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ee.svg");
pub const EG: &[u8] = include_bytes!("../../resources/countries_flags/4x3/eg.svg");
pub const EH: &[u8] = include_bytes!("../../resources/countries_flags/4x3/eh.svg");
pub const ER: &[u8] = include_bytes!("../../resources/countries_flags/4x3/er.svg");
pub const ES: &[u8] = include_bytes!("../../resources/countries_flags/4x3/es.svg");
pub const ET: &[u8] = include_bytes!("../../resources/countries_flags/4x3/et.svg");
pub const FI: &[u8] = include_bytes!("../../resources/countries_flags/4x3/fi.svg");
pub const FJ: &[u8] = include_bytes!("../../resources/countries_flags/4x3/fj.svg");
pub const FK: &[u8] = include_bytes!("../../resources/countries_flags/4x3/fk.svg");
pub const FM: &[u8] = include_bytes!("../../resources/countries_flags/4x3/fm.svg");
pub const FO: &[u8] = include_bytes!("../../resources/countries_flags/4x3/fo.svg");
pub const FR: &[u8] = include_bytes!("../../resources/countries_flags/4x3/fr.svg");
pub const GA: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ga.svg");
pub const GB: &[u8] = include_bytes!("../../resources/countries_flags/4x3/gb.svg");
pub const GD: &[u8] = include_bytes!("../../resources/countries_flags/4x3/gd.svg");
pub const GE: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ge.svg");
pub const GF: &[u8] = include_bytes!("../../resources/countries_flags/4x3/gf.svg");
pub const GG: &[u8] = include_bytes!("../../resources/countries_flags/4x3/gg.svg");
pub const GH: &[u8] = include_bytes!("../../resources/countries_flags/4x3/gh.svg");
pub const GI: &[u8] = include_bytes!("../../resources/countries_flags/4x3/gi.svg");
pub const GL: &[u8] = include_bytes!("../../resources/countries_flags/4x3/gl.svg");
pub const GM: &[u8] = include_bytes!("../../resources/countries_flags/4x3/gm.svg");
pub const GN: &[u8] = include_bytes!("../../resources/countries_flags/4x3/gn.svg");
pub const GP: &[u8] = include_bytes!("../../resources/countries_flags/4x3/gp.svg");
pub const GQ: &[u8] = include_bytes!("../../resources/countries_flags/4x3/gq.svg");
pub const GR: &[u8] = include_bytes!("../../resources/countries_flags/4x3/gr.svg");
pub const GS: &[u8] = include_bytes!("../../resources/countries_flags/4x3/gs.svg");
pub const GT: &[u8] = include_bytes!("../../resources/countries_flags/4x3/gt.svg");
pub const GU: &[u8] = include_bytes!("../../resources/countries_flags/4x3/gu.svg");
pub const GW: &[u8] = include_bytes!("../../resources/countries_flags/4x3/gw.svg");
pub const GY: &[u8] = include_bytes!("../../resources/countries_flags/4x3/gy.svg");
pub const HK: &[u8] = include_bytes!("../../resources/countries_flags/4x3/hk.svg");
pub const HM: &[u8] = include_bytes!("../../resources/countries_flags/4x3/hm.svg");
pub const HN: &[u8] = include_bytes!("../../resources/countries_flags/4x3/hn.svg");
pub const HR: &[u8] = include_bytes!("../../resources/countries_flags/4x3/hr.svg");
pub const HT: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ht.svg");
pub const HU: &[u8] = include_bytes!("../../resources/countries_flags/4x3/hu.svg");
pub const ID: &[u8] = include_bytes!("../../resources/countries_flags/4x3/id.svg");
pub const IE: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ie.svg");
pub const IL: &[u8] = include_bytes!("../../resources/countries_flags/4x3/il.svg");
pub const IM: &[u8] = include_bytes!("../../resources/countries_flags/4x3/im.svg");
pub const IN: &[u8] = include_bytes!("../../resources/countries_flags/4x3/in.svg");
pub const IO: &[u8] = include_bytes!("../../resources/countries_flags/4x3/io.svg");
pub const IQ: &[u8] = include_bytes!("../../resources/countries_flags/4x3/iq.svg");
pub const IR: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ir.svg");
pub const IS: &[u8] = include_bytes!("../../resources/countries_flags/4x3/is.svg");
pub const IT: &[u8] = include_bytes!("../../resources/countries_flags/4x3/it.svg");
pub const JE: &[u8] = include_bytes!("../../resources/countries_flags/4x3/je.svg");
pub const JM: &[u8] = include_bytes!("../../resources/countries_flags/4x3/jm.svg");
pub const JO: &[u8] = include_bytes!("../../resources/countries_flags/4x3/jo.svg");
pub const JP: &[u8] = include_bytes!("../../resources/countries_flags/4x3/jp.svg");
pub const KE: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ke.svg");
pub const KG: &[u8] = include_bytes!("../../resources/countries_flags/4x3/kg.svg");
pub const KH: &[u8] = include_bytes!("../../resources/countries_flags/4x3/kh.svg");
pub const KI: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ki.svg");
pub const KM: &[u8] = include_bytes!("../../resources/countries_flags/4x3/km.svg");
pub const KN: &[u8] = include_bytes!("../../resources/countries_flags/4x3/kn.svg");
pub const KP: &[u8] = include_bytes!("../../resources/countries_flags/4x3/kp.svg");
pub const KR: &[u8] = include_bytes!("../../resources/countries_flags/4x3/kr.svg");
pub const KW: &[u8] = include_bytes!("../../resources/countries_flags/4x3/kw.svg");
pub const KY: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ky.svg");
pub const KZ: &[u8] = include_bytes!("../../resources/countries_flags/4x3/kz.svg");
pub const LA: &[u8] = include_bytes!("../../resources/countries_flags/4x3/la.svg");
pub const LB: &[u8] = include_bytes!("../../resources/countries_flags/4x3/lb.svg");
pub const LC: &[u8] = include_bytes!("../../resources/countries_flags/4x3/lc.svg");
pub const LI: &[u8] = include_bytes!("../../resources/countries_flags/4x3/li.svg");
pub const LK: &[u8] = include_bytes!("../../resources/countries_flags/4x3/lk.svg");
pub const LR: &[u8] = include_bytes!("../../resources/countries_flags/4x3/lr.svg");
pub const LS: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ls.svg");
pub const LT: &[u8] = include_bytes!("../../resources/countries_flags/4x3/lt.svg");
pub const LU: &[u8] = include_bytes!("../../resources/countries_flags/4x3/lu.svg");
pub const LV: &[u8] = include_bytes!("../../resources/countries_flags/4x3/lv.svg");
pub const LY: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ly.svg");
pub const MA: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ma.svg");
pub const MC: &[u8] = include_bytes!("../../resources/countries_flags/4x3/mc.svg");
pub const MD: &[u8] = include_bytes!("../../resources/countries_flags/4x3/md.svg");
pub const ME: &[u8] = include_bytes!("../../resources/countries_flags/4x3/me.svg");
pub const MF: &[u8] = include_bytes!("../../resources/countries_flags/4x3/mf.svg");
pub const MG: &[u8] = include_bytes!("../../resources/countries_flags/4x3/mg.svg");
pub const MH: &[u8] = include_bytes!("../../resources/countries_flags/4x3/mh.svg");
pub const MK: &[u8] = include_bytes!("../../resources/countries_flags/4x3/mk.svg");
pub const ML: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ml.svg");
pub const MM: &[u8] = include_bytes!("../../resources/countries_flags/4x3/mm.svg");
pub const MN: &[u8] = include_bytes!("../../resources/countries_flags/4x3/mn.svg");
pub const MO: &[u8] = include_bytes!("../../resources/countries_flags/4x3/mo.svg");
pub const MP: &[u8] = include_bytes!("../../resources/countries_flags/4x3/mp.svg");
pub const MQ: &[u8] = include_bytes!("../../resources/countries_flags/4x3/mq.svg");
pub const MR: &[u8] = include_bytes!("../../resources/countries_flags/4x3/mr.svg");
pub const MS: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ms.svg");
pub const MT: &[u8] = include_bytes!("../../resources/countries_flags/4x3/mt.svg");
pub const MU: &[u8] = include_bytes!("../../resources/countries_flags/4x3/mu.svg");
pub const MV: &[u8] = include_bytes!("../../resources/countries_flags/4x3/mv.svg");
pub const MW: &[u8] = include_bytes!("../../resources/countries_flags/4x3/mw.svg");
pub const MX: &[u8] = include_bytes!("../../resources/countries_flags/4x3/mx.svg");
pub const MY: &[u8] = include_bytes!("../../resources/countries_flags/4x3/my.svg");
pub const MZ: &[u8] = include_bytes!("../../resources/countries_flags/4x3/mz.svg");
pub const NA: &[u8] = include_bytes!("../../resources/countries_flags/4x3/na.svg");
pub const NC: &[u8] = include_bytes!("../../resources/countries_flags/4x3/nc.svg");
pub const NE: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ne.svg");
pub const NF: &[u8] = include_bytes!("../../resources/countries_flags/4x3/nf.svg");
pub const NG: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ng.svg");
pub const NI: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ni.svg");
pub const NL: &[u8] = include_bytes!("../../resources/countries_flags/4x3/nl.svg");
pub const NO: &[u8] = include_bytes!("../../resources/countries_flags/4x3/no.svg");
pub const NP: &[u8] = include_bytes!("../../resources/countries_flags/4x3/np.svg");
pub const NR: &[u8] = include_bytes!("../../resources/countries_flags/4x3/nr.svg");
pub const NU: &[u8] = include_bytes!("../../resources/countries_flags/4x3/nu.svg");
pub const NZ: &[u8] = include_bytes!("../../resources/countries_flags/4x3/nz.svg");
pub const OM: &[u8] = include_bytes!("../../resources/countries_flags/4x3/om.svg");
pub const PA: &[u8] = include_bytes!("../../resources/countries_flags/4x3/pa.svg");
pub const PE: &[u8] = include_bytes!("../../resources/countries_flags/4x3/pe.svg");
pub const PF: &[u8] = include_bytes!("../../resources/countries_flags/4x3/pf.svg");
pub const PG: &[u8] = include_bytes!("../../resources/countries_flags/4x3/pg.svg");
pub const PH: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ph.svg");
pub const PK: &[u8] = include_bytes!("../../resources/countries_flags/4x3/pk.svg");
pub const PL: &[u8] = include_bytes!("../../resources/countries_flags/4x3/pl.svg");
pub const PM: &[u8] = include_bytes!("../../resources/countries_flags/4x3/pm.svg");
pub const PN: &[u8] = include_bytes!("../../resources/countries_flags/4x3/pn.svg");
pub const PR: &[u8] = include_bytes!("../../resources/countries_flags/4x3/pr.svg");
pub const PS: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ps.svg");
pub const PT: &[u8] = include_bytes!("../../resources/countries_flags/4x3/pt.svg");
pub const PW: &[u8] = include_bytes!("../../resources/countries_flags/4x3/pw.svg");
pub const PY: &[u8] = include_bytes!("../../resources/countries_flags/4x3/py.svg");
pub const QA: &[u8] = include_bytes!("../../resources/countries_flags/4x3/qa.svg");
pub const RE: &[u8] = include_bytes!("../../resources/countries_flags/4x3/re.svg");
pub const RO: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ro.svg");
pub const RS: &[u8] = include_bytes!("../../resources/countries_flags/4x3/rs.svg");
pub const RU: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ru.svg");
pub const RW: &[u8] = include_bytes!("../../resources/countries_flags/4x3/rw.svg");
pub const SA: &[u8] = include_bytes!("../../resources/countries_flags/4x3/sa.svg");
pub const SB: &[u8] = include_bytes!("../../resources/countries_flags/4x3/sb.svg");
pub const SC: &[u8] = include_bytes!("../../resources/countries_flags/4x3/sc.svg");
pub const SD: &[u8] = include_bytes!("../../resources/countries_flags/4x3/sd.svg");
pub const SE: &[u8] = include_bytes!("../../resources/countries_flags/4x3/se.svg");
pub const SG: &[u8] = include_bytes!("../../resources/countries_flags/4x3/sg.svg");
pub const SH: &[u8] = include_bytes!("../../resources/countries_flags/4x3/sh.svg");
pub const SI: &[u8] = include_bytes!("../../resources/countries_flags/4x3/si.svg");
pub const SJ: &[u8] = include_bytes!("../../resources/countries_flags/4x3/sj.svg");
pub const SK: &[u8] = include_bytes!("../../resources/countries_flags/4x3/sk.svg");
pub const SL: &[u8] = include_bytes!("../../resources/countries_flags/4x3/sl.svg");
pub const SM: &[u8] = include_bytes!("../../resources/countries_flags/4x3/sm.svg");
pub const SN: &[u8] = include_bytes!("../../resources/countries_flags/4x3/sn.svg");
pub const SO: &[u8] = include_bytes!("../../resources/countries_flags/4x3/so.svg");
pub const SR: &[u8] = include_bytes!("../../resources/countries_flags/4x3/sr.svg");
pub const SS: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ss.svg");
pub const ST: &[u8] = include_bytes!("../../resources/countries_flags/4x3/st.svg");
pub const SV: &[u8] = include_bytes!("../../resources/countries_flags/4x3/sv.svg");
pub const SX: &[u8] = include_bytes!("../../resources/countries_flags/4x3/sx.svg");
pub const SY: &[u8] = include_bytes!("../../resources/countries_flags/4x3/sy.svg");
pub const SZ: &[u8] = include_bytes!("../../resources/countries_flags/4x3/sz.svg");
pub const TC: &[u8] = include_bytes!("../../resources/countries_flags/4x3/tc.svg");
pub const TD: &[u8] = include_bytes!("../../resources/countries_flags/4x3/td.svg");
pub const TF: &[u8] = include_bytes!("../../resources/countries_flags/4x3/tf.svg");
pub const TG: &[u8] = include_bytes!("../../resources/countries_flags/4x3/tg.svg");
pub const TH: &[u8] = include_bytes!("../../resources/countries_flags/4x3/th.svg");
pub const TJ: &[u8] = include_bytes!("../../resources/countries_flags/4x3/tj.svg");
pub const TK: &[u8] = include_bytes!("../../resources/countries_flags/4x3/tk.svg");
pub const TL: &[u8] = include_bytes!("../../resources/countries_flags/4x3/tl.svg");
pub const TM: &[u8] = include_bytes!("../../resources/countries_flags/4x3/tm.svg");
pub const TN: &[u8] = include_bytes!("../../resources/countries_flags/4x3/tn.svg");
pub const TO: &[u8] = include_bytes!("../../resources/countries_flags/4x3/to.svg");
pub const TR: &[u8] = include_bytes!("../../resources/countries_flags/4x3/tr.svg");
pub const TT: &[u8] = include_bytes!("../../resources/countries_flags/4x3/tt.svg");
pub const TV: &[u8] = include_bytes!("../../resources/countries_flags/4x3/tv.svg");
pub const TW: &[u8] = include_bytes!("../../resources/countries_flags/4x3/tw.svg");
pub const TZ: &[u8] = include_bytes!("../../resources/countries_flags/4x3/tz.svg");
pub const UA: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ua.svg");
pub const UG: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ug.svg");
pub const UM: &[u8] = include_bytes!("../../resources/countries_flags/4x3/um.svg");
pub const US: &[u8] = include_bytes!("../../resources/countries_flags/4x3/usa.svg");
pub const UY: &[u8] = include_bytes!("../../resources/countries_flags/4x3/uy.svg");
pub const UZ: &[u8] = include_bytes!("../../resources/countries_flags/4x3/uz.svg");
pub const VA: &[u8] = include_bytes!("../../resources/countries_flags/4x3/va.svg");
pub const VC: &[u8] = include_bytes!("../../resources/countries_flags/4x3/vc.svg");
pub const VE: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ve.svg");
pub const VG: &[u8] = include_bytes!("../../resources/countries_flags/4x3/vg.svg");
pub const VI: &[u8] = include_bytes!("../../resources/countries_flags/4x3/vi.svg");
pub const VN: &[u8] = include_bytes!("../../resources/countries_flags/4x3/vn.svg");
pub const VU: &[u8] = include_bytes!("../../resources/countries_flags/4x3/vu.svg");
pub const WF: &[u8] = include_bytes!("../../resources/countries_flags/4x3/wf.svg");
pub const WS: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ws.svg");
pub const YE: &[u8] = include_bytes!("../../resources/countries_flags/4x3/ye.svg");
pub const YT: &[u8] = include_bytes!("../../resources/countries_flags/4x3/yt.svg");
pub const ZA: &[u8] = include_bytes!("../../resources/countries_flags/4x3/za.svg");
pub const ZM: &[u8] = include_bytes!("../../resources/countries_flags/4x3/zm.svg");
pub const ZW: &[u8] = include_bytes!("../../resources/countries_flags/4x3/zw.svg");
pub const HOME: &[u8] = include_bytes!("../../resources/countries_flags/4x3/zz-home.svg");
pub const MULTICAST: &[u8] = include_bytes!("../../resources/countries_flags/4x3/zz-multicast.svg");
pub const BROADCAST: &[u8] = include_bytes!("../../resources/countries_flags/4x3/zz-broadcast.svg");
pub const UNKNOWN: &[u8] = include_bytes!("../../resources/countries_flags/4x3/zz-unknown.svg");
pub const COMPUTER: &[u8] = include_bytes!("../../resources/countries_flags/4x3/zz-computer.svg");

pub fn get_flag_from_language_code(language: &str) -> Svg<Renderer> {
    Svg::new(Handle::from_memory(Vec::from(match language {
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
    .width(Length::Fixed(FLAGS_WIDTH_SMALL))
}

#[allow(clippy::too_many_lines)]
fn get_flag_from_country_code(
    country: &str,
    width: f32,
    is_local: bool,
    traffic_type: TrafficType,
    language: Language,
) -> (Svg<Renderer>, String) {
    let mut tooltip = country.to_string();
    let svg = Svg::new(Handle::from_memory(Vec::from(match country {
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
        _ => {
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
    country: &str,
    width: f32,
    is_local: bool,
    traffic_type: TrafficType,
    language: Language,
    style: StyleType,
) -> Tooltip<'static, Message> {
    let (content, tooltip) =
        get_flag_from_country_code(country, width, is_local, traffic_type, language);

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
