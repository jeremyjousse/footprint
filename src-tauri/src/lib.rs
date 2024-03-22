mod adapter;
mod domain;
mod infrastructure;
mod use_case;

use infrastructure::database::create_database_connection_pool;
use infrastructure::state::DbState;

// use tauri::App;
use tauri::Manager;

use adapter::primary::tauri_command::patient::patient_add_command::patient_add_command;
use adapter::primary::tauri_command::patient::patient_detail_command::patient_detail_command;
use adapter::primary::tauri_command::patient::patient_list_command::patient_list_command;
use adapter::primary::tauri_command::patient::patient_update_command::patient_update_command;

// #[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .setup(|app| {
            let app_handle = app.handle();
            app.manage(DbState {
                global: create_database_connection_pool(app_handle).unwrap(),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            patient_add_command,
            patient_detail_command,
            patient_list_command,
            patient_update_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
