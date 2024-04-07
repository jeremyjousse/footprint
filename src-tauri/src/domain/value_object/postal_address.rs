use celes::Country;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PostalAddress {
    pub city: Option<String>, // TODO sanitize
    pub country: Option<Country>,
    pub postal_code: Option<String>, // TODO sanitize
    pub street: Option<String>,      // TODO sanitize
}
