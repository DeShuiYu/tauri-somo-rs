mod scheme;

mod connections;
use connections::{get_all_connections};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/


#[tauri::command]
fn command_network_info() -> serde_json::Value {

    serde_json::json!( get_all_connections())
    
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![command_network_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
