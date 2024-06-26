use sqlx::{postgres::PgRow, Error, FromRow, Row};
use validator::validate_email;
use color_eyre::eyre::{eyre, Result};
use std::hash::Hash;
use secrecy::{ExposeSecret, Secret};

#[derive(Clone, Debug)]
pub struct Email(Secret<String>);

impl Email {
    pub fn parse(email: Secret<String>) -> Result<Email> {
        if !validate_email(email.expose_secret()) {
            return Err(eyre!(format!("Invalid email address: {}", email.expose_secret())));
        } else {
            Ok(Self(email))
        }
    }
}

impl PartialEq for Email {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}

impl Eq for Email {}

impl Hash for Email {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.expose_secret().hash(state);
    }
}

impl AsRef<Secret<String>> for Email {
    fn as_ref(&self) -> &Secret<String> {
        &self.0
    }
}

// impl From<String> for Email {
//     fn from(email: String) -> Self {
//         Self(email)
//     }
// }

// impl<'r> FromRow<'r, PgRow> for Email {
//     fn from_row(row: &'r PgRow) -> Result<Self, Error> {
//         let email_text = row.try_get("email")?;
//         Ok(Email(email_text))
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use secrecy::Secret;

    #[test]
    fn test_email_paser() {
        let email = Secret::new("user@mail.com".to_string());

        let result = Email::parse(email.clone()).is_ok();
        assert_eq!(result, true)
    }

    #[test]
    fn test_invalid_email() {
        let email = Secret::new("user.mail.com".to_string());

        let result = Email::parse(email.clone()).is_err();
        assert_eq!(result, true)
    }

    #[test]
    fn test_invalid_email_spaces() {
        let email = Secret::new("     @     mail.com".to_string());

        let result = Email::parse(email.clone()).is_err();
        assert_eq!(result, true)
    }


}
