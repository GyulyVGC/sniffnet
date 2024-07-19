use std::net::IpAddr;

use maxminddb::{MaxMindDBError, Reader};
use serde::Deserialize;

pub enum MmdbReader {
    Default(Reader<&'static [u8]>),
    Custom(Reader<Vec<u8>>),
}

impl MmdbReader {
    pub fn from(mmdb_path: &String, default_mmdb: &'static [u8]) -> MmdbReader {
        if let Ok(custom_reader) = maxminddb::Reader::open_readfile(mmdb_path) {
            return MmdbReader::Custom(custom_reader);
        }
        let default_reader = maxminddb::Reader::from_source(default_mmdb).unwrap();
        MmdbReader::Default(default_reader)
    }

    pub fn lookup<'a, T: Deserialize<'a>>(&'a self, ip: IpAddr) -> Result<T, MaxMindDBError> {
        match self {
            MmdbReader::Default(reader) => reader.lookup(ip),
            MmdbReader::Custom(reader) => reader.lookup(ip),
        }
    }
}
