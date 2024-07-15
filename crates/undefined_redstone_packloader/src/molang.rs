use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug, Default)]
#[serde(untagged)]
pub enum MolangExpression {
    #[default]
    Null,
    ValueExpression(Value),
}