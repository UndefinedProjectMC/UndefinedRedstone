pub mod connection;
pub mod packet;
pub mod ip;
mod client_connection;
pub mod client;
pub mod encryption;
pub mod skin;
pub mod client_recv;
pub mod protocol;

use bevy_ecs::schedule::IntoSystemConfigs;
use std::any::Any;
use std::sync::Arc;
use bevy_app::{App, Plugin, PreStartup, PreUpdate};
use bevy_ecs::event::Event;
use bevy_ecs::label::DynEq;
use bevy_ecs::prelude::{Commands, FromWorld, IntoSystemSetConfigs, SystemSet};
use bevy_ecs::schedule::ScheduleLabel;
use bevy_ecs::system::{Res, Resource};
use bevy_ecs::world::World;
use flume::{Receiver, Sender};
use rak_rs::Motd;
use rand::random;
use bevy_async_ecs::AsyncWorld;
use binary_util::BinaryIo;
use undefined_redstone_asyncmanager::URAsyncManager;
use undefined_redstone_core::startup::URStartupSet;
use undefined_redstone_core::utils::server_properties::ServerProperties;
use undefined_redstone_log::t;
use undefined_redstone_packloader::URPackLoadSet;
use undefined_redstone_type::minecraft_version::{MinecraftVersion, MinecraftVersions};
use undefined_redstone_type::protocol_versions::MinecraftProtocolVersions;
use crate::client_recv::recv_event_loop;
use crate::connection::{accept_connection, new_client};
use crate::packet::packet_factory::PacketFactory;
use crate::protocol::ProtocolInfo;
use crate::protocol::server::handshake::CompressionAlgorithm;

#[derive(SystemSet, Clone, Eq, Debug, PartialEq, Hash)]
pub struct URClientConnectionRecvSet;

#[derive(Resource, Clone)]
pub struct URNetworkSettings {
    pub inner: Arc<URNetworkSettingsInner>,
    pub server_port: u16,
    pub server_motd: Motd,
    pub guid: u64,
    pub compression_algorithm: CompressionAlgorithm,
}

impl URNetworkSettings {
    pub fn new(server_port: u16, server_motd: Motd, compression_algorithm: CompressionAlgorithm) -> URNetworkSettings {
        Self {
            inner: Arc::new(URNetworkSettingsInner::new()),
            server_port,
            server_motd,
            guid: random(),
            compression_algorithm,
        }
    }
}

pub struct URNetworkSettingsInner {
    pub sender: Sender<PacketFactory>,
    pub receiver: Receiver<PacketFactory>
}

impl URNetworkSettingsInner {
    pub fn new() -> Self {
        let (sender, receiver) = flume::unbounded();
        Self {
            sender,
            receiver,
        }
    }
}

pub(crate) fn init_network(mut commands: Commands, server_properties: Res<ServerProperties>) {
    URAsyncManager::global().insert("network");
    let compression_algorithm = if server_properties.enable_snappy {
        CompressionAlgorithm::Snappy
    } else {
        CompressionAlgorithm::Zlib
    };
    commands.insert_resource(URNetworkSettings::new(
        server_properties.ipv4_port,
        server_properties.to_motd(),
        compression_algorithm
    ))
}

/// warning: This is a temp func!
pub(crate) fn init_protocol() {
    ProtocolInfo::new_global(
        MinecraftProtocolVersions::single(686),
        MinecraftVersions::single(MinecraftVersion::new(1, 21, 2))
    )
}

#[derive(SystemSet, Clone, Eq, Debug, PartialEq, Hash)]
pub struct URNetworkInitSet;

#[derive(SystemSet, Clone, Eq, Debug, PartialEq, Hash)]
pub struct URClientEnterSet;

pub struct URNetworkPlugin;

impl Plugin for URNetworkPlugin {
    fn build(&self, app: &mut App) {
        let accept_connection_enter = |network_settings: Res<URNetworkSettings>| {
            println!("{}", t!("console.ipv4", port = network_settings.server_port));
            let _guard = URAsyncManager::global().enter("network").unwrap();
            tokio::spawn(accept_connection(network_settings.clone()));
        };
        let new_client_enter = |world: &mut World| {
            let async_world = AsyncWorld::from_world(world);
            let _guard = URAsyncManager::global().enter("network").unwrap();
            tokio::spawn(new_client(async_world));
        };
        app.configure_sets(PreStartup, (URNetworkInitSet.after(URStartupSet),
                                        URClientEnterSet.after(URPackLoadSet)))
            .configure_sets(PreUpdate, URClientConnectionRecvSet)
            .add_systems(PreStartup, init_protocol.in_set(URStartupSet))
            .add_systems(PreStartup, init_network.in_set(URNetworkInitSet).before(accept_connection_enter))
            .add_systems(PreStartup, accept_connection_enter.in_set(URNetworkInitSet))
            .add_systems(PreStartup, new_client_enter.in_set(URClientEnterSet))
            .add_systems(PreUpdate, recv_event_loop.in_set(URClientConnectionRecvSet));
    }
}
