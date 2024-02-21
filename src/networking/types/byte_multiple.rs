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
    const B_MUL: u64 = 1;
    const KB_MUL: u64 = 1_000;
    const MB_MUL: u64 = 1_000_000;
    const GB_MUL: u64 = 1_000_000_000;
    const TB_MUL: u64 = 1_000_000_000_000;
    const PB_MUL: u64 = 1_000_000_000_000_000;

    pub fn multiplier(self) -> u64 {
        match self {
            ByteMultiple::B => ByteMultiple::B_MUL,
            ByteMultiple::KB => ByteMultiple::KB_MUL,
            ByteMultiple::MB => ByteMultiple::MB_MUL,
            ByteMultiple::GB => ByteMultiple::GB_MUL,
            ByteMultiple::TB => ByteMultiple::TB_MUL,
            ByteMultiple::PB => ByteMultiple::PB_MUL,
        }
    }

    pub fn get_char(&self) -> &str {
        match self {
            ByteMultiple::B => "",
            ByteMultiple::KB => "K",
            ByteMultiple::MB => "M",
            ByteMultiple::GB => "G",
            ByteMultiple::TB => "T",
            ByteMultiple::PB => "P",
        }
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

        let byte_multiple = match bytes {
            x if (0..u128::from(ByteMultiple::KB_MUL)).contains(&x) => ByteMultiple::B,
            x if (u128::from(ByteMultiple::KB_MUL)..u128::from(ByteMultiple::MB_MUL))
                .contains(&x) =>
            {
                ByteMultiple::KB
            }
            x if (u128::from(ByteMultiple::MB_MUL)..u128::from(ByteMultiple::GB_MUL))
                .contains(&x) =>
            {
                ByteMultiple::MB
            }
            x if (u128::from(ByteMultiple::GB_MUL)..u128::from(ByteMultiple::TB_MUL))
                .contains(&x) =>
            {
                ByteMultiple::GB
            }
            x if (u128::from(ByteMultiple::TB_MUL)..u128::from(ByteMultiple::PB_MUL))
                .contains(&x) =>
            {
                ByteMultiple::TB
            }
            _ => ByteMultiple::PB,
        };

        #[allow(clippy::cast_precision_loss)]
        let multiplier = byte_multiple.multiplier() as f32;
        n /= multiplier;
        let precision = usize::from(byte_multiple != ByteMultiple::B && n <= 9.95);
        format!("{n:.precision$} {byte_multiple}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpret_suffix_correctly() {
        assert_eq!(from_char_to_multiple('B'), ByteMultiple::B);
        assert_eq!(from_char_to_multiple('k'), ByteMultiple::KB);
        assert_eq!(from_char_to_multiple('M'), ByteMultiple::MB);
        assert_eq!(from_char_to_multiple('g'), ByteMultiple::GB);
    }

    #[test]
    fn test_interpret_unknown_suffix_correctly() {
        assert_eq!(from_char_to_multiple('T'), ByteMultiple::B);
        assert_eq!(from_char_to_multiple('p'), ByteMultiple::B);
    }
}
