use crate::{
    adapter::secondary::sqlite::patient::patient_repository::{delete, get},
    domain::entity::patient::Patient,
    infrastructure::{
        error::Error,
        type_alias::{f, Result},
    },
};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    SqliteConnection,
};
use uuid::Uuid;

pub fn patient_delete_use_case(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    patient_id: Uuid,
) -> Result<Option<Patient>> {
    let patient = get(connection, patient_id)?;

    match patient {
        Some(_) => {
            delete(connection, patient_id)?
            // TODO add check of consultation relations
        }
        None => {
            return Err(Error::PatientRepository(f!(
                "Patient {} does not exists for deletion",
                patient_id
            )))
        }
    };

    Ok(patient)
}
