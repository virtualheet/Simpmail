//! # rizzMail
//!
//! A temporary email management library for Rust beginners

use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;
use reqwest::Client;
use std::fs;

pub mod error;
pub mod types;
pub use error::{Result, RizzMailError};
pub use types::{EmailKeyword, Message, Attachment};

// client struct
pub struct RizzMail {
    genrated_emails: HashMap<String, String>,
    client: Client,  // http client
    base_url: String,  //api base url
}

// main implementation
impl RizzMail {
    pub fn new() -> Self {
        RizzMail {
            genrated_emails: HashMap::new(),
            client: Client::new(),
            base_url: "https://api.example-temp-mail.com".to_string(),
        }
    }

    // with random genrate
    pub async fn generate_email(&mut self, keyword: EmailKeyword) -> Result<String> {
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
    pub async fn genrate_custom_email(&mut self, prefix: &str) -> Result<String> {
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

    pub async fn is_mail_genrated(&self, email: &str) -> bool {
        self.genrated_emails.contains_key(email)
    }

    pub async  fn list_genrated_emails(&self) -> Vec<String> {
      self.genrated_emails.keys().cloned().collect()
    }

    pub async fn get_messages(&self, email: &str) -> Result<Vec<Message>> {
      if !self.genrated_emails.contains_key(email) {
        return Err(RizzMailError::InvalidEmail(
          "Email not found".to_string(),
        ));
      }

      Ok(vec![])
    }

    pub async fn download_attachement(&self, attachment_id: &str) -> Result<Vec<u8>> {
      // let url = format!("{}/attachments/{}", self.base_url, attachment_id);
      // let response = self.client.get(url).send().await?;
      // let bytes = response.bytes().await?;
      // Ok(bytes.to_vec())

      Ok(b"dummy file content".to_vec())
    }

    pub async fn save_attachement(&self, attachment: &Attachment, directory: &str) -> Result<()> {
      let data = self.download_attachement(&attachment.id).await?;
      let path = format!("{}/{}", directory, attachment.filename);
      fs::write(&path, data)
            .map_err(|e| RizzMailError::GenerationFailed(format!("Failed to save: {}", e)))?;
        
      Ok(())
    }


    pub async fn get_unread_messages(&self, email: &str) -> Result<Vec<Message>> {
     let all_messages = self.get_messages(email).await?;

      Ok(all_messages)
    }

    pub async fn delete_email(&mut self, email: &str) -> Result<()> {
      self.genrated_emails.remove(email);
      Ok(())
    }


}

#[cfg(test)]
mod tests {
  use super::*;

#[tokio::test]
async fn can_generate_email() {
    let mut client = RizzMail::new();
    let email = client.generate_email(EmailKeyword::Love).await.unwrap();

    assert!(email.contains("@example.com"));
}

#[tokio::test]
async fn can_grenrate_custom_mail () {
  let mut client = RizzMail::new();
  let email =  client.genrate_custom_email("custom_prefix").await.unwrap();
  assert!(email.contains("@example.com"));

}


#[tokio::test]
async fn can_get_messages() {
    let mut client = RizzMail::new();
    let email: String = client.generate_email(EmailKeyword::Love).await.unwrap();
    
    let messages = client.get_messages(&email).await.unwrap();
    assert_eq!(messages.len(), 0);  // Should be empty for now
}

#[tokio::test]
async fn can_delete_email() {
  let mut client = RizzMail::new();
  let email: String = client.generate_email(EmailKeyword::Love).await.unwrap();
  client.delete_email(&email).await.unwrap();
  assert!(!client.is_mail_genrated(&email).await);
}









}