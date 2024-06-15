use sqlx::{postgres::PgRow, Error, FromRow, Row};
use validator::validate_email;

#[derive(Eq, Hash, Clone, Debug, PartialEq)]
pub struct Email(String);

impl Email {
    pub fn parse(email: String) -> Result<Self, String> {
        if !validate_email(&email) {
            return Err("Invalid email address".to_string());
        } else {
            Ok(Self(email))
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for Email {
    fn from(email: String) -> Self {
        Self(email)
    }
}

impl<'r> FromRow<'r, PgRow> for Email {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        let email_text = row.try_get("email")?;
        Ok(Email(email_text))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_paser() {
        let email = "user@mail.com".to_string();

        let result = Email::parse(email.clone()).is_ok();
        assert_eq!(result, true)
    }

    #[test]
    fn test_invalid_email() {
        let email = "user.mail.com".to_string();

        let result = Email::parse(email.clone()).is_err();
        assert_eq!(result, true)
    }

    #[test]
    fn test_invalid_email_spaces() {
        let email = "     @     mail.com".to_string();

        let result = Email::parse(email.clone()).is_err();
        assert_eq!(result, true)
    }


}
