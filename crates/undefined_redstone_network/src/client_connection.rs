use rak_rs::connection::{Connection, RecvError};
use crate::encryption::MinecraftPacketEncryption;
use crate::packet::batch_packet::{BatchPacket, OriginBatchPacket};
use crate::packet::decoder::PackerDecoder;
use crate::packet::encoder::PackerEncoder;
use crate::protocol::MinecraftPacket;
use crate::protocol::server::Disconnect;

pub struct ClientConnection {
    pub(crate) connection: Connection,
    pub(crate) encoder: PackerEncoder,
    pub(crate) decoder: PackerDecoder,
    pub(crate) encryption: Option<MinecraftPacketEncryption>,
    disconnect: bool
}

impl ClientConnection {
    pub fn new(connection: Connection,
               encoder: PackerEncoder,
               decoder: PackerDecoder,
               encryption: Option<MinecraftPacketEncryption>) -> Self {
        Self {
            connection,
            encoder,
            decoder,
            encryption,
            disconnect: false,
        }
    }

    pub async fn send_packet(&mut self, packet: MinecraftPacket, immediate: bool) -> anyhow::Result<()> {
        self.send_batch(BatchPacket::single(packet), immediate).await
    }

    pub async fn send_origin_packet(&mut self, packet: Vec<u8>, immediate: bool) -> anyhow::Result<()> {
        self.send_origin_batch(OriginBatchPacket::single(packet), immediate).await
    }

    pub async fn send_batch(&mut self, packet: BatchPacket, immediate: bool) -> anyhow::Result<()> {
        let bytes = self.encoder.encode(packet, self.encryption.as_mut())?;
        Ok(self.connection.send(&bytes, immediate).await?)
    }

    pub async fn send_origin_batch(&mut self, packet: OriginBatchPacket, immediate: bool) -> anyhow::Result<()> {
        let bytes = self.encoder.encode_origin(packet, self.encryption.as_mut())?;
        Ok(self.connection.send(&bytes, immediate).await?)
    }

    pub async fn recv_packet(&mut self) -> anyhow::Result<OriginBatchPacket> {
        let packet_data = self.recv0().await?;
        self.decoder.decode_origin(packet_data, self.encryption.as_mut()).await
    }

    async fn recv0(&mut self) -> Result<Vec<u8>, RecvError> {
        self.connection.recv().await
    }

    pub fn disable_compression(&mut self) {
        self.decoder.set_compression_algorithm(None);
        self.encoder.set_compression_algorithm(None);
    }

    pub async fn disconnect(&mut self, reason: &str, hide_disconnect_screen: bool) -> anyhow::Result<()> {
        self.disable_compression();
        self.send_packet(MinecraftPacket::Disconnect(Disconnect {
            hide_disconnect_screen,
            kick_message: reason.to_string(),
        }), true).await?;
        self.disconnect = true;
        self.connection.close().await;
        Ok(())
    }
}