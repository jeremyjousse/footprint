use serde::Deserialize;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsultationListFilter {
    pub patient_id: Option<String>,
}
