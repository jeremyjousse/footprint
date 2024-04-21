use tauri::{AppHandle, Manager};

use crate::{
    domain::entity::payment::Payment,
    infrastructure::{error::Error, state::DbState},
    use_case::payment_add::payment_add_use_case,
};

use super::payment_dto::PaymentDto;

#[tauri::command]
pub fn payment_add_command(
    app_handle: AppHandle,
    payment: PaymentDto,
) -> Result<PaymentDto, Error> {
    let db_state: tauri::State<'_, DbState> = app_handle.state();
    let mut connection = db_state.global.clone().get().unwrap();

    payment_add_use_case(&mut connection, Payment::from(payment)).map(PaymentDto::from)
}
