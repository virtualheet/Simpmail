#[derive(Debug)]

pub enum RizzMailError {
    InvalidEmail(String),
    GenerationFailed(String),
    IOError(String),
    NetworkError(String),
    AttachmentError(String),   
}

impl std::fmt::Display for RizzMailError {
     fn fmt (&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RizzMailError::InvalidEmail(e) => write!(f, "Invalid email: {}", e),
            RizzMailError::GenerationFailed(e) => write!(f, "Generation failed of email: {}", e),
            RizzMailError::IOError(e) => write!(f, "IO error: {}", e),
            RizzMailError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            RizzMailError::AttachmentError(msg) => write!(f, "Attachment error: {}", msg),
        }
     }
}

impl std::error::Error for RizzMailError {}

pub type Result<T> = std::result::Result<T, RizzMailError>;