use crate::{domain::entity::payment::Payment, infrastructure::type_alias::Result};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    SqliteConnection,
};

use crate::adapter::secondary::sqlite::payment::payment_repository::update;

pub fn payment_update_use_case(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    payment: Payment,
) -> Result<Payment> {
    update(connection, payment)
}
