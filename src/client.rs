use crate::{models::*, error::SempmailError, Result};
use reqwest::Client;

pub struct Sempmail {
    username: String,
    domain: Domain,
    client: Client,
}

impl Sempmail {
    pub fn new(username: &str, domain: Option<Domain>) -> Self {
        Self {
            username: username.to_string(),
            domain: domain.unwrap_or(Domain::OneSecMail),
            client: Client::new(),
        }
    }

    pub async fn get_messages(&self) -> Result<Vec<Message>> {
        // Implement API calls to temporary email services
        todo!("Implement message retrieval")
    }

    pub fn get_email_address(&self) -> String {
        format!("{}@{}", self.username, self.get_domain_string())
    }

    fn get_domain_string(&self) -> &str {
        match self.domain {
            Domain::OneSecMail => "1secmail.com",
            Domain::TempMail => "tempmail.org",
        }
    }
}