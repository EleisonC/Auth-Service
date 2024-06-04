use rand::Rng;

#[derive(Clone, Debug, PartialEq)]
pub struct TwoFACode(String);

impl TwoFACode {
    pub fn parse(code: String) -> Result<Self, String> {
        if code.chars().count() <= 6 {
            return Ok(Self(code))
        } else {
            return Err(format!("Error parsing code"))
        }
    }
}

impl Default for TwoFACode {
    fn default () -> Self {
        let mut rng = rand::thread_rng();

        let two_fa = rng.gen_range(100000..1000000);
        Self(two_fa.to_string())
    }
}

impl AsRef<str> for TwoFACode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}