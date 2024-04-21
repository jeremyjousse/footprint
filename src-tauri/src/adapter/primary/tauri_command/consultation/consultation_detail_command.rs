use tauri::{AppHandle, Manager};
use uuid::Uuid;

use crate::{
    infrastructure::{error::Error, state::DbState},
    use_case::consultation_detail::consultation_detail_use_case,
};

use super::consultation_dto::ConsultationDto;

#[tauri::command]
pub fn consultation_detail_command(
    app_handle: AppHandle,
    id: String,
) -> Result<Option<ConsultationDto>, Error> {
    let db_state: tauri::State<'_, DbState> = app_handle.state();
    let mut connection = db_state.global.clone().get().unwrap();
    let id = Uuid::parse_str(&id)?;
    let consultation_result = consultation_detail_use_case(&mut connection, id);
    match consultation_result {
        Ok(consultation) => Ok(consultation.map(ConsultationDto::from)),
        Err(err) => Err(err),
    }
}
