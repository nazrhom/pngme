#![allow(unreachable_code, dead_code)]
use std::io::{BufReader, Read};
use std::{string::FromUtf8Error};
use std::fmt;
use crate::chunk_type::ChunkType;
use crc;

// crc v: CRC-32/ISO-HDLC
#[derive(Eq, PartialEq, Debug)]
pub struct Chunk {
  length: u32,
  chunk_type: ChunkType,
  chunk_data: Vec<u8>,
  crc: u32
}

const checksum_calc: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);

impl TryFrom<&[u8]> for Chunk {
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut reader = BufReader::new(value);
        let mut u32_buffer: [u8; 4] = [0; 4];
        
        reader.read_exact(&mut u32_buffer).map_err(|_e| "nope")?;
        let len = u32::from_be_bytes(u32_buffer);

        reader.read_exact(&mut u32_buffer).map_err(|_e| "nope")?;
        let chunk_type = ChunkType::try_from(u32_buffer)?;

        let mut chunk_data: Vec<u8> = vec![0; len as usize];
        reader.read_exact(&mut chunk_data).map_err(|_e| "nope")?;
        
        reader.read_exact(&mut u32_buffer).map_err(|_e| "nope")?;
        let crc = u32::from_be_bytes(u32_buffer);

        let expected_crc = checksum_calc.checksum(&[&chunk_type.bytes(), chunk_data.as_slice()].concat());

        if crc != expected_crc {
          return Err("invalid crc")
        }
        Ok(Chunk {
          length: len,
          chunk_type,
          chunk_data,
          crc
        })
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}


impl Chunk {
  pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
    let crc = checksum_calc.checksum(&[&chunk_type.bytes(), data.as_slice()].concat());
    Chunk {
      length: data.len() as u32,
      chunk_type,
      chunk_data: data,
      crc,
    }
  }

  pub fn length(&self) -> u32 {
    self.length
  }

  pub fn chunk_type(&self) -> &ChunkType {
    &self.chunk_type
  }

  pub fn data(&self) -> &[u8] {
    &self.chunk_data
  }

  pub fn crc(&self) -> u32 {
    self.crc
  }

  pub fn data_as_string(&self) -> Result<String, FromUtf8Error> {
    String::from_utf8(self.chunk_data.clone())
  }

  pub fn as_bytes(&self) -> Vec<u8> {
    let len = self.length.to_be_bytes().into_iter();
    let ct = self.chunk_type.bytes().into_iter();
    let cd = self.chunk_data.iter().copied();
    let crc = self.crc.to_be_bytes().into_iter();
    len.chain(ct).chain(cd).chain(crc).collect()
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