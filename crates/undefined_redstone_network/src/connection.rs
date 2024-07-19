use rak_rs::connection::Connection;
use rak_rs::Listener;
use undefined_redstone_log::t;
use crate::ip::IPService;
use crate::packet::packet_factory::PacketFactory;
use crate::packet::packet_handler::{ServerPacketHandler, ServerPacketHandlerStatus};
use crate::URNetworkSettings;

pub(crate) async fn accept_connection(settings: URNetworkSettings) {
    let mut listener = Listener::bind(format!("0.0.0.0:{}", settings.0.server_port)).await.unwrap();
    listener.id = settings.0.guid;
    listener.motd = settings.0.server_motd.clone();
    listener.start().await.unwrap();
    loop {
        if let Ok(connection) = listener.accept().await {
            tokio::spawn(handle_connection(connection, settings.clone()));
        }
    }
}

pub(crate) async fn handle_connection(connection: Connection, settings: URNetworkSettings) {
    let mut packet_factory = PacketFactory::new(connection, settings);
    loop {
        let packets = packet_factory.recv_packet().await;
        if let Ok(status) = ServerPacketHandler::handle_packet(&mut packet_factory, packets).await {
            match status {
                ServerPacketHandlerStatus::Closed => {
                    println!("Closed");
                    return;
                }
                ServerPacketHandlerStatus::CreatePlayer => {
                    let username = packet_factory.data.username.clone();
                    let ip = packet_factory.connection.address.ip().to_string();
                    let ip_service = IPService::new(ip.clone()).await;
                    let locate = ip_service.locate;
                    let os_list = vec!["Unknown", "Android", "iOS", "macOS",  "FireOS", "GearVR", "HoloLens", "Windows", "Windows", "EducalVersion","Dedicated", "PlayStation4", "Switch", "XboxOne"];
                    let os = os_list.get(packet_factory.data.device_os as usize).cloned().unwrap_or("Unknown");
                    let device = packet_factory.data.device_model.clone();
                    let xuid = packet_factory.data.uuid.to_string();
                    println!("{}", t!("console.player.join", player = username, ip = ip, ip_locate = locate, os = os, device = device, xuid = xuid))
                }
                _ => {
                    continue;
                }
            }
        }
    }
}