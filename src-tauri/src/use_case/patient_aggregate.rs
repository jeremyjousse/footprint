use crate::{
    adapter::{
        primary::tauri_command::consultation,
        secondary::sqlite::{
            consultation::consultation_repository::find as find_consultations,
            payment::payment_repository::find as find_payments,
        },
    },
    domain::{
        aggregate::patient::PatientAggregate,
        entity::patient::Patient,
        value_object::{
            consultation_list_filter::ConsultationListFilter, listing_sort::ListingSort,
            payment_list_filter::PaymentListFilter,
        },
    },
    infrastructure::type_alias::Result,
};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    SqliteConnection,
};
use uuid::Uuid;

use crate::adapter::secondary::sqlite::patient::patient_repository::get;

pub fn patient_aggregate_use_case(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    id: Uuid,
) -> Result<Option<PatientAggregate>> {
    let result_patient_option = get(connection, id);

    match result_patient_option {
        Ok(Some(patient)) => {
            let consultation_filter = ConsultationListFilter {
                patient_id: Some(patient.id.clone().to_string()),
            };
            let sort = ListingSort::default("created_at".to_string());

            let consultations = find_consultations(connection, consultation_filter, sort.clone())?;
            let consultation_filter = PaymentListFilter {
                consultations: Some(
                    consultations
                        .clone()
                        .into_iter()
                        .map(|consultation| consultation.id.to_string())
                        .collect(),
                ),
                consultation_id: None,
            };
            let payments = find_payments(connection, consultation_filter, sort);

            let patient_aggregate = PatientAggregate {
                patient: patient.into(),
                consultations: Vec::new(),
            };
            Ok(Some(patient_aggregate))
        }
        Ok(None) => Ok(None),
        Err(err) => Err(err),
    }

    // patient.map(|patient| PatientAggregate {})
}
