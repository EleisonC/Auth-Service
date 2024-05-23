#[derive(Clone, Debug, PartialEq)]
pub struct User {
    pub email: String,
    pub password: String,
    pub requires2fa: bool
}

impl User {
    pub fn new(email: String, password: String, requires2fa: bool) -> Self {
        User {
            email,
            password,
            requires2fa
        }
    }
}
