use rand::Rng;
use color_eyre::eyre::{eyre, Context, Report, Result};

#[derive(Clone, Debug, PartialEq)]
pub struct TwoFACode(String);

impl TwoFACode {
    pub fn parse(code: String) -> Result<Self> {
        let code_as_u32 = code.parse::<u32>().wrap_err("Invalid 2FA code")?;

        if (100_000..=999_999).contains(&code_as_u32) {
            Ok(Self(code))
        } else {
           Err(eyre!("Invalid 2FA code"))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_fa_code_parse() {
        let fa_code = "999999".to_string();

        let result = TwoFACode::parse(fa_code).is_ok();
        assert_eq!(result, true)
    }
}
