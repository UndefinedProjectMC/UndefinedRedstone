use serde::{Deserialize, Deserializer};
use serde_json::{Map, Value};

#[derive(Deserialize)]
struct IPServiceData {
    pub city: String,
    pub isp: String,
    pub province: String,
    pub nation: String,
    pub nation_code: String,
    pub latitude: f32,
    pub longitude: f32,
}

impl IPServiceData {
    pub fn get_locate(&self) -> String {
        if self.nation_code == "CN" {
            format!("{}{}{}", self.province, self.city, self.isp)
        } else {
            format!("{} {} {} {}", self.nation, self.province, self.city, self.isp)
        }
    }
}

#[derive(Deserialize)]
struct IPServiceInner {
    pub data: Map<String, Value>
}

pub struct IPService {
    pub locate: String,
    pub latitude: f32,
    pub longitude: f32,
}

impl IPService {
    async fn get(ip: String) -> Result<IPServiceData, reqwest::Error>{
        let data = if ip == "127.0.0.1" {
            reqwest::get("https://webapi-pc.meitu.com/common/ip_location").await?.json::<IPServiceInner>().await?
        }else {
            reqwest::get(format!("https://webapi-pc.meitu.com/common/ip_location?ip={}", ip)).await?.json::<IPServiceInner>().await?
        };
        //获取hashmap第一个元素
        let data = data.data.into_iter().next().unwrap().1;
        let data = <IPServiceData as Deserialize>::deserialize(data).unwrap();
        Ok(data)
    }
    pub async fn new(ip: String) -> Self {
        let data = IPService::get(ip).await.unwrap();
        Self {
            locate: data.get_locate(),
            latitude: data.latitude,
            longitude: data.longitude,
        }
    }
}