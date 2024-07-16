use std::fs;
use chrono::Local;
use regex::Regex;
use crate::{MinecraftJsonTypes, MinecraftJsonTypesStruct};
use crate::pack_loader::PackLoaderTrait;
use crate::pack_loader::zipped_loader::ResourcePackZippedLoader;

#[test]
fn create_component() {
    let x = fs::read_to_string("D:/RustRoverProjects/undefined_redstone/behavior_component.txt").unwrap();
    let x: Vec<&str> = x.split("\r\n").collect();
    //regex
    let re = Regex::new(r"[_.]").unwrap();
    let mut final_text = vec![];
    for name in x {
        let split: Vec<&str> = re.split(&name).collect();
        let mut final_case = String::new();
        //转驼峰
        for s in split {
            let t = s.to_ascii_uppercase();
            let tx = format!("{}{}", &t[..1], &s[1..]);
            final_case += tx.as_str();
        }
        let minecraft_name = format!("minecraft:{}", name);
        final_text.push(format!("   {} = \"{}\",", final_case, minecraft_name))
    }
    println!("{}", final_text.join("\n"));
}

#[test]
fn test_component() {
    let json = fs::read_to_string("D:/RustRoverProjects/undefined_redstone/allay.json").unwrap();
    let types: MinecraftJsonTypesStruct = serde_json::from_str(&json).unwrap();
    let types = types.get();
    match types {
        MinecraftJsonTypes::Empty => {
            println!("empty")
        }
        MinecraftJsonTypes::MinecraftEntityContent(entity_content) => {
            println!("entity: {:#?}", entity_content);
        }
    }
}

#[test]
fn test_loader() {
    let loader = ResourcePackZippedLoader::new("C:/Users/NiuBi/Desktop/resource_packs/");
    let vec = loader.get_resource_packs();
    for i in vec {
        println!("{:#?}", i)
    }
}