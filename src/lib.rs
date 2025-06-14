//! # rizzMail
//!
//! A temporary email management library for Rust beginners

use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub mod error;
pub mod types;
pub use error::{Result, RizzMailError};
pub use types::EmailKeyword;

// client struct
pub struct RizzMail {
    pub genrated_emails: HashMap<String, String>,
}

// main implementation
impl RizzMail {
    pub fn new() -> Self {
        RizzMail {
            genrated_emails: HashMap::new(),
        }
    }

    // with random genrate
    pub fn generate_email(&mut self, keyword: EmailKeyword) -> Result<String> {
        let keywords = keyword.get_keywords();

        if keywords.is_empty() {
            return Err(RizzMailError::GenerationFailed(
                "No keywords found".to_string(),
            ));
        }

        let selected = keywords[rand::thread_rng().gen_range(0..keywords.len())];

        let random_num: u32 = rand::thread_rng().gen_range(1000..9999);
        let email = format!("{}{}@example.com", selected, random_num);

        let token = Uuid::new_v4().to_string();
        self.genrated_emails.insert(email.clone(), token);

        Ok(email)
    }

    // with own prefix genrate
    pub fn genrate_custom_email(&mut self, prefix: &str) -> Result<String> {
        if prefix.is_empty() {
            return Err(RizzMailError::GenerationFailed(
                "No keywords found".to_string(),
            ));
        }

        let random_num: u32 = rand::thread_rng().gen_range(1000..9999);
        let email = format!("{}{}@example.com", prefix, random_num);

        let token = Uuid::new_v4().to_string();
        self.genrated_emails.insert(email.clone(), token);

        Ok(email)
    }



    // helper methhods

    pub fn is_mail_genrated(&self, email: &str) -> bool {
        self.genrated_emails.contains_key(email)
    }

    pub fn list_genrated_emails(&self) -> Vec<String> {
      self.genrated_emails.keys().cloned().collect()
    }

    
}

#[test]
fn can_generate_email() {
    let mut client = RizzMail::new();
    let email = client.generate_email(EmailKeyword::Love).unwrap();

    assert!(email.contains("@example.com"));
}

#[test]
fn can_grenrate_custom_mail () {
  let mut client = RizzMail::new();
  let email =  client.genrate_custom_email("custom_prefix").unwrap();
  assert!(email.contains("@example.com"));

}
