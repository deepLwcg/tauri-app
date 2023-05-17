use quick_xml::de;
use quick_xml::DeError;
use serde::de::DeserializeOwned;
use crate::client::{Detail, Menu, Page, RssClass, RssVideos, Video, Videos};

pub async fn request_get(url: &str) -> Result<String, reqwest::Error> {
    let content = reqwest::get(url)
        .await?
        .text()
        .await?;
    Ok(content)
}

pub async fn request_get_json<T: DeserializeOwned>(url: &str) -> Result<T, reqwest::Error> {
    let content = reqwest::get(url)
        .await?
        .json::<T>()
        .await?;
    Ok(content)
}

pub fn read_xml_list(xml: String) -> Result<RssClass, DeError> {
    de::from_str(xml.as_str())
}

pub fn read_xml_videos(xml: String) -> Result<RssVideos<Video>, DeError> {
    de::from_str(xml.as_str())
}

pub fn read_xml_details(xml: String) -> Result<RssVideos<Detail>, DeError> {
    de::from_str(xml.as_str())
}

pub fn get_menus(rss: RssClass) -> Option<Vec<Menu>> {
    Some(rss.clazz.menus)
}

pub fn get_videos(rss: RssVideos<Video>) -> Option<Vec<Video>> {
    Some(rss.list.videos.unwrap_or(Vec::new()))
}

pub fn get_videos_page(mut rss: RssVideos<Video>) -> Option<Videos<Video>> {
    if rss.list.record_count == 0 {
        let empty:Vec<Video> = Vec::new();
        rss.list.videos = Some(empty);
    };
    Some(rss.list)
}

pub fn get_page(rss: RssClass) -> Option<Page> {
    Some(rss.list)
}

pub fn get_details(rss: RssVideos<Detail>) -> Option<Vec<Detail>> {
    Some(rss.list.videos.unwrap_or(Vec::new()))
}