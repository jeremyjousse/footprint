use chrono::{NaiveDateTime, Utc};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    SqliteConnection,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    adapter::primary::tauri_command::payment::payment_dto::PaymentDto,
    domain::{
        entity::consultation::Consultation,
        helper::data_time::date_time_serializer,
        value_object::{
            consultation_location::ConsultationLocation, consultation_status::ConsultationStatus,
        },
    },
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsultationDto {
    #[serde(with = "date_time_serializer")]
    pub appointment_date_time: NaiveDateTime,
    pub consultation_type: String,
    pub id: Option<String>,
    pub location: ConsultationLocation,
    pub note: Option<String>,
    pub patient_id: String,
    // pub payments: Option<Vec<PaymentDto>>,
    pub price: f64,
    pub status: ConsultationStatus,
}

impl From<Consultation> for ConsultationDto {
    fn from(value: Consultation) -> Self {
        ConsultationDto {
            appointment_date_time: value.appointment_date_time.naive_utc(),
            id: Some(value.id.to_string()),
            price: value.price,
            consultation_type: value.consultation_type,
            location: value.location,
            note: value.note,
            patient_id: value.patient_id.to_string(),
            status: value.status,
        }
    }
}

impl From<ConsultationDto> for Consultation {
    fn from(value: ConsultationDto) -> Consultation {
        let fake_date = None;

        Consultation {
            appointment_date_time: value.appointment_date_time.and_utc(),
            created_at: match fake_date {
                Some(create_at) => create_at,
                None => Utc::now(),
            },
            consultation_type: value.consultation_type,
            id: match value.id {
                // TODO refactor as multiple usage
                Some(uuid) => Uuid::parse_str(&uuid).unwrap_or(Uuid::new_v4()), // TODO do we really want to create a new uuid?
                None => Uuid::new_v4(),
            },
            location: value.location,
            note: value.note,
            patient_id: Uuid::parse_str(&value.patient_id).unwrap(),
            price: value.price,
            status: value.status,
            updated_at: match fake_date {
                Some(create_at) => create_at,
                None => Utc::now(),
            },
        }
    }
}
