use sqlx::{postgres::PgRow, Error, FromRow, Row};
use color_eyre::eyre::{eyre, Result};
#[derive(Eq, Hash, Clone, Debug, PartialEq)]
pub struct Password(String);

impl Password {
    pub fn parse(password: String) -> Result<Password> {
        if password.is_empty() || password.trim().to_string().capacity() < 8 {
            return Err(eyre!("Invalid password".to_string()));
        } else {
            Ok(Self(password))
        }
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for Password {
    fn from(password: String) -> Self {
        Self(password)
    }
}

impl<'r> FromRow<'r, PgRow> for Password {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        let pass_hash = row.try_get("password_hash")?;
        Ok(Password(pass_hash))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_paser() {
        let password = "password123".to_string();

        let result = Password::parse(password.clone()).expect("Should be a valid password");
        assert_eq!(result.as_ref(), password)
    }

    #[test]
    fn test_invalid_password() {
        let password = "passwor".to_string();

        let result = Password::parse(password.clone()).unwrap();
        
        assert_eq!(result.as_ref(), password)
    }
}
