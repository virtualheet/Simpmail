//! # SimpMail
//! 
//! A temporary email management library for Rust beginners

pub mod types;

pub use types::EmailKeyword;


// client struct
pub struct SimpMail {

}

// main implementation
impl SimpMail {
    pub fn new() -> Self {
        SimpMail {
            
        }
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