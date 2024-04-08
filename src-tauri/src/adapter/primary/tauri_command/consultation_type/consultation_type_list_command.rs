use tauri::{AppHandle, Manager};

use crate::{
    infrastructure::{error::Error, state::DbState},
    use_case::consultation_type_list::consultation_type_list_use_case,
};

use super::consultation_type_dto::ConsultationTypeDto;

#[tauri::command]
pub fn consultation_type_list_command(
    app_handle: AppHandle,
) -> Result<Vec<ConsultationTypeDto>, Error> {
    let db_state: tauri::State<'_, DbState> = app_handle.state();
    let mut connection = db_state.global.clone().get().unwrap();

    let consultation_types = consultation_type_list_use_case(&mut connection)?;

    Ok(consultation_types
        .into_iter()
        .map(ConsultationTypeDto::from)
        .collect::<Vec<ConsultationTypeDto>>())
}
