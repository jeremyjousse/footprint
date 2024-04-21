use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentListFilter {
    pub consultation_id: Option<String>,
    pub consultations: Option<Vec<String>>,
}
