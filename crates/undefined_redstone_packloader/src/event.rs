use serde::Deserialize;
use crate::filter::MinecraftFilterType;

#[derive(Deserialize, Debug, Default)]
pub struct MinecraftEvent {
    #[serde(default)]
    pub filters: MinecraftFilterType,
    #[serde(default)]
    pub event: String,
    #[serde(default)]
    pub target: String,
}