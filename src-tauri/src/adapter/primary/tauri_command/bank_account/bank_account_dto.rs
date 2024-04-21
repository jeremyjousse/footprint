use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entity::bank_account::BankAccount;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BankAccountDto {
    pub id: Option<String>,
    pub account_number: String,
    pub bank_name: String,
    pub bank_cheque_deposit_number: u32,
}

impl From<BankAccount> for BankAccountDto {
    fn from(value: BankAccount) -> Self {
        BankAccountDto {
            id: Some(value.id.to_string()),
            account_number: value.account_number,
            bank_name: value.bank_name,
            bank_cheque_deposit_number: value.bank_cheque_deposit_number,
        }
    }
}

impl From<BankAccountDto> for BankAccount {
    fn from(value: BankAccountDto) -> Self {
        BankAccount {
            id: match value.id {
                // TODO refactor as multiple usage
                Some(uuid) => Uuid::parse_str(&uuid).unwrap_or(Uuid::new_v4()), // TODO do we really want to create a new uuid?
                None => Uuid::new_v4(),
            },
            bank_name: value.bank_name,
            account_number: value.account_number,
            bank_cheque_deposit_number: value.bank_cheque_deposit_number,
        }
    }
}
