use crate::{domain::entity::bank_account::BankAccount, infrastructure::type_alias::Result};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    SqliteConnection,
};

use crate::adapter::secondary::sqlite::bank_account::bank_account_repository::add;

pub fn bank_account_add_use_case(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    bank_account: BankAccount,
) -> Result<BankAccount> {
    add(connection, bank_account)
}
