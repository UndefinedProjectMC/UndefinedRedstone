use bytes::Bytes;
use binary_util::ByteReader;
use crate::NbtValue;

pub trait NbtReadTrait {
    fn read(reader: ByteReader) -> anyhow::Result<NbtValue>;
}

pub struct NbtReader {
    reader: ByteReader
}

impl NbtReader {
    pub fn from_reader(reader: ByteReader) -> NbtReader {
        Self {
            reader
        }
    }

    pub fn from_slice(slice: &[u8]) -> NbtReader {
        Self {
            reader: ByteReader::from(slice)
        }
    }

    pub fn from_bytes(bytes: Bytes) -> NbtReader {
        Self {
            reader: ByteReader::from(bytes)
        }
    }

    pub fn from_vec(vec: Vec<u8>) -> NbtReader {
        Self {
            reader: ByteReader::from(vec)
        }
    }

    pub fn read<T: NbtReadTrait>(&self) -> anyhow::Result<NbtValue> {
        T::read(self.reader.clone())
    }
}