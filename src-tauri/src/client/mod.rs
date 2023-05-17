pub(crate) mod api;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct RssClass {
    #[serde(rename = "list")]
    pub list: Page,
    #[serde(rename = "class")]
    pub clazz: Menus,
}

#[derive(Default, Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct RssVideos<T> {
    #[serde(rename = "list")]
    pub list: Videos<T>,
}

#[derive(Default, Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Page {
    #[serde(rename(serialize = "page", deserialize = "page"))]
    pub page: u32,
    #[serde(rename(serialize = "page_count", deserialize = "pagecount"))]
    pub page_count: u32,
    #[serde(rename(serialize = "page_size", deserialize = "pagesize"))]
    pub page_size: u32,
    #[serde(rename(serialize = "record_count", deserialize = "recordcount"))]
    pub record_count: u32,
}

#[derive(Default, Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Videos<T> {
    #[serde(rename(serialize = "page", deserialize = "page"))]
    pub page: u32,
    #[serde(rename(serialize = "page_count", deserialize = "pagecount"))]
    pub page_count: u32,
    #[serde(rename(serialize = "page_size", deserialize = "pagesize"))]
    pub page_size: u32,
    #[serde(rename(serialize = "record_count", deserialize = "recordcount"))]
    pub record_count: u32,
    #[serde(rename(serialize = "videos", deserialize = "video"))]
    pub videos: Option<Vec<T>>,
}

#[derive(Default, Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Video {
    pub id: IntValue,
    pub tid: IntValue,
    pub name: StrValue,
    #[serde(rename = "type")]
    pub _type: StrValue,
    pub pic: StrValue,
    pub lang: StrValue,
    pub year: StrValue,
    pub note: StrValue,
    pub actor: StrValue,
    pub director: StrValue,
    pub last: StrValue,
}


#[derive(Default, Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Detail {
    pub id: IntValue,
    pub tid: IntValue,
    pub name: StrValue,
    #[serde(rename = "type")]
    pub _type: StrValue,
    pub pic: StrValue,
    pub lang: StrValue,
    pub year: StrValue,
    pub note: StrValue,
    pub actor: StrValue,
    pub director: StrValue,
    pub last: StrValue,
    pub dl: Option<Dl>,
    pub des: StrValue,
}


#[derive(Default, Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(into = "Option<Vec<Dd>>")]
pub struct Dl {
    pub dd: Option<Vec<Dd>>,
}

impl Into<Option<Vec<Dd>>> for Dl {
    fn into(self) -> Option<Vec<Dd>> {
        let option = self.dd;
        if let Some(mut dds) = option {
            for mut x in dds.iter_mut() {
                x.addrs = Vec::new();
                let value = x.value.as_str();
                if value.len() > 0 {
                    let intermediate_state: Vec<&str> = value.split("#").collect();
                    for (index, text) in intermediate_state.into_iter().enumerate() {
                        if text.len() <= 0 {
                            continue;
                        }
                        let final_state: Vec<&str> = text.split("$").collect();
                        let play_option = if final_state.len() == 2 {
                            let url = final_state.get(1).unwrap().to_string();
                            if url.len() > 0 {
                                Some(Play {
                                    title: final_state.get(0).unwrap().to_string(),
                                    url,
                                })
                            } else { None }
                        } else if final_state.len() == 1 {
                            let url = final_state.get(0).unwrap().to_string();
                            if url.len() > 0 {
                                Some(Play {
                                    title: index.to_string(),
                                    url,
                                })
                            } else { None }
                        } else { None };
                        if let Some(play) = play_option {
                            x.addrs.push(play);
                        }
                    }
                }
            }
            return Some(dds);
        }
        None
    }
}

#[derive(Default, Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Dd {
    pub flag: String,
    #[serde(default, skip_serializing, rename(serialize = "value", deserialize = "$value"))]
    pub value: String,
    #[serde(skip_deserializing)]
    pub addrs: Vec<Play>,
}

#[derive(Default, Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Play {
    pub title: String,
    pub url: String,
}


#[derive(Default, Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Menus {
    #[serde(rename(serialize = "menus", deserialize = "ty"))]
    pub menus: Vec<Menu>,
}

#[derive(Default, Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Menu {
    pub id: u32,
    #[serde(rename(serialize = "name", deserialize = "$value"))]
    pub name: String,
}

#[derive(Default, Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(into = "u32")]
pub struct IntValue {
    #[serde(default, rename(serialize = "value", deserialize = "$value"))]
    pub value: u32,
}

impl Into<u32> for IntValue {
    fn into(self) -> u32 {
        self.value
    }
}

#[derive(Default, Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(into = "String")]
pub struct StrValue {
    #[serde(default, rename(serialize = "value", deserialize = "$value"))]
    pub value: String,
}

impl Into<String> for StrValue {
    fn into(self) -> String {
        self.value
    }
}
