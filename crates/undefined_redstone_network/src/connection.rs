use rak_rs::connection::Connection;
use rak_rs::Listener;
use undefined_redstone_protocol::server::handshake::CompressionAlgorithm;
use crate::packet::packet_factory::PacketFactory;
use crate::URNetworkSettings;

pub(crate) async fn accept_connection(settings: URNetworkSettings) {
    let mut listener = Listener::bind(format!("0.0.0.0:{}", settings.0.server_port)).await.unwrap();
    listener.id = settings.0.guid;
    listener.motd = settings.0.server_motd.clone();
    listener.start().await.unwrap();
    let compression_algorithm = settings.0.compression_algorithm;
    loop {
        if let Ok(connection) = listener.accept().await {
            tokio::spawn(handle_connection(connection, compression_algorithm));
        }
    }
}

pub(crate) async fn handle_connection(connection: Connection, compression_algorithm: CompressionAlgorithm) {
    let mut packet_factory = PacketFactory::new(connection, compression_algorithm);
    packet_factory.recv_packet().await;
}