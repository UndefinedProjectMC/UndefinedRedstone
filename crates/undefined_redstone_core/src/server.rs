use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use bevy_ecs::entity::Entity;
use bevy_ecs::system::Resource;
use uuid::Uuid;
use undefined_redstone_type::gamemode::Gamemode;
use undefined_redstone_nbt::compound::CompoundNbt;
use undefined_redstone_nbt::NbtValue;

#[derive(Resource, Clone)]
pub struct Server {
    pub server_version: String,
    clients: HashMap<Uuid, Entity>,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            server_version: "1.0.0ALPHA(fjord)".to_string(),
            clients: HashMap::new(),
        }
    }
}

impl Server {
    pub fn push_client(&mut self, uuid: Uuid, entity: Entity) {
        self.clients.insert(uuid, entity);
    }

    pub fn get_online_clients_mut(&mut self) -> &mut HashMap<Uuid, Entity> {
        &mut self.clients
    }

    pub fn get_online_clients(&self) -> &HashMap<Uuid, Entity> {
        &self.clients
    }

    pub fn get_client(&self, uuid: Uuid) -> Option<Entity> {
        self.clients.get(&uuid).cloned()
    }

    pub fn get_offline_player_nbt(&self, uuid: Uuid, default_gamemode: Gamemode) -> Option<CompoundNbt> {
        if uuid.is_nil() { return None }
        //搜索db

        //如果没搜索到，就创建
        let now_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
        let mut nbt = CompoundNbt::new(None);
        nbt.insert("firstPlayed", NbtValue::Long(now_time))//第一次游玩时间
            .insert("lastPlayed", NbtValue::Long(now_time))//最后一次游玩时间
            .insert("Pos", NbtValue::List(vec![
                NbtValue::Double(0.0),
                NbtValue::Double(60.0),
                NbtValue::Double(0.0),
            ]))//玩家最后一次所在的位置
            .insert("Level", NbtValue::String("UndefinedRedstone Level".to_string()))//所在世界的名称
            .insert("Inventory", NbtValue::List(vec![]))//物品栏
            .insert("Achievements", NbtValue::Compound(CompoundNbt::new(None)))//成就
            .insert("EXP", NbtValue::Int(0))//经验值
            .insert("expLevel", NbtValue::Int(0))//经验等级
            .insert("playerGameType", NbtValue::Int(default_gamemode.to_i32()))
            .insert("Motion", NbtValue::List(vec![
                NbtValue::Double(0.0),
                NbtValue::Double(0.0),
                NbtValue::Double(0.0),
            ]))
            .insert("Rotation", NbtValue::List(vec![
                NbtValue::Float(0.0),
                NbtValue::Float(0.0),
            ]))
            .insert("FallDistance", NbtValue::Float(0.0))
            .insert("Fire", NbtValue::Short(0))
            .insert("Air", NbtValue::Short(300))
            .insert("OnGround", NbtValue::Byte(1))
            .insert("Invulnerable", NbtValue::Byte(0));
        //保存玩家nbt

        Some(nbt)
    }
}