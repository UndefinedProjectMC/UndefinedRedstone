use std::env;
use bevy_app::{App, Plugin, PreStartup};
use bevy_ecs::change_detection::{Res, ResMut};
use bevy_ecs::prelude::{Commands, IntoSystemConfigs, Resource};
use rust_i18n::t;
use tokio::runtime::Runtime;
use undefined_redstone_network::URNetworkSettingsInner;
use undefined_redstone_protocol::ProtocolInfo;
use undefined_redstone_protocol::server::handshake::CompressionAlgorithm;
use undefined_redstone_type::minecraft_version::{MinecraftVersion, MinecraftVersions};
use undefined_redstone_type::protocol_versions::MinecraftProtocolVersions;
use crate::server::Server;
use crate::utils::server_properties::ServerProperties;

#[derive(Resource, Default)]
struct StartupTimestamp {
    start: i64,
    end: i64,
}


fn startup(mut ms: ResMut<StartupTimestamp>, server: Res<Server>) {
    ms.start = chrono::Local::now().timestamp();
    rust_i18n::set_locale("zh-CN");
    println!("{}", t!("console.startup", version = server.server_version));
    println!("{}", t!("console.warning"));
}

pub(crate) fn init_properties(mut commands: Commands) {
    let mut current_dir = env::current_dir().unwrap();
    current_dir.push("server_properties.toml");
    commands.insert_resource(ServerProperties::load(current_dir).unwrap());
}

/// warning: This is a temp func!
pub(crate) fn init_protocol() {
    ProtocolInfo::new_global(
        MinecraftProtocolVersions::single(686),
        MinecraftVersions::single(MinecraftVersion::new(1, 21, 2))
    )
}

pub(crate) fn init_network_settings(mut commands: Commands, server_properties: Res<ServerProperties>) {
    let compression_algorithm = if server_properties.enable_snappy {
        CompressionAlgorithm::Snappy
    } else {
        CompressionAlgorithm::Zlib
    };
    commands.insert_resource(URNetworkSettingsInner::new(
        Runtime::new().unwrap(),
        server_properties.ipv4_port,
        server_properties.to_motd(),
        compression_algorithm
    ));
}

pub struct URStartupPlugin;
impl Plugin for URStartupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Server>()
            .init_resource::<StartupTimestamp>()
            .add_systems(PreStartup, startup)
            .add_systems(PreStartup, init_properties)
            .add_systems(PreStartup, init_network_settings.after(init_properties))
            .add_systems(PreStartup, init_protocol);
    }
}