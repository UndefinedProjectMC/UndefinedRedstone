use std::env;
use bevy_app::{App, Plugin, PreStartup, Startup};
use bevy_ecs::schedule::IntoSystemSetConfigs;
use bevy_ecs::prelude::{IntoSystemConfigs, SystemSet};
use bevy_ecs::system::Commands;
use chrono::Local;
use crate::entity::{EntityContentManager, MinecraftEntityContent};
use serde::Deserialize;
use undefined_redstone_core::startup::{finish_startup, URStartupSet};
use undefined_redstone_log::t;
use crate::pack_loader::PackLoaderTrait;
use crate::pack_loader::zipped_loader::ResourcePackZippedLoader;
use crate::pack_manager::ResourcePackManager;

pub mod manifest;
pub mod pack_loader;
pub mod sub_pack;
pub mod pack;
pub mod block;
pub mod entity;
pub mod event;
pub mod filter;
pub mod range;
pub mod molang;
pub mod block_states;
pub mod trigger;
pub mod r#macro;
#[cfg(test)]
pub mod test;
pub mod component_group;
pub mod entity_types;
mod pack_manager;

types_export![MinecraftJsonTypesStruct, MinecraftJsonTypes,
    MinecraftEntityContent = "minecraft:entity",
];

#[derive(SystemSet, Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub struct URPackLoadSet;

pub struct URPackLoaderPlugin;

impl Plugin for URPackLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            PreStartup,
            URPackLoadSet.after(URStartupSet)
        )
            .add_systems(PreStartup, load_pack.in_set(URPackLoadSet));
    }
}

fn load_pack(mut command: Commands) {
    //resource packs
    let start = Local::now().timestamp_millis();
    let mut current_dir = env::current_dir().unwrap();
    current_dir.push("resource_packs");
    let zipped_loader = ResourcePackZippedLoader::new(current_dir);
    let packs = zipped_loader.get_resource_packs();
    let end = Local::now().timestamp_millis();
    println!("{}", t!("console.resource_pack", count = packs.len(), ms = end - start));
    let mut manager = ResourcePackManager::from_vec(packs);

    //behavior packs
    let start = Local::now().timestamp_millis();
    let mut current_dir = env::current_dir().unwrap();
    current_dir.push("behavior_packs");
    let zipped_loader = ResourcePackZippedLoader::new(current_dir);
    let packs = zipped_loader.get_resource_packs();
    let end = Local::now().timestamp_millis();
    println!("{}", t!("console.behavior_pack", count = packs.len(), ms = end - start));
    manager.packs.extend(packs);

    let entity_manager = EntityContentManager::from_map(manager.get_entities());

    println!("{}", t!("console.resource_pack.entities", count = entity_manager.len()));

    command.insert_resource(manager);
    command.insert_resource(entity_manager);
}
