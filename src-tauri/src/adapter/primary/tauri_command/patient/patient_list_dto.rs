use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::domain::entity::patient::Patient;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PatientListDto {
    pub created_at: DateTime<Utc>,
    pub first_name: String,
    pub id: String,
    pub last_name: String,
}

impl From<Patient> for PatientListDto {
    fn from(patient: Patient) -> Self {
        Self {
            created_at: patient.created_at,
            first_name: patient.personal_name.first_name,
            id: patient.id.to_string(),
            last_name: patient.personal_name.last_name,
        }
    }
}
