use sqlx::FromRow;

use super::{Email, Password};

#[derive(Clone, Debug, PartialEq, FromRow)]
pub struct User {
    #[sqlx(flatten)]
    pub email: Email,
    #[sqlx(flatten, rename = "password_hash")]
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

