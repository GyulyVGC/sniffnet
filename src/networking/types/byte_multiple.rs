use std::fmt;

use serde::{Deserialize, Serialize};

/// Enum representing the possible observed values of IP protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ByteMultiple {
    /// A Byte
    B,
    /// 10^3 Bytes
    KB,
    /// 10^6 Bytes
    MB,
    /// 10^9 Bytes
    GB,
    /// 10^12 Bytes
    TB,
    /// 10^15 Bytes
    PB,
}

impl fmt::Display for ByteMultiple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl ByteMultiple {
    pub fn multiplier(self) -> u64 {
        match self {
            ByteMultiple::B => 1,
            ByteMultiple::KB => 1_000,
            ByteMultiple::MB => 1_000_000,
            ByteMultiple::GB => 1_000_000_000,
            ByteMultiple::TB => 1_000_000_000_000,
            ByteMultiple::PB => 1_000_000_000_000_000,
        }
    }

    fn from_num_bytes(bytes: u128) -> Self {
        match bytes {
            x if (u128::MIN..u128::from(ByteMultiple::KB.multiplier())).contains(&x) => {
                ByteMultiple::B
            }
            x if (u128::from(ByteMultiple::KB.multiplier())
                ..u128::from(ByteMultiple::MB.multiplier()))
                .contains(&x) =>
            {
                ByteMultiple::KB
            }
            x if (u128::from(ByteMultiple::MB.multiplier())
                ..u128::from(ByteMultiple::GB.multiplier()))
                .contains(&x) =>
            {
                ByteMultiple::MB
            }
            x if (u128::from(ByteMultiple::GB.multiplier())
                ..u128::from(ByteMultiple::TB.multiplier()))
                .contains(&x) =>
            {
                ByteMultiple::GB
            }
            x if (u128::from(ByteMultiple::TB.multiplier())
                ..u128::from(ByteMultiple::PB.multiplier()))
                .contains(&x) =>
            {
                ByteMultiple::TB
            }
            _ => ByteMultiple::PB,
        }
    }

    pub fn get_char(self) -> String {
        self.to_string()
            .strip_suffix('B')
            .unwrap_or_default()
            .to_owned()
    }

    pub fn from_char(ch: char) -> Self {
        match ch.to_ascii_uppercase() {
            'K' => ByteMultiple::KB,
            'M' => ByteMultiple::MB,
            'G' => ByteMultiple::GB,
            'T' => ByteMultiple::TB,
            'P' => ByteMultiple::PB,
            _ => ByteMultiple::B,
        }
    }

