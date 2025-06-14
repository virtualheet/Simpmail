use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum EmailKeyword {
    Love,
    Gaming,
    Work,
}

impl EmailKeyword {
    pub fn get_keywords(&self) -> Vec<&str> {
        match self {
            EmailKeyword::Love => vec!["love", "romance", "relationship", "rizz", "dating"],
            EmailKeyword::Gaming => vec!["gaming", "game", "play", "lol", "blow"],
            EmailKeyword::Work => vec!["work", "job", "career", "job", "workout"],
        }
        
    }
}

impl std::fmt::Display for EmailKeyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            EmailKeyword::Love => "Love",
            EmailKeyword::Gaming => "Gaming", 
            EmailKeyword::Work => "Work",
        };
        write!(f, "{}", name)
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub from: String,
    pub to: String,
    pub subject: String,
    pub body: String,
    pub received_at: String,
    pub attachments: Vec<Attachment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub id: String,
    pub filename: String,
    pub content_type: String,  //"image/png"
    pub size: u64,
}
