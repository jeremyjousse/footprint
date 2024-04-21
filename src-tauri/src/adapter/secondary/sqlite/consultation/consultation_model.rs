use chrono::NaiveDateTime;
use diesel::{associations::Identifiable, deserialize::Queryable, prelude::Insertable, Selectable};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

use crate::domain::{
    entity::consultation::Consultation,
    value_object::{
        consultation_location::ConsultationLocation, consultation_status::ConsultationStatus,
    },
};

use super::consultation_schema::consultations;

#[derive(
    Clone, Debug, Deserialize, Identifiable, Insertable, PartialEq, Queryable, Selectable, Serialize,
)]
#[diesel(table_name = consultations)]
#[diesel(primary_key(id))]
pub struct DbConsultation {
    pub appointment_date_time: NaiveDateTime,
    pub consultation_type: String,
    pub created_at: NaiveDateTime,
    pub id: String,
    pub location: String,
    pub note: Option<String>,
    pub patient_id: String,
    pub price: f64,
    pub status: String,
    pub updated_at: NaiveDateTime,
}

impl From<Consultation> for DbConsultation {
    fn from(value: Consultation) -> DbConsultation {
        DbConsultation {
            appointment_date_time: value.appointment_date_time.naive_utc(),
            created_at: value.created_at.naive_utc(),
            id: value.id.to_string(),
            price: value.price,
            updated_at: value.updated_at.naive_utc(),
            consultation_type: value.consultation_type,
            location: value.location.to_string(),
            note: value.note,
            patient_id: value.patient_id.to_string(),
            status: value.status.to_string(),
        }
    }
}

impl From<DbConsultation> for Consultation {
    fn from(value: DbConsultation) -> Consultation {
        Consultation {
            appointment_date_time: value.appointment_date_time.and_utc(),
            created_at: value.created_at.and_utc(),
            id: Uuid::parse_str(&value.id).unwrap(),
            price: value.price,
            consultation_type: value.consultation_type,
            location: ConsultationLocation::from_str(&value.location).unwrap(),
            note: value.note,
            patient_id: Uuid::parse_str(&value.patient_id).unwrap(),
            status: ConsultationStatus::from_str(&value.status).unwrap(),
            updated_at: value.updated_at.and_utc(),
        }
    }
}
