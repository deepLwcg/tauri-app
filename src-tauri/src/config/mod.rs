use serde::{Serialize, Deserialize};

pub(crate) mod yaml;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Configs {
    pub name: String,
    pub user: User,
    pub sites: Vec<Site>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct User {
    pub id:String,
    pub username: String,
    pub password: String,
}


#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Site {
    pub id: u32,
    pub name: String,
    pub original_name: String,
    pub url: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub resolve_address: Option<String>,
    pub help: Option<String>,
}
