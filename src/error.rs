#[derive(Debug)]

pub enum SimpMailError {
    InvalidEmail(String),
    GenerationFailed(String),
}

impl std::fmt::Display for SimpMailError {
     fn fmt (&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimpMailError::InvalidEmail(e) => write!(f, "Invalid email: {}", e),
            SimpMailError::GenerationFailed(e) => write!(f, "Generation failed of email: {}", e),
        }
     }
}

impl std::error::Error for SimpMailError {}

pub type Result<T> = std::result::Result<T, SimpMailError>;