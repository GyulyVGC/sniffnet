use std::net::IpAddr;
use std::sync::Arc;

use crate::location;
use crate::utils::error_logger::{ErrorLogger, Location};
use maxminddb::{MaxMindDbError, Reader};
use serde::Deserialize;

#[derive(Clone)]
pub struct MmdbReaders {
    pub country: Arc<MmdbReader>,
    pub asn: Arc<MmdbReader>,
}

pub enum MmdbReader {
    Default(Reader<&'static [u8]>),
    Custom(Reader<Vec<u8>>),
    Empty,
}

impl MmdbReader {
    pub fn from(mmdb_path: &String, default_mmdb: &'static [u8]) -> MmdbReader {
        if let Ok(custom_reader) = maxminddb::Reader::open_readfile(mmdb_path) {
            return MmdbReader::Custom(custom_reader);
        }
        match maxminddb::Reader::from_source(default_mmdb) {
            Ok(default_reader) => MmdbReader::Default(default_reader),
            err_res => {
                let _ = err_res.log_err(location!());
                MmdbReader::Empty
            }
        }
    }

    pub fn lookup<'a, T: Deserialize<'a>>(
        &'a self,
        ip: IpAddr,
    ) -> Result<Option<T>, MaxMindDbError> {
        match self {
            MmdbReader::Default(reader) => reader.lookup(ip),
            MmdbReader::Custom(reader) => reader.lookup(ip),
            MmdbReader::Empty => Ok(None),
        }
    }
}
