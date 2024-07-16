use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, Read};
use std::ops::DerefMut;
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

fn get_dir_files(zip: &mut ZipArchive<File>, dir_path: &'static str) -> Vec<String> {
    //转移所有权, 防止出现重复引用
    let names = {
        zip.file_names().collect::<Vec<&str>>()
            .iter()
            .map(|str| {
                str.to_string()
            })
            .collect::<Vec<String>>()
    };
    let mut vec = vec![];
    for name in names {
        let split: Vec<&str> = name.split("/").collect();
        if split.len() > 1 {
            let str = split[..split.len() - 1].join("/");
            if str.eq_ignore_ascii_case(dir_path) {
                if let Ok(mut file) = zip.by_name(&name) {
                    if file.is_file() {
                        let mut str = String::new();
                        if let Ok(_) = file.read_to_string(&mut str) {
                            vec.push(str)
                        }
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
            manifest_json.to_manifest().ok()
        }else {
            return None
        }
    }

    fn get_entities(zip: &mut ZipArchive<File>) -> HashMap<String, MinecraftEntityContent> {
        let files = get_dir_files(zip, "entities");
        let mut map = HashMap::new();
        for json in files {
            if let Ok(types) = serde_json::from_str::<MinecraftJsonTypesStruct>(&json) {
                let types = types.get();
                match types {
                    MinecraftJsonTypes::Empty => {}
                    MinecraftJsonTypes::MinecraftEntityContent(content) => {
                        map.insert(content.description.identifier.clone(), content);
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