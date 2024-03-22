use crate::{domain::entity::patient::Patient, infrastructure::type_alias::Result};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    SqliteConnection,
};

use crate::adapter::secondary::sqlite::patient::patient_repository::find;

pub fn patient_list_use_case(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<Vec<Patient>> {
    find(connection)
}
