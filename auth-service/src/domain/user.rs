use super::{Email, Password};

#[derive(Clone, Debug, PartialEq)]
pub struct User {
    pub email: Email,
    pub password: Password,
    pub requires2fa: bool
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

