#[cfg(windows)]
extern crate winres;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

use once_cell::sync::Lazy;
use rustrict::{Censor, Trie, Type};

include!("./src/networking/types/service_query.rs");
include!("./src/networking/types/protocol.rs");

const WINDOWS_ICON_PATH: &str = "./resources/packaging/windows/graphics/sniffnet.ico";
const SERVICES_LIST_PATH: &str = "./services.txt";

fn main() {
    println!("cargo:rerun-if-changed={WINDOWS_ICON_PATH}");
    println!("cargo:rerun-if-changed={SERVICES_LIST_PATH}");

    set_icon();
    build_services_phf();
}

fn set_icon() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon(WINDOWS_ICON_PATH);
        res.compile().unwrap();
    }
}

fn build_services_phf() {
    let out_path = Path::new(&env::var("OUT_DIR").unwrap()).join("services.rs");
    let mut output = BufWriter::new(File::create(out_path).unwrap());

    let mut services_map = phf_codegen::Map::new();

    let input = BufReader::new(File::open(SERVICES_LIST_PATH).unwrap());
    let mut num_entries = 0;
    for line_res in input.lines() {
        // we want to panic if one of the lines is err...
        let line = line_res.unwrap();
        let mut parts = line.split('\t');
        // we want to panic if one of the service names is invalid
        let val = get_valid_service_fmt_const(parts.next().unwrap());
        // we want to panic if port is not a u16, or protocol is not TCP or UDP
        let key = get_valid_service_query(parts.next().unwrap());
        assert!(parts.next().is_none());
        services_map.entry(key, &val);
        num_entries += 1;
    }
    assert_eq!(num_entries, 12078);

    writeln!(
        &mut output,
        "#[allow(clippy::unreadable_literal)]\n\
        static SERVICES: phf::Map<ServiceQuery, Service> = {};",
        services_map.build()
    )
    .unwrap();
}

fn get_valid_service_fmt_const(s: &str) -> String {
    match s.trim() {
        invalid
            if ["", "unknown", "-"].contains(&invalid)
                || !invalid.is_ascii()
                || invalid.starts_with('#')
                || invalid.contains(' ')
                || invalid.contains('?') =>
        {
            panic!("Invalid service name found: {invalid}")
        }
        inappropriate
            if Censor::from_str(inappropriate)
                .with_trie(&SAFE_WORDS_FOR_SERVICE_NAME)
                .analyze()
                .is(Type::INAPPROPRIATE) =>
        {
            panic!("Inappropriate service name found: {inappropriate}")
        }
        name => format!("Service::Name(\"{name}\")"),
    }
}

fn get_valid_service_query(s: &str) -> ServiceQuery {
    let mut parts = s.split('/');
    let port = parts.next().unwrap().parse::<u16>().unwrap();
    let protocol_str = parts.next().unwrap();
    let protocol = match protocol_str {
        "tcp" => Protocol::TCP,
        "udp" => Protocol::UDP,
        invalid => panic!("Invalid protocol found: {invalid}"),
    };
    assert!(parts.next().is_none());
    ServiceQuery(port, protocol)
}

pub static SAFE_WORDS_FOR_SERVICE_NAME: Lazy<Trie> = Lazy::new(|| {
    let mut safe_words = Trie::default();
    for word in [
        "npp",
        "emfis-cntl",
        "ardus-cntl",
        "pmip6-cntl",
        "mpp",
        "ipp",
        "vpp",
        "epp",
        "kink",
        "kvm-via-ip",
        "dpp",
        "slinkysearch",
        "alta-ana-lm",
        "vpps-qua",
        "vpps-via",
        "ibm-pps",
        "ppsms",
        "ppsuitemsg",
        "icpps",
        "rap-listen",
        "cadabra-lm",
        "pay-per-view",
        "sixtrak",
        "cvmon",
        "houdini-lm",
        "dic-aida",
        "p2pq",
        "bigbrother",
        "bintec-admin",
        "zymed-zpp",
        "cvmmon",
        "btpp2sectrans",
        "conclave-cpp",
        "btpp2audctr1",
        "tclprodebugger",
        "bintec-capi",
        "bintec-tapi",
        "dicom-iscl",
        "dicom-tls",
        "nmsigport",
        "ppp",
        "tl1-telnet",
        "opcon-xps",
        "netwatcher-mon",
        "netwatcher-db",
        "xnm-ssl",
        "edm-mgr-cntrl",
        "isoft-p2p",
        "must-p2p",
        "p2pgroup",
        "quasar-server",
        "int-rcv-cntrl",
        "faxstfx-port",
        "sunlps-http",
        "fagordnc",
        "p2pcommunity",
        "minger",
        "assuria-slm",
        "wcpp",
        "plcy-net-svcs",
        "assyst-dr",
        "mobile-p2p",
        "assuria-ins",
        "taep-as-svc",
        "nlg-data",
        "dj-ice",
        "x500ms",
        "X11:7",
        "p2p-sip",
        "p4p-portal",
        "bmc-perf-agent",
        "ntz-p2p-storage",
        "citrixupp",
        "freezexservice",
        "p2pevolvenet",
        "papachi-p2p-srv",
        "espeasy-p2p",
        "pim-port",
        "vp2p",
        "dicom",
        "icpp",
        "sauterdongle",
        "vocaltec-hos",
        "BackOrifice",
        "dhanalakshmi",
        "3gpp-w1ap",
        "pmsm-webrctl",
        "bif-p2p",
    ] {
        safe_words.set(word, Type::SAFE);
    }
    safe_words
});
