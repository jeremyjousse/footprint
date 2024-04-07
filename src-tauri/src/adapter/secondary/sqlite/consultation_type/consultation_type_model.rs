use chrono::NaiveDateTime;
use diesel::{associations::Identifiable, deserialize::Queryable, prelude::Insertable, Selectable};
use serde::{Deserialize, Serialize};

use crate::domain::entity::consultation_type::ConsultationType;

use super::consultation_type_schema::consultation_types::{self};

#[derive(
    Clone, Debug, Deserialize, Identifiable, Insertable, PartialEq, Queryable, Selectable, Serialize,
)]
#[diesel(table_name = consultation_types)]
#[diesel(primary_key(id))]
pub struct DbConsultationType {
    pub created_at: NaiveDateTime,
    pub id: String,
    pub name: String,
    pub price: f64,
    pub updated_at: NaiveDateTime,
}

impl From<ConsultationType> for DbConsultationType {
    fn from(value: ConsultationType) -> DbConsultationType {
        DbConsultationType {
            created_at: value.created_at.naive_utc(),
            id: value.id.to_string(),
            name: value.name,
            price: value.price,
            updated_at: value.updated_at.naive_utc(),
        }
    }
}

impl From<DbConsultationType> for ConsultationType {
    fn from(value: DbConsultationType) -> ConsultationType {
        ConsultationType {
            created_at: value.created_at.and_utc(),
            id: uuid::Uuid::parse_str(&value.id).unwrap(),
            name: value.name,
            price: value.price,
            updated_at: value.updated_at.and_utc(),
        }
    }
}
