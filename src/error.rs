#[derive(Debug)]

pub enum RizzMailError {
    InvalidEmail(String),
    GenerationFailed(String),
}

impl std::fmt::Display for RizzMailError {
     fn fmt (&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RizzMailError::InvalidEmail(e) => write!(f, "Invalid email: {}", e),
            RizzMailError::GenerationFailed(e) => write!(f, "Generation failed of email: {}", e),
        }
     }
}

impl std::error::Error for RizzMailError {}

pub type Result<T> = std::result::Result<T, RizzMailError>;