use std::fmt::Display;
use std::str::FromStr;

use tracing::debug;

use crate::config::config;
use crate::crypt::{encrypt_into_b64u, EncryptContent, Error, Result};
use crate::utils::b64::{b64u_decode, b64u_encode};
use crate::utils::time_utils::{now_utc, now_utc_plus_sec_to_str, parse_time};

#[derive(Debug)]
pub struct Token {
    pub ident: String,
    pub exp: String,
    pub sign_b64u: String,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}",
            b64u_encode(&self.ident),
            b64u_encode(&self.exp),
            self.sign_b64u
        )
    }
}

impl FromStr for Token {
    type Err = Error;

    fn from_str(token_str: &str) -> std::result::Result<Self, Self::Err> {
        let splitted: Vec<&str> = token_str.split(".").collect();

        if splitted.len() != 3 {
            return Err(Error::TokenInvalidFormat);
        }
        Ok(Token {
            ident: b64u_decode(splitted[0]).map_err(|_| Error::TokentCannotDecodeIdent)?,
            exp: b64u_decode(splitted[1]).map_err(|_| Error::TokenCannotDecodeExp)?,
            sign_b64u: splitted[2].to_string(),
        })
    }
}

pub fn generate_web_token(user: &str, salt: &str) -> Result<Token> {
    let config = &config();
    _generate_token(user, config.TOKEN_DURATION, salt, &config.TOKEN_KEY)
}

pub fn verify_web_token_signature(token: &Token, salt: &str) -> Result<()> {
    let config = &config();
    _token_verify_signature(token, salt, &config.TOKEN_KEY)?;
    Ok(())
}

fn _generate_token(ident: &str, duration_sec: f64, salt: &str, key: &[u8]) -> Result<Token> {
    let exp = now_utc_plus_sec_to_str(duration_sec);
    let ident = ident.to_string();
    let sign_b64u = _token_sign_into_b64u(&ident, &exp, salt, key)?;
    debug!(
        "{:<12} - Token generation expiration time : {:?}",
        "CRYPT MOD", exp
    );

    Ok(Token {
        ident,
        exp,
        sign_b64u,
    })
}

fn _token_sign_into_b64u(ident: &str, exp: &str, salt: &str, key: &[u8]) -> Result<String> {
    let content = format!("{}.{}", b64u_encode(ident), b64u_encode(exp));
    let signature = encrypt_into_b64u(
        key,
        &EncryptContent {
            content,
            salt: salt.to_string(),
        },
    )?;
    Ok(signature)
}

fn _token_verify_signature(origin_token: &Token, salt: &str, key: &[u8]) -> Result<()> {
    let new_sign_b64u = _token_sign_into_b64u(&origin_token.ident, &origin_token.exp, salt, key)?;

    if new_sign_b64u != origin_token.sign_b64u {
        debug!("{:<12} - Signature does not match anymore", "CRYPT MOD");
        return Err(Error::TokenSignatureNotMatching);
    }
    let origin_exp = parse_time(&origin_token.exp).map_err(|_| Error::TokenTimeNotIso)?;
    if origin_exp < now_utc() {
        debug!(
            "{:<12} - Token expire at {:?} and current time is {:?}",
            "CRYPT MOD",
            origin_exp,
            now_utc()
        );
        return Err(Error::TokenExpired);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use anyhow::Result;

    use super::*;

    #[test]
    fn test_token_display_ok() -> Result<()> {
        let fx_token_str = "ZngtaWRlbnQtMDE.MjAyNS0wMS0wMVQxMDo1NjowMFo.something-b64u";
        let fx_token = Token {
            ident: "fx-ident-01".to_string(),
            exp: "2025-01-01T10:56:00Z".to_string(),
            sign_b64u: "something-b64u".to_string(),
        };
        //println!("--> {fx_token}");
        assert_eq!(fx_token.to_string(), fx_token_str);
        Ok(())
    }

    #[test]
    fn test_parse_from_str_ok() -> Result<()> {
        let fx_token_str = "ZngtaWRlbnQtMDE.MjAyNS0wMS0wMVQxMDo1NjowMFo.something-b64u";
        let fx_token = Token {
            ident: "fx-ident-01".to_string(),
            exp: "2025-01-01T10:56:00Z".to_string(),
            sign_b64u: "something-b64u".to_string(),
        };
        //println!("--> {fx_token:?}");
        let decoded_token: Token = fx_token_str.parse()?;
        assert_eq!(format!("{decoded_token:?}"), format!("{fx_token:?}"));
        Ok(())
    }

    #[test]
    fn validate_web_token_ok() -> Result<()> {
        let fx_user = "user_one";
        let fx_salt = "pepper";
        let fx_duration = 0.02;
        let token_key = &config().TOKEN_KEY;

        let fx_token = _generate_token(&fx_user, fx_duration, &fx_salt, &token_key)?;

        thread::sleep(Duration::from_millis(10));

        let res = verify_web_token_signature(&fx_token, fx_salt);

        res?;

        Ok(())
    }

    #[test]
    fn validate_web_token_err_expired() -> Result<()> {
        let fx_user = "user_one";
        let fx_salt = "pepper";
        let fx_duration = 0.02;
        let token_key = &config().TOKEN_KEY;

        let fx_token = _generate_token(&fx_user, fx_duration, &fx_salt, &token_key)?;

        thread::sleep(Duration::from_millis(25));

        let res = verify_web_token_signature(&fx_token, fx_salt);

        assert!(
            matches!(res, Err(Error::TokenExpired),),
            "Should have matched Error::TokenExpired, but was {res:?}"
        );
        Ok(())
    }
}
