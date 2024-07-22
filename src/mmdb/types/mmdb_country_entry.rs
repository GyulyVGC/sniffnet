use serde::Deserialize;

use crate::countries::types::country::Country;

#[derive(Deserialize)]
#[serde(transparent)]
pub struct MmdbCountryEntry<'a> {
    #[serde(borrow)]
    inner: MmdbCountryEntryInner<'a>,
}

impl MmdbCountryEntry<'_> {
    pub fn get_country(&self) -> Country {
        self.inner.get_country()
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum MmdbCountryEntryInner<'a> {
    #[serde(borrow)]
    Standard(StandardCountryEntry<'a>),
    #[serde(borrow)]
    Ipinfo(IpinfoCountryEntry<'a>),
}

impl MmdbCountryEntryInner<'_> {
    fn get_country(&self) -> Country {
        match self {
            Self::Standard(StandardCountryEntry {
                country: Some(StandardCountryEntryInner { iso_code: Some(c) }),
            })
            | Self::Ipinfo(IpinfoCountryEntry { country: Some(c) }) => Country::from_str(c),
            _ => Country::ZZ,
        }
    }
}

#[derive(Deserialize)]
struct StandardCountryEntry<'a> {
    #[serde(borrow)]
    country: Option<StandardCountryEntryInner<'a>>,
}

#[derive(Deserialize)]
struct StandardCountryEntryInner<'a> {
    iso_code: Option<&'a str>,
}

#[derive(Deserialize)]
struct IpinfoCountryEntry<'a> {
    country: Option<&'a str>,
}
