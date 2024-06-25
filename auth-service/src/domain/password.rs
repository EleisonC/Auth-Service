use sqlx::{postgres::PgRow, Error, FromRow, Row};
use color_eyre::eyre::{eyre, Result};
use secrecy::{ExposeSecret, Secret};

#[derive(Clone, Debug)]
pub struct Password(Secret<String>);

impl PartialEq for Password {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}

impl Password {
    pub fn parse(password: Secret<String>) -> Result<Password> {
        if password.expose_secret().is_empty() || password.expose_secret().trim().to_string().capacity() < 8 {
            return Err(eyre!("Invalid password".to_string()));
        } else {
            Ok(Self(password))
        }
    }
}

impl AsRef<Secret<String>> for Password {
    fn as_ref(&self) -> &Secret<String> {
        &self.0
    }
}

impl From<Secret<String>> for Password {
    fn from(password: Secret<String>) -> Self {
        Self(password)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use secrecy::Secret;

    #[test]
    fn test_password_paser() {
        let password = Secret::new("password123".to_string());

        let result = Password::parse(password).is_ok();
        assert!(result)
    }

    #[test]
    fn test_invalid_password() {
        let password = Secret::new("passwor".to_string());

        let result = Password::parse(password.clone()).is_err();

        assert!(result);
    }
}
