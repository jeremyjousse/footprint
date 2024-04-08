// Have a look at Tauri's definition
// https://github.com/tauri-apps/tauri-plugin-http/blob/v2/src/error.rs

use std::sync::PoisonError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),

    #[error("ConsultationRepository {0}")]
    ConsultationRepository(String),

    #[error("PatientRepository {0}")]
    PatientRepository(String),

    // #[error("Keyring {0}")]
    // Keyring(#[from] keyring::Error),
    #[error(transparent)]
    IO(#[from] std::io::Error),

    // #[error("Document parsing error {0}")]
    // Parsing(String),
    #[error("Mutex poisoned")]
    Poison(String),
    // #[error(transparent)]
    // Network(#[from] reqwest::Error),

    // #[error(transparent)]
    // UrlParse(#[from] url::ParseError),
    #[error(transparent)]
    DieselR2d2(#[from] diesel::r2d2::PoolError),

    #[error(transparent)]
    Diesel(#[from] diesel::result::Error),
    // #[error(transparent)]
    // Json(#[from] serde_json::Error),
    #[error(transparent)]
    Uuid(#[from] uuid::Error),
    // #[error("tokio-cron-scheduler error")]
    // JobScheduler(#[from] tokio_cron_scheduler::JobSchedulerError),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl<T> From<PoisonError<T>> for Error {
    fn from(error: PoisonError<T>) -> Self {
        Error::Poison(error.to_string())
    }
}
