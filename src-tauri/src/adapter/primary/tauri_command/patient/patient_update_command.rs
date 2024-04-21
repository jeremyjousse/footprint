use tauri::{AppHandle, Manager};

use crate::{
    domain::entity::patient::Patient,
    infrastructure::{error::Error, state::DbState},
    use_case::patient_update::patient_update_use_case,
};

use super::patient_dto::PatientDto;

#[tauri::command]
pub fn patient_update_command(
    app_handle: AppHandle,
    patient: PatientDto,
) -> Result<PatientDto, Error> {
    let db_state: tauri::State<'_, DbState> = app_handle.state();
    let mut connection = db_state.global.clone().get().unwrap();

    patient_update_use_case(&mut connection, Patient::from(patient)).map(PatientDto::from)
}
