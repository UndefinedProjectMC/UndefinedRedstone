pub mod zipped_loader;
pub mod pack;

use crate::pack::ResourcePack;

pub trait PackLoaderTrait {
    fn get_resource_packs(&self) -> Vec<ResourcePack>;
}