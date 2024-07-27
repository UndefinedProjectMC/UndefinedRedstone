use std::fs;
use std::path::PathBuf;
use serde::Serialize;
use binary_util::ByteReader;
use undefined_redstone_nbt::local::BedrockLocalNbt;
use undefined_redstone_nbt::NbtValue;
use undefined_redstone_nbt::reader::NbtReader;
use crate::world_data::MinecraftWorldData;

pub struct MinecraftWorldDataReader;

impl MinecraftWorldDataReader {
    pub fn new(file_path: PathBuf) -> anyhow::Result<MinecraftWorldData> {
        let file_bytes = fs::read(file_path)?;
        let mut byte_reader = ByteReader::from(file_bytes);

        //验证magic number是否为level.dat的magic number
        let _ = byte_reader.read_bytes(8)?;

        //读取level.dat
        let world_data = byte_reader.as_slice();
        let world_data = NbtReader::from_slice(world_data);
        Self::from_nbt(world_data.read::<BedrockLocalNbt>()?)
    }

    pub fn from_nbt(nbt: NbtValue) -> anyhow::Result<MinecraftWorldData> {
        let nbt = nbt.as_compound().ok_or(anyhow::anyhow!("Invalid level.dat"))?.clone();
        //临时解决办法, 先这样吧
        let json = serde_json::to_string(&nbt)?;
        let world_data: MinecraftWorldData = serde_json::from_str(json.as_str())?;
        Ok(world_data)
    }
}