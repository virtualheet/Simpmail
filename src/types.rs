#[derive(Debug, Clone)]
pub enum EmailKeyword {
    Love,
    Gaming,
    Work,
}

impl EmailKeyword {
    pub fn get_keywords(&self) -> Vec<&str> {
        match self {
            EmailKeyword::Love => vec!["love", "romance", "relationship", "rizz" , "dating"],
            EmailKeyword::Gaming => vec!["gaming", "game", "play", "lol" ,"blow"],
            EmailKeyword::Work => vec!["work", "job", "career", "job", "workout"],
        }
    }
}


