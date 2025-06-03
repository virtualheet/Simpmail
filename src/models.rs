use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub from: String,
    pub subject: String,
    pub body: String,
    pub timestamp: String,
    pub attachments: Vec<Attachment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub filename: String,
    pub content_type: String,
    pub size: u64,
}

#[derive(Debug, Clone)]
pub enum Domain {
    OneSecMail,
    TempMail,
    // Add more domains
}