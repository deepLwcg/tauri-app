// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, WindowBuilder, Wry};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn toggle_fullscreen(handle: tauri::AppHandle<Wry>) {
    let player_window = handle.get_window("player_window").unwrap();
    println!("{}",player_window.title().unwrap());
    player_window.set_fullscreen(!player_window.is_fullscreen().unwrap()).unwrap();
}


#[tauri::command]
async fn open_player_window(title: String, url: String, handle: tauri::AppHandle<Wry>) {
    let main = handle.get_window("main").unwrap();
    let size = main.outer_size().unwrap();
    let url = tauri::WindowUrl::App("/player".into());
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


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,toggle_fullscreen,open_player_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
