use rand::Rng;
use color_eyre::eyre::{eyre, Context, Report, Result};
use secrecy::{ExposeSecret, Secret};
#[derive(Clone, Debug)]
pub struct TwoFACode(Secret<String>);

impl PartialEq for TwoFACode {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}

impl TwoFACode {
    pub fn parse(code: Secret<String>) -> Result<TwoFACode> {
        let code_as_u32 = code.expose_secret().parse::<u32>().wrap_err("Invalid 2FA code")?;

        if (100_000..=999_999).contains(&code_as_u32) {
            Ok(Self(code))
        } else {
           Err(eyre!("Invalid 2FA code"))
        }
    }
}

impl Default for TwoFACode {
    fn default () -> TwoFACode {
        let mut rng = rand::thread_rng();

        let two_fa = rng.gen_range(100000..1000000);
        Self(Secret::new(two_fa.to_string()))
    }
}

impl AsRef<Secret<String>> for TwoFACode {
    fn as_ref(&self) -> &Secret<String> {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_fa_code_parse() {
        let fa_code = Secret::new("999999".to_string());

        let result = TwoFACode::parse(fa_code).is_ok();
        assert_eq!(result, true)
    }
}
