use crate::{domain::entity::consultation::Consultation, infrastructure::type_alias::Result};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    SqliteConnection,
};
use uuid::Uuid;

use crate::adapter::secondary::sqlite::consultation::consultation_repository::get;

pub fn consultation_detail_use_case(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    id: Uuid,
) -> Result<Option<Consultation>> {
    get(connection, id)
}
