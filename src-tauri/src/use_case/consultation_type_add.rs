use crate::{
    domain::entity::consultation_type::ConsultationType, infrastructure::type_alias::Result,
};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    SqliteConnection,
};

use crate::adapter::secondary::sqlite::consultation_type::consultation_type_repository::add;

pub fn consultation_type_add_use_case(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    consultation_type: ConsultationType,
) -> Result<ConsultationType> {
    add(connection, consultation_type)
}
