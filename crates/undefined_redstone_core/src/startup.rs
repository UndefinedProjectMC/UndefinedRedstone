use std::env;
use bevy_app::{App, Plugin, PreStartup};
use bevy_ecs::change_detection::{Res, ResMut};
use bevy_ecs::prelude::{Commands, IntoSystemConfigs, Resource, SystemSet};
use undefined_redstone_log::t;
use crate::server::Server;
use crate::utils::server_properties::ServerProperties;
use bevy_ecs::schedule::IntoSystemSetConfigs;

#[derive(SystemSet, Eq, PartialEq, Debug, Hash, Clone)]
pub struct URLogInitSet;
#[derive(SystemSet, Eq, PartialEq, Debug, Hash, Clone)]
pub struct URPreStartupSet;
#[derive(SystemSet, Eq, PartialEq, Debug, Hash, Clone)]
pub struct URStartupSet;

#[derive(Resource, Default)]
pub struct StartupTimestamp {
    start: i64,
    end: i64,
}


fn startup(mut ms: ResMut<StartupTimestamp>, server: Res<Server>) {
    ms.start = chrono::Local::now().timestamp_millis();
    println!("{}", t!("console.startup", version = server.server_version));
    println!("{}", t!("console.warning"));
}

fn init_log() {
    undefined_redstone_log::set_locale("zh-CN");
}

pub(crate) fn init_properties(mut commands: Commands) {
    let mut current_dir = env::current_dir().unwrap();
    current_dir.push("server_properties.toml");
    commands.insert_resource(ServerProperties::load(current_dir).unwrap());
}

pub fn finish_startup(mut ms: ResMut<StartupTimestamp>) {
    ms.end = chrono::Local::now().timestamp_millis();
    let ms = ms.end - ms.start;
    println!("{}", t!("console.finish", ms = ms));
}

pub struct URStartupPlugin;
impl Plugin for URStartupPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(PreStartup, (URLogInitSet, URPreStartupSet.after(URLogInitSet), URStartupSet.after(URPreStartupSet)))
            .init_resource::<Server>()
            .init_resource::<StartupTimestamp>()
            .add_systems(PreStartup, init_log.in_set(URLogInitSet))
            .add_systems(PreStartup, startup.in_set(URPreStartupSet))
            .add_systems(PreStartup, init_properties.in_set(URStartupSet));
    }
}