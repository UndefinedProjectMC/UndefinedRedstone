use std::collections::HashMap;
use uuid::Uuid;
use undefined_redstone_type::minecraft_version::MinecraftVersion;
use crate::sub_pack::SubPackInformation;

#[derive(Clone, Debug)]
pub enum PackScope {
    World,
    Any
}

#[derive(Clone, Debug)]
pub enum ResourcePackCapabilities {
    Chemistry,
    EditorExtension,
    ExperimentalCustomUI,
    Raytraced,
}

#[derive(Clone, Debug)]
pub struct ResourcePackDependencies {
    pub module_name: String,
    pub uuid: Uuid,
    pub version: MinecraftVersion,
}

#[derive(Clone, Debug)]
pub struct ResourcePackInformation {
    pub description: String,
    pub version: MinecraftVersion,
    pub uuid: Uuid,
    pub pack_len: usize,
    pub encryption_key: String,
}

#[derive(Clone, Debug)]
pub struct ResourcePackMetadata {
    pub authors: Vec<String>,
    pub license: Option<String>,
    pub generated_with: Vec<HashMap<String, Vec<String>>>,
    pub product_type: Option<String>,
    pub url: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ResourcePackManifest {
    pub format_version: u16,
    pub name: String,
    pub information: ResourcePackInformation,
    pub min_engine_version: Option<MinecraftVersion>,
    pub sub_packs: Vec<SubPackInformation>,
    pub allow_random_seed: bool,
    pub base_game_version: Option<MinecraftVersion>,
    pub lock_template_options: bool,
    pub pack_scope: Option<PackScope>,
    pub dependencies: Vec<ResourcePackDependencies>,
    pub capabilities: Vec<ResourcePackCapabilities>,
}