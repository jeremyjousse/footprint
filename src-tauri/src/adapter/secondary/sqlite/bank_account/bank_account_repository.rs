use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection,
};

use crate::{domain::entity::bank_account::BankAccount, infrastructure::type_alias::Result};

use super::{
    bank_account_model::DbBankAccount,
    bank_account_schema::bank_accounts::{self, bank_name},
};

pub fn add(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    bank_account: BankAccount,
) -> Result<BankAccount> {
    let rows_inserted = diesel::insert_into(bank_accounts::table)
        .values(&DbBankAccount::from(bank_account))
        .get_result::<DbBankAccount>(connection)
        .unwrap();

    Ok(BankAccount::from(rows_inserted))
}

pub fn find(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<Vec<BankAccount>> {
    let db_bank_accounts: Vec<DbBankAccount> = bank_accounts::table
        .order(bank_name.asc())
        .load::<DbBankAccount>(connection)?;

    let bank_accounts = db_bank_accounts
        .into_iter()
        .map(BankAccount::from)
        .collect();

    Ok(bank_accounts)
}
