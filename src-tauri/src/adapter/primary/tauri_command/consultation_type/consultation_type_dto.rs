use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entity::consultation_type::ConsultationType;

#[derive(Serialize, Deserialize)]
pub struct ConsultationTypeDto {
    pub id: Option<String>,
    pub name: String,
    pub price: f64,
}

impl From<ConsultationType> for ConsultationTypeDto {
    fn from(value: ConsultationType) -> Self {
        ConsultationTypeDto {
            id: Some(value.id.to_string()),
            name: value.name,
            price: value.price,
        }
    }
}

impl From<ConsultationTypeDto> for ConsultationType {
    fn from(value: ConsultationTypeDto) -> Self {
        // TODO remove
        let fake_date = None;

        ConsultationType {
            created_at: match fake_date {
                Some(create_at) => create_at,
                None => Utc::now(),
            },
            id: match value.id {
                // TODO refactor as multiple usage
                Some(uuid) => Uuid::parse_str(&uuid).unwrap_or(Uuid::new_v4()), // TODO do we really want to create a new uuid?
                None => Uuid::new_v4(),
            },
            name: value.name,
            price: value.price,
            updated_at: match fake_date {
                Some(create_at) => create_at,
                None => Utc::now(),
            },
        }
    }
}
