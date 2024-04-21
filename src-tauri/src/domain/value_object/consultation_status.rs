use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};

#[derive(
    Clone, Debug, Deserialize, EnumString, EnumIter, Eq, PartialEq, Serialize, strum_macros::Display,
)]
pub enum ConsultationStatus {
    Cancelled,
    Done,
    Pending,
}
