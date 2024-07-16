use std::collections::HashMap;
use serde::Deserialize;
use uuid::Uuid;
use undefined_redstone_type::minecraft_version::{MinecraftVersion, MinecraftVersionEnum};
use crate::sub_pack::SubPackInformation;

#[derive(Clone, Debug)]
pub enum PackScope {
    World,
    Any
}

#[derive(Clone, Debug)]
pub enum ResourcePackCapabilities {
    Unknown(String),
    Chemistry,
    EditorExtension,
    ExperimentalCustomUI,
    Raytraced,
}

#[derive(Clone, Debug)]
pub struct ResourcePackDependencies {
    pub module_name: Option<String>,
    pub uuid: Option<Uuid>,
    pub version: Option<MinecraftVersion>,
}

#[derive(Clone, Debug)]
pub struct ResourcePackInformation {
    pub description: String,
    pub version: MinecraftVersion,
    pub uuid: Uuid,
    pub pack_len: usize,
    pub encryption_key: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ResourcePackMetadata {
    #[serde(default)]
    pub authors: Vec<String>,
    #[serde(default)]
    pub license: Option<String>,
    #[serde(default)]
    pub generated_with: Vec<HashMap<String, Vec<String>>>,
    #[serde(default)]
    pub product_type: Option<String>,
    #[serde(default)]
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
    pub metadata: Option<ResourcePackMetadata>
}

#[derive(Deserialize, Debug)]
pub(crate) struct ResourcePackManifestJsonHeader {
    #[serde(default)]
    pub allow_random_seed: bool,
    #[serde(default)]
    pub base_game_version: Option<MinecraftVersionEnum>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub lock_template_options: bool,
    #[serde(default)]
    pub min_engine_version: Option<MinecraftVersionEnum>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub pack_scope: Option<String>,
    #[serde(default)]
    pub uuid: String,
    pub version: MinecraftVersionEnum,
}

#[derive(Deserialize, Debug)]
pub(crate) struct ResourcePackManifestJsonModules {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(alias = "type")]
    #[serde(default)]
    pub module_type: String,
    #[serde(default)]
    pub uuid: String,
    #[serde(default)]
    pub version: Option<MinecraftVersionEnum>,
    #[serde(default)]
    pub language: Option<String>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct ResourcePackManifestJsonDependencies {
    #[serde(default)]
    pub uuid: Option<String>,
    #[serde(default)]
    pub module_name: Option<String>,
    #[serde(default)]
    pub version: Option<MinecraftVersionEnum>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct ResourcePackManifestJson {
    pub format_version: u16,
    #[serde(default)]
    pub header: Option<ResourcePackManifestJsonHeader>,
    #[serde(default)]
    pub modules: Vec<ResourcePackManifestJsonModules>,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(default)]
    pub dependencies: Vec<ResourcePackManifestJsonDependencies>,
    #[serde(default)]
    pub metadata: Option<ResourcePackMetadata>
}

impl ResourcePackManifestJson {
    fn to_option_version(version: Option<MinecraftVersionEnum>) -> Option<MinecraftVersion> {
        if let Some(version) = version {
            version.to_version().ok()
        } else {
            None
        }
    }
    pub fn to_manifest(self) -> anyhow::Result<ResourcePackManifest> {
        let header = self.header.unwrap();
        let uuid = Uuid::parse_str(&header.uuid).unwrap();
        let version = header.version.to_version()?;
        let pack_scope = match header.pack_scope {
            None => {
                None
            }
            Some(pack_scope) => {
                if pack_scope == "world" {
                    Some(PackScope::World)
                } else {
                    Some(PackScope::Any)
                }
            }
        };

        //JsonDependencies转Dependencies
        let dependencies: Vec<ResourcePackDependencies> = self.dependencies.iter().map(|dependency| {
            let uuid = dependency.uuid.as_ref().map(|uuid| Uuid::parse_str(uuid).unwrap());
            let module_name = dependency.module_name.clone();
            let version = dependency.version.clone();
            ResourcePackDependencies {
                module_name,
                uuid,
                version: Self::to_option_version(version),
            }
        }).collect();

        //JsonCapabilities转Capabilities
        let capabilities: Vec<ResourcePackCapabilities> = self.capabilities.iter().map(|capability| {
            match capability.as_str() {
                "chemistry" => ResourcePackCapabilities::Chemistry,
                "editor_extension" => ResourcePackCapabilities::EditorExtension,
                "experimental_custom_ui" => ResourcePackCapabilities::ExperimentalCustomUI,
                "raytraced" => ResourcePackCapabilities::Raytraced,
                _ => ResourcePackCapabilities::Unknown(capability.to_string()),
            }
        }).collect();

        Ok(ResourcePackManifest {
            format_version: self.format_version,
            name: header.name,
            information: ResourcePackInformation {
                description: header.description,
                version,
                uuid,
                pack_len: 0,
                encryption_key: String::new(),
            },
            min_engine_version: Self::to_option_version(header.min_engine_version),
            sub_packs: vec![],
            allow_random_seed: header.allow_random_seed,
            base_game_version: Self::to_option_version(header.base_game_version),
            lock_template_options: header.lock_template_options,
            pack_scope,
            dependencies,
            capabilities,
            metadata: self.metadata,
        })
    }
}