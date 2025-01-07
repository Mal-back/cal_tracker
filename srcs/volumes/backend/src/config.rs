use std::{env, str::FromStr, sync::OnceLock};

use crate::{Error, Result};

#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct Config {
    // WEB
    pub WEB_FOLDER: String,

    // DB :
    pub DB_URL: String,

    // CRYPT :
    pub PWD_KEY: Vec<u8>,
    pub TOKEN_KEY: Vec<u8>,
    pub TOKEN_DURATION: f64,
}

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env().unwrap_or_else(|e| panic!("FATAL - WHILE LOADING CONF - {e:?}"))
    })
}

impl Config {
    pub fn load_from_env() -> Result<Self> {
        Ok(Self {
            WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?,
            DB_URL: get_env("SERVICE_DB_URL")?,
            PWD_KEY: get_env_b64_as_u8("SERVICE_PWD_KEY")?,
            TOKEN_KEY: get_env_b64_as_u8("SERVICE_TOKEN_KEY")?,
            TOKEN_DURATION: get_env_parse("SERVICE_TOKEN_DURATION_SECS")?,
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_e| Error::ConfigMissingEnv(name))
}

fn get_env_b64_as_u8(name: &'static str) -> Result<Vec<u8>> {
    base64_url::decode(&get_env(name)?).map_err(|_| Error::ConfigWrongFormat(name))
}

fn get_env_parse<T: FromStr>(name: &'static str) -> Result<T> {
    let val = get_env(name)?;

    val.parse().map_err(|_| Error::ConfigWrongFormat(name))
}
