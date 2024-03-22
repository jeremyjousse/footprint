use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
    SqliteConnection,
};
use uuid::Uuid;

use crate::{
    domain::entity::patient::Patient,
    infrastructure::{
        error::Error,
        type_alias::{f, Result},
    },
};

use super::{
    patient_model::DbPatient,
    patient_schema::patients::{
        self, birthdate, city, country, diabetic, email, first_name, id, last_name,
        long_duration_disease, mobile_phone, national_insurance_number, notes, phone, postal_code,
        profession, street, title, updated_at,
    },
};

pub fn add(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    patient: Patient,
) -> Result<Patient> {
    let rows_inserted = diesel::insert_into(patients::table)
        .values(&DbPatient::from(patient))
        .get_result::<DbPatient>(connection)
        .unwrap();

    Ok(Patient::from(rows_inserted))
}

pub fn find(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<Vec<Patient>> {
    let db_patients: Vec<DbPatient> = patients::table
        .order(last_name.asc())
        .load::<DbPatient>(connection)?;

    let patients = db_patients.into_iter().map(Patient::from).collect();

    Ok(patients)
}

pub fn get(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    get_id: Uuid,
) -> Result<Option<Patient>> {
    let db_patient = patients::table
        .find(get_id.to_string())
        .select(DbPatient::as_select())
        .first(connection)
        .optional();

    match db_patient {
        Ok(Some(db_patient)) => Ok(Some(Patient::from(db_patient))),
        Ok(None) => Ok(None),
        Err(_) => Err(Error::PatientRepository(f!(
            "An error occurred while fetching patient {}",
            get_id.to_string(),
        ))),
    }
}

pub fn update(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    patient: Patient,
) -> Result<Patient> {
    let db_patient = DbPatient::from(patient);
    let db_patient = diesel::update(patients::table)
        .filter(id.eq(db_patient.id))
        .set((
            birthdate.eq(db_patient.birthdate),
            city.eq(db_patient.city),
            country.eq(db_patient.country),
            diabetic.eq(db_patient.diabetic),
            email.eq(db_patient.email),
            first_name.eq(&db_patient.first_name),
            last_name.eq(&db_patient.last_name),
            long_duration_disease.eq(&db_patient.long_duration_disease),
            national_insurance_number.eq(&db_patient.national_insurance_number),
            mobile_phone.eq(&db_patient.mobile_phone),
            notes.eq(&db_patient.notes),
            phone.eq(&db_patient.phone),
            postal_code.eq(&db_patient.postal_code),
            profession.eq(&db_patient.profession),
            street.eq(&db_patient.street),
            title.eq(&db_patient.title),
            updated_at.eq(chrono::Utc::now().naive_utc()),
        ))
        .get_result::<DbPatient>(connection)
        .expect("Error saving library track");

    Ok(Patient::from(db_patient))
}

pub fn delete() {}

pub fn count() {}
