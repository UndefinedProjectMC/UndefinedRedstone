use std::io::{Error, Read};
use binary_util::{BinaryIo, ByteReader, ByteWriter};
use binary_util::interfaces::{Reader, Writer};
use flate2::Compression;
use flate2::read::{DeflateDecoder, DeflateEncoder};
use snap::read::{FrameDecoder, FrameEncoder};

#[derive(Clone, Debug)]
pub struct NetworkSettings {
    pub compression_threshold: i16,
    pub compression_algorithm: CompressionAlgorithm,
    pub client_throttle_enabled: bool,
    pub client_throttle_threshold: i8,
    pub client_throttle_scalar: f32
}

impl Reader<NetworkSettings> for NetworkSettings {
    fn read(buf: &mut ByteReader) -> Result<NetworkSettings, Error> {
        let compression_threshold = buf.read_i16_le()?;
        let compression_algorithm = CompressionAlgorithm::read(buf)?;
        let client_throttle_enabled = buf.read_bool()?;
        let client_throttle_threshold = buf.read_i8()?;
        let client_throttle_scalar = buf.read_f32()?;
        Ok(NetworkSettings {
            compression_threshold,
            compression_algorithm,
            client_throttle_enabled,
            client_throttle_threshold,
            client_throttle_scalar
        })
    }
}

impl Writer for NetworkSettings {
    fn write(&self, buf: &mut ByteWriter) -> Result<(), Error> {
        buf.write_i16_le(self.compression_threshold)?;
        self.compression_algorithm.write(buf)?;
        buf.write_bool(self.client_throttle_enabled)?;
        buf.write_i8(self.client_throttle_threshold)?;
        buf.write_f32(self.client_throttle_scalar)?;
        Ok(())
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompressionAlgorithm {
    Zlib,
    Snappy
}

impl CompressionAlgorithm {
    pub fn decode(&self, buf: &[u8]) -> Result<Vec<u8>, Error> {
        match self {
            CompressionAlgorithm::Zlib => {
                let mut vec = vec![];
                let mut reader = DeflateDecoder::new(buf);
                match reader.read_to_end(&mut vec) {
                    Ok(_) => Ok(vec),
                    Err(error) => Err(error),
                }
            }
            CompressionAlgorithm::Snappy => {
                let mut frame_decoder = FrameDecoder::new(buf);
                let mut vec = vec![];
                frame_decoder.read_to_end(&mut vec)?;
                Ok(vec)
            }
        }
    }

    pub fn encode(&self, buf: &[u8]) -> Result<Vec<u8>, Error> {
        match self {
            CompressionAlgorithm::Zlib => {
                let mut vec = vec![];
                let mut writer = DeflateEncoder::new(buf, Compression::default());
                match writer.read_to_end(&mut vec) {
                    Ok(_) => Ok(vec),
                    Err(error) => Err(error),
                }
            }
            CompressionAlgorithm::Snappy => {
                let mut frame_encoder = FrameEncoder::new(buf);
                let mut vec = vec![];
                frame_encoder.read_to_end(&mut vec)?;
                Ok(vec)
            }
        }
    }
}

impl Reader<CompressionAlgorithm> for CompressionAlgorithm {
    fn read(buf: &mut ByteReader) -> Result<CompressionAlgorithm, Error> {
        Ok(match buf.read_i16_le()? {
            0 => CompressionAlgorithm::Zlib,
            1 => CompressionAlgorithm::Snappy,
            _ => return Err(Error::other("Cannot read compression algorithm."))
        })
    }
}

impl Writer for CompressionAlgorithm {
    fn write(&self, buf: &mut ByteWriter) -> Result<(), Error> {
        buf.write_i16_le(match self {
            CompressionAlgorithm::Zlib => 0,
            CompressionAlgorithm::Snappy => 1,
        })
    }
}

#[derive(Clone, Debug, BinaryIo)]
pub struct ServerToClientHandshake {
    pub jwt: String
}