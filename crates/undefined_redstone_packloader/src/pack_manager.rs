use std::collections::HashMap;
use std::sync::Arc;
use bevy_ecs::system::Resource;
use crate::entity::MinecraftEntityContent;
use crate::pack::ResourcePack;

#[derive(Resource)]
pub struct ResourcePackManager {
    pub packs: Vec<ResourcePack>,
}

impl ResourcePackManager {
    pub fn new() -> Self {
        Self {
            packs: vec![],
        }
    }

    pub fn from_vec(packs: Vec<ResourcePack>) -> Self {
        Self {
            packs,
        }
    }

    pub fn push(&mut self, pack: ResourcePack) {
        self.packs.push(pack);
    }

    pub fn extends(&mut self, packs: Vec<ResourcePack>) {
        self.packs.extend(packs);
    }

    pub fn get_entities(&self) -> HashMap<String, Arc<MinecraftEntityContent>> {
        let mut map = HashMap::new();
        for pack in &self.packs {
            map.extend(pack.entities.clone());
        }
        return map
    }
}