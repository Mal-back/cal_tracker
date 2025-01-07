use super::{Error, Result};

pub fn b64u_encode(content: &str) -> String {
    base64_url::encode(content)
}

pub fn b64u_decode(content: &str) -> Result<String> {
    base64_url::decode(content)
        .ok()
        .and_then(|v| String::from_utf8(v).ok())
        .ok_or(Error::B64uDecodeFail)
}
