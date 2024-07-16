use std::fmt::Error;
use std::sync::Arc;
use binary_util::ByteReader;
use binary_util::interfaces::Reader;
use undefined_redstone_protocol::encryption::MinecraftPacketEncryption;
use undefined_redstone_protocol::server::handshake::CompressionAlgorithm;
use crate::packet::batch_packet::BatchPacket;

pub struct PackerDecoder {
    compression_algorithm: Option<CompressionAlgorithm>,
}

impl PackerDecoder {
    pub fn new() -> PackerDecoder {
        Self {
            compression_algorithm: None,
        }
    }

    pub fn set_compression_algorithm(&mut self, compression_algorithm: Option<CompressionAlgorithm>) {
        self.compression_algorithm = compression_algorithm;
    }

    pub async fn decode(&self, packet_data: Vec<u8>, encryption: Option<&mut MinecraftPacketEncryption>) -> anyhow::Result<BatchPacket> {
        let mut data = packet_data.clone();
        if let Some(encryption) = encryption {
            let temp = encryption.decode(&packet_data[1..]).await?;
            data = vec![ 0xfe ];
            data.extend(temp);
        }
        if let Some(compression) = self.compression_algorithm {
            let bytes = match &data[1] {
                0 | 1 => {//如果是Zlib压缩或Snappy压缩
                    compression.decode(&data[2..])?
                },
                255 => {//如果没有被压缩
                    data[2..].to_vec()
                },
                _ => {//不支持的压缩格式
                    return Err(anyhow::Error::msg("Unsupported Compressor"))
                }
            };
            //拼接
            let mut vec = vec![ 0xfe ];
            vec.extend(bytes);
            data = vec;
        }
        Ok(BatchPacket::read(&mut ByteReader::from(data))?)
    }
}