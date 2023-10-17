use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub sha: String,
    pub url: String,
    pub tree: Vec<Tree>,
    pub truncated: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tree {
    pub path: String,
    pub mode: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub sha: String,
    pub url: String,
    pub size: Option<i64>,
}

