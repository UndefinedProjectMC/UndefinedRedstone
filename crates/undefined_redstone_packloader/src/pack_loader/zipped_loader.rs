use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use chrono::Local;
use crate::pack::ResourcePack;
use crate::pack_loader::pack::zipped::ZippedResourcePack;
use crate::pack_loader::PackLoaderTrait;

fn get_extension(filename: &Path) -> String {
    let path = filename.canonicalize().expect("ResourcePackZippedLoader error");
    let filepath = path.to_str();
    let name = filepath.unwrap().split('/');
    let names: Vec<&str> = name.collect();
    let extension = names.last().expect("ResourcePackZippedLoader error");
    let extension: Vec<&str> = extension.split(".").collect();
    extension[extension.len() - 1].to_string()
}


pub struct ResourcePackZippedLoader {
    dir_path: PathBuf,
}

impl ResourcePackZippedLoader {
    pub fn new<P: AsRef<Path>>(dir_path: P) -> Self {
        Self {
            dir_path: dir_path.as_ref().to_path_buf()
        }
    }
}

impl PackLoaderTrait for ResourcePackZippedLoader {
    fn get_resource_packs(&self) -> Vec<ResourcePack> {
        if let Ok(read_dir) = fs::read_dir(&self.dir_path) {
            let mut resource_packs = vec![];
            for entry in read_dir {
                if let Ok(entry) = entry {
                    if let Ok(metadata) = entry.metadata() {
                        if metadata.is_file() {
                            let extension = get_extension(entry.path().as_path());
                            let extension = extension.as_str();
                            if extension == "zip" || extension == "mcpack" {
                                if let Ok(mut file) = fs::File::open(entry.path()) {
                                    let mut bytes = vec![];
                                    if let Ok(_) = file.read_to_end(&mut bytes) {
                                        let start = Local::now().timestamp_millis();
                                        if let Ok(mut zip) = zip::ZipArchive::new(file) {
                                            let end = Local::now().timestamp_millis();
                                            if let Some(resource_pack) = ZippedResourcePack::get_resource_pack(&mut zip, bytes) {
                                                resource_packs.push(resource_pack)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            resource_packs
        }else {
            vec![]
        }
    }
}