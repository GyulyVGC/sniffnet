use std::fmt;

use serde::{Deserialize, Serialize};

/// Enum representing the possible observed values of IP protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ByteMultiple {
    /// A Byte
    B,
    /// A thousand Bytes
    KB,
    /// A million Bytes
    MB,
    /// A billion Bytes
    GB,
}

impl fmt::Display for ByteMultiple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl ByteMultiple {
    pub fn get_multiplier(self) -> u64 {
        match self {
            ByteMultiple::B => 1,
            ByteMultiple::KB => 1_000,
            ByteMultiple::MB => 1_000_000,
            ByteMultiple::GB => 1_000_000_000,
        }
    }

    pub fn get_char(&self) -> &str {
        match self {
            ByteMultiple::B => "",
            ByteMultiple::KB => "K",
            ByteMultiple::MB => "M",
            ByteMultiple::GB => "G",
        }
    }
}

pub fn from_char_to_multiple(ch: char) -> ByteMultiple {
    match ch.to_ascii_uppercase() {
        'K' => ByteMultiple::KB,
        'M' => ByteMultiple::MB,
        'G' => ByteMultiple::GB,
        _ => ByteMultiple::B,
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
