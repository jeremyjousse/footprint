use tauri::{AppHandle, Manager};

use crate::{
    domain::value_object::{
        consultation_list_filter::ConsultationListFilter, listing_sort::ListingSort,
    },
    infrastructure::{error::Error, state::DbState},
    use_case::consultation_list::consultation_list_use_case,
};

use super::consultation_dto::ConsultationDto;

#[tauri::command]
pub fn consultation_list_command(
    app_handle: AppHandle,
    filter: ConsultationListFilter,
    sort: ListingSort,
) -> Result<Vec<ConsultationDto>, Error> {
    let db_state: tauri::State<'_, DbState> = app_handle.state();
    let mut connection = db_state.global.clone().get().unwrap();

    let consultation_types = consultation_list_use_case(&mut connection, filter, sort)?;

    Ok(consultation_types
        .into_iter()
        .map(ConsultationDto::from)
        .collect::<Vec<ConsultationDto>>())
}
