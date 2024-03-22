use super::error::Error;

// Result type alias with automatic infrastructure error link
pub type Result<T> = core::result::Result<T, Error>;

// Generic tuple struct for newtype pattern, mostly to implement external trait on external type
// see https://doc.rust-lang.org/rust-by-example/generics/new_types.html
pub struct W<T>(pub T);

// Aliases
pub use std::format as f;
