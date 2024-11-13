// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rand::prelude::SliceRandom;
use tauri::Manager;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct FishInfo {
    id: u32,
    name: String,
    average_size: String,
    average_weight: String,
    average_lifespan: String,
    habitat: String,
    diet: String,
    blurb: String,
    image_path: String,
}

// Load the fish data from the JSON file
fn load_fish_data() -> Vec<FishInfo> {
    let fish_data = include_str!("../assets/fish_data.json");
    serde_json::from_str(fish_data).unwrap()
}

#[tauri::command]
fn get_random_fish() -> FishInfo {
    let fish_data = load_fish_data();
    let random_fish = fish_data.choose(&mut rand::thread_rng()).unwrap();
    random_fish.clone()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_random_fish])
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            window_shadows::set_shadow(window, true).unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
