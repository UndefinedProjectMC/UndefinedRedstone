use std::sync::Arc;
use binary_util::interfaces::Writer;
use undefined_redstone_protocol::encryption::MinecraftPacketEncryption;
use undefined_redstone_protocol::server::handshake::CompressionAlgorithm;
use crate::packet::batch_packet::BatchPacket;

pub struct PackerEncoder {
    compression_algorithm: Option<CompressionAlgorithm>,
}

impl PackerEncoder {
    pub fn new() -> Self {
        Self {
            compression_algorithm: None,
        }
    }

    pub fn set_compression_algorithm(&mut self, compression_algorithm: Option<CompressionAlgorithm>) {
        self.compression_algorithm = compression_algorithm;
    }

    pub fn encode(&self, packet: BatchPacket, encryption: Option<&mut MinecraftPacketEncryption>) -> anyhow::Result<Vec<u8>> {
        let bytes = packet.write_to_bytes()?;
        let bytes = bytes.as_slice().to_vec();
        let mut bytes = bytes[1..].to_vec();
        if let Some(compression_algorithm) = self.compression_algorithm {
            let mut temp = vec![compression_algorithm.get_bytes()];
            temp.extend(compression_algorithm.encode(&mut bytes)?);
            bytes = temp;
        }
        if let Some(encryption) = encryption {
            bytes = encryption.encode(&bytes)?;
        }
        let mut final_bytes = vec![ 0xfe ];
        final_bytes.extend(bytes);
        Ok(final_bytes)
    }
}