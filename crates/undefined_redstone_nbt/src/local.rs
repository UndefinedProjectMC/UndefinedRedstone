use std::collections::HashMap;
use binary_util::ByteReader;
use crate::compound::CompoundNbt;
use crate::NbtValue;
use crate::reader::NbtReadTrait;

pub struct BedrockLocalNbt;

impl BedrockLocalNbt {
    #[inline]
    fn read_string(reader: &mut ByteReader) -> anyhow::Result<String> {
        let len = reader.read_i16_le()?;
        let vec = reader.read_bytes(len as usize)?.to_vec();
        Ok(String::from_utf8(vec)?)
    }
    #[inline]
    fn read_i8_array(reader: &mut ByteReader) -> anyhow::Result<Vec<i8>> {
        let len = reader.read_i32_le()?;
        Ok(reader.read_i8_bytes(len as usize)?)
    }

    #[inline]
    fn read_i32_le_array(reader: &mut ByteReader) -> anyhow::Result<Vec<i32>> {
        let len = reader.read_i32_le()?;
        Ok(reader.read_i32_le_array(len as usize)?)
    }

    fn read_i64_le_array(reader: &mut ByteReader) -> anyhow::Result<Vec<i64>> {
        let len = reader.read_i32_le()?;
        Ok(reader.read_i64_le_array(len as usize)?)
    }
    #[inline]
    fn read_any(reader: &mut ByteReader, type_id: u8) -> anyhow::Result<NbtValue> {
        match type_id {
            1 => Ok(NbtValue::Byte(reader.read_i8()?)),
            2 => Ok(NbtValue::Short(reader.read_i16_le()?)),
            3 => Ok(NbtValue::Int(reader.read_i32_le()?)),
            4 => Ok(NbtValue::Long(reader.read_i64_le()?)),
            5 => Ok(NbtValue::Float(reader.read_f32_le()?)),
            6 => Ok(NbtValue::Double(reader.read_f64_le()?)),
            7 => Ok(NbtValue::ByteArray(Self::read_i8_array(reader)?)),
            8 => Ok(NbtValue::String(Self::read_string(reader)?)),
            9 => Ok(NbtValue::List(Self::read_list(reader)?)),
            10 => Ok(NbtValue::Compound(Self::read_compound(reader, None)?)),
            11 => Ok(NbtValue::IntArray(Self::read_i32_le_array(reader)?)),
            12 => Ok(NbtValue::LongArray(Self::read_i64_le_array(reader)?)),
            _ => Err(anyhow::anyhow!("Invalid NBT type: {:?}", type_id))
        }
    }
    #[inline]
    fn read_list(reader: &mut ByteReader) -> anyhow::Result<Vec<NbtValue>> {
        let type_id = reader.read_u8()?;
        let len = reader.read_i32_le()?;
        let mut list = Vec::with_capacity(len as usize);
        for _ in 0..len {
            list.push(Self::read_any(reader, type_id)?);
        }
        Ok(list)
    }

    #[inline]
    fn read_compound(reader: &mut ByteReader, name: Option<String>) -> anyhow::Result<CompoundNbt> {
        let mut map = HashMap::new();
        while let Ok(type_id) = reader.read_u8() {
            if type_id == 0 {
                break;
            }
            let key = Self::read_string(reader)?;
            let value = Self::read_any(reader, type_id)?;
            map.insert(key, value);
        }
        Ok(CompoundNbt::from_map(name, map))
    }
}

impl NbtReadTrait for BedrockLocalNbt {
    fn read(mut reader: ByteReader) -> anyhow::Result<NbtValue> {
        match reader.read_u8()? {
            9 => Ok(NbtValue::List(Self::read_list(&mut reader)?)),
            10 => {
                let name = Self::read_string(&mut reader)?;
                Ok(NbtValue::Compound(Self::read_compound(&mut reader, Some(name))?))
            },
            _ => Err(anyhow::anyhow!("Invalid NBT tag type"))
        }
    }
}