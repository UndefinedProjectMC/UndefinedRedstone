use std::env;
use bevy_app::{App, Plugin, PreStartup};
use bevy_ecs::prelude::{IntoSystemConfigs, SystemSet};
use bevy_ecs::system::Commands;
use undefined_redstone_core::startup::URStartupSet;
use crate::world_loader::WorldLoaderTrait;
use crate::world_loader::zipped_loader::WorldZippedLoader;
use bevy_ecs::schedule::IntoSystemSetConfigs;
use chrono::Local;
use undefined_redstone_log::t;
use crate::world_loader::dir_loader::WorldDirectoryLoader;

pub mod world_loader;
pub mod world;
pub mod data_reader;
pub mod world_data;
pub mod world_layers;
pub mod world_policies;

pub struct URWorldManager {
}

pub struct URWorldPlugin;

#[derive(SystemSet, Debug, Default, Hash, Eq, PartialEq, Clone)]
pub struct URWorldSet;

impl Plugin for URWorldPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            PreStartup,
            URWorldSet.after(URStartupSet)
        )
            .add_systems(PreStartup, load_world.in_set(URWorldSet));
    }
}

fn load_world(mut command: Commands) {
    //world
    let start = Local::now().timestamp_millis();
    let mut current_dir = env::current_dir().unwrap();
    current_dir.push("worlds");
    let loader = WorldDirectoryLoader::new(current_dir);
    let worlds = loader.get_worlds();
    for world in &worlds {
        println!("[DEBUG] world name(FILE): {}", world.world_name);
        println!("[DEBUG] world name(NBT): {}", world.world_data.world_name);
    }
    let end = Local::now().timestamp_millis();

    println!("{}", t!("console.level.load", count = worlds.len(), ms = end - start));
}
