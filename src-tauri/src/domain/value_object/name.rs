use crate::infrastructure::error::Error;
use crate::infrastructure::type_alias::Result;

// TODO https://www.howtocodeit.com/articles/ultimate-guide-rust-newtypes

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Name(String);

// #[derive(Clone, Debug, Error)]
// #[error("name cannot be empty")]
// pub struct NameEmptyError;

impl Name {
    pub fn new(raw: &str) -> Result<Self> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            Err(Error::Generic("name cannot be empty".into()))
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }
}
