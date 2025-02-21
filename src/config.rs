use std::sync::OnceLock;
use crate::error::Result;

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env().unwrap_or_else(|ex| {
            panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")
        })
    })
}

pub struct Config {
    pub api_key: String,
    pub secret_key: String,
    pub recv_window: String,
    pub api_url: String
}

impl Config {
    fn load_from_env() -> Result<Config> {
        let api_key = "".to_string();
        let secret_key = "".to_string();
        let recv_window = "5000".to_string();
        let api_url = "https://api.bybit.com".to_string();
        Ok(Config {
            api_key,
            secret_key,
            recv_window,
            api_url
        })
    }
}
