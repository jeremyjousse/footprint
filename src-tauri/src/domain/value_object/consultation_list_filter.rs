use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsultationListFilter {
    pub patient_id: Option<String>,
}
