pub mod connection;
pub mod packet;
pub mod ip;

use bevy_ecs::schedule::IntoSystemConfigs;
use std::any::Any;
use std::sync::Arc;
use bevy_app::{App, Plugin, PreStartup};
use bevy_ecs::label::DynEq;
use bevy_ecs::prelude::{Commands, IntoSystemSetConfigs, SystemSet};
use bevy_ecs::schedule::ScheduleLabel;
use bevy_ecs::system::{Res, ResMut, Resource};
use tokio::runtime::Runtime;
use rak_rs::Motd;
use rand::random;
use undefined_redstone_asyncmanager::URAsyncManager;
use undefined_redstone_core::startup::{finish_startup, URStartupSet};
use undefined_redstone_core::utils::server_properties::ServerProperties;
use undefined_redstone_log::t;
use undefined_redstone_protocol::server::handshake::CompressionAlgorithm;
use crate::connection::accept_connection;

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
    pub compression_algorithm: CompressionAlgorithm
}

impl URNetworkSettingsInner {
    pub fn new(server_port: u16, server_motd: Motd, compression_algorithm: CompressionAlgorithm) -> Self {
        Self {
            server_port,
            server_motd,
            guid: random(),
            compression_algorithm,
        }
    }
}

pub(crate) fn init_network(mut commands: Commands, server_properties: Res<ServerProperties>, mut async_manager: ResMut<URAsyncManager>) {
    async_manager.insert("network");
    let compression_algorithm = if server_properties.enable_snappy {
        CompressionAlgorithm::Snappy
    } else {
        CompressionAlgorithm::Zlib
    };
    commands.insert_resource(URNetworkSettings::new(
        server_properties.ipv4_port,
        server_properties.to_motd(),
        compression_algorithm
    ));
}

#[derive(SystemSet, Clone, Eq, Debug, PartialEq, Hash)]
pub struct URNetworkInitSet;

pub struct URNetworkPlugin;

impl Plugin for URNetworkPlugin {
    fn build(&self, app: &mut App) {
        let accept_connection_enter = |network_settings: Res<URNetworkSettings>, async_manager: Res<URAsyncManager>| {
            println!("{}", t!("console.ipv4", port = network_settings.0.server_port));
            let _guard = async_manager.enter("network").unwrap();
            tokio::spawn(accept_connection(network_settings.clone()));
        };
        app.configure_sets(PreStartup, URNetworkInitSet.after(URStartupSet))
            .add_systems(PreStartup, init_network.in_set(URNetworkInitSet).before(accept_connection_enter))
            .add_systems(PreStartup, accept_connection_enter.in_set(URNetworkInitSet));
    }
}