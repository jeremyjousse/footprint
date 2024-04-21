mod adapter;
mod domain;
mod infrastructure;
mod tests;
mod use_case;

use infrastructure::database::create_database_connection_pool;
use infrastructure::state::DbState;

// use tauri::App;
use tauri::Manager;

use adapter::primary::tauri_command::consultation::consultation_add_command::consultation_add_command;
use adapter::primary::tauri_command::consultation::consultation_detail_command::consultation_detail_command;
use adapter::primary::tauri_command::consultation::consultation_list_command::consultation_list_command;
use adapter::primary::tauri_command::consultation::consultation_update_command::consultation_update_command;
use adapter::primary::tauri_command::consultation_type::consultation_type_add_command::consultation_type_add_command;
use adapter::primary::tauri_command::consultation_type::consultation_type_list_command::consultation_type_list_command;
use adapter::primary::tauri_command::patient::patient_add_command::patient_add_command;
use adapter::primary::tauri_command::patient::patient_detail_command::patient_detail_command;
use adapter::primary::tauri_command::patient::patient_list_command::patient_list_command;
use adapter::primary::tauri_command::patient::patient_update_command::patient_update_command;

// #[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let app_handle = app.handle();
            app.manage(DbState {
                global: create_database_connection_pool(app_handle).unwrap(),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            consultation_add_command,
            consultation_detail_command,
            consultation_list_command,
            consultation_update_command,
            consultation_type_add_command,
            consultation_type_list_command,
            patient_add_command,
            patient_detail_command,
            patient_list_command,
            patient_update_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
