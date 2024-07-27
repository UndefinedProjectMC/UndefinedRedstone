use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use crate::world::MinecraftWorld;
use crate::world_loader::dir_loader::WorldDirectoryLoader;
use crate::world_loader::WorldLoaderTrait;

fn get_extension(filename: &Path) -> String {
    let path = filename.canonicalize().expect("WorldZippedLoader error");
    let filepath = path.to_str();
    let name = filepath.unwrap().split('/');
    let names: Vec<&str> = name.collect();
    let extension = names.last().expect("WorldZippedLoader error");
    let extension: Vec<&str> = extension.split(".").collect();
    extension[extension.len() - 1].to_string()
}

fn get_name(filename: &Path) -> String {
    let path = filename.canonicalize().expect("WorldZippedLoader error");
    let filepath = path.to_str();
    let name = filepath.unwrap().split('/');
    let names: Vec<&str> = name.collect();
    let extension = names.last().expect("WorldZippedLoader error");
    let extension: Vec<&str> = extension.split(".").collect();
    let x = extension[..extension.len() - 1].to_vec();
    //转string
    x.join("")
}


pub struct WorldZippedLoader {
    dir_path: PathBuf,
}

impl WorldZippedLoader {
    pub fn new<P: AsRef<Path>>(dir_path: P) -> Self {
        Self {
            dir_path: dir_path.as_ref().to_path_buf()
        }
    }
}

impl WorldLoaderTrait for WorldZippedLoader {
    fn get_worlds(&self) -> Vec<MinecraftWorld> {
        if let Ok(read_dir) = fs::read_dir(&self.dir_path) {
            for entry in read_dir {
                if let Ok(entry) = entry {
                    if let Ok(metadata) = entry.metadata() {
                        if metadata.is_file() {
                            let file_name = get_name(entry.path().as_path());
                            let extension = get_extension(entry.path().as_path());
                            let extension = extension.as_str();
                            if extension == "zip" || extension == "mcworld" {
                                if let Ok(mut file) = fs::File::open(entry.path()) {
                                    let mut bytes = vec![];
                                    if let Ok(_) = file.read_to_end(&mut bytes) {
                                        if let Ok(mut zip) = zip::ZipArchive::new(file) {
                                            let mut path = self.dir_path.clone();
                                            path.push(file_name);
                                            if !path.exists() {
                                                let _ = zip.extract(path);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            let dir_loader = WorldDirectoryLoader::new(self.dir_path.clone());
            dir_loader.get_worlds()
        }else {
            vec![]
        }
    }
}