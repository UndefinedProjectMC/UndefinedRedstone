use std::io;
use std::str::FromStr;

#[derive(PartialEq, Eq, Ord, PartialOrd, Clone, Copy, Debug)]
pub struct MinecraftVersion {
    v0: u8,
    v1: u8,
    v2: u8,
}

impl MinecraftVersion {
    pub fn new(v0: u8, v1: u8, v2: u8) -> Self {
        Self {
            v0,
            v1,
            v2
        }
    }

    pub fn from_str(version: &str) -> anyhow::Result<Self> {
        let split: Vec<&str> = version.split(".").collect();
        if split.len() != 3 {
            return Err(anyhow::Error::msg("unknown version"))
        }
        Ok(Self {
            v0: u8::from_str(split[0].trim())?,
            v1: u8::from_str(split[1].trim())?,
            v2: u8::from_str(split[2].trim())?
        })
    }

    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.v0, self.v1, self.v2)
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct MinecraftVersions {
    vec: Vec<MinecraftVersion>
}

impl MinecraftVersions {
    pub fn new() -> Self {
        Self {
            vec: vec![]
        }
    }

    pub fn from_vec(vec: Vec<MinecraftVersion>) -> Self {
        let mut vec = vec;
        vec.sort();
        Self {
            vec
        }
    }

    pub fn single(version: MinecraftVersion) -> Self {
        Self::from_vec(vec![version])
    }

    pub fn from_str(version: &str) -> anyhow::Result<Self> {
        Ok(Self::single(MinecraftVersion::from_str(version)?))
    }

    pub fn push(&mut self, version: MinecraftVersion) {
        self.vec.push(version);
        self.vec.sort();
    }

    pub fn remove(&mut self, version: MinecraftVersion) {
        if let Some(i) = self.vec.iter().position(|x| *x == version) {
            self.vec.remove(i);
        }
    }

    pub fn get_min_version(&self) -> Option<MinecraftVersion> {
        self.vec.first().cloned()
    }

    pub fn get_max_version(&self) -> Option<MinecraftVersion> {
        self.vec.last().cloned()
    }
}