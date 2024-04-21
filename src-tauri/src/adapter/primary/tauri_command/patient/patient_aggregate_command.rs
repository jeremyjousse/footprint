use tauri::{AppHandle, Manager};
use uuid::Uuid;

use crate::{
    domain::aggregate::patient::PatientAggregate,
    infrastructure::{error::Error, state::DbState},
    use_case::patient_aggregate::patient_aggregate_use_case,
};

use super::patient_dto::PatientDto;

#[tauri::command]
pub fn patient_aggregate_command(
    app_handle: AppHandle,
    id: String,
) -> Result<Option<PatientAggregate>, Error> {
    let db_state: tauri::State<'_, DbState> = app_handle.state();
    let mut connection = db_state.global.clone().get().unwrap();
    let id = Uuid::parse_str(&id)?;
    let patient_aggregate_result = patient_aggregate_use_case(&mut connection, id);
    match patient_aggregate_result {
        Ok(patient_aggregate) => Ok(patient_aggregate.map(PatientAggregate::from)),
        Err(err) => Err(err),
    }
}
