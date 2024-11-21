// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod nfc;

use rand::prelude::SliceRandom;
use tauri::Manager;

use nfc::NfcReader;
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

/// This function will setup the NFC reader to be ready to read a tag
fn setup_nfc_reader() -> Result<Arc<Mutex<NfcReader>>, Box<dyn std::error::Error>> {
    let nfc_reader = NfcReader::new()?;
    Ok(Arc::new(Mutex::new(nfc_reader)))
}

/// This function is called when the NFC reader is ready to read a tag
/// It will loop until a tag is read, then call get_fish_by_id with the tag id
/// and then emit the fish data to be listened for by the frontend
fn nfc_reader_loop(nfc_reader: Arc<Mutex<NfcReader>>, app_handle: tauri::AppHandle) {
    // Clone the Arc to move into the thread
    let nfc_reader = Arc::clone(&nfc_reader);
    thread::spawn(move || {
        loop {
            // Lock the NFC reader for exclusive access
            let uid_result = {
                let mut reader = nfc_reader.lock().unwrap();
                reader.read_tag()
            };

            match uid_result {
                Ok(uid) => {
                    // Convert UID (Vec<u8>) to u32. This assumes the UID fits into u32.
                    // Adjust this conversion based on your actual UID format.
                    if uid.len() >= 4 {
                        let id = u32::from_be_bytes([uid[0], uid[1], uid[2], uid[3]]);
                        println!("Read UID: {:?}, converted ID: {}", uid, id);
                        let fish = get_fish_by_id(id);
                        // Emit the fish data to the frontend
                        app_handle.emit_all("nfc-tag-read", fish).unwrap();
                    } else {
                        eprintln!("UID length is less than 4 bytes: {:?}", uid);
                    }
                }
                Err(e) => {
                    eprintln!("Error reading NFC tag: {}", e);
                }
            }

            // Sleep for a short duration to prevent tight looping
            thread::sleep(std::time::Duration::from_millis(500));
        }
    });
}

/// This function will get the fish data by the id of the fish
fn get_fish_by_id(id: u32) -> FishInfo {
    let fish_data = load_fish_data();
    fish_data
        .iter()
        .find(|fish| fish.id == id)
        .cloned()
        .expect(&format!("No fish found with ID: {}", id))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .setup(|app| {
            let window = app.get_window("main").expect("Failed to get main window");
            window_shadows::set_shadow(window, true).expect("Failed to set window shadow");

            // Setup NFC Reader
            let nfc_reader = match setup_nfc_reader() {
                Ok(reader) => reader,
                Err(e) => {
                    eprintln!("Failed to set up NFC reader: {}", e);
                    std::process::exit(1);
                }
            };

            // Start the NFC reader loop
            nfc_reader_loop(Arc::clone(&nfc_reader), app.handle());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
