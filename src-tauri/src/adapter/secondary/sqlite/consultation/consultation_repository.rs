use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
    SqliteConnection,
};
use log::info;
use uuid::Uuid;

use crate::{
    adapter::primary::tauri_command::consultation,
    domain::{
        entity::consultation::Consultation,
        value_object::{
            consultation_list_filter::ConsultationListFilter, listing_sort::ListingSort,
        },
    },
    infrastructure::{
        error::Error,
        type_alias::{f, Result},
    },
};

use super::{
    consultation_model::DbConsultation,
    consultation_schema::consultations::{
        self, appointment_date_time, consultation_type, created_at, id, location, note, patient_id,
        price, status, updated_at,
    },
};

pub fn add(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    consultation: Consultation,
) -> Result<Consultation> {
    let rows_inserted = diesel::insert_into(consultations::table)
        .values(&DbConsultation::from(consultation))
        .get_result::<DbConsultation>(connection)
        .unwrap();

    Ok(Consultation::from(rows_inserted))
}

pub fn find(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    filter: ConsultationListFilter,
    sort: ListingSort,
) -> Result<Vec<Consultation>> {
    let mut consultations_query = consultations::table.into_boxed();
    if filter.patient_id.is_some() {
        // let mut consultation_id = String::new();
        consultations_query = consultations_query.filter(patient_id.eq(filter.patient_id.unwrap()));
    }

    consultations_query = match sort {
        ListingSort { by, order } if by == *"appointment_date_time" && order == *"asc" => {
            consultations_query.order(appointment_date_time.asc())
        }
        ListingSort { by, order } if by == *"appointment_date_time" && order == *"desc" => {
            consultations_query.order(appointment_date_time.desc())
        }
        _ => consultations_query.order(created_at.desc()),
    };

    let db_consultations: Vec<DbConsultation> = consultations_query
        .order(appointment_date_time.desc())
        .load::<DbConsultation>(connection)?;

    let consultations = db_consultations
        .into_iter()
        .map(Consultation::from)
        .collect();

    Ok(consultations)
}

pub fn get(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    get_id: Uuid,
) -> Result<Option<Consultation>> {
    println!(" Consultation get_id {:?}", get_id);
    let db_consultation = consultations::table
        .find(get_id.to_string())
        .select(DbConsultation::as_select())
        .first(connection)
        .optional();

    match db_consultation {
        Ok(Some(db_consultation)) => Ok(Some(Consultation::from(db_consultation))),
        Ok(None) => Ok(None),
        Err(_) => Err(Error::ConsultationRepository(f!(
            "An error occurred while fetching consultation {}",
            get_id,
        ))),
    }
}

pub fn update(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    consultation: Consultation,
) -> Result<Consultation> {
    let db_consultation = DbConsultation::from(consultation);
    let db_consultation = diesel::update(consultations::table)
        .filter(id.eq(db_consultation.id))
        .set((
            appointment_date_time.eq(db_consultation.appointment_date_time),
            consultation_type.eq(db_consultation.consultation_type),
            location.eq(db_consultation.location),
            note.eq(db_consultation.note),
            price.eq(db_consultation.price),
            status.eq(db_consultation.status),
            updated_at.eq(chrono::Utc::now().naive_utc()),
        ))
        .get_result::<DbConsultation>(connection)
        .expect("Error saving library track");

    Ok(Consultation::from(db_consultation))
}
