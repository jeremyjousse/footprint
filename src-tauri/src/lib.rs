mod adapter;
mod domain;

use log::info;

use adapter::primary::tauri_command::patient_add::patient_add_command;

#[tauri::command]
fn greet(name: &str) -> String {
    // Events should be emitted from `Window` object only.
    // `tauri-sys` expects window name to be part of event payload.

    info!("greetings {}", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .invoke_handler(tauri::generate_handler![greet, patient_add_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
