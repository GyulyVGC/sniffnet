#[cfg(windows)]
extern crate winres;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

fn main() {
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

    let input = BufReader::new(File::open("./services.txt").unwrap());
    for line in input.lines().flatten() {
        let mut parts = line.split('\t');
        let service = format!("\"{}\"", parts.next().unwrap());
        let key = parts.next().unwrap().to_uppercase();
        services_map.entry(key, &service);
    }

    writeln!(
        &mut file,
        "#[allow(clippy::unreadable_literal)]\n\
        static SERVICES: phf::Map<&'static str, &'static str> = {};",
        services_map.build()
    )
    .unwrap();
}
