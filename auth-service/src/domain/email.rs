#[derive(Eq, Hash, Clone, Debug, PartialEq)]
pub struct Email(String);

impl Email {
    pub fn parse(email: String) -> Result<Email, String> {
        if email.is_empty() || !email.contains("@") || email.trim().to_string().capacity() <= 1 {
            return Err("Invalid email address".to_string());
        } else {
            Ok(Email(email))
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

        let result = Email::parse(email.clone()).expect("Should be a valid password");
        assert_eq!(result.as_ref(), email)
    }

    #[test]
    fn test_invalid_email() {
        let email = "user.mail.com".to_string();

        let result = Email::parse(email.clone());
        assert_eq!(result, Err("Invalid email address".to_string()))
    }

}
