use celes::Country;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PostalAddress {
    pub city: Option<String>,
    pub country: Option<Country>,
    pub postal_code: Option<String>,
    pub street: Option<String>,
}
