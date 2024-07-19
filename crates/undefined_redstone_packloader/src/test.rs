use std::fs;
use bevy_ecs::world::World;
use regex::Regex;
use crate::{MinecraftJsonTypes, MinecraftJsonTypesStruct};
use undefined_redstone_type::display_name::DisplayName;
use crate::entity::EntityContentManager;
use crate::pack_loader::PackLoaderTrait;
use crate::pack_loader::zipped_loader::ResourcePackZippedLoader;
use crate::pack_manager::ResourcePackManager;

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
    let mut world = World::new();

    //加载行为包
    let loader = ResourcePackZippedLoader::new("D:/RustRoverProjects/undefined_redstone/behavior_packs/");
    let vec = loader.get_resource_packs();

    //初始化manager
    let manager = ResourcePackManager::from_vec(vec);
    let content_manager = EntityContentManager::from_map(manager.get_entities());

    //生成玩家
    let content = content_manager.get("minecraft:player").unwrap();
    println!("{:#?}", content);
    let mut player = content.spawn(&mut world);
    player.insert(DisplayName("YouZikua".to_string()));
}