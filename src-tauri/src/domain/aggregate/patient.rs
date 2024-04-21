use serde::Serialize;

use crate::{
    adapter::primary::tauri_command::patient::patient_dto::PatientDto,
    domain::entity::patient::Patient,
};

use super::consultation::ConsultationAggregate;

#[derive(Clone, Debug, Serialize)]
pub struct PatientAggregate {
    pub patient: PatientDto,
    pub consultations: Vec<ConsultationAggregate>,
}
