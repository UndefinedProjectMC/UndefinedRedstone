pub mod connection;
pub mod packet;
pub mod ip;
pub mod client;
mod client_connection;

use bevy_ecs::schedule::IntoSystemConfigs;
use std::any::Any;
use std::sync::Arc;
use bevy_app::{App, Plugin, PreStartup};
use bevy_ecs::label::DynEq;
use bevy_ecs::prelude::{Commands, FromWorld, IntoSystemSetConfigs, SystemSet};
use bevy_ecs::schedule::ScheduleLabel;
use bevy_ecs::system::{Res, Resource};
use bevy_ecs::world::World;
use flume::{Receiver, Sender};
use rak_rs::Motd;
use rand::random;
use bevy_async_ecs::AsyncWorld;
use undefined_redstone_asyncmanager::URAsyncManager;
use undefined_redstone_core::startup::URStartupSet;
use undefined_redstone_core::utils::server_properties::ServerProperties;
use undefined_redstone_log::t;
use undefined_redstone_packloader::entity::EntityContentManager;
use undefined_redstone_packloader::URPackLoadSet;
use undefined_redstone_protocol::server::handshake::CompressionAlgorithm;
use crate::connection::{accept_connection, new_client};
use crate::packet::packet_factory::PacketFactory;

#[derive(Resource, Clone)]
pub struct URNetworkSettings(Arc<URNetworkSettingsInner>);

impl URNetworkSettings {
    pub fn new(server_port: u16, server_motd: Motd, compression_algorithm: CompressionAlgorithm) -> URNetworkSettings {
        URNetworkSettings(Arc::new(URNetworkSettingsInner::new(server_port, server_motd, compression_algorithm)))
    }
}

pub struct URNetworkSettingsInner {
    pub server_port: u16,
    pub server_motd: Motd,
    pub guid: u64,
    pub compression_algorithm: CompressionAlgorithm,
    pub sender: Sender<PacketFactory>,
    pub receiver: Receiver<PacketFactory>
}

impl URNetworkSettingsInner {
    pub fn new(server_port: u16, server_motd: Motd, compression_algorithm: CompressionAlgorithm) -> Self {
        let (sender, receiver) = flume::unbounded();
        Self {
            server_port,
            server_motd,
            guid: random(),
            compression_algorithm,
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

#[derive(SystemSet, Clone, Eq, Debug, PartialEq, Hash)]
pub struct URNetworkInitSet;

#[derive(SystemSet, Clone, Eq, Debug, PartialEq, Hash)]
pub struct URClientEnterSet;

pub struct URNetworkPlugin;

impl Plugin for URNetworkPlugin {
    fn build(&self, app: &mut App) {
        let accept_connection_enter = |network_settings: Res<URNetworkSettings>| {
            println!("{}", t!("console.ipv4", port = network_settings.0.server_port));
            let _guard = URAsyncManager::global().enter("network").unwrap();
            tokio::spawn(accept_connection(network_settings.clone()));
        };
        let new_client_enter = |world: &mut World| {
            let settings = world.resource::<URNetworkSettings>().clone();
            let entity_manager = world.resource::<EntityContentManager>().clone();
            let async_world = AsyncWorld::from_world(world);
            let _guard = URAsyncManager::global().enter("network").unwrap();
            tokio::spawn(new_client(async_world, settings, entity_manager));
        };
        app.configure_sets(PreStartup, (URNetworkInitSet.after(URStartupSet), URClientEnterSet.after(URPackLoadSet)))
            .add_systems(PreStartup, init_network.in_set(URNetworkInitSet).before(accept_connection_enter))
            .add_systems(PreStartup, accept_connection_enter.in_set(URNetworkInitSet))
            .add_systems(PreStartup, new_client_enter.in_set(URClientEnterSet));
    }
}