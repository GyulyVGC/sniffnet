#[cfg(windows)]
extern crate winres;

use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

include!("./src/networking/types/service_query.rs");
include!("./src/networking/types/protocol.rs");

const SERVICES_LIST_PATH: &str = "./services.txt";

fn main() {
    println!("cargo:rerun-if-changed={SERVICES_LIST_PATH}");

    set_icon();
    build_services_phf();
}

fn set_icon() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("resources/packaging/windows/graphics/sniffnet.ico");
        res.compile().unwrap();
    }
}

fn build_services_phf() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("services.rs");
    let mut file = BufWriter::new(File::create(path).unwrap());

    let mut services_map = phf_codegen::Map::new();

    let input = BufReader::new(File::open(SERVICES_LIST_PATH).unwrap());
    let mut num_entries = 0;
    let mut distinct_services = HashSet::new();
    for line_res in input.lines() {
        // we want to panic if one of the lines is err...
        let line = line_res.unwrap();
        let mut parts = line.split('\t');
        // just to count and verify number of distinct services
        let service_str = parts.next().unwrap();
        distinct_services.insert(service_str.to_string());
        // we want to panic if one of the service names is invalid
        let val = get_valid_service_fmt_const(service_str);
        // we want to panic if port is not a u16, or protocol is not TCP or UDP
        let key = get_valid_service_query(parts.next().unwrap());
        assert!(parts.next().is_none());
        services_map.entry(key, &val);
        num_entries += 1;
    }
    assert_eq!(num_entries, 12066);
    assert_eq!(distinct_services.len(), 6438);

    writeln!(
        &mut file,
        "#[allow(clippy::unreadable_literal)]\n\
        static SERVICES: phf::Map<ServiceQuery, Service> = {};",
        services_map.build()
    )
    .unwrap();
}

fn get_valid_service_fmt_const(s: &str) -> String {
    match s.trim() {
        invalid
            if ["", "unknown", "?", "-"].contains(&invalid)
                || !invalid.is_ascii()
                || invalid.starts_with('#')
                || invalid.contains(' ') =>
        {
            panic!("Invalid service name found: {invalid}")
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
