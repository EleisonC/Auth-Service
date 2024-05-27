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
