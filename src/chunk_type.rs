use std::fmt;
use std::convert::TryFrom;
use std::str::{self, FromStr};

use crate::{Error, Result};

/// A validated PNG chunk type. See the PNG spec for more details.
/// http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
#[derive(Debug,Clone,PartialEq,Eq)]
pub struct ChunkType(String);

impl ChunkType {
    /// Creates a new ChunkType. Checks the basic properties that a ChunkType must have according to the PNG spec:
    /// the ChunkType consists of 4 characters and all four bytes are represented by the characters A-Z or a-z.
    fn build(chunk_type: String) -> Result<ChunkType> {
        if chunk_type.len() == 4 && chunk_type.chars().all(|c| c.is_ascii_alphabetic()) {
            Ok(ChunkType(chunk_type))
        } else {
            Err("invalid chunk type".into())
        }
    }

    /// Returns the raw bytes contained in this chunk
    pub fn bytes(&self) -> [u8; 4] {
        self.0.as_bytes().try_into().unwrap()
    }

    /// Returns true if the reserved byte is valid. Other properties is is validated during construction.
    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }

    /// Returns the property state of the first byte as described in the PNG spec
    pub fn is_critical(&self) -> bool {
        Self::fifth_bit_is_zero(self.0.as_bytes()[0])
    }

    /// Returns the property state of the second byte as described in the PNG spec
    pub fn is_public(&self) -> bool {
        Self::fifth_bit_is_zero(self.0.as_bytes()[1])
    }

    /// Returns the property state of the third byte as described in the PNG spec
    pub fn is_reserved_bit_valid(&self) -> bool {
        Self::fifth_bit_is_zero(self.0.as_bytes()[2])
    }

    /// Returns the property state of the fourth byte as described in the PNG spec
    pub fn is_safe_to_copy(&self) -> bool {
        !Self::fifth_bit_is_zero(self.0.as_bytes()[3])
    }

    /// Check the property state of byte
    fn fifth_bit_is_zero(byte: u8) -> bool {
        byte & 0x20 == 0
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self> {
        let chunk_type = str::from_utf8(&value[..])?;
        ChunkType::from_str(chunk_type)
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        ChunkType::build(s.to_owned())
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
        assert!(!chunk.is_valid());

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