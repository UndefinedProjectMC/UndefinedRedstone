use serde::{Deserialize, Deserializer};

#[derive(Debug, Default)]
pub struct Gamerules {

}

impl Gamerules {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl<'de> Deserialize<'de> for Gamerules {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        Ok(Self::new())
    }
}