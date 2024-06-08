use uuid::{Uuid};

#[derive(Debug, Clone, PartialEq)]
pub struct LoginAttemptId(String);

impl LoginAttemptId {
    pub fn parse(id: String) -> Result<Self, String> {
       if let Ok(id) = Uuid::parse_str(&id) {
            Ok(Self(id.to_string()))
        } else {
            return Err(format!("Error during parse"))
        }
    }
}

impl Default for LoginAttemptId {
    fn default() -> Self {
        let id = Uuid::new_v4();
        Self(id.to_string())
    }
}

impl AsRef<str> for LoginAttemptId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login_attempt_id_parse() {
        let attempt_id = "ebfabdee-d0ca-416d-b4de-a0b01f5b2ec5".to_string();

        let result = LoginAttemptId::parse(attempt_id.clone()).is_ok();
        assert_eq!(result, true)
    }   
}
