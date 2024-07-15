use crate::entity::MinecraftEntityContent;
use serde::Deserialize;

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

types_export![MinecraftJsonTypesStruct, MinecraftJsonTypes,
    MinecraftEntityContent = "minecraft:entity",
];