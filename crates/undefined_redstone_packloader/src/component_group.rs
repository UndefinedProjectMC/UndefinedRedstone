use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct ComponentGroup(pub Vec<String>);