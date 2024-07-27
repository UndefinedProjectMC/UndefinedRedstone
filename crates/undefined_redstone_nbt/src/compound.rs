use std::collections::HashMap;
use std::fmt;
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::NbtValue;

#[derive(Clone, Debug)]
pub struct CompoundNbt {
    pub name: Option<String>,
    map: HashMap<String, NbtValue>
}

impl CompoundNbt {
    pub fn new(name: Option<String>) -> Self {
        Self {
            name,
            map: HashMap::new()
        }
    }

    pub fn from_map(name: Option<String>, map: HashMap<String, NbtValue>) -> Self {
        Self {
            name,
            map
        }
    }

    pub fn get(&self, key: &str) -> Option<&NbtValue> {
        self.map.get(key)
    }

    pub fn insert(&mut self, key: &str, value: NbtValue) -> &mut Self {
        self.map.insert(key.to_string(), value);
        self
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &NbtValue)> {
        self.map.iter()
    }
}

impl Serialize for CompoundNbt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut map = serializer.serialize_map(Some(self.map.len()))?;
        for (key, value) in &self.map {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}