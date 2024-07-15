use std::sync::Arc;
use undefined_redstone_protocol::encryption::MinecraftPacketEncryption;
use undefined_redstone_protocol::server::handshake::CompressionAlgorithm;

pub struct PackerEncoder {
    compression_algorithm: Option<CompressionAlgorithm>,
    encryption: Option<Arc<MinecraftPacketEncryption>>
}

impl PackerEncoder {
    pub fn new() -> Self {
        Self {
            compression_algorithm: None,
            encryption: None,
        }
    }
}