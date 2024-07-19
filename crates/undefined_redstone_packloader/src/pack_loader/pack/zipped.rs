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
    let names = zip.file_names().collect::<Vec<&str>>().clone();
    let file_name =  names.iter().find(|str| {
        let split: Vec<&str> = str.split("/").collect();
        let str = split[..split.len()].join("/");
        str.eq_ignore_ascii_case(path) || str.ends_with(path)
    }).ok_or(ZipError::Io(Error::other("cannot get zip file")))?.to_string();
    let x = file_name.as_str();
    zip.by_name(x)
}

fn get_dir_files(zip: &mut ZipArchive<File>, dir_path: &str) -> Vec<String> {
    let mut vec = vec![];
    // 转移所有权, 防止出现重复引用
    let name = zip.file_names().map(|str| {
        str.to_owned()
    }).collect::<Vec<String>>();
    name.iter().map(|name| PathBuf::from(name))
        .filter(|path| path.components().count() > 1) //直接过滤掉不符合条件的路径
        .filter(|path| {
            let parent = path.parent().unwrap();
            parent.to_str().unwrap().eq_ignore_ascii_case(dir_path)
        })
        .for_each(|path_buf| {
            if let Ok(mut file) = zip.by_name(path_buf.to_str().unwrap()) {
                if file.is_file() {
                    let mut str = String::new();
                    if let Ok(_) = file.read_to_string(&mut str) {
                        vec.push(str);
                    }
                }
            }
        });
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
        let files = get_dir_files(zip, "entities");
        let mut map = HashMap::new();
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
        let mut sha256 = sha2::Sha256::new();
        sha256.update(file_bytes.as_slice());
        let sha256 = sha256.finalize().to_vec();
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