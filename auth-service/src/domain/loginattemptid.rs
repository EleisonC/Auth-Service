use uuid::{Uuid};
use color_eyre::eyre::{Context, Result};
use secrecy::{ExposeSecret, Secret};

#[derive(Debug, Clone)]
pub struct LoginAttemptId(Secret<String>);

impl PartialEq for LoginAttemptId {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}
impl LoginAttemptId {
    pub fn parse(id: Secret<String>) -> Result<LoginAttemptId> {
        let parsed_id = Uuid::parse_str(&id.expose_secret()).wrap_err("invalid login attempt id")?;
        Ok(Self(id))
    }
}

impl Default for LoginAttemptId {
    fn default() -> Self {
        let id = Uuid::new_v4();
        Self(Secret::new(id.to_string()))
    }
}

impl AsRef<Secret<String>> for LoginAttemptId {
    fn as_ref(&self) -> &Secret<String> {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login_attempt_id_parse() {
        let attempt_id = "ebfabdee-d0ca-416d-b4de-a0b01f5b2ec5".to_string();

        let result = LoginAttemptId::parse(Secret::new(attempt_id.clone())).is_ok();
        assert_eq!(result, true)
    }   
}
