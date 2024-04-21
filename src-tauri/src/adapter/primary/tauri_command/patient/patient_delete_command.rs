use tauri::{AppHandle, Manager};
use uuid::Uuid;

use crate::{
    infrastructure::{error::Error, state::DbState},
    use_case::patient_delete::patient_delete_use_case,
};

use super::patient_dto::PatientDto;

#[tauri::command]
pub fn patient_delete_command(
    app_handle: AppHandle,
    id: String,
) -> Result<Option<PatientDto>, Error> {
    let db_state: tauri::State<'_, DbState> = app_handle.state();
    let mut connection = db_state.global.clone().get().unwrap();
    let id = Uuid::parse_str(&id)?;
    // let patient_result = patient_detail_use_case(&mut connection, id);
    let patient_result = patient_delete_use_case(&mut connection, id);
    match patient_result {
        Ok(patient) => Ok(patient.map(PatientDto::from)),
        Err(err) => Err(err),
    }
}
