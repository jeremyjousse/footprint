use crate::{domain::entity::consultation::Consultation, infrastructure::type_alias::Result};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    SqliteConnection,
};

use crate::adapter::secondary::sqlite::consultation::consultation_repository::add;

pub fn consultation_add_use_case(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    consultation: Consultation,
) -> Result<Consultation> {
    add(connection, consultation)
}
