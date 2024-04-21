use crate::{
    adapter::{
        primary::tauri_command::{
            consultation::consultation_dto::ConsultationDto, payment::payment_dto::PaymentDto,
        },
        secondary::sqlite::payment::payment_repository::find,
    },
    domain::{
        aggregate::consultation::ConsultationAggregate,
        value_object::{listing_sort::ListingSort, payment_list_filter::PaymentListFilter},
    },
    infrastructure::type_alias::Result,
};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    SqliteConnection,
};
use uuid::Uuid;

use crate::adapter::secondary::sqlite::consultation::consultation_repository::get;

pub fn consultation_detail_use_case(
    connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    id: Uuid,
) -> Result<Option<ConsultationAggregate>> {
    let mut consultation = get(connection, id)?;
    return match consultation {
        Some(consultation) => {
            let payment_list_filter: PaymentListFilter = PaymentListFilter {
                consultation_id: Some(id.to_string()),
                consultations: None,
            };
            let payment_list_sort: ListingSort = ListingSort {
                by: "payed_at".into(),
                order: "asc".into(),
            };
            let mut payments = find(connection, payment_list_filter, payment_list_sort)?;

            let consultation_aggregate = ConsultationAggregate {
                consultation: ConsultationDto::from(consultation),
                payments: payments.into_iter().map(PaymentDto::from).collect(),
            };
            Ok(Some(consultation_aggregate))
        }
        None => Ok(None),
    };
}
