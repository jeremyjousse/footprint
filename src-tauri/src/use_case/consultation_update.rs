use crate::{
    adapter::secondary::sqlite::consultation::consultation_repository::update,
    domain::entity::consultation::Consultation, infrastructure::type_alias::Result,
};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    SqliteConnection,
};

pub fn consultation_update_use_case(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    consultation: Consultation,
) -> Result<Consultation> {
    update(connection, consultation)
}
