// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub(crate) mod client;
pub(crate) mod common;
pub(crate) mod config;

use quick_xml::DeError;
use reqwest::Error;
use tauri::{Manager, WindowBuilder, Wry};
use crate::client::api::{get_details, get_menus, get_videos_page, read_xml_details, read_xml_list, read_xml_videos, request_get};
use crate::client::{Detail, Menu, RssClass, RssVideos, Video, Videos};
use crate::config::Site;
use crate::config::yaml::CONFIG;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn toggle_fullscreen(handle: tauri::AppHandle<Wry>) {
    let player_window = handle.get_window("player_window").unwrap();
    println!("{}", player_window.title().unwrap());
    player_window.set_fullscreen(!player_window.is_fullscreen().unwrap()).unwrap();
}


#[tauri::command]
async fn open_player_window(title: String, url: String, handle: tauri::AppHandle<Wry>) {
    let main = handle.get_window("main").unwrap();
    let size = main.outer_size().unwrap();
    let url = tauri::WindowUrl::App(url.into());
    let new_window = WindowBuilder::new(&handle, "player_window", url)
        .always_on_top(true)
        .decorations(true)
        .title(title)
        .inner_size(size.width.clone() as f64, size.height.clone() as f64)
        .center()
        .build()
        .unwrap();
    new_window
        .show()
        .expect("error while opening new window");
}

#[tauri::command]
fn get_sites() -> Vec<Site> {
    CONFIG::sites()
}

#[tauri::command]
async fn get_videos_type(id: u32) -> Vec<Menu> {
    let configs = CONFIG::sites_map();
    let site = configs.get(&id).unwrap();
    let url = format!("{}?ac=list", site.url);
    let xml_result = request_get(url.as_str()).await;
    match xml_result {
        Ok(xml) => {
            let result = read_xml_list(xml);
            match result {
                Ok(data) => {
                    get_menus(data).unwrap()
                }
                Err(_) => Vec::new()
            }
        }
        Err(_) => Vec::new()
    }
}

#[tauri::command]
async fn get_videos(id: u32) -> Option<Videos<Video>> {
    let configs = CONFIG::sites_map();
    let site = configs.get(&id).unwrap();
    let url = format!("{}?ac=videolist", site.url);
    let xml_result = request_get(url.as_str()).await;
    match xml_result {
        Ok(xml) => {
            let result = read_xml_videos(xml);
            match result {
                Ok(data) => {
                    Some(get_videos_page(data).unwrap())
                }
                Err(_) => None
            }
        }
        Err(_) => None
    }
}

#[tauri::command]
async fn open_video(id: u32, vid: u32) -> Option<Detail> {
    let configs = CONFIG::sites_map();
    let site = configs.get(&id).unwrap();
    let url = format!("{}?ac=videolist&ids={}", site.url, vid.to_string());
    let xml_result = request_get(url.as_str()).await;
    match xml_result {
        Ok(xml) => {
            let result = read_xml_details(xml);
            match result {
                Ok(rss) => {
                    let mut videos = get_details(rss).unwrap();
                    if videos.len() > 0 {
                        Some(videos.remove(0))
                    } else {
                        None
                    }
                }
                Err(_) => None
            }
        }
        Err(_) => None
    }
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,toggle_fullscreen,open_player_window,get_sites,get_videos_type,get_videos,open_video])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
