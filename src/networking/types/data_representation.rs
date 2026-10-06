use crate::translations::translations::{
    bytes_exceeded_translation, bytes_translation, packets_exceeded_translation,
    packets_translation,
};
use crate::translations::translations_4::{bits_exceeded_translation, bits_translation};
use crate::translations::types::language::Language;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct DataRepr {
    pub(crate) data_unit: DataUnit,
    pub(crate) per_second: bool,
}

impl DataRepr {
    pub fn formatted_string(self, amount: u128) -> String {
        let suffix = if self.per_second { "/s" } else { "" };
        format!("{}{}", self.data_unit.formatted_string(amount), suffix)
    }

    #[cfg(test)]
    pub fn packets(per_second: bool) -> DataRepr {
        DataRepr {
            data_unit: DataUnit::Packets,
            per_second,
        }
    }

    #[cfg(test)]
    pub fn bytes(per_second: bool) -> DataRepr {
        DataRepr {
            data_unit: DataUnit::Bytes,
            per_second,
        }
    }

    #[cfg(test)]
    pub fn bits(per_second: bool) -> DataRepr {
        DataRepr {
            data_unit: DataUnit::Bits,
            per_second,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum DataUnit {
    Packets,
    #[default]
    Bytes,
    Bits,
}

impl DataUnit {
    pub(crate) const ALL: [DataUnit; 3] = [DataUnit::Bits, DataUnit::Bytes, DataUnit::Packets];

    pub fn get_label(&self, language: Language) -> &str {
        match self {
            DataUnit::Packets => packets_translation(language),
            DataUnit::Bytes => bytes_translation(language),
            DataUnit::Bits => bits_translation(language),
        }
    }

    /// Returns a String representing a quantity of traffic (packets / bytes / bits) with the proper multiple if applicable
    pub fn formatted_string(self, amount: u128) -> String {
        if self == DataUnit::Packets {
            return amount.to_string();
        }

        #[allow(clippy::cast_precision_loss)]
        let mut n = amount as f32;

        let byte_multiple = ByteMultiple::from_amount(amount);

        #[allow(clippy::cast_precision_loss)]
        let multiplier = byte_multiple.multiplier() as f32;
        n /= multiplier;
        if n > 999.0 && byte_multiple != ByteMultiple::PB {
            // this allows representing e.g. 999_999 as 999 KB instead of 1000 KB
            n = 999.0;
        }
        let precision = usize::from(byte_multiple != ByteMultiple::B && n <= 9.95);
        format!("{n:.precision$} {}", byte_multiple.pretty_print(self))
    }

    pub fn data_exceeded_translation(&self, language: Language) -> &str {
        match self {
            DataUnit::Packets => packets_exceeded_translation(language),
            DataUnit::Bytes => bytes_exceeded_translation(language),
            DataUnit::Bits => bits_exceeded_translation(language),
        }
    }
}

/// Represents a Byte or bit multiple for displaying values in a human-readable format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ByteMultiple {
    /// A Byte
    #[default]
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

    fn from_amount(bytes: u128) -> Self {
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

    pub fn get_char(self) -> &'static str {
        match self {
            Self::B => "",
            Self::KB => "K",
            Self::MB => "M",
            Self::GB => "G",
            Self::TB => "T",
            Self::PB => "P",
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

    fn pretty_print(self, repr: DataUnit) -> String {
        match repr {
            DataUnit::Packets => String::new(),
            DataUnit::Bytes => format!("{}B", self.get_char()),
            DataUnit::Bits => format!("{}b", self.get_char()),
        }
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
        assert_eq!(
            format!("{}", ByteMultiple::B.pretty_print(DataUnit::Packets)),
            ""
        );
        assert_eq!(
            format!("{}", ByteMultiple::B.pretty_print(DataUnit::Bytes)),
            "B"
        );
        assert_eq!(
            format!("{}", ByteMultiple::B.pretty_print(DataUnit::Bits)),
            "b"
        );
        assert_eq!(
            format!("{}", ByteMultiple::KB.pretty_print(DataUnit::Packets)),
            ""
        );
        assert_eq!(
            format!("{}", ByteMultiple::KB.pretty_print(DataUnit::Bytes)),
            "KB"
        );
        assert_eq!(
            format!("{}", ByteMultiple::KB.pretty_print(DataUnit::Bits)),
            "Kb"
        );
        assert_eq!(
            format!("{}", ByteMultiple::MB.pretty_print(DataUnit::Packets)),
            ""
        );
        assert_eq!(
            format!("{}", ByteMultiple::MB.pretty_print(DataUnit::Bytes)),
            "MB"
        );
        assert_eq!(
            format!("{}", ByteMultiple::MB.pretty_print(DataUnit::Bits)),
            "Mb"
        );
        assert_eq!(
            format!("{}", ByteMultiple::GB.pretty_print(DataUnit::Packets)),
            ""
        );
        assert_eq!(
            format!("{}", ByteMultiple::GB.pretty_print(DataUnit::Bytes)),
            "GB"
        );
        assert_eq!(
            format!("{}", ByteMultiple::GB.pretty_print(DataUnit::Bits)),
            "Gb"
        );
        assert_eq!(
            format!("{}", ByteMultiple::TB.pretty_print(DataUnit::Packets)),
            ""
        );
        assert_eq!(
            format!("{}", ByteMultiple::TB.pretty_print(DataUnit::Bytes)),
            "TB"
        );
        assert_eq!(
            format!("{}", ByteMultiple::TB.pretty_print(DataUnit::Bits)),
            "Tb"
        );
        assert_eq!(
            format!("{}", ByteMultiple::PB.pretty_print(DataUnit::Packets)),
            ""
        );
        assert_eq!(
            format!("{}", ByteMultiple::PB.pretty_print(DataUnit::Bytes)),
            "PB"
        );
        assert_eq!(
            format!("{}", ByteMultiple::PB.pretty_print(DataUnit::Bits)),
            "Pb"
        );
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
        assert_eq!(DataUnit::Packets.formatted_string(u128::MIN), "0");
        assert_eq!(DataUnit::Bytes.formatted_string(u128::MIN), "0 B");
        assert_eq!(DataUnit::Bits.formatted_string(u128::MIN), "0 b");

        assert_eq!(DataUnit::Packets.formatted_string(1), "1");
        assert_eq!(DataUnit::Bytes.formatted_string(1), "1 B");
        assert_eq!(DataUnit::Bits.formatted_string(1), "1 b");

        assert_eq!(DataUnit::Packets.formatted_string(82), "82");
        assert_eq!(DataUnit::Bytes.formatted_string(82), "82 B");
        assert_eq!(DataUnit::Bits.formatted_string(82), "82 b");

        assert_eq!(DataUnit::Packets.formatted_string(999), "999");
        assert_eq!(DataUnit::Bytes.formatted_string(999), "999 B");
        assert_eq!(DataUnit::Bits.formatted_string(999), "999 b");

        assert_eq!(DataUnit::Packets.formatted_string(1_000), "1000");
        assert_eq!(DataUnit::Bytes.formatted_string(1_000), "1.0 KB");
        assert_eq!(DataUnit::Bits.formatted_string(1_000), "1.0 Kb");

        assert_eq!(DataUnit::Packets.formatted_string(1_090), "1090");
        assert_eq!(DataUnit::Bytes.formatted_string(1_090), "1.1 KB");
        assert_eq!(DataUnit::Bits.formatted_string(1_090), "1.1 Kb");

        assert_eq!(DataUnit::Packets.formatted_string(1_990), "1990");
        assert_eq!(DataUnit::Bytes.formatted_string(1_990), "2.0 KB");
        assert_eq!(DataUnit::Bits.formatted_string(1_990), "2.0 Kb");

        assert_eq!(DataUnit::Packets.formatted_string(9_090), "9090");
        assert_eq!(DataUnit::Bytes.formatted_string(9_090), "9.1 KB");
        assert_eq!(DataUnit::Bits.formatted_string(9_090), "9.1 Kb");

        assert_eq!(DataUnit::Packets.formatted_string(9_950), "9950");
        assert_eq!(DataUnit::Bytes.formatted_string(9_950), "9.9 KB");
        assert_eq!(DataUnit::Bits.formatted_string(9_950), "9.9 Kb");

        assert_eq!(DataUnit::Packets.formatted_string(9_951), "9951");
        assert_eq!(DataUnit::Bytes.formatted_string(9_951), "10 KB");
        assert_eq!(DataUnit::Bits.formatted_string(9_951), "10 Kb");

        assert_eq!(DataUnit::Packets.formatted_string(71_324), "71324");
        assert_eq!(DataUnit::Bytes.formatted_string(71_324), "71 KB");
        assert_eq!(DataUnit::Bits.formatted_string(71_324), "71 Kb");

        assert_eq!(DataUnit::Packets.formatted_string(821_789), "821789");
        assert_eq!(DataUnit::Bytes.formatted_string(821_789), "822 KB");
        assert_eq!(DataUnit::Bits.formatted_string(821_789), "822 Kb");

        assert_eq!(DataUnit::Packets.formatted_string(999_499), "999499");
        assert_eq!(DataUnit::Bytes.formatted_string(999_499), "999 KB");
        assert_eq!(DataUnit::Bits.formatted_string(999_499), "999 Kb");

        assert_eq!(DataUnit::Packets.formatted_string(999_999), "999999");
        assert_eq!(DataUnit::Bytes.formatted_string(999_999), "999 KB");
        assert_eq!(DataUnit::Bits.formatted_string(999_999), "999 Kb");

        assert_eq!(DataUnit::Packets.formatted_string(1_000_000), "1000000");
        assert_eq!(DataUnit::Bytes.formatted_string(1_000_000), "1.0 MB");
        assert_eq!(DataUnit::Bits.formatted_string(1_000_000), "1.0 Mb");

        assert_eq!(DataUnit::Packets.formatted_string(3_790_000), "3790000");
        assert_eq!(DataUnit::Bytes.formatted_string(3_790_000), "3.8 MB");
        assert_eq!(DataUnit::Bits.formatted_string(3_790_000), "3.8 Mb");

        assert_eq!(DataUnit::Packets.formatted_string(9_950_000), "9950000");
        assert_eq!(DataUnit::Bytes.formatted_string(9_950_000), "9.9 MB");
        assert_eq!(DataUnit::Bits.formatted_string(9_950_000), "9.9 Mb");

        assert_eq!(DataUnit::Packets.formatted_string(9_951_000), "9951000");
        assert_eq!(DataUnit::Bytes.formatted_string(9_951_000), "10 MB");
        assert_eq!(DataUnit::Bits.formatted_string(9_951_000), "10 Mb");

        assert_eq!(DataUnit::Packets.formatted_string(49_499_000), "49499000");
        assert_eq!(DataUnit::Bytes.formatted_string(49_499_000), "49 MB");
        assert_eq!(DataUnit::Bits.formatted_string(49_499_000), "49 Mb");

        assert_eq!(DataUnit::Packets.formatted_string(49_500_000), "49500000");
        assert_eq!(DataUnit::Bytes.formatted_string(49_500_000), "50 MB");
        assert_eq!(DataUnit::Bits.formatted_string(49_500_000), "50 Mb");

        assert_eq!(DataUnit::Packets.formatted_string(670_900_000), "670900000");
        assert_eq!(DataUnit::Bytes.formatted_string(670_900_000), "671 MB");
        assert_eq!(DataUnit::Bits.formatted_string(670_900_000), "671 Mb");

        assert_eq!(DataUnit::Packets.formatted_string(998_199_999), "998199999");
        assert_eq!(DataUnit::Bytes.formatted_string(998_199_999), "998 MB");
        assert_eq!(DataUnit::Bits.formatted_string(998_199_999), "998 Mb");

        assert_eq!(DataUnit::Packets.formatted_string(999_999_999), "999999999");
        assert_eq!(DataUnit::Bytes.formatted_string(999_999_999), "999 MB");
        assert_eq!(DataUnit::Bits.formatted_string(999_999_999), "999 Mb");

        assert_eq!(
            DataUnit::Packets.formatted_string(1_000_000_000),
            "1000000000"
        );
        assert_eq!(DataUnit::Bytes.formatted_string(1_000_000_000), "1.0 GB");
        assert_eq!(DataUnit::Bits.formatted_string(1_000_000_000), "1.0 Gb");

        assert_eq!(
            DataUnit::Packets.formatted_string(7_770_000_000),
            "7770000000"
        );
        assert_eq!(DataUnit::Bytes.formatted_string(7_770_000_000), "7.8 GB");
        assert_eq!(DataUnit::Bits.formatted_string(7_770_000_000), "7.8 Gb");

        assert_eq!(
            DataUnit::Packets.formatted_string(9_950_000_000),
            "9950000000"
        );
        assert_eq!(DataUnit::Bytes.formatted_string(9_950_000_000), "9.9 GB");
        assert_eq!(DataUnit::Bits.formatted_string(9_950_000_000), "9.9 Gb");

        assert_eq!(
            DataUnit::Packets.formatted_string(9_951_000_000),
            "9951000000"
        );
        assert_eq!(DataUnit::Bytes.formatted_string(9_951_000_000), "10 GB");
        assert_eq!(DataUnit::Bits.formatted_string(9_951_000_000), "10 Gb");

        assert_eq!(
            DataUnit::Packets.formatted_string(19_951_000_000),
            "19951000000"
        );
        assert_eq!(DataUnit::Bytes.formatted_string(19_951_000_000), "20 GB");
        assert_eq!(DataUnit::Bits.formatted_string(19_951_000_000), "20 Gb");

        assert_eq!(
            DataUnit::Packets.formatted_string(399_951_000_000),
            "399951000000"
        );
        assert_eq!(DataUnit::Bytes.formatted_string(399_951_000_000), "400 GB");
        assert_eq!(DataUnit::Bits.formatted_string(399_951_000_000), "400 Gb");

        assert_eq!(
            DataUnit::Packets.formatted_string(999_999_999_999),
            "999999999999"
        );
        assert_eq!(DataUnit::Bytes.formatted_string(999_999_999_999), "999 GB");
        assert_eq!(DataUnit::Bits.formatted_string(999_999_999_999), "999 Gb");

        assert_eq!(
            DataUnit::Packets.formatted_string(1_000_000_000_000),
            "1000000000000"
        );
        assert_eq!(
            DataUnit::Bytes.formatted_string(1_000_000_000_000),
            "1.0 TB"
        );
        assert_eq!(DataUnit::Bits.formatted_string(1_000_000_000_000), "1.0 Tb");

        assert_eq!(
            DataUnit::Packets.formatted_string(9_950_000_000_000),
            "9950000000000"
        );
        assert_eq!(
            DataUnit::Bytes.formatted_string(9_950_000_000_000),
            "9.9 TB"
        );
        assert_eq!(DataUnit::Bits.formatted_string(9_950_000_000_000), "9.9 Tb");

        assert_eq!(
            DataUnit::Packets.formatted_string(9_951_000_000_000),
            "9951000000000"
        );
        assert_eq!(DataUnit::Bytes.formatted_string(9_951_000_000_000), "10 TB");
        assert_eq!(DataUnit::Bits.formatted_string(9_951_000_000_000), "10 Tb");

        assert_eq!(
            DataUnit::Packets.formatted_string(999_950_000_000_000),
            "999950000000000"
        );
        assert_eq!(
            DataUnit::Bytes.formatted_string(999_950_000_000_000),
            "999 TB"
        );
        assert_eq!(
            DataUnit::Bits.formatted_string(999_950_000_000_000),
            "999 Tb"
        );

        assert_eq!(
            DataUnit::Packets.formatted_string(999_999_999_999_999),
            "999999999999999"
        );
        assert_eq!(
            DataUnit::Bytes.formatted_string(999_999_999_999_999),
            "999 TB"
        );
        assert_eq!(
            DataUnit::Bits.formatted_string(999_999_999_999_999),
            "999 Tb"
        );

        assert_eq!(
            DataUnit::Packets.formatted_string(1_000_000_000_000_000),
            "1000000000000000"
        );
        assert_eq!(
            DataUnit::Bytes.formatted_string(1_000_000_000_000_000),
            "1.0 PB"
        );
        assert_eq!(
            DataUnit::Bits.formatted_string(1_000_000_000_000_000),
            "1.0 Pb"
        );

        assert_eq!(
            DataUnit::Packets.formatted_string(1_000_000_000_000_000_0),
            "10000000000000000"
        );
        assert_eq!(
            DataUnit::Bytes.formatted_string(1_000_000_000_000_000_0),
            "10 PB"
        );
        assert_eq!(
            DataUnit::Bits.formatted_string(1_000_000_000_000_000_0),
            "10 Pb"
        );
        assert_eq!(
            DataUnit::Packets.formatted_string(999_999_999_000_000_000),
            "999999999000000000"
        );
        assert_eq!(
            DataUnit::Bytes.formatted_string(999_999_999_000_000_000),
            "1000 PB"
        );
        assert_eq!(
            DataUnit::Bits.formatted_string(999_999_999_000_000_000),
            "1000 Pb"
        );

        assert_eq!(
            DataUnit::Packets.formatted_string(u128::MAX / 2),
            "170141183460469231731687303715884105727"
        );
        assert_eq!(
            DataUnit::Bytes.formatted_string(u128::MAX / 2),
            "170141184077655307190272 PB"
        );
        assert_eq!(
            DataUnit::Bits.formatted_string(u128::MAX / 2),
            "170141184077655307190272 Pb"
        );

        assert_eq!(
            DataUnit::Packets.formatted_string(u128::MAX),
            "340282366920938463463374607431768211455"
        );
        assert_eq!(DataUnit::Bytes.formatted_string(u128::MAX), "inf PB");
        assert_eq!(DataUnit::Bits.formatted_string(u128::MAX), "inf Pb");
    }

    #[test]
    fn test_data_repr_formatted_string() {
        let data_repr = DataRepr {
            data_unit: DataUnit::Bytes,
            per_second: false,
        };
        assert_eq!(data_repr.formatted_string(1_000), "1.0 KB");

        let data_repr_per_second = DataRepr {
            data_unit: DataUnit::Bits,
            per_second: true,
        };
        assert_eq!(data_repr_per_second.formatted_string(1_000_000), "1.0 Mb/s");

        let data_repr_per_second = DataRepr {
            data_unit: DataUnit::Packets,
            per_second: true,
        };
        assert_eq!(
            data_repr_per_second.formatted_string(1_000_000),
            "1000000/s"
        );

        let data_repr_per_second = DataRepr {
            data_unit: DataUnit::Packets,
            per_second: false,
        };
        assert_eq!(data_repr_per_second.formatted_string(1_000_000), "1000000");
    }
}
