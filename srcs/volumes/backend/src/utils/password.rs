use crate::utils::Error;

use super::Result;

pub fn check_password_safety(pwd_clear: &str) -> Result<()> {
    if !pwd_clear.chars().any(|c| c.is_lowercase())
        || !pwd_clear.chars().any(|c| c.is_uppercase())
        || !pwd_clear.chars().any(|c| c.is_numeric())
        || !pwd_clear.chars().any(|c| !c.is_alphanumeric()) {
        Err(Error::PasswordIsUnsafe)
    } else {
        Ok(())
    }
}
