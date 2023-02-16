use std::convert::TryFrom;
use std::fmt;
use std::io::{BufReader, Read};

use crate::chunk_type::ChunkType;
use crate::{Error, Result};

use crc::{self, Crc};

const CRC_HDLC: Crc<u32> = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);

/// A validated PNG chunk. See the PNG Spec for more details
/// http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
#[derive(Debug,Clone)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    /// Creates a new chunk of type `ChunkType` containing  `data`
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let chained_data: Vec<_> = chunk_type.bytes()
            .iter()
            .copied()
            .chain(data.iter().copied())
            .collect();

        let crc = CRC_HDLC.checksum(&chained_data);
        let length = data.len() as u32;

        Chunk { length, chunk_type, data, crc }
    }

    /// The length of the data portion of this chunk
    pub fn length(&self) -> u32 {
        self.length
    }

    /// The `ChunkType` of this chunk
    pub fn chunk_type(&self) -> &ChunkType  {
        &self.chunk_type
    }

    /// The raw data contained in this chunk in bytes
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// The CRC of this chunk
    pub fn crc(&self) -> u32 {
        self.crc
    }

    /// Returns the data stored in this chunk as a `String`. This function will return an error
    /// if the stored data is not valid UTF-8.
    pub fn data_as_string(&self) -> Result<String> {
        Ok(String::from_utf8(self.data.to_owned())?)
    }

    /// Returns this chunk as a byte sequences described by the PNG spec.
    /// The following data is included in this byte sequence in order:
    /// 1. Length of the data *(4 bytes)*
    /// 2. Chunk type *(4 bytes)*
    /// 3. The data itself *(`length` bytes)*
    /// 4. The CRC of the chunk type and data *(4 bytes)*
    pub fn as_bytes(&self) -> Vec<u8> {
        let length = u32::to_be_bytes(self.length);
        let crc = u32::to_be_bytes(self.crc);

        length.into_iter()
            .chain(self.chunk_type.bytes().into_iter())
            .chain(self.data.iter().copied())
            .chain(crc.into_iter())
            .collect()
    }

}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        let mut reader = BufReader::new(value);
        let mut buffer: [u8; 4] = [0; 4];

        reader.read_exact(&mut buffer)?;
        let length = u32::from_be_bytes(buffer);

        reader.read_exact(&mut buffer)?;
        let chunk_type = ChunkType::try_from(buffer)?;

        let mut data = vec![0u8; length as usize];
        reader.read_exact(&mut data)?;

        reader.read_exact(&mut buffer)?;
        let crc = u32::from_be_bytes(buffer);

        let chunk = Chunk::new(chunk_type, data);

        if chunk.crc != crc {
            Err("invalid chunk CRC".into())
        } else {
            Ok(chunk)
        }
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Chunk (Type: {}; CRC: {}; Length: {})", self.chunk_type, self.crc, self.length)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
    }
}