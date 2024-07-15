use std::io::Error;
use binary_util::{ByteReader, ByteWriter};
use binary_util::interfaces::{Reader, Writer};
use undefined_redstone_packloader::pack::ResourcePack;

#[derive(Clone, Debug)]
pub struct ResourcePackInfo {
    pub must_accept: bool,
    pub scripting: bool,
    pub has_addon_packs: bool,
    pub force_server_packs: bool,
    pub behavior_packs: Vec<ResourcePack>,
    pub resource_packs: Vec<ResourcePack>,

}

fn encode_packs(buf: &mut ByteWriter, packs: &Vec<ResourcePack>, behavior: bool) -> Result<(), Error> {
    buf.write_i16_le(packs.len() as i16)?;
    for pack in packs {
        let information = &pack.manifest.information;
        buf.write_string(&information.uuid.to_string())?;
        buf.write_string(&information.version.to_string())?;
        buf.write_u32_le(information.pack_len as u32)?;
        buf.write_string(&information.encryption_key)?;
        buf.write_string("")?; //TODO: sub-pack name
        buf.write_string("")?;//content identity
        buf.write_bool(false)?;//scripting
        if !behavior {
            buf.write_bool(false)?;//raytracing capable
        }
    }
    Ok(())
}

impl Writer for ResourcePackInfo {
    fn write(&self, buf: &mut ByteWriter) -> Result<(), Error> {
        buf.write_bool(self.must_accept)?;
        buf.write_bool(self.has_addon_packs)?;
        buf.write_bool(self.scripting)?;
        buf.write_bool(self.force_server_packs)?;
        encode_packs(buf, &self.behavior_packs, true)?;
        encode_packs(buf, &self.resource_packs, true)?;
        buf.write_var_u32(0)?;//cdn entries
        Ok(())
    }
}

impl Reader<ResourcePackInfo> for ResourcePackInfo {
    fn read(buf: &mut ByteReader) -> Result<ResourcePackInfo, Error> {
        todo!()
    }
}
#[derive(Clone, Debug)]
pub struct ExperimentData {
    pub name: String,
    pub is_enabled: bool
}

impl ExperimentData {
    pub fn new(name: &str, is_enabled: bool) -> Self {
        Self {
            name: name.to_string(),
            is_enabled,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ResourcePackStack {
    pub must_accept: bool,
    pub behavior_pack_stack: Vec<ResourcePack>,
    pub resource_pack_stack: Vec<ResourcePack>,
    pub experiments: Vec<ExperimentData>,
    pub game_version: String,
    pub is_has_editor_packs: bool,
}

impl Writer for ResourcePackStack {
    fn write(&self, buf: &mut ByteWriter) -> Result<(), Error> {
        buf.write_bool(self.must_accept)?;
        buf.write_var_i32(self.behavior_pack_stack.len() as i32)?;
        for pack in &self.behavior_pack_stack {
            let information = &pack.manifest.information;
            buf.write_string(&information.uuid.to_string())?;
            buf.write_string(&information.version.to_string())?;
            buf.write_string("")?; //TODO: sub-pack name
        }

        buf.write_var_i32(self.resource_pack_stack.len() as i32)?;
        for pack in &self.resource_pack_stack {
            let information = &pack.manifest.information;
            buf.write_string(&information.uuid.to_string())?;
            buf.write_string(&information.version.to_string())?;
            buf.write_string("")?; //TODO: sub-pack name
        }

        buf.write_string(self.game_version.as_str())?;
        buf.write_i32_le(self.experiments.len() as i32)?;
        for experiment in &self.experiments {
            buf.write_string(experiment.name.as_str())?;
            buf.write_bool(experiment.is_enabled)?;
        }
        buf.write_bool(true)?;//Were experiments previously toggled
        buf.write_bool(self.is_has_editor_packs)?;
        Ok(())
    }
}

impl Reader<ResourcePackStack> for ResourcePackStack {
    fn read(buf: &mut ByteReader) -> Result<ResourcePackStack, Error> {
        todo!()
    }
}