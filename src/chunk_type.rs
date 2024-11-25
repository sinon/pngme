use std::fmt::{Display, Result};
use std::str::{from_utf8, FromStr};

#[derive(Debug, PartialEq, Eq)]
struct ChunkType {
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
    fn bytes(&self) -> [u8; 4] {
        self.data
    }
    fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }
    fn is_critical(&self) -> bool {
        // Ancillary bit: bit 5 of first byte
        // 0 (uppercase) = critical, 1 (lowercase) = ancillary.
        dbg!(is_5th_bit_set(self.data[0]));
        !is_5th_bit_set(self.data[0])
    }
    fn is_public(&self) -> bool {
        // Private bit: bit 5 of second byte
        // 0 (uppercase) = public, 1 (lowercase) = private.
        !is_5th_bit_set(self.data[1])
    }
    fn is_reserved_bit_valid(&self) -> bool {
        // Reserved bit: bit 5 of third byte
        // Must be 0 (uppercase) in files conforming to this version of PNG.
        !is_5th_bit_set(self.data[2])
    }
    fn is_safe_to_copy(&self) -> bool {
        // Safe-to-copy bit: bit 5 of fourth byte
        // 0 (uppercase) = unsafe to copy, 1 (lowercase) = safe to copy.
        is_5th_bit_set(self.data[3])
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    fn try_from(value: [u8; 4]) -> std::result::Result<ChunkType, &'static str> {
        for v in value {
            if !v.is_ascii() {
                return Err("The supplied value contains non-ascii values");
            }
        }
        let ct = ChunkType { data: value };
        Ok(ct)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ChunkTypeParseError;
impl FromStr for ChunkType {
    type Err = ChunkTypeParseError;

    fn from_str(s: &str) -> std::result::Result<ChunkType, ChunkTypeParseError> {
        if s.len() != 4 {
            return Err(ChunkTypeParseError);
        }
        if !s.is_ascii() {
            return Err(ChunkTypeParseError);
        }
        if !s.chars().all(|x| x.is_alphabetic()) {
            return Err(ChunkTypeParseError);
        }

        let bytes = s.as_bytes();
        let ct = ChunkType {
            data: [bytes[0], bytes[1], bytes[2], bytes[3]],
        };
        Ok(ct)
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
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
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
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
        let chunk = ChunkType::from_str("Rust").unwrap();
        dbg!(&chunk);
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        dbg!(&chunk);
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
