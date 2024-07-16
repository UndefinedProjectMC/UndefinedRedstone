use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ComponentGroup(pub Vec<String>);