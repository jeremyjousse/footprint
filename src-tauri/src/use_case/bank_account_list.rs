use crate::{domain::entity::bank_account::BankAccount, infrastructure::type_alias::Result};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    SqliteConnection,
};

use crate::adapter::secondary::sqlite::bank_account::bank_account_repository::find;

pub fn bank_account_list_use_case(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<Vec<BankAccount>> {
    find(connection)
}
