use std::fmt;
use std::str::{from_utf8, FromStr};

use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum ChunkTypeError {
    #[snafu(display("non-ascii char: `{value}` supplied"))]
    NonAsciiChar { value: u8 },
    #[snafu(display("non-ascii in: `{value}`"))]
    NonAsciiStr { value: String },
    #[snafu(display("chunk type must be 4 bytes long"))]
    WrongLength,
    #[snafu(display("chunk type must be alphabetic"))]
    NonAlpha,
    #[snafu(display("reserved bit is invalid"))]
    InvalidReservedBit,
}

/// A validated PNG chunk type. See the PNG spec for more details.
/// <http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html>
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ChunkType {
    data: [u8; 4],
}

fn is_5th_bit_set(value: u8) -> bool {
    (value as char).is_lowercase()
    // TODO: Not spec compliant as shouldn't leverage lower/upper
    // Bitwise AND with a mask that has only the 5th bit set
    // 5th bit is at position 4 (0-indexed from right)
    // (value & (1 << 4)) != 0
}

impl ChunkType {
    /// Returns the raw bytes contained in this chunk
    #[allow(dead_code)]
    fn bytes(&self) -> [u8; 4] {
        self.data
    }
    // Returns true if the reserved byte is valid and all four bytes are represented by the characters A-Z or a-z.
    /// Note that this chunk type should always be valid as it is validated during construction.
    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }
    /// Returns the property state of the first byte as described in the PNG spec
    #[allow(dead_code)]
    fn is_critical(&self) -> bool {
        // Ancillary bit: bit 5 of first byte
        // 0 (uppercase) = critical, 1 (lowercase) = ancillary.
        dbg!(is_5th_bit_set(self.data[0]));
        !is_5th_bit_set(self.data[0])
    }
    /// Returns the property state of the second byte as described in the PNG spec
    #[allow(dead_code)]
    fn is_public(&self) -> bool {
        // Private bit: bit 5 of second byte
        // 0 (uppercase) = public, 1 (lowercase) = private.
        !is_5th_bit_set(self.data[1])
    }
    /// Returns the property state of the third byte as described in the PNG spec
    fn is_reserved_bit_valid(&self) -> bool {
        // Reserved bit: bit 5 of third byte
        // Must be 0 (uppercase) in files conforming to this version of PNG.
        !is_5th_bit_set(self.data[2])
    }
    /// Returns the property state of the fourth byte as described in the PNG spec
    #[allow(dead_code)]
    fn is_safe_to_copy(&self) -> bool {
        // Safe-to-copy bit: bit 5 of fourth byte
        // 0 (uppercase) = unsafe to copy, 1 (lowercase) = safe to copy.
        is_5th_bit_set(self.data[3])
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ChunkTypeError;

    fn try_from(value: [u8; 4]) -> Result<ChunkType, Self::Error> {
        for v in value {
            ensure!(v.is_ascii(), NonAsciiCharSnafu { value: v });
        }
        let ct = ChunkType { data: value };
        Ok(ct)
    }
}

impl FromStr for ChunkType {
    type Err = ChunkTypeError;
    fn from_str(s: &str) -> Result<ChunkType, Self::Err> {
        ensure!(s.len() == 4, WrongLengthSnafu);
        ensure!(
            s.is_ascii(),
            NonAsciiStrSnafu {
                value: s.to_string()
            }
        );
        ensure!(s.chars().all(|x| x.is_alphabetic()), NonAlphaSnafu);
        let bytes = s.as_bytes();
        let ct = ChunkType {
            data: [bytes[0], bytes[1], bytes[2], bytes[3]],
        };
        ensure!(ct.is_valid(), InvalidReservedBitSnafu);
        Ok(ct)
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", from_utf8(&self.data).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let result = ChunkType::from_str("Rust");
        assert!(result.is_err());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let result = ChunkType::from_str("Rust");
        assert!(result.is_err());
        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
