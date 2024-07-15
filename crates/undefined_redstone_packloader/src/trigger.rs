use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct MinecraftTrigger {
    #[serde(default)]
    pub event: String,
    #[serde(default)]
    pub target: String,
}