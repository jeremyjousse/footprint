use log::{debug, info};
use std::{
    alloc::System,
    fs,
    path::{Path, PathBuf},
};
use tauri::{AppHandle, Manager};

use crate::infrastructure::type_alias::Result;

pub const ICLOUD_DRIVE_RELATIVE_PATH: &str =
    "Library/Mobile Documents/com~apple~CloudDocs/Footprint";

pub const GLOBAL_DB_FILE: &str = "global.sqlite";
pub const GLOBAL_SETTINGS_FILE: &str = "settings.json";

pub fn global_db_path(app_handle: &AppHandle) -> Result<PathBuf> {
    let mut app_data_dir = check_and_create_app_data_dir(app_handle)?;
    info!("global_db_path {:?}", app_data_dir);
    println!("global_db_path {:?}", app_data_dir);

    app_data_dir.push(GLOBAL_DB_FILE);
    debug!("global_db_path {:?}", app_data_dir);
    Ok(app_data_dir)
}

pub fn user_settings_path(app_handle: &AppHandle) -> Result<PathBuf> {
    let mut global_settings_path = app_handle.path().app_data_dir().unwrap();
    global_settings_path.push(GLOBAL_SETTINGS_FILE);
    Ok(global_settings_path)
}

fn check_and_create_app_data_dir(app_handle: &AppHandle) -> Result<PathBuf> {
    let app_data = app_handle.path().app_data_dir().unwrap();
    let path = Path::new(app_data.to_str().unwrap()); // TODO ? and error
    if !path.exists() {
        fs::create_dir(app_data.to_str().unwrap())?;
    }
    if !path.is_dir() {
        // panic
    }
    Ok(app_data)
}
