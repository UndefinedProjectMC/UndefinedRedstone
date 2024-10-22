use serde::Deserialize;
use crate::block_states::MinecraftBlockStates;

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum MinecraftBlockType {
    BlockName(String),
    Block(MinecraftBlock)
}

impl Default for MinecraftBlockType {
    fn default() -> Self {
        MinecraftBlockType::BlockName("minecraft:air".to_string())
    }
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct MinecraftBlock {
    pub name: String,
    pub states: MinecraftBlockStates,
}