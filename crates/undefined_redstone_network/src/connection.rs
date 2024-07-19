use bevy_ecs::world::{EntityWorldMut, World};
use bevy_async_ecs::AsyncWorld;
use rak_rs::connection::Connection;
use rak_rs::Listener;
use undefined_redstone_log::t;
use undefined_redstone_packloader::entity::EntityContentManager;
use undefined_redstone_type::display_name::DisplayName;
use crate::client::MinecraftClient;
use crate::client_connection::ClientConnection;
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
    let mut packet_factory = PacketFactory::new(connection, settings.clone());
    loop {
        let packets = packet_factory.recv_packet().await;
        if let Ok(status) = ServerPacketHandler::handle_packet(&mut packet_factory, packets).await {
            match status {
                ServerPacketHandlerStatus::Closed => {
                    println!("Closed");
                    return;
                }
                ServerPacketHandlerStatus::CreatePlayer => {
                    if let Ok(_) = settings.0.sender.send(packet_factory) {
                        println!("new client");
                    }
                    return;
                }
                _ => {
                    continue;
                }
            }
        }
    }
}

pub(crate) async fn new_client(world: AsyncWorld, settings: URNetworkSettings, entity_manager: EntityContentManager) {
    loop {
        for _ in 0..settings.0.receiver.len() {
            tokio::spawn(create_new_client(world.clone(), settings.clone(), entity_manager.clone()));
        }
    }
}

pub(crate) async fn create_new_client(world: AsyncWorld, settings: URNetworkSettings, entity_manager: EntityContentManager) {
    if let Ok(factory) = settings.0.receiver.try_recv() {
        let username = factory.data.username.clone();
        let ip = factory.connection.address.ip().to_string();
        let ip_service = IPService::new(ip.clone()).await;
        let locate = ip_service.locate;
        let os_list = vec!["Unknown", "Android", "iOS", "macOS", "FireOS", "GearVR", "HoloLens", "Windows", "Windows", "Education", "Dedicated", "PlayStation", "Switch", "Xbox"];
        let os = os_list.get(factory.data.device_os as usize).cloned().unwrap_or("Unknown");
        let device = factory.data.device_model.clone();
        let xuid = factory.data.uuid.to_string();
        println!("{}", t!("console.player.join", player = username, ip = ip, ip_locate = locate, os = os, device = device, xuid = xuid));
        spawn_client(world, settings, factory, entity_manager).await;
    }
}

pub(crate) async fn spawn_client(world: AsyncWorld, settings: URNetworkSettings, factory: PacketFactory, entity_manager: EntityContentManager) {
    let entity = entity_manager.get("minecraft:player").unwrap();
    let username = factory.data.username.clone();
    let usernamex = username.clone();
    let xuid = factory.data.uuid.to_string();
    world.apply(move |mut world: &mut World| {
        let mut client = world.spawn(MinecraftClient {
            connection: factory.into_client_connection(),
        });
        client.insert(DisplayName(usernamex));
        entity.insert(&mut client);
    }).await;
    println!("{}", t!("console.player.spawn", player = username, xuid = xuid));
}