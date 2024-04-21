use tauri::{AppHandle, Manager};

use crate::{
    domain::entity::bank_account::BankAccount,
    infrastructure::{error::Error, state::DbState},
    use_case::bank_account_add::bank_account_add_use_case,
};

use super::bank_account_dto::BankAccountDto;

#[tauri::command]
pub fn bank_account_add_command(
    app_handle: AppHandle,
    bank_account: BankAccountDto,
) -> Result<BankAccountDto, Error> {
    let db_state: tauri::State<'_, DbState> = app_handle.state();
    let mut connection = db_state.global.clone().get().unwrap();

    bank_account_add_use_case(&mut connection, BankAccount::from(bank_account))
        .map(BankAccountDto::from)
}
