use tauri::{AppHandle, Manager};

use crate::{
    domain::entity::consultation_type::ConsultationType,
    infrastructure::{error::Error, state::DbState},
    use_case::consultation_type_add::consultation_type_add_use_case,
};

use super::consultation_type_dto::ConsultationTypeDto;

#[tauri::command]
pub fn consultation_type_add_command(
    app_handle: AppHandle,
    consultation_type: ConsultationTypeDto,
) -> Result<ConsultationTypeDto, Error> {
    let db_state: tauri::State<'_, DbState> = app_handle.state();
    let mut connection = db_state.global.clone().get().unwrap();

    consultation_type_add_use_case(&mut connection, ConsultationType::from(consultation_type))
        .map(ConsultationTypeDto::from)
}
