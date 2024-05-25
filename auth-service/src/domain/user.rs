#[derive(Clone, Debug, PartialEq)]
pub struct User {
    pub email: Email,
    pub password: Password,
    pub requires2fa: bool
}

#[derive(Eq, Hash, Clone, Debug, PartialEq)]
pub struct Password(String);
#[derive(Eq, Hash, Clone, Debug, PartialEq)]
pub struct Email(String);

impl Password {
    pub fn parse(password: String) -> Result<Password, String> {
        if password.is_empty() || password.trim().to_string().capacity() < 8 {
            return Err("Invalid password".to_string());
        } else {
            Ok(Password(password))
        }
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

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

impl User {
    pub fn new(email: Email, password: Password, requires2fa: bool) -> Self {
        User {
            email,
            password,
            requires2fa
        }
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
    fn test_password_paser() {
        let password = "password123".to_string();

        let result = Password::parse(password.clone()).expect("Should be a valid password");
        assert_eq!(result.as_ref(), password)
    }

    #[test]
    fn test_invalid_email() {
        let email = "user.mail.com".to_string();

        let result = Email::parse(email.clone());
        assert_eq!(result, Err("Invalid email address".to_string()))
    }

    #[test]
    fn test_invalid_password() {
        let password = "passwor".to_string();

        let result = Password::parse(password.clone());
        assert_eq!(result, Err("Invalid password".to_string()))
    }
}
