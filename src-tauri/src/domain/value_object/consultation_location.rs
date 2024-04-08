use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Clone, Debug, Display, Deserialize, EnumString, PartialEq, Serialize)]
#[strum(serialize_all = "kebab-case", ascii_case_insensitive)]
pub enum ConsultationLocation {
    Clinic,
    Home,
    Hospital,
}
