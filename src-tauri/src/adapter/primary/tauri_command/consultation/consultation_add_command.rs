use tauri::{AppHandle, Manager};

use crate::{
    domain::entity::consultation::Consultation,
    infrastructure::{error::Error, state::DbState},
    use_case::consultation_add::consultation_add_use_case,
};

use super::consultation_dto::ConsultationDto;

#[tauri::command]
pub fn consultation_add_command(
    app_handle: AppHandle,
    consultation: ConsultationDto,
) -> Result<ConsultationDto, Error> {
    let db_state: tauri::State<'_, DbState> = app_handle.state();
    let mut connection1 = db_state.global.clone().get().unwrap();

    consultation_add_use_case(&mut connection1, Consultation::from(consultation))
        .map(ConsultationDto::from)
}
