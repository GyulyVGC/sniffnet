use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct PortCollection {
    pub(crate) ports: Vec<u16>,
    pub(crate) ranges: Vec<RangeInclusive<u16>>,
}

impl PortCollection {
    const SEPARATOR: char = ',';
    const RANGE_SEPARATOR: char = '-';

    pub const PLACEHOLDER_STR: &'static str = "0-65535";

    pub(crate) fn new(str: &str) -> Option<Self> {
        let str = str.replace(' ', "");

        if str.is_empty() {
            return Some(Self::default());
        }

        let mut ports = Vec::new();
        let mut ranges = Vec::new();

        let objects: Vec<&str> = str.split(Self::SEPARATOR).collect();
        for object in objects {
            if object.contains(Self::RANGE_SEPARATOR) {
                // port range
                let mut subparts = object.split(Self::RANGE_SEPARATOR);
                let (lower_str, upper_str) =
                    (subparts.next().unwrap_or(""), subparts.next().unwrap_or(""));
                let lower_port_res = u16::from_str(lower_str);
                let upper_port_res = u16::from_str(upper_str);
                if lower_port_res.is_ok() && upper_port_res.is_ok() {
                    let lower_port = lower_port_res.unwrap();
                    let upper_port = upper_port_res.unwrap();
                    let range = RangeInclusive::new(lower_port, upper_port);
                    if range.is_empty() {
                        return None;
                    }
                    ranges.push(range);
                } else {
                    return None;
                }
            } else {
                // individual port
                if let Ok(port) = u16::from_str(object) {
                    ports.push(port);
                } else {
                    return None;
                }
            }
        }

        Some(Self { ports, ranges })
    }

    pub(crate) fn contains(&self, port: Option<u16>) -> bool {
        // ignore port filter in case of ICMP
        if port.is_none() {
            return true;
        }

        for range in &self.ranges {
            if range.contains(&port.unwrap()) {
                return true;
            }
        }
        self.ports.contains(&port.unwrap())
    }
}

impl Default for PortCollection {
    fn default() -> Self {
        PortCollection {
            ports: vec![],
            ranges: vec![RangeInclusive::new(u16::MIN, u16::MAX)],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::networking::types::port_collection::PortCollection;

    #[test]
    fn test_default_collection_contains_everything() {
        let collection = PortCollection::default();
        assert!(collection.contains(Some(0)));
        assert!(collection.contains(Some(1)));
        assert!(collection.contains(Some(2)));
        assert!(collection.contains(Some(80)));
        assert!(collection.contains(Some(8080)));
        assert!(collection.contains(Some(55333)));
        assert!(collection.contains(Some(65535)));
    }

    #[test]
    fn test_new_port_collections() {
        assert_eq!(
            PortCollection::new("0").unwrap(),
            PortCollection {
                ports: vec![0],
                ranges: vec![]
            }
        );

        assert_eq!(
            PortCollection::new(" 0 ").unwrap(),
            PortCollection {
                ports: vec![0],
                ranges: vec![]
            }
        );

        assert_eq!(
            PortCollection::new("1,2,3,4,999").unwrap(),
            PortCollection {
                ports: vec![1, 2, 3, 4, 999],
                ranges: vec![]
            }
        );

        assert_eq!(
            PortCollection::new("1, 2, 3, 4, 900-999").unwrap(),
            PortCollection {
                ports: vec![1, 2, 3, 4],
                ranges: vec![900..=999]
            }
        );

        assert_eq!(
            PortCollection::new("1 - 999").unwrap(),
            PortCollection {
                ports: vec![],
                ranges: vec![1..=999]
            }
        );

        assert_eq!(
            PortCollection::new("   1,2,10-20,3,4,  999-1200    ").unwrap(),
            PortCollection {
                ports: vec![1, 2, 3, 4],
                ranges: vec![10..=20, 999..=1200]
            }
        );
    }

    #[test]
    fn test_new_port_collections_invalid() {
        assert_eq!(PortCollection::new("1,2,10-20,3,4,-1200"), None);

        assert_eq!(PortCollection::new("1,2,10-20,3,4,999:1200"), None);

        assert_eq!(PortCollection::new("1,2,10-20,3,4,999-1200,"), None);

        assert_eq!(PortCollection::new("999-1"), None);

        assert_eq!(PortCollection::new("1:999"), None);
    }

    #[test]
    fn test_port_collection_contains() {
        let collection = PortCollection::new("1,2,25-30,55,101-117").unwrap();
        assert!(collection.contains(Some(1)));
        assert!(collection.contains(Some(2)));
        assert!(collection.contains(Some(25)));
        assert!(collection.contains(Some(27)));
        assert!(collection.contains(Some(30)));
        assert!(collection.contains(Some(55)));
        assert!(collection.contains(Some(101)));
        assert!(collection.contains(Some(109)));
        assert!(collection.contains(Some(117)));
        assert!(!collection.contains(Some(4)));
        assert!(!collection.contains(Some(24)));
        assert!(!collection.contains(Some(31)));
        assert!(!collection.contains(Some(100)));
        assert!(!collection.contains(Some(118)));
        assert!(!collection.contains(Some(8080)));
    }
}
