use crate::world::MinecraftWorld;

pub mod zipped_loader;
mod worlds;
pub mod dir_loader;

pub trait WorldLoaderTrait {
    fn get_worlds(&self) -> Vec<MinecraftWorld>;
}