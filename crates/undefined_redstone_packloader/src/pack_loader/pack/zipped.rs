use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, Read};
use std::ops::DerefMut;
use std::path::PathBuf;
use std::sync::Arc;
use chrono::Local;
use sha2::Digest;
use zip::read::ZipFile;
use zip::result::{ZipError, ZipResult};
use zip::ZipArchive;
use crate::manifest::{ResourcePackManifest, ResourcePackManifestJson};
use crate::{MinecraftJsonTypes, MinecraftJsonTypesStruct};
use crate::entity::MinecraftEntityContent;
use crate::pack::ResourcePack;

pub struct ZippedResourcePack;

fn get_file_by_name<'a>(zip: &'a mut ZipArchive<File>, path: &'static str) -> ZipResult<ZipFile<'a>> {
    // 创建一个中间变量来存储文件名
    let file_name = {
        zip.file_names()
            .find(|&name| {
                let str = if let Some(pos) = name.rfind('/') {
                    &name[pos + 1..] // 仅取最后的文件名部分
                } else {
                    name
                };
                str.eq_ignore_ascii_case(path) || str.ends_with(path)
            })
            .ok_or(ZipError::Io(Error::other("cannot get zip file")))?
            .to_string() // 转换为 String
    };
    zip.by_name(&file_name)
}

fn get_dir_files(zip: &mut ZipArchive<File>, dir_path: &str) -> Vec<String> {
    let mut vec = Vec::new();
    let dir_path = dir_path.to_ascii_lowercase();
    for i in 0..zip.len() {
        if let Ok(mut file) = zip.by_index(i) {
            if file.is_file() {
                let file_name = file.name().to_ascii_lowercase();
                if file_name.starts_with(&dir_path) && file_name[dir_path.len()..].starts_with('/') {
                    let mut content = String::new();
                    if file.read_to_string(&mut content).is_ok() {
                        vec.push(content);
                    }
                }
            }
        }
    }
    vec
}

impl ZippedResourcePack {
    fn get_manifest(zip: &mut ZipArchive<File>) -> Option<ResourcePackManifest> {
        let mut manifest = get_file_by_name(zip, "manifest.json").unwrap();
        if manifest.is_file() {
            let mut str = String::new();
            manifest.read_to_string(&mut str).ok()?;
            let manifest_json: ResourcePackManifestJson = serde_json::from_str(str.as_str()).unwrap();
            let result = manifest_json.to_manifest().ok();
            result
        }else {
            return None
        }
    }

    fn get_entities(zip: &mut ZipArchive<File>) -> HashMap<String, Arc<MinecraftEntityContent>> {
        let mut map = HashMap::new();
        let mut files = get_dir_files(zip, "entities");
        files.extend(get_dir_files(zip, "entity"));
        for json in files {
            if let Ok(types) = serde_json::from_str::<MinecraftJsonTypesStruct>(&json) {
                let types = types.get();
                match types {
                    MinecraftJsonTypes::Empty => {}
                    MinecraftJsonTypes::MinecraftEntityContent(content) => {
                        map.insert(content.description.identifier.clone(), Arc::new(content));
                    }
                }
            }
        }
        map
    }

    pub fn get_resource_pack(zip: &mut ZipArchive<File>, file_bytes: Vec<u8>) -> Option<ResourcePack> {
        /*
        let mut sha256 = sha2::Sha256::new();
        sha256.update(file_bytes.as_slice());
        let sha256 = sha256.finalize().to_vec();
         */
        //manifest
        let encryption_key = String::new();
        let mut manifest = Self::get_manifest(zip)?;
        manifest.information.encryption_key = encryption_key;
        manifest.information.pack_len = file_bytes.len();
        //entities
        let entities = Self::get_entities(zip);
        Some(ResourcePack::new(manifest, entities))
    }
}