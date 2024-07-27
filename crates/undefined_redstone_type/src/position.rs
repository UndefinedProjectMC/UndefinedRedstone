use std::fmt;
use std::fmt::Write;
use serde::de::{Error, SeqAccess, Visitor};
use serde::Deserialize;

#[derive(Debug, Copy, Clone)]
pub struct MinecraftPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl MinecraftPosition {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            x,
            y,
            z,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.x, self.y, self.z)
    }
}

impl<'de> Deserialize<'de> for MinecraftPosition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct MinecraftPositionVisitor;
        impl<'de> Visitor<'de> for MinecraftPositionVisitor
        {
            type Value = MinecraftPosition;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("[i32, i32, i32]")
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let x = seq.next_element()?;
                let y = seq.next_element()?;
                let z = seq.next_element()?;

                Ok(MinecraftPosition::new(x.unwrap(), y.unwrap(), z.unwrap()))
            }
        }
        deserializer.deserialize_any(MinecraftPositionVisitor {})
    }
}