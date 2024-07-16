use std::collections::HashMap;
use crate::entity::MinecraftEntityContent;
use crate::manifest::ResourcePackManifest;

#[derive(Clone, Debug)]
pub struct ResourcePack {
    pub manifest: ResourcePackManifest,
    pub entities: HashMap<String, MinecraftEntityContent>
}

impl ResourcePack {
    pub fn new(manifest: ResourcePackManifest, entities: HashMap<String, MinecraftEntityContent>) -> Self {
        Self {
            manifest,
            entities,
        }
    }
}