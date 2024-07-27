use std::fs;
use std::path::PathBuf;
use crate::data_reader::MinecraftWorldDataReader;
use crate::world::MinecraftWorld;

pub struct DirectoryWorld;

impl DirectoryWorld {
    pub fn get_world(path: PathBuf) -> anyhow::Result<MinecraftWorld> {
        //world name
        let mut level_name_path = path.clone();
        level_name_path.push("levelname.txt");
        let world_name = fs::read_to_string(level_name_path).unwrap_or("UndefinedRedstone Level".to_string());

        //world data
        let mut level_dat_path = path.clone();
        level_dat_path.push("level.dat");
        let world_data = MinecraftWorldDataReader::new(level_dat_path).unwrap();

        Ok(MinecraftWorld {
            world_name,
            world_data,
        })
    }
}