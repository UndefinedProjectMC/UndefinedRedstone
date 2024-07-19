use std::collections::HashMap;
use std::sync::Arc;
use crate::entity::MinecraftEntityContent;
use crate::manifest::ResourcePackManifest;

#[derive(Clone, Debug)]
pub struct ResourcePack {
    pub manifest: ResourcePackManifest,
    pub entities: HashMap<String, Arc<MinecraftEntityContent>>
}

impl ResourcePack {
    pub fn new(manifest: ResourcePackManifest, entities: HashMap<String, Arc<MinecraftEntityContent>>) -> Self {
        Self {
            manifest,
            entities,
        }
    }
}