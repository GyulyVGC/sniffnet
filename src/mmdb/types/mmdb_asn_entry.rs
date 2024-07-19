use serde::Deserialize;

use crate::networking::types::asn::Asn;

#[derive(Deserialize)]
pub struct MmdbAsnEntry<'a> {
    #[serde(alias = "autonomous_system_number", alias = "asn")]
    code: MmdbAsnCode<'a>,
    #[serde(alias = "autonomous_system_organization", alias = "as_name")]
    name: Option<&'a str>,
}

impl MmdbAsnEntry<'_> {
    pub fn get_asn(&self) -> Asn {
        Asn {
            code: self.code.get_code(),
            name: self.name.unwrap_or_default().to_string(),
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum MmdbAsnCode<'a> {
    Int(Option<u32>),
    Str(Option<&'a str>),
}

impl MmdbAsnCode<'_> {
    fn get_code(&self) -> String {
        match self {
            Self::Int(Some(code)) => code.to_string(),
            Self::Str(Some(code)) => (*code).to_string(),
            _ => String::new(),
        }
    }
}
