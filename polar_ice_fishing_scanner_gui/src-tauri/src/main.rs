// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::Manager;
use std::io::BufRead;
use std::os::unix::fs::FileTypeExt;




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
fn load_fish_data(app_handle: &tauri::AppHandle) -> Vec<FishInfo> {
    let resource_path = app_handle
        .path_resolver()
        .resolve_resource("assets/fish_data.json")
        .expect("Failed to resolve resource path");

    let file = File::open(resource_path).expect("Failed to open fish_data.json");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Failed to parse fish data JSON")
}

/// Create a named pipe if it doesn't already exist.
fn ensure_fifo_exists(fifo_path: &str) {
    if let Ok(metadata) = std::fs::metadata(fifo_path) {
        if metadata.file_type().is_fifo() {
            return; // FIFO already exists
        } else {
            panic!("Path exists but is not a FIFO: {}", fifo_path);
        }
    }

    // Use a system call to create the FIFO
    if let Err(err) = std::process::Command::new("mkfifo").arg(fifo_path).status() {
        panic!("Failed to create FIFO: {}", err);
    }
}

/// Start a thread to continuously read from the named pipe and fetch associated fish data.
fn start_nfc_thread(app_handle: tauri::AppHandle) {
    thread::spawn(move || {
        let fifo_path = "/tmp/nfc_fifo";
        ensure_fifo_exists(fifo_path);

        // Load fish data
        let fish_data = load_fish_data(&app_handle);
        let fish_data = Arc::new(Mutex::new(fish_data));

        loop {
            // Open the FIFO for reading
            let file = match File::open(fifo_path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Failed to open named pipe: {}", err);
                    thread::sleep(Duration::from_millis(500));
                    continue;
                }
            };

            let reader = BufReader::new(file);

            for line in reader.lines() {
                let line = match line {
                    Ok(line) => line.trim().to_string(),
                    Err(err) => {
                        eprintln!("Error reading from named pipe: {}", err);
                        continue;
                    }
                };

                // Parse the ID
                let id = match line.parse::<u32>() {
                    Ok(id) => id,
                    Err(err) => {
                        eprintln!("Error parsing ID from named pipe: {}", err);
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

            // Sleep briefly before checking the FIFO again
            thread::sleep(Duration::from_millis(100));
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
