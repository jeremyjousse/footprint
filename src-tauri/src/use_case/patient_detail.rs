use crate::{domain::entity::patient::Patient, infrastructure::type_alias::Result};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    SqliteConnection,
};
use uuid::Uuid;

use crate::adapter::secondary::sqlite::patient::patient_repository::get;

pub fn patient_detail_use_case(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    id: Uuid,
) -> Result<Option<Patient>> {
    get(connection, id)
}
