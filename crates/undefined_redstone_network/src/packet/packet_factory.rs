use tokio::io::AsyncWriteExt;
use rak_rs::connection::Connection;
use undefined_redstone_protocol::encryption::MinecraftPacketEncryption;
use undefined_redstone_protocol::MinecraftPacket;
use undefined_redstone_protocol::server::Disconnect;
use undefined_redstone_protocol::skin::Skin;
use uuid::Uuid;
use binary_util::ByteWriter;
use binary_util::interfaces::Writer;
use rak_rs::protocol::reliability::Reliability;
use rak_rs::rakrs_debug;
use crate::client_connection::ClientConnection;
use crate::packet::batch_packet::BatchPacket;
use crate::packet::decoder::PackerDecoder;
use crate::packet::encoder::PackerEncoder;
use crate::URNetworkSettings;

pub struct PacketFactoryData {
    pub protocol_version: u32,
    pub uuid: Uuid,
    pub username: String,
    pub client_id: i64,
    pub skin: Option<Skin>,
    pub device_os: u8,
    pub device_model: String,
    pub language_code: String,
    pub game_version: String,
    pub server_address: String,
}

impl PacketFactoryData {
    pub fn new() -> Self {
        Self {
            protocol_version: 0,
            uuid: Uuid::nil(),
            username: String::new(),
            client_id: 0,
            skin: None,
            device_os: 0,
            device_model: String::new(),
            language_code: String::new(),
            game_version: String::new(),
            server_address: String::new(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PacketFactoryStatus {
    None,
    Logging,
    Handshaking,
    ResourcePack,
    PreSpawn
}

pub struct PacketFactory {
    pub(crate) status: PacketFactoryStatus,
    pub(crate) disconnect: bool,
    pub(crate) connection: Connection,
    encoder: PackerEncoder,
    decoder: PackerDecoder,
    encryption: Option<MinecraftPacketEncryption>,
    pub(crate) settings: URNetworkSettings,
    pub(crate) protocol_version: u32,
    pub(crate) data: PacketFactoryData
}

impl PacketFactory {
    pub fn new(connection: Connection, settings: URNetworkSettings) -> Self {
        Self {
            status: PacketFactoryStatus::None,
            disconnect: false,
            connection,
            encoder: PackerEncoder::new(),
            decoder: PackerDecoder::new(),
            encryption: None,
            settings,
            protocol_version: 0,
            data: PacketFactoryData::new(),
        }
    }

    pub fn enable_compression(&mut self) {
        self.decoder.set_compression_algorithm(Some(self.settings.0.compression_algorithm));
        self.encoder.set_compression_algorithm(Some(self.settings.0.compression_algorithm));
    }

    pub fn disable_compression(&mut self) {
        self.decoder.set_compression_algorithm(None);
        self.encoder.set_compression_algorithm(None);
    }

    pub fn enable_encryption(&mut self, secret_key: Vec<u8>) -> anyhow::Result<()> {
        self.encryption = Some(MinecraftPacketEncryption::new(secret_key, self.protocol_version)?);
        Ok(())
    }

    pub async fn recv_packet(&mut self) -> anyhow::Result<BatchPacket> {
        let packet_data = self.connection.recv().await?;
        self.decoder.decode(packet_data, self.encryption.as_mut()).await
    }

    pub async fn send_packet(&mut self, packet: MinecraftPacket, immediate: bool) -> anyhow::Result<()> {
        self.send_batch(BatchPacket::single(packet), immediate).await
    }

    pub async fn send_batch(&mut self, packet: BatchPacket, immediate: bool) -> anyhow::Result<()> {
        let bytes = self.encoder.encode(packet, self.encryption.as_mut())?;
        self.connection.send(&bytes, immediate).await?;
        Ok(())
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

    pub fn into_client_connection(mut self) -> ClientConnection {
        let mut sender = self.connection.send_queue.clone();

        let (receive_sender, packet_receiver) = flume::unbounded();
        let receive_loop = tokio::spawn(async move {
            loop {
                if let Ok(packets) = self.recv_packet().await {
                    let _ = receive_sender.send(packets);
                }
            }
        });
        let (packet_sender, send_receiver) = flume::unbounded::<(BatchPacket, bool)>();
        let send_loop = tokio::spawn(async move {
            loop {
                if let Ok((packets, immediate)) = send_receiver.recv_async().await {
                    if let Ok(buffer) = packets.write_to_bytes() {
                        let buffer = buffer.as_slice();
                        let mut q = sender.write().await;
                        let _ = q
                            .insert(buffer, Reliability::ReliableOrd, immediate, Some(0))
                            .await;
                    }
                }
            }
        });
        ClientConnection {
            receive_loop,
            send_loop,
            packet_receiver,
            packet_sender,
        }
    }
}