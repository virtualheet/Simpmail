pub mod client;
pub mod models;
pub mod error;
pub mod domains;

pub use client::Sempmail;
pub use models::{Message, Attachment, Domain};
pub use error::SempmailError;

pub type Result<T> = std::result::Result<T, SempmailError>;