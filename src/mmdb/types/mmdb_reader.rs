use maxminddb::Reader;

pub enum MmdbReader {
    Default(Reader<&'static [u8]>),
    Custom(Reader<Vec<u8>>),
}

impl MmdbReader {
    pub fn from(mmdb_path: &String, default_mmdb: &'static [u8]) -> MmdbReader {
        let default_reader = maxminddb::Reader::from_source(default_mmdb).unwrap();
        if mmdb_path.is_empty() {
            MmdbReader::Default(default_reader)
        } else {
            let custom_reader_result = maxminddb::Reader::open_readfile(mmdb_path);
            if let Ok(custom_reader) = custom_reader_result {
                return MmdbReader::Custom(custom_reader);
            }
            MmdbReader::Default(default_reader)
        }
    }
}
