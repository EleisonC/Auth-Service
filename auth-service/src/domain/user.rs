use sqlx::FromRow;

use super::{Email, Password};

#[derive(Clone, Debug, PartialEq, FromRow)]
pub struct User {
    #[sqlx(flatten)]
    pub email: Email,
    #[sqlx(rename = "password_hash", flatten)]
    pub password: Password,
    #[sqlx(rename = "requires_2fa")]
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

