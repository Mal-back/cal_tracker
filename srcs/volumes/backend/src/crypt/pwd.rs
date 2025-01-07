use super::{Error, Result};
use crate::config::{self, config};
use crate::crypt::{encrypt_into_b64u, EncryptContent};

pub fn encrypt_pwd(to_enc: &EncryptContent) -> Result<String> {
    let key = &config().PWD_KEY;

    let encrypted_password = encrypt_into_b64u(&key, to_enc)?;

    Ok(format!("#01#{encrypted_password}"))
}

pub fn validate_password(to_verify: &EncryptContent, pwd_ref: &str) -> Result<()> {
    let to_check = encrypt_pwd(to_verify)?;

    if to_check != pwd_ref {
        Err(Error::PasswordNotMatching)
    } else {
        Ok(())
    }
}
