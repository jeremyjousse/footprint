use chrono::{NaiveDateTime, Utc};
use diesel::{associations::Identifiable, deserialize::Queryable, prelude::Insertable, Selectable};
use serde::{Deserialize, Serialize};

use crate::domain::entity::bank_account::BankAccount;

use super::bank_account_schema::bank_accounts::{self};

#[derive(
    Clone, Debug, Deserialize, Identifiable, Insertable, PartialEq, Queryable, Selectable, Serialize,
)]
#[diesel(table_name = bank_accounts)]
#[diesel(primary_key(id))]
pub struct DbBankAccount {
    pub created_at: NaiveDateTime,
    pub id: String,
    pub account_number: String,
    pub bank_name: String,
    pub bank_cheque_deposit_number: i32,
    pub updated_at: NaiveDateTime,
}

impl From<BankAccount> for DbBankAccount {
    fn from(value: BankAccount) -> DbBankAccount {
        DbBankAccount {
            created_at: Utc::now().naive_utc(), // TODO ???: use the provided timestamp
            id: value.id.to_string(),
            updated_at: Utc::now().naive_utc(),
            account_number: value.account_number,
            bank_name: value.bank_name,
            bank_cheque_deposit_number: value.bank_cheque_deposit_number as i32,
        }
    }
}

impl From<DbBankAccount> for BankAccount {
    fn from(value: DbBankAccount) -> BankAccount {
        BankAccount {
            id: uuid::Uuid::parse_str(&value.id).unwrap(),
            bank_name: value.bank_name,
            account_number: value.account_number,
            bank_cheque_deposit_number: value.bank_cheque_deposit_number as u32,
        }
    }
}
