// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bstr::io::BufReadExt;
use tauri::Manager;

use std::sync::{Arc, Mutex};
use std::thread;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct FishInfo {
    id: u32,
    name: String,
    average_size: String,
    average_weight: String,
    average_lifespan: String,
    habitat: String,
    diet: String,
    endangered_status: String,
    blurb: String,
    image_path: String,
    fun_fact: String,
}

/// Load the fish data from the JSON file
fn load_fish_data() -> Vec<FishInfo> {
    let fish_data = include_str!("../assets/fish_data.json");
    serde_json::from_str(fish_data).expect("Failed to parse fish data JSON")
}

/// This function starts a python script that reads NFC tags and prints the ID of the tag, collect with anonymous pipe and when data is collected it fetches the associated fish data with the id and emits it to the frontend
fn start_nfc_thread(app_handle: tauri::AppHandle) {
    thread::spawn(move || {
        // Load fish data
        let fish_data = load_fish_data();
        let fish_data = Arc::new(Mutex::new(fish_data));

        // Start the Python script
        let mut python = std::process::Command::new("python")
            .arg("./assets/nfc_reader/nfc_reader.py")
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to start Python script");

        let stdout = python.stdout.take().expect("Failed to open stdout");

        let reader = std::io::BufReader::new(stdout);

        // Read lines from the Python script's output
        for line in reader.byte_lines() {
            let line = match line {
                Ok(bytes) => String::from_utf8(bytes).expect("Invalid UTF-8"),
                Err(err) => {
                    eprintln!("Error reading line from Python script: {}", err);
                    continue;
                }
            };

            // Parse the ID
            let id = match line.trim().parse::<u32>() {
                Ok(id) => id,
                Err(err) => {
                    eprintln!("Error parsing ID from Python script output: {}", err);
                    continue;
                }
            };

            // Find fish data
            let current_fish_data = fish_data.lock().unwrap();
            if let Some(fish) = current_fish_data.iter().find(|fish| fish.id == id) {
                // Emit the fish data to the frontend
                if let Err(err) = app_handle.emit_all("fishData", fish) {
                    eprintln!("Failed to emit fish data: {}", err);
                } else {
                    println!("Fish data emitted: {:?}", fish);
                }
            } else {
                eprintln!("Warning: No fish found with ID: {}", id);
            }
        }

        // Ensure the Python process is terminated if the thread ends
        if let Err(err) = python.kill() {
            eprintln!("Failed to kill Python process: {}", err);
        }
    });
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .setup(|app| {
            start_nfc_thread(app.handle().clone());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
