use tauri::{AppHandle, Manager};

use crate::{
    infrastructure::{error::Error, state::DbState},
    use_case::bank_account_list::bank_account_list_use_case,
};

use super::bank_account_dto::BankAccountDto;

#[tauri::command]
pub fn bank_account_list_command(app_handle: AppHandle) -> Result<Vec<BankAccountDto>, Error> {
    let db_state: tauri::State<'_, DbState> = app_handle.state();
    let mut connection = db_state.global.clone().get().unwrap();

    let bank_accounts = bank_account_list_use_case(&mut connection)?;
    println!("{:?}", bank_accounts);
    Ok(bank_accounts
        .into_iter()
        .map(BankAccountDto::from)
        .collect::<Vec<BankAccountDto>>())
}
