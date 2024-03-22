use crate::{domain::entity::patient::Patient, infrastructure::type_alias::Result};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    SqliteConnection,
};

use crate::adapter::secondary::sqlite::patient::patient_repository::add;

pub fn patient_add_use_case(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    patient: Patient,
) -> Result<Patient> {
    add(connection, patient)
}
