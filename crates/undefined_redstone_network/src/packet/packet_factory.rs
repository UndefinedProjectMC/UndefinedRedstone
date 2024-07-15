use std::sync::Arc;
use rak_rs::connection::Connection;
use undefined_redstone_protocol::encryption::MinecraftPacketEncryption;
use undefined_redstone_protocol::MinecraftPacket;
use undefined_redstone_protocol::server::handshake::CompressionAlgorithm;
use crate::packet::batch_packet::BatchPacket;
use crate::packet::decoder::PackerDecoder;
use crate::packet::encoder::PackerEncoder;

pub struct PacketFactory {
    connection: Connection,
    encoder: PackerEncoder,
    decoder: PackerDecoder,
    encryption: Option<MinecraftPacketEncryption>,
    compression_algorithm: CompressionAlgorithm
}

impl PacketFactory {
    pub fn new(connection: Connection, compression_algorithm: CompressionAlgorithm) -> Self {
        Self {
            connection,
            encoder: PackerEncoder::new(),
            decoder: PackerDecoder::new(),
            encryption: None,
            compression_algorithm,
        }
    }

    async fn fetch_result(&mut self) -> anyhow::Result<BatchPacket> {
        let packet_data = self.connection.recv().await?;
        self.decoder.decode(packet_data, self.encryption.as_mut()).await
    }

    pub async fn recv_packet(&mut self) {
        if let Ok(batch_packet) = self.fetch_result().await {
            for packet in batch_packet {

            }
        }
    }

    pub async fn send_packet(&mut self) {

    }

    pub async fn send_batch(&mut self) {

    }
}