use serde::{Deserialize, Deserializer};

#[derive(Debug, Default)]
pub struct ClientAbilities {

}

impl ClientAbilities {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl<'de> Deserialize<'de> for ClientAbilities {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        //读取compound
        Ok(ClientAbilities::new())
    }
}