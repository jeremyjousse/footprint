use tauri::{AppHandle, Manager};

use crate::{
    infrastructure::{error::Error, state::DbState},
    use_case::patient_list::patient_list_use_case,
};

use super::patient_list_dto::PatientListDto;

#[tauri::command]
pub fn patient_list_command(app_handle: AppHandle) -> Result<Vec<PatientListDto>, Error> {
    let db_state: tauri::State<'_, DbState> = app_handle.state();
    let mut connection = db_state.global.clone().get().unwrap();

    let patients = patient_list_use_case(&mut connection)?;

    Ok(patients
        .into_iter()
        .map(PatientListDto::from)
        .collect::<Vec<PatientListDto>>())
}
