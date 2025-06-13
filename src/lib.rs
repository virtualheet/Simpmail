//! # SimpMail
//! 
//! A temporary email management library for Rust beginners

use std::collections::HashMap;
use rand::Rng;
use uuid::Uuid;

pub mod types;
pub mod error;
pub use types::EmailKeyword;
pub use error::{SimpMailError, Result};

// client struct
pub struct SimpMail {
  pub genrated_emails : HashMap<String, String>
}

// main implementation
impl SimpMail {
    pub fn new() -> Self {
        SimpMail {
            genrated_emails : HashMap::new()
        }
    }  
    pub fn generate_email(&mut self,keyword: EmailKeyword) -> String {
       let keywords = keyword.get_keywords();
       let selected = keywords[rand::thread_rng().gen_range(0..keywords.len())];

       let random_num: u32 = rand::thread_rng().gen_range(1000..9999);
       let email = format!("{}{}@example.com", selected, random_num);

       let token = Uuid::new_v4().to_string();
       self.genrated_emails.insert(email.clone(), token);
      //  ok(email)
       email
      }
}

#[cfg(test)]
mod tests {
      use super::*;

      #[test]
      fn can_create_simpmail() {
        let client = SimpMail::new();
      }

      #[test]
      fn email_keyword_has_keywords() {
        let love_keywords = EmailKeyword::Love.get_keywords();
        assert!(love_keywords.contains(&"love"));
        assert_eq!(love_keywords.len(), 5);
      }
}


#[test]
fn can_generate_email() {
    let mut client = SimpMail::new();
    let email = client.generate_email(EmailKeyword::Love);
    
    assert!(email.contains("@example.com"));
    assert!(email.contains("love") || email.contains("heart") || email.contains("sweet"));
}