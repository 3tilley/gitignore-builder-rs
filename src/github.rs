use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Properties2 {
    pub path: Struct,
    pub mode: Struct,
    #[serde(rename = "type")]
    pub r#type: Struct,
    pub size: Struct,
    pub sha: Struct,
    pub url: Struct,
}

#[derive(Serialize, Deserialize)]
struct Struct2 {
    pub path: String,
    pub mode: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub size: i64,
    pub sha: String,
    pub url: String,
    pub properties: Properties2,
    pub required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Size {
    #[serde(rename = "type")]
    pub r#type: String,
    pub examples: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
struct Struct1 {
    #[serde(rename = "type")]
    pub r#type: String,
    pub examples: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Properties1 {
    pub path: Struct1,
    pub mode: Struct1,
    #[serde(rename = "type")]
    pub r#type: Struct1,
    pub sha: Struct1,
    pub size: Size,
    pub url: Struct1,
}

#[derive(Serialize, Deserialize)]
struct Items {
    #[serde(rename = "type")]
    pub r#type: String,
    pub properties: Properties1,
}

#[derive(Serialize, Deserialize)]
struct Tree {
    pub description: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub items: Items,
    pub examples: Vec<Struct2>,
}

#[derive(Serialize, Deserialize)]
struct Url {
    #[serde(rename = "type")]
    pub r#type: String,
    pub format: String,
}

#[derive(Serialize, Deserialize)]
struct Struct {
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Serialize, Deserialize)]
struct Properties {
    pub sha: Struct,
    pub url: Url,
    pub truncated: Struct,
    pub tree: Tree,
}

#[derive(Serialize, Deserialize)]
struct Root {
    pub title: String,
    pub description: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub properties: Properties,
    pub required: Vec<String>,
}