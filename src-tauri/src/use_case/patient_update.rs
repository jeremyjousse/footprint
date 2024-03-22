use crate::{
    adapter::secondary::sqlite::patient::patient_repository::update,
    domain::entity::patient::Patient, infrastructure::type_alias::Result,
};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    SqliteConnection,
};

pub fn patient_update_use_case(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    patient: Patient,
) -> Result<Patient> {
    update(connection, patient)
}
