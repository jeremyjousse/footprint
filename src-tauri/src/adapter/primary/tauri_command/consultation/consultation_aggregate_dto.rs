use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    SqliteConnection,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    adapter::primary::tauri_command::payment::payment_dto::PaymentDto,
    domain::{
        aggregate::consultation::ConsultationAggregate,
        entity::consultation::Consultation,
        helper::data_time::date_time_serializer,
        value_object::{
            consultation_location::ConsultationLocation, consultation_status::ConsultationStatus,
        },
    },
};

use super::consultation_dto::ConsultationDto;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsultationAggregateDto {
    pub consultation: ConsultationDto,
    pub payments: Vec<PaymentDto>,
}

impl From<ConsultationAggregate> for ConsultationAggregateDto {
    fn from(value: ConsultationAggregate) -> Self {
        ConsultationAggregateDto {
            consultation: ConsultationDto::from(value.consultation),
            payments: value.payments.into_iter().map(PaymentDto::from).collect(),
        }
    }
}