    /// Returns a String representing a quantity of bytes with its proper multiple (B, KB, MB, GB, TB)
    pub fn formatted_string(bytes: u128) -> String {
        #[allow(clippy::cast_precision_loss)]
        let mut n = bytes as f32;

        let byte_multiple = ByteMultiple::from_num_bytes(bytes);

        #[allow(clippy::cast_precision_loss)]
        let multiplier = byte_multiple.multiplier() as f32;
        n /= multiplier;
        if n > 999.0 && byte_multiple != ByteMultiple::PB {
            // this allows representing e.g. 999_999 as 999 KB instead of 1000 KB
            n = 999.0;
        }
        let precision = usize::from(byte_multiple != ByteMultiple::B && n <= 9.95);
        format!("{n:.precision$} {byte_multiple}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpret_suffix_correctly() {
        assert_eq!(ByteMultiple::from_char('B'), ByteMultiple::B);
        assert_eq!(ByteMultiple::from_char('k'), ByteMultiple::KB);
        assert_eq!(ByteMultiple::from_char('M'), ByteMultiple::MB);
        assert_eq!(ByteMultiple::from_char('g'), ByteMultiple::GB);
        assert_eq!(ByteMultiple::from_char('t'), ByteMultiple::TB);
        assert_eq!(ByteMultiple::from_char('P'), ByteMultiple::PB);
    }

    #[test]
    fn test_interpret_unknown_suffix_correctly() {
        assert_eq!(ByteMultiple::from_char('E'), ByteMultiple::B);
        assert_eq!(ByteMultiple::from_char('y'), ByteMultiple::B);
    }

    #[test]
    fn test_byte_multiple_display() {
        assert_eq!(format!("{}", ByteMultiple::B), "B");
        assert_eq!(format!("{}", ByteMultiple::KB), "KB");
        assert_eq!(format!("{}", ByteMultiple::MB), "MB");
        assert_eq!(format!("{}", ByteMultiple::GB), "GB");
        assert_eq!(format!("{}", ByteMultiple::TB), "TB");
        assert_eq!(format!("{}", ByteMultiple::PB), "PB");
    }

    #[test]
    fn test_byte_multiple_get_char() {
        assert_eq!(ByteMultiple::B.get_char(), "");
        assert_eq!(ByteMultiple::KB.get_char(), "K");
        assert_eq!(ByteMultiple::MB.get_char(), "M");
        assert_eq!(ByteMultiple::GB.get_char(), "G");
        assert_eq!(ByteMultiple::TB.get_char(), "T");
        assert_eq!(ByteMultiple::PB.get_char(), "P");
    }

    #[test]
    fn test_byte_multiple_multiplier() {
        assert_eq!(ByteMultiple::B.multiplier(), 1);
        assert_eq!(ByteMultiple::KB.multiplier(), 1_000);
        assert_eq!(ByteMultiple::MB.multiplier(), 1_000_000);
        assert_eq!(ByteMultiple::GB.multiplier(), 1_000_000_000);
        assert_eq!(ByteMultiple::TB.multiplier(), 1_000_000_000_000);
        assert_eq!(ByteMultiple::PB.multiplier(), 1_000_000_000_000_000);
    }

    #[test]
    fn test_byte_multiple_formatted_string() {
        assert_eq!(ByteMultiple::formatted_string(u128::MIN), "0 B");
        assert_eq!(ByteMultiple::formatted_string(1), "1 B");
        assert_eq!(ByteMultiple::formatted_string(82), "82 B");
        assert_eq!(ByteMultiple::formatted_string(999), "999 B");
        assert_eq!(ByteMultiple::formatted_string(1_000), "1.0 KB");
        assert_eq!(ByteMultiple::formatted_string(1_090), "1.1 KB");
        assert_eq!(ByteMultiple::formatted_string(1_990), "2.0 KB");
        assert_eq!(ByteMultiple::formatted_string(9_090), "9.1 KB");
        assert_eq!(ByteMultiple::formatted_string(9_950), "9.9 KB");
        assert_eq!(ByteMultiple::formatted_string(9_951), "10 KB");
        assert_eq!(ByteMultiple::formatted_string(71_324), "71 KB");
        assert_eq!(ByteMultiple::formatted_string(821_789), "822 KB");
        assert_eq!(ByteMultiple::formatted_string(999_499), "999 KB");
        assert_eq!(ByteMultiple::formatted_string(999_999), "999 KB");
        assert_eq!(ByteMultiple::formatted_string(1_000_000), "1.0 MB");
        assert_eq!(ByteMultiple::formatted_string(3_790_000), "3.8 MB");
        assert_eq!(ByteMultiple::formatted_string(9_950_000), "9.9 MB");
        assert_eq!(ByteMultiple::formatted_string(9_951_000), "10 MB");
        assert_eq!(ByteMultiple::formatted_string(49_499_000), "49 MB");
        assert_eq!(ByteMultiple::formatted_string(49_500_000), "50 MB");
        assert_eq!(ByteMultiple::formatted_string(670_900_000), "671 MB");
        assert_eq!(ByteMultiple::formatted_string(998_199_999), "998 MB");
        assert_eq!(ByteMultiple::formatted_string(999_999_999), "999 MB");
        assert_eq!(ByteMultiple::formatted_string(1_000_000_000), "1.0 GB");
        assert_eq!(ByteMultiple::formatted_string(7_770_000_000), "7.8 GB");
        assert_eq!(ByteMultiple::formatted_string(9_950_000_000), "9.9 GB");
        assert_eq!(ByteMultiple::formatted_string(9_951_000_000), "10 GB");
        assert_eq!(ByteMultiple::formatted_string(19_951_000_000), "20 GB");
        assert_eq!(ByteMultiple::formatted_string(399_951_000_000), "400 GB");
        assert_eq!(ByteMultiple::formatted_string(999_999_999_999), "999 GB");
        assert_eq!(ByteMultiple::formatted_string(1_000_000_000_000), "1.0 TB");
        assert_eq!(ByteMultiple::formatted_string(9_950_000_000_000), "9.9 TB");
        assert_eq!(ByteMultiple::formatted_string(9_951_000_000_000), "10 TB");
        assert_eq!(
            ByteMultiple::formatted_string(999_950_000_000_000),
            "999 TB"
        );
        assert_eq!(
            ByteMultiple::formatted_string(999_999_999_999_999),
            "999 TB"
        );
        assert_eq!(
            ByteMultiple::formatted_string(1_000_000_000_000_000),
            "1.0 PB"
        );
        assert_eq!(
            ByteMultiple::formatted_string(1_000_000_000_000_000_0),
            "10 PB"
        );
        assert_eq!(
            ByteMultiple::formatted_string(999_999_999_000_000_000),
            "1000 PB"
        );
        assert_eq!(
            ByteMultiple::formatted_string(1_000_000_000_000_000_000_000),
            "1000000 PB"
        );
        assert_eq!(
            ByteMultiple::formatted_string(u128::MAX / 2),
            "170141184077655307190272 PB"
        );
        assert_eq!(ByteMultiple::formatted_string(u128::MAX), "inf PB");
    }
}
