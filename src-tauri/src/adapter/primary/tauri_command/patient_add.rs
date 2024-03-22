// remember to call `.manage(MyState::default())`

use log::info;
use tauri::AppHandle;

use crate::domain::entity::patient::PatientDto;

#[tauri::command]
pub async fn patient_add_command(app_handle: AppHandle, patient: PatientDto) -> Result<String, String> {
    info!("patient: {:?}", patient);
    println!("patient: {:?}", patient);
    Ok("uuid".to_string())
}
