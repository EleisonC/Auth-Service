use uuid::{Uuid};
use color_eyre::eyre::{Context, Result};

#[derive(Debug, Clone, PartialEq)]
pub struct LoginAttemptId(String);

impl LoginAttemptId {
    pub fn parse(id: String) -> Result<Self> {
        let parsed_id = Uuid::parse_str(&id).wrap_err("invalid login attempt id")?;
        Ok(Self(parsed_id.to_string()))
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
