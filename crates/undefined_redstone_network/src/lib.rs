pub mod connection;
pub mod packet;

use std::any::Any;
use std::sync::Arc;
use bevy_app::{App, Plugin, Startup};
use bevy_ecs::label::DynEq;
use bevy_ecs::schedule::ScheduleLabel;
use bevy_ecs::system::{Res, Resource};
use tokio::runtime::Runtime;
use rak_rs::Motd;
use rand::random;
use undefined_redstone_protocol::server::handshake::CompressionAlgorithm;
use crate::connection::accept_connection;

#[derive(Resource, Clone)]
pub struct URNetworkSettings(Arc<URNetworkSettingsInner>);

pub struct URNetworkSettingsInner {
    pub tokio_runtime: Runtime,
    pub server_port: u16,
    pub server_motd: Motd,
    pub guid: u64,
    pub compression_algorithm: CompressionAlgorithm
}

impl URNetworkSettingsInner {
    pub fn new(tokio_runtime: Runtime, server_port: u16, server_motd: Motd, compression_algorithm: CompressionAlgorithm) -> URNetworkSettings {
        URNetworkSettings(Arc::new(Self {
            tokio_runtime,
            server_port,
            server_motd,
            guid: random(),
            compression_algorithm,
        }))
    }
}

#[derive(ScheduleLabel, Clone, Eq, Debug, PartialEq, Hash)]
pub struct URNetworkPreStartup;

pub struct URNetworkPlugin;

impl Plugin for URNetworkPlugin {
    fn build(&self, app: &mut App) {
        let accept_connection_enter = |network_settings: Res<URNetworkSettings>| {
            let _guard = network_settings.0.tokio_runtime.enter();
            tokio::spawn(accept_connection(network_settings.clone()));
        };
        app.add_systems(Startup, accept_connection_enter);
    }
}