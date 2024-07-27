use serde::{Deserialize, Serialize};
use crate::compound::CompoundNbt;

pub mod compound;
pub mod reader;
pub mod local;

#[derive(Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum NbtValue {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(Vec<NbtValue>),
    Compound(CompoundNbt),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl NbtValue {
    pub fn as_i8(&self) -> Option<i8> {
        match self {
            NbtValue::Byte(value) => Some(*value),
            _ => None
        }
    }

    pub fn as_i16(&self) -> Option<i16> {
        match self {
            NbtValue::Short(value) => Some(*value),
            _ => None
        }
    }

    pub fn as_i32(&self) -> Option<i32> {
        match self {
            NbtValue::Int(value) => Some(*value),
            _ => None
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            NbtValue::Long(value) => Some(*value),
            _ => None
        }
    }

    pub fn as_f32(&self) -> Option<f32> {
        match self {
            NbtValue::Float(value) => Some(*value),
            _ => None
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            NbtValue::Double(value) => Some(*value),
            _ => None
        }
    }

    pub fn as_i8_array(&self) -> Option<&Vec<i8>> {
        match self {
            NbtValue::ByteArray(value) => Some(value),
            _ => None
        }
    }

    pub fn as_string(&self) -> Option<&String> {
        match self {
            NbtValue::String(value) => Some(value),
            _ => None
        }
    }

    pub fn as_list(&self) -> Option<&Vec<NbtValue>> {
        match self {
            NbtValue::List(value) => Some(value),
            _ => None
        }
    }

    pub fn as_compound(&self) -> Option<&CompoundNbt> {
        match self {
            NbtValue::Compound(value) => Some(value),
            _ => None
        }
    }

    pub fn as_i32_array(&self) -> Option<&Vec<i32>> {
        match self {
            NbtValue::IntArray(value) => Some(value),
            _ => None
        }
    }

    pub fn as_i64_array(&self) -> Option<&Vec<i64>> {
        match self {
            NbtValue::LongArray(value) => Some(value),
            _ => None
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        let b = self.as_i8()?;
        Some(b != 0)
    }
}

pub fn deserialize_nbt_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: serde::Deserializer<'de>,
{
    let b = i8::deserialize(deserializer)?;
    Ok(b != 0)
}