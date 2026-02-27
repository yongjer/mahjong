mod mahjong_core;

use mahjong_core::{analyze, AnalysisResult};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn analyze_hand(hand_str: String, discards: Vec<String>) -> Result<AnalysisResult, String> {
    Ok(analyze(&hand_str, &discards))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, analyze_hand])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
