use std::io::Error;
use std::str::FromStr;
use base64::Engine;
use binary_util::{ByteReader, ByteWriter};
use binary_util::interfaces::{Reader, Writer};
use hmac::{Hmac, KeyInit};
use jwt::{Header, Token, VerifyWithKey};
use serde_json::Value;
use sha2::Sha384;
use uuid::Uuid;
use crate::encryption::MinecraftEncryption;
use crate::skin::Skin;

#[derive(Clone, Debug)]
pub struct Login {
    pub username: String,
    pub protocol: u32,
    pub identity: Uuid,
    pub identity_public_key: String,
    pub client_id: i64,
    pub skin: Skin,
    pub issue_unix_time: i64,
    pub device_os: u8,
    pub device_model: String,
    pub language_code: String,
    pub game_version: String,
    pub server_address: String,
    pub auth: bool,
    pub verified: bool,
}

impl Reader<Login> for Login {
    fn read(buf: &mut ByteReader) -> Result<Login, Error> {
        let mut protocol = buf.read_u32()?;
        if protocol == 0 {
            buf.read_u16()?;
            protocol = buf.read_u32()?;
        }
        let mut issue_unix_time = -1;
        let mut username = String::new();
        let mut identity = Uuid::nil();
        let mut identity_public_key = String::new();
        let mut client_id = 0;
        let mut skin = None;
        let mut device_os = 0;
        let mut device_model = String::new();
        let mut language_code = String::new();
        let mut game_version = String::new();
        let mut server_address = String::new();
        let buf = buf.read_sized_slice()?;
        let mut buf = ByteReader::from(buf);
        //解析chain data
        let len = buf.read_u32_le()? as usize;

        let chain_data = buf.read_bytes(len)?;
        let chain_data = chain_data.as_ref();

        let mut auth = false;
        let mut verified = false;

        let map = serde_json::from_slice::<Value>(chain_data).map_err(|error| Error::other(error))?.as_object()
            .ok_or(Error::other("Cannot decode json"))?.clone();
        let chains = map.get("chain").ok_or(Error::other("Cannot decode json"))?
            .as_array().ok_or(Error::other("Cannot decode json"))?.clone();
        for chain in chains {
            let chain = chain.as_str().ok_or(Error::other("Cannot decode json"))?;
            //验证
            let token: Token<Header, Value, _> = Token::parse_unverified(chain).map_err(|error| Error::other(error))?;
            if let Some(chain_map) = decode(chain, 0) {
                let x5u_value = chain_map.get("x5u").ok_or(Error::other("Cannot decode json"))?;
                let x5u = x5u_value.as_str().ok_or(Error::other("Cannot decode json"))?;
                if x5u == MinecraftEncryption::MOJANG_PUBLIC_KEY {
                    auth = true;
                }else {
                    auth = false;
                }
                let key: Hmac<Sha384> = Hmac::new_from_slice(MinecraftEncryption::MOJANG_PUBLIC_KEY.as_bytes()).unwrap();
                if let Ok(_) = token.verify_with_key(&key) {
                    verified = true;
                } else {
                    verified = false;
                }
            }

            //获取data
            if let Some(chain_map) = decode(chain, 1) {
                println!("json: {}", chain_map.to_string());
                let chain_map = chain_map.as_object().ok_or(Error::other("Cannot decode json"))?;
                if let Some(extra_data) = chain_map.get("extraData") {
                    let extra_data = extra_data.as_object().ok_or(Error::other("Cannot decode json"))?;
                    if let Some(iat) = chain_map.get("iat") {
                        issue_unix_time = iat.as_i64().ok_or(Error::other("Cannot decode json"))? * 1000;
                    }
                    if let Some(display_name) = extra_data.get("displayName") {
                        username = display_name.as_str().ok_or(Error::other("Cannot decode json"))?.to_string();
                    }
                    if let Some(ident) = extra_data.get("identity") {
                        identity = Uuid::from_str(ident.as_str().ok_or(Error::other("Cannot decode json"))?).map_err(|_| Error::other("Cannot decode json"))?;

                    }
                }
                if let Some(key) = chain_map.get("identityPublicKey") {
                    identity_public_key = key.as_str().ok_or(Error::other("Cannot decode json"))?.to_string();
                }
            }
        }
        //解析skin data
        let len = buf.read_u32_le()? as usize;
        let skin_data = buf.read_bytes(len)?;
        let skin_data = skin_data.as_ref();
        let skin_data = String::from_utf8_lossy(skin_data).to_string();
        let skin_data = skin_data.as_str();
        let skin_token = decode(skin_data, 1).ok_or(Error::other("Cannot decode json"))?;
        if let Some(id) = skin_token.get("ClientRandomId") {
            client_id = id.as_i64().ok_or(Error::other("Cannot decode json"))?;
        }
        if let Some(os) = skin_token.get("DeviceOS") {
            device_os = os.as_u64().ok_or(Error::other("Cannot decode json"))? as u8;
        }
        if let Some(version) = skin_token.get("GameVersion") {
            game_version = version.as_str().ok_or(Error::other("Cannot decode json"))?.to_string();
        }
        if let Some(model) = skin_token.get("DeviceModel") {
            device_model = model.as_str().ok_or(Error::other("Cannot decode json"))?.to_string();
        }
        if let Some(code) = skin_token.get("LanguageCode") {
            language_code = code.as_str().ok_or(Error::other("Cannot decode json"))?.to_string();
        }
        if let Some(address) = skin_token.get("ServerAddress") {
            server_address = address.as_str().ok_or(Error::other("Cannot decode json"))?.to_string();
        }
        let skin_width = skin_token.get("SkinImageWidth").ok_or(Error::other("Cannot decode json"))?
            .as_u64().ok_or(Error::other("Cannot decode json"))? as u32;
        let skin_height = skin_token.get("SkinImageHeight").ok_or(Error::other("Cannot decode json"))?
            .as_u64().ok_or(Error::other("Cannot decode json"))? as u32;
        if let Some(skin_data) = skin_token.get("SkinData") {
            if let Some(skin_id) = skin_token.get("SkinId") {
                let skin_data = skin_data.as_str().ok_or(Error::other("Cannot decode json"))?;
                let skin_id = skin_id.as_str().ok_or(Error::other("Cannot decode json"))?;
                skin = Skin::new_base64(skin_data, skin_width, skin_height, skin_id);
            }
        }
        let skin = skin.ok_or(Error::other("Invalid skin"))?;
        Ok(Self {
            username,
            protocol,
            identity,
            identity_public_key,
            client_id,
            skin,
            issue_unix_time,
            device_os,
            device_model,
            language_code,
            game_version,
            server_address,
            auth,
            verified,
        })
    }
}

impl Writer for Login {
    fn write(&self, buf: &mut ByteWriter) -> Result<(), Error> {
        Ok(())
    }
}

fn decode(token: &str, index: usize) -> Option<Value> {
    let base: Vec<&str> = token.split(".").collect();
    if base.len() < 2 {
        None
    }else {
        let decode = base64::prelude::BASE64_STANDARD_NO_PAD.decode(base.get(index)?).unwrap();
        let decode = decode.as_slice();
        serde_json::from_slice::<Value>(decode).ok()
    }
}